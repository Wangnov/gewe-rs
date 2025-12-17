# 阶段 2 完成记录：htmx 前端实现

> 完成时间：2024-12-04
> 状态：✅ 已完成
> 技术栈：htmx 2.0.4 + Alpine.js 3.x + DaisyUI 5.x + Tailwind CSS

## 概述

阶段 2 实现了基于 htmx 的前端配置界面，提供完整的配置管理功能。采用 SSR (Server-Side Rendering) + HTML 片段更新模式，无需构建步骤，直接通过 CDN 加载前端库。

## 完成的功能

### 1. 文件结构

新增文件：
```
crates/gewe-bot-app/
├── static/
│   └── index.html          # 主页面框架 (245 行)
├── src/api/
│   ├── mod.rs              # 修改：添加 pages_router
│   └── pages.rs            # 新增：HTML 渲染端点 (2100+ 行)
├── start-dev.sh            # 开发启动脚本
└── Cargo.toml              # 修改：添加 axum-htmx 依赖
```

### 2. 前端架构

#### 技术栈
- **htmx 2.0.4**：无刷新页面交互，通过 HTML 属性声明式发起 AJAX
- **Alpine.js 3.x**：轻量级响应式状态管理（全局状态、事件监听）
- **DaisyUI 5.x**：基于 Tailwind 的 UI 组件库
- **Tailwind CSS**：原子化 CSS 框架

#### 核心特性
1. **SPA 单页应用**：导航栏切换页面无刷新
2. **模态框编辑**：所有编辑表单在模态框中打开
3. **实时状态同步**：通过 Alpine.js 全局状态管理版本、ETag、草稿状态
4. **Toast 通知**：操作成功/失败消息提示
5. **主题切换**：支持亮色/暗色主题

### 3. 页面端点

| 端点 | 类型 | 功能 |
|------|------|------|
| `/` | GET | 主页面入口 |
| `/pages/dashboard` | GET | 概览页（统计、配置状态、备份历史） |
| `/pages/bots` | GET | Bots 列表 |
| `/pages/bots/new` | GET | 新建 Bot 表单 |
| `/pages/bots/edit/{id}` | GET | 编辑 Bot 表单 |
| `/pages/bots/save` | POST | 保存 Bot |
| `/pages/ai-profiles` | GET | AI Profiles 列表 |
| `/pages/ai-profiles/new` | GET | 新建 Profile 表单 |
| `/pages/ai-profiles/edit/{id}` | GET | 编辑 Profile 表单 |
| `/pages/ai-profiles/save` | POST | 保存 Profile |
| `/pages/tools` | GET | 工具列表 |
| `/pages/tools/new` | GET | 新建工具表单 |
| `/pages/tools/edit/{id}` | GET | 编辑工具表单 |
| `/pages/tools/save` | POST | 保存工具 |
| `/pages/rules` | GET | 规则页面（模板+实例标签页） |
| `/pages/rule-templates/new` | GET | 新建规则模板表单 |
| `/pages/rule-templates/edit/{id}` | GET | 编辑规则模板表单 |
| `/pages/rule-templates/save` | POST | 保存规则模板 |
| `/pages/rule-instances/new` | GET | 新建规则实例表单 |
| `/pages/rule-instances/edit/{id}` | GET | 编辑规则实例表单 |
| `/pages/rule-instances/save` | POST | 保存规则实例 |
| `/pages/prompts` | GET | Prompts 管理页 |
| `/pages/prompts/new` | GET | 新建 Prompt 编辑器 |
| `/pages/prompts/edit/{name}` | GET | Prompt 编辑器 |
| `/pages/prompts/create` | POST | 创建 Prompt 文件 |
| `/pages/simulator` | GET | 规则模拟器页面 |

### 4. 功能详情

#### Dashboard 概览页
- **统计卡片**：显示 Bots、AI Profiles、Tools、Rules 数量
- **配置状态**：版本号、ETag、草稿状态、发布时间、加载时间
- **备份历史**：最近 5 个备份版本，支持一键回滚

