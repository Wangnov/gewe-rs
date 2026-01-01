-- Postgres Schema for Gewe Bot Config Management
-- Version: 1.0
-- Description: 支持配置版本管理、发布、回滚和 Prompts 存储

-- ============================================================================
-- 配置发布记录表
-- ============================================================================
CREATE TABLE IF NOT EXISTS config_releases (
    id BIGSERIAL PRIMARY KEY,
    version INTEGER NOT NULL UNIQUE,
    config_json JSONB NOT NULL,
    remark TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(255)
);

CREATE INDEX idx_config_releases_version ON config_releases(version DESC);
CREATE INDEX idx_config_releases_created_at ON config_releases(created_at DESC);

COMMENT ON TABLE config_releases IS '配置发布版本记录';
COMMENT ON COLUMN config_releases.version IS '版本号，递增';
COMMENT ON COLUMN config_releases.config_json IS '完整的 AppConfigV2 JSON';
COMMENT ON COLUMN config_releases.remark IS '版本说明';

-- ============================================================================
-- 当前配置表（草稿 + 活动配置）
-- ============================================================================
CREATE TABLE IF NOT EXISTS config_current (
    id INTEGER PRIMARY KEY DEFAULT 1 CHECK (id = 1), -- 单行表
    config_json JSONB NOT NULL,
    draft_json JSONB,
    current_version INTEGER NOT NULL DEFAULT 0,
    etag VARCHAR(64) NOT NULL,
    draft_etag VARCHAR(64),
    last_published_at TIMESTAMPTZ,
    last_saved_at TIMESTAMPTZ,
    last_reload_at TIMESTAMPTZ,
    last_reload_result TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE config_current IS '当前活动配置和草稿';
COMMENT ON COLUMN config_current.config_json IS '已发布的配置 JSON';
COMMENT ON COLUMN config_current.draft_json IS '草稿配置 JSON（未发布）';
COMMENT ON COLUMN config_current.current_version IS '当前版本号';
COMMENT ON COLUMN config_current.etag IS '已发布配置的 ETag（SHA256）';
COMMENT ON COLUMN config_current.draft_etag IS '草稿配置的 ETag';

-- 初始化单行记录
INSERT INTO config_current (config_json, etag)
VALUES ('{"config_version": 2, "bots": [], "ai_profiles": [], "tools": [], "rule_templates": [], "rule_instances": []}'::jsonb, '')
ON CONFLICT (id) DO NOTHING;

-- ============================================================================
-- Prompts 存储表（可选）
-- ============================================================================
CREATE TABLE IF NOT EXISTS prompts (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    content TEXT NOT NULL,
    size INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_prompts_name ON prompts(name);
CREATE INDEX idx_prompts_updated_at ON prompts(updated_at DESC);

COMMENT ON TABLE prompts IS 'Prompt 文件存储（可选，也可继续使用文件系统）';
COMMENT ON COLUMN prompts.name IS '文件名，例如 ai_system.txt';
COMMENT ON COLUMN prompts.content IS 'Prompt 内容';
COMMENT ON COLUMN prompts.size IS '内容字节数';

-- ============================================================================
-- 触发器：自动更新 updated_at
-- ============================================================================
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_config_current_updated_at BEFORE UPDATE ON config_current
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_prompts_updated_at BEFORE UPDATE ON prompts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
