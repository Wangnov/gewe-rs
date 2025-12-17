#!/bin/bash

# 开发环境启动脚本
# 设置测试用的环境变量

export GEWE_BOT_TOKEN_MAIN="test_token_here"
export GEWE_WEBHOOK_SECRET_MAIN="test_secret_here"
export GEMINI_API_KEY="your_gemini_api_key"

# 启动服务
cargo run -p gewe-bot-app -- config/bot-app.v2.toml