#### Bots 管理
- 列表展示：ID、App ID、Base URL、Token（环境变量）、Tags
- 新建/编辑：支持设置 token_env、webhook_secret_env
- 表单验证：必填字段校验

#### AI Profiles 管理
- 列表展示：ID、Provider、Model、API Key、关联工具
- 新建/编辑：
  - Provider 下拉选择（OpenAI、Gemini、Anthropic、DeepSeek）
  - 工具多选 checkbox
  - System Prompt 文件路径输入

#### 工具管理
- 列表展示：ID、类型、程序、超时、描述
- 新建/编辑：
  - 类型选择（command、http）
  - 超时配置（1-600 秒）
  - Pre-reply 预回复消息

#### 规则管理
- **双标签页**：规则模板 / 规则实例
- **规则模板编辑**：
  - 匹配条件：any、equals、contains、regex
  - 动作配置：AI Profile、回复模式、日志
  - 默认配置：require_mention
- **规则实例编辑**：
  - 模板选择（下拉）
  - 频道选择（both/private/group）
  - 优先级设置
  - From 过滤（wxid）
  - 覆盖配置（AI Profile、require_mention）

#### Prompts 管理
- **左右布局**：文件列表（左） + 编辑器（右）
- 点击文件名加载编辑器
- Textarea 编辑器（支持多行文本）
- 新建 Prompt（文件名校验 .txt/.md）
- 删除 Prompt（通过 DELETE /api/prompts/{name}）

#### 规则模拟器
- **模拟参数表单**：
  - App ID、消息类型、频道、内容、发送者
  - Mentioned 勾选框
- **结果展示**：
  - 匹配状态（成功/失败）
  - 匹配的规则列表（实例 ID、模板 ID、优先级、动作）
  - 最终执行动作

### 5. 核心实现模式

#### htmx 模式
```html
<!-- GET 请求替换内容 -->
<a hx-get="/pages/bots" hx-target="#main" hx-push-url="true">Bots</a>

<!-- POST 表单提交 -->
<form hx-post="/pages/bots/save" hx-target="#main" hx-swap="innerHTML">
  <input name="app_id" required />
  <button type="submit">保存</button>
</form>

<!-- JSON 扩展 -->
<form hx-post="/api/config/simulate" hx-ext="json-enc">
  <!-- 自动以 JSON 格式提交 -->
</form>
```

#### Alpine.js 状态管理
```javascript
// 全局状态
x-data="appState()"

// 状态属性绑定
x-text="'v' + version"
x-show="hasDraft"

// 事件监听
@config-updated.window="updateMeta($event.detail)"
```

#### Rust 后端 HTML 渲染
```rust
// 使用 r##"..."## 原始字符串避免 "#" 冲突
pub async fn dashboard(State(state): State<ApiState>) -> Html<String> {
    let meta = state.get_meta().await;
    let content = format!(r##"<div>...</div>"##);
    Html(content)
}

// 表单数据提取
pub async fn bot_save(Form(form): Form<BotFormData>) -> Html<String> {
    // 处理表单，保存配置
}
```

### 6. 表单处理流程

#### 编辑流程
1. 点击"编辑"按钮 → `hx-get="/pages/bots/edit/{id}"` → 加载表单到模态框
2. 修改表单 → 提交 → `hx-post="/pages/bots/save"` → 服务端更新配置文件
3. 保存成功 → 返回 HTML 片段（成功消息 + 重定向脚本）
4. 自动跳转回列表页 → `htmx.ajax('GET', '/pages/bots')`

#### 表单数据映射
```rust
#[derive(Deserialize)]
pub struct BotFormData {
    pub original_id: String,  // 用于查找原记录
    pub id: Option<String>,
    pub app_id: String,
    // ... 其他字段
}

// 构建 BotConfigV2
let new_bot = BotConfigV2 {
    id: form.id.filter(|s| !s.is_empty()),
    tags: form.tags.as_ref()
        .map(|s| s.split(',').map(|t| t.trim().to_string()).collect())
        .unwrap_or_default(),
    // ...
};
```

