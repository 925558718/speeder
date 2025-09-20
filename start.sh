#!/bin/bash

echo "🚀 启动内网测速工具..."

# 检查Docker是否运行
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker未运行，请先启动Docker"
    exit 1
fi

# 构建并启动服务
echo "📦 构建Docker镜像..."
docker-compose up --build -d

# 等待服务启动
echo "⏳ 等待服务启动..."
sleep 5

# 检查服务状态
if docker-compose ps | grep -q "Up"; then
    echo "✅ 服务启动成功！"
    echo "🌐 访问地址: http://localhost:8080"
    echo "📊 健康检查: http://localhost:8080/api/ping"
    echo ""
    echo "🛑 停止服务: docker-compose down"
else
    echo "❌ 服务启动失败，请检查日志: docker-compose logs"
fi
