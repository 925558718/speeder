# Speeder - 内网测速工具

一个基于Rust和Warp框架的内网测速工具，支持Docker部署。

## 功能特性

- 📥 下载速度测试 (多包大小测试)
- 📤 上传速度测试  
- 🏓 延迟测试
- ⚡ 综合测速
- 🐳 Docker部署支持
- 📊 性能分析 (小包vs大包)

## 快速开始

### 使用Docker Compose (推荐)

```bash
git clone <repository-url>
cd speeder
docker-compose up -d
```

访问 http://localhost:3030

### 使用Docker镜像

```bash
# 拉取最新镜像
docker pull ghcr.io/your-username/speeder:latest

# 运行容器
docker run -p 3030:3030 ghcr.io/your-username/speeder:latest
```

### 本地开发

```bash
# 安装依赖
cargo build

# 运行服务
cargo run
```

## 测试功能

### 下载速度测试
- **小包 (0.5MB)**: 测试50次
- **中小包 (1MB)**: 测试30次  
- **中包 (5MB)**: 测试20次
- **大包 (10MB)**: 测试15次
- **超大包 (50MB)**: 测试10次

### 性能分析
- 计算小包平均速度 (≤1MB)
- 计算大包平均速度 (≥10MB)
- 显示性能差异百分比
- 显示速度范围 (最小值-最大值)

## API接口

- `GET /api/download?size=XMB` - 下载速度测试
- `POST /api/upload` - 上传速度测试
- `GET /api/ping` - 延迟测试
- `GET /api/speedtest` - 综合测速

## 技术栈

- **后端**: Rust + Warp
- **前端**: HTML + CSS + JavaScript
- **部署**: Docker + Docker Compose
- **CI/CD**: GitHub Actions

## 开发

### 构建

```bash
cargo build --release
```

### 测试

```bash
cargo test
```

### 代码检查

```bash
cargo fmt
cargo clippy
```

## 发布

创建标签即可自动构建和发布Docker镜像：

```bash
git tag v1.0.0
git push origin v1.0.0
```

## 许可证

MIT License