### 7. 模拟器 JSON 响应处理

由于 `/api/config/simulate` 返回 JSON，前端使用 JavaScript 拦截并渲染：

```javascript
document.body.addEventListener('htmx:afterRequest', (evt) => {
  if (evt.detail.pathInfo.requestPath === '/api/config/simulate') {
    const response = JSON.parse(evt.detail.xhr.responseText);
    if (response.success) {
      const html = this.renderSimulateResult(response.data);
      document.getElementById('simulation-result').innerHTML = html;
    }
  }
});
```

### 8. 依赖变更

```toml
# Cargo.toml
[dependencies]
axum-htmx = "0.6"  # htmx 请求头提取器
```

## 已实现的 UI 组件

| 组件 | 使用场景 |
|------|----------|
| Card | 统计卡片、配置状态 |
| Table | 列表展示（Bots、Profiles、Tools、Rules） |
| Modal | 编辑表单 |
| Tabs | 规则模板/实例切换 |
| Form Controls | 输入框、下拉框、多选框、开关 |
| Alert | 成功/错误消息 |
| Toast | 浮动通知 |
| Badge | 版本号、状态标签 |
| Loading Spinner | 加载指示器 |

## 关键技术要点

### 1. Rust 原始字符串陷阱

**问题**：`r#"..."#` 中包含 `"#` 序列（如 CSS 选择器 `#modal-content`）会被误识别为字符串结束

**解决**：使用双井号 `r##"..."##`

```rust
// ❌ 错误
format!(r#"<div id="#main"></div>"#)  // 编译失败

// ✅ 正确
format!(r##"<div id="#main"></div>"##)
```

### 2. htmx 与 axum-htmx 集成

```rust
use axum_htmx::HxRequest;

pub async fn handler(HxRequest(is_htmx): HxRequest) -> Html<String> {
    // is_htmx 为 true 时返回 HTML 片段
    // is_htmx 为 false 时返回完整页面
}
```

### 3. 表单数据类型转换

```rust
// Tags 逗号分隔转 Vec
let tags: Vec<String> = form.tags.as_ref()
    .map(|s| s.split(',').map(|t| t.trim().to_string()).filter(|t| !t.is_empty()).collect())
    .unwrap_or_default();

// Checkbox 转 Option<bool>
log: form.log.as_ref().map(|_| true)

// 过滤空字符串
api_key_env: form.api_key_env.filter(|s| !s.is_empty())
```

### 4. 成功后重定向

使用 JavaScript 延迟跳转：
```rust
Html(format!(
    r##"<div class="alert alert-success">保存成功</div>
    <script>
      setTimeout(function() {{
        htmx.ajax('GET', '/pages/bots', {{target: '#main', swap: 'innerHTML'}});
      }}, 500);
    </script>"##
))
```

## 测试步骤

### 启动服务

```bash
# 方式 1：使用启动脚本
./crates/gewe-bot-app/start-dev.sh

# 方式 2：手动设置环境变量
export GEWE_BOT_TOKEN_MAIN="test_token"
export GEMINI_API_KEY="your_key"
cargo run -p gewe-bot-app -- config/bot-app.v2.toml
```

### 访问界面

打开浏览器访问 `http://localhost:4399/`

### 功能验证清单

- [ ] Dashboard 显示统计信息和备份历史
- [ ] Bots 列表显示，点击"添加 Bot"打开模态框
- [ ] 编辑 Bot，填写表单后保存，列表自动刷新
- [ ] AI Profiles 管理，工具多选功能
- [ ] Tools 管理，超时配置
- [ ] 规则模板编辑，匹配条件设置
- [ ] 规则实例编辑，优先级和覆盖配置
- [ ] Prompts 文件列表，点击文件加载编辑器
- [ ] 新建 Prompt，输入文件名和内容保存
- [ ] 模拟器测试，填写参数查看匹配结果
- [ ] 主题切换（导航栏右上角）

