use warp::Filter;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct SpeedTestResult {
    download_speed: f64,
    upload_speed: f64,
    latency: u64,
    timestamp: String,
}

#[derive(Serialize, Deserialize)]
struct TestData {
    data: Vec<u8>,
}

// 全局状态存储测试数据
type TestDataStore = Arc<Mutex<Vec<u8>>>;

#[tokio::main]
async fn main() {
    let test_data_store: TestDataStore = Arc::new(Mutex::new(vec![0u8; 1024 * 1024])); // 1MB测试数据
    
    // 静态文件服务
    let static_files = warp::path("static")
        .and(warp::fs::dir("static"));
    
    // 主页路由
    let index = warp::path::end()
        .and(warp::fs::file("static/index.html"));
    
    // 下载测速端点
    let download_test = warp::path("api")
        .and(warp::path("download"))
        .and(warp::path::end())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and(with_test_data(test_data_store.clone()))
        .and_then(handle_download_test);
    
    
    // 上传测速端点
    let upload_test = warp::path("api")
        .and(warp::path("upload"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_test_data(test_data_store.clone()))
        .and_then(handle_upload_test);
    
    // 延迟测试端点
    let ping_test = warp::path("api")
        .and(warp::path("ping"))
        .and(warp::path::end())
        .and_then(handle_ping_test);
    
    // 综合测速端点
    let speed_test = warp::path("api")
        .and(warp::path("speedtest"))
        .and(warp::path::end())
        .and(with_test_data(test_data_store.clone()))
        .and_then(handle_speed_test);
    
    let routes = index
        .or(static_files)
        .or(download_test)
        .or(upload_test)
        .or(ping_test)
        .or(speed_test)
        .with(warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type"])
            .allow_methods(vec!["GET", "POST", "OPTIONS"]));
    
    println!("内网测速服务器启动在 http://0.0.0.0:8080");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080))
        .await;
}

fn with_test_data(
    test_data: TestDataStore,
) -> impl Filter<Extract = (TestDataStore,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || test_data.clone())
}

async fn handle_download_test(
    query: std::collections::HashMap<String, String>,
    _test_data: TestDataStore,
) -> Result<impl warp::Reply, warp::Rejection> {
    // 从查询参数中获取数据大小，默认为10MB
    let data_size_mb = query
        .get("size")
        .and_then(|s| s.replace("MB", "").parse::<usize>().ok())
        .unwrap_or(10);

    // 限制最大大小，避免内存问题
    let data_size_mb = data_size_mb.min(100);

    // 生成指定大小的测试数据
    let test_data_bytes = vec![0u8; data_size_mb * 1024 * 1024];

    // 设置响应头，防止缓存
    let response = warp::reply::Response::new(test_data_bytes.into());
    let response = warp::reply::with_header(response, "Content-Type", "application/octet-stream");
    let response = warp::reply::with_header(response, "Cache-Control", "no-store, no-cache, must-revalidate");
    let response = warp::reply::with_header(response, "Pragma", "no-cache");
    let response = warp::reply::with_header(response, "Expires", "0");

    Ok(response)
}


async fn handle_upload_test(
    test_data: TestData,
    _store: TestDataStore,
) -> Result<impl warp::Reply, warp::Rejection> {
    // 接收上传的数据，计算大小
    let _data_size = test_data.data.len();
    
    // 返回简单的成功响应
    let result = SpeedTestResult {
        download_speed: 0.0,
        upload_speed: 0.0, // 速度由客户端计算
        latency: 0,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    
    Ok(warp::reply::json(&result))
}

async fn handle_ping_test() -> Result<impl warp::Reply, warp::Rejection> {
    let start = Instant::now();
    // 模拟一些处理时间
    tokio::time::sleep(Duration::from_millis(1)).await;
    let latency = start.elapsed().as_millis() as u64;
    
    let result = SpeedTestResult {
        download_speed: 0.0,
        upload_speed: 0.0,
        latency,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    
    Ok(warp::reply::json(&result))
}

async fn handle_speed_test(test_data: TestDataStore) -> Result<impl warp::Reply, warp::Rejection> {
    let test_duration = Duration::from_secs(60); // 测试60秒
    
    // 下载测试
    let download_start = Instant::now();
    let mut download_bytes = 0u64;
    while download_start.elapsed() < test_duration {
        let data = test_data.lock().await;
        let chunk_size = data.len();
        download_bytes += chunk_size as u64;
        drop(data);
        
        // 不添加延迟，让数据持续传输
    }
    let download_actual_duration = download_start.elapsed();
    let download_mb = download_bytes as f64 / (1024.0 * 1024.0);
    let download_speed = download_mb / download_actual_duration.as_secs_f64();
    
    // 上传测试（模拟）
    let upload_start = Instant::now();
    let mut upload_bytes = 0u64;
    let chunk_size = 1024 * 512; // 512KB
    while upload_start.elapsed() < test_duration {
        upload_bytes += chunk_size as u64;
        
        // 不添加延迟，让数据持续传输
    }
    let upload_actual_duration = upload_start.elapsed();
    let upload_mb = upload_bytes as f64 / (1024.0 * 1024.0);
    let upload_speed = upload_mb / upload_actual_duration.as_secs_f64();
    
    // 延迟测试
    let ping_start = Instant::now();
    tokio::time::sleep(Duration::from_millis(1)).await;
    let latency = ping_start.elapsed().as_millis() as u64;
    
    let result = SpeedTestResult {
        download_speed,
        upload_speed,
        latency,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    
    Ok(warp::reply::json(&result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::request;

    #[tokio::test]
    async fn test_ping_endpoint() {
        let filter = warp::path("api")
            .and(warp::path("ping"))
            .and(warp::get())
            .and_then(handle_ping_test);

        let response = request()
            .method("GET")
            .path("/api/ping")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_download_endpoint() {
        let test_data = Arc::new(tokio::sync::Mutex::new(vec![0u8; 1024 * 1024])); // 1MB test data
        let filter = warp::path("api")
            .and(warp::path("download"))
            .and(warp::get())
            .and(warp::query::<std::collections::HashMap<String, String>>())
            .and(with_test_data(test_data.clone()))
            .and_then(handle_download_test);

        let response = request()
            .method("GET")
            .path("/api/download?size=1MB")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_upload_endpoint() {
        let test_data = Arc::new(tokio::sync::Mutex::new(vec![0u8; 1024 * 1024]));
        let filter = warp::path("api")
            .and(warp::path("upload"))
            .and(warp::post())
            .and(warp::body::bytes())
            .map(|bytes: warp::hyper::body::Bytes| TestData { data: bytes.to_vec() })
            .and(with_test_data(test_data.clone()))
            .and_then(handle_upload_test);

        let test_bytes = vec![0u8; 1024]; // 1KB test data
        let response = request()
            .method("POST")
            .path("/api/upload")
            .body(test_bytes)
            .reply(&filter)
            .await;

        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_speedtest_endpoint() {
        let test_data = Arc::new(tokio::sync::Mutex::new(vec![0u8; 1024 * 1024]));
        let filter = warp::path("api")
            .and(warp::path("speedtest"))
            .and(warp::get())
            .and(with_test_data(test_data.clone()))
            .and_then(handle_speed_test);

        let response = request()
            .method("GET")
            .path("/api/speedtest")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), 200);
    }
}
