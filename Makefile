# gewe-rs Makefile
# 使用: make <target>

.PHONY: help dev build build-release build-frontend test check clean publish publish-dry migrate fmt setup

# 默认目标：显示帮助
help:
	@echo "可用命令:"
	@echo "  make dev            - 启动开发环境"
	@echo "  make build          - 构建所有 crate (debug)"
	@echo "  make build-release  - 构建所有 crate (release)"
	@echo "  make build-frontend - 构建前端 (待实现)"
	@echo "  make test           - 运行所有测试"
	@echo "  make check          - 检查代码 (cargo check + clippy)"
	@echo "  make fmt            - 格式化代码"
	@echo "  make clean          - 清理构建产物"
	@echo "  make publish-dry    - 测试发布 (dry-run)"
	@echo "  make publish        - 发布到 crates.io"
	@echo "  make migrate        - 运行数据库迁移"
	@echo "  make setup          - 安装开发依赖"

# 启动开发环境
dev:
	./crates/gewe-bot-app/start-dev.sh

# 构建 (debug)
build:
	cargo build --workspace

# 构建 (release)
build-release:
	cargo build --workspace --release

# 构建前端 (占位，待实现)
build-frontend:
	@echo "前端尚未实现，请先在 frontend/ 目录创建前端项目"
	@echo "预期: cd frontend && npm install && npm run build"

# 运行测试
test:
	cargo test --workspace

# 代码检查
check:
	cargo check --workspace
	cargo clippy --workspace -- -D warnings

# 格式化代码
fmt:
	cargo fmt --all

# 清理
clean:
	cargo clean
	rm -rf frontend/dist
	rm -rf frontend/node_modules

# 数据库迁移
migrate:
	sqlx migrate run --source crates/gewe-bot-app/migrations

# 发布测试 (dry-run)
publish-dry:
	cargo ws publish --dry-run

# 发布到 crates.io (按依赖顺序，自动处理)
publish:
	cargo ws publish

# 安装开发依赖
setup:
	cargo binstall cargo-workspaces -y || cargo install cargo-workspaces
	cargo install sqlx-cli --no-default-features --features sqlite
	@echo "前端依赖请在 frontend/ 目录创建后安装"