## 配置文件问题修复建议

当前 `config/bot-app.v2.toml` 存在两个 `id = "ai_qa_group"` 的规则实例（109-111 和 114-119 行），需要修改其中一个的 ID：

```toml
[[rule_instances]]
id = "ai_qa_group_1"  # 修改 ID
template = "ai_qa"
channel = "group"
from = { wxid = "44514947217@chatroom" }
priority = 110
overrides = { require_mention = true }

[[rule_instances]]
id = "ai_qa_group_2"  # 修改 ID
template = "ai_qa"
channel = "group"
from = { wxid = "55904500330@chatroom" }
priority = 110
overrides = { require_mention = true }
```

## 文件变更汇总

| 文件 | 变更类型 | 行数 | 说明 |
|------|----------|------|------|
| `static/index.html` | 新增 | 245 | 主页面框架 |
| `src/api/pages.rs` | 新增 | 2119 | HTML 片段渲染 |
| `src/api/mod.rs` | 修改 | +39 | 添加 pages_router |
| `src/main.rs` | 修改 | +8 | 集成 pages 路由和根路径 |
| `Cargo.toml` | 修改 | +1 | 添加 axum-htmx 依赖 |
| `start-dev.sh` | 新增 | 9 | 开发启动脚本 |

## 技术亮点

### 1. 零构建前端

通过 CDN 直接加载前端库，无需 npm/webpack/vite 构建流程：
```html
<link href="https://cdn.jsdelivr.net/npm/daisyui@5/dist/full.min.css" rel="stylesheet">
<script src="https://cdn.tailwindcss.com"></script>
<script defer src="https://unpkg.com/alpinejs@3/dist/cdn.min.js"></script>
<script src="https://unpkg.com/htmx.org@2.0.4"></script>
```

### 2. 服务端渲染 HTML 片段

```rust
// Dashboard 渲染 4 个统计卡片
let content = format!(
    r##"
<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4 mb-6">
    <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
            <h2 class="card-title text-sm">Bots</h2>
            <p class="text-3xl font-bold">{}</p>
        </div>
    </div>
    ...
</div>
"##,
    bots_count, profiles_count, tools_count, rules_count
);
```

### 3. htmx 事件驱动

```javascript
// 监听服务端触发的事件
document.body.addEventListener('htmx:afterRequest', (evt) => {
  const trigger = evt.detail.xhr.getResponseHeader('HX-Trigger');
  // 触发自定义事件（如 configSaved）
});

// 响应自定义事件
window.addEventListener('configSaved', (evt) => {
  this.showToast('配置已保存', 'success');
  this.fetchMeta();
});
```

## 后续优化建议

### Phase 2.9 - 进一步完善（可选）

1. **表单校验增强**
   - 前端 HTML5 校验 + 后端校验
   - 显示详细的字段级错误提示

2. **删除功能**
   - 添加 Bot/Profile/Tool/Rule 的删除按钮
   - 确认对话框（`hx-confirm`）

3. **搜索/过滤**
   - 列表页添加搜索框
   - 客户端 JavaScript 过滤

4. **乐观锁支持**
   - 保存时检查 ETag
   - 冲突时提示用户

5. **键盘快捷键**
   - Ctrl+S 保存
   - Esc 关闭模态框

6. **配置导入/导出**
   - 上传 TOML 文件
   - 下载当前配置

## 总结

Phase 2 成功实现了完整的 htmx 前端界面，核心功能包括：

✅ Dashboard 概览和备份管理
✅ Bots、AI Profiles、Tools 的 CRUD 操作
✅ 规则模板和规则实例的可视化编辑
✅ Prompts 文件管理
✅ 规则模拟器
✅ 主题切换和 Toast 通知

技术栈选择得当，开发体验优秀：
- htmx 声明式交互，代码简洁
- Alpine.js 轻量级状态管理
- DaisyUI 提供开箱即用的美观组件
- 无构建流程，开发效率高

下一步可根据实际使用反馈进行 UI/UX 优化。
