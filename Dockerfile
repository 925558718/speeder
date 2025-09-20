# 使用官方Rust镜像作为构建环境
FROM rust:1.75 as builder

# 设置工作目录
WORKDIR /app

# 复制Cargo.toml和Cargo.lock
COPY Cargo.toml Cargo.lock ./

# 创建src目录并复制源代码
RUN mkdir src
COPY src/ ./src/

# 构建应用
RUN cargo build --release

# 使用轻量级的Debian镜像作为运行环境
FROM debian:bookworm-slim

# 安装必要的运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 创建非root用户
RUN useradd -r -s /bin/false speeder

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/speeder /app/speeder

# 复制静态文件
COPY static/ ./static/

# 更改文件所有者
RUN chown -R speeder:speeder /app

# 切换到非root用户
USER speeder

# 暴露端口
EXPOSE 8080

# 设置环境变量
ENV RUST_LOG=info

# 启动应用
CMD ["./speeder"]
