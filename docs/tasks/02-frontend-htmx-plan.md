# 阶段 2 开发计划：htmx 前端实现

> 状态：📋 计划中
> 技术栈：htmx 2.x + Alpine.js 3.x + Tailwind CSS + DaisyUI
> 参考文档已通过 Context7 查阅，包含具体代码示例

## 技术选型

| 技术 | 版本 | CDN | 用途 |
|------|------|-----|------|
| htmx | 2.0.4 | `unpkg.com/htmx.org@2.0.4` | 无刷新页面交互 |
| Alpine.js | 3.x | `unpkg.com/alpinejs@3` | 轻量级响应式状态 |
| DaisyUI | 5.x | `cdn.jsdelivr.net/npm/daisyui@5` | UI 组件库 |
| Tailwind CSS | CDN | `cdn.tailwindcss.com` | 样式框架 |

## 项目结构

```
crates/gewe-bot-app/
├── src/
│   └── api/
│       ├── mod.rs         # 添加 pages 路由
│       ├── config.rs      # JSON API（已完成）
│       ├── prompts.rs     # JSON API（已完成）
│       ├── state.rs       # 状态管理（已完成）
│       └── pages.rs       # 新增：HTML 页面渲染
└── static/
    ├── index.html         # SPA 入口
    └── css/
        └── app.css        # 自定义样式（可选）
```

## Rust 后端集成：axum-htmx

### 添加依赖

```toml
# Cargo.toml
[dependencies]
axum-htmx = "0.6"  # htmx 请求头提取器
```

### htmx 请求检测

```rust
use axum_htmx::HxRequest;

// 检测是否为 htmx 请求，返回不同内容
async fn handler(HxRequest(is_htmx): HxRequest) -> impl IntoResponse {
    if is_htmx {
        // 返回 HTML 片段
        Html("<div>Partial content</div>")
    } else {
        // 返回完整页面
        Html(include_str!("../../static/index.html"))
    }
}
```

### 可用的 htmx 提取器

```rust
use axum_htmx::{
    HxRequest,      // HX-Request 头（bool）
    HxTarget,       // HX-Target 头（目标元素 ID）
    HxTrigger,      // HX-Trigger 头（触发元素 ID）
    HxTriggerName,  // HX-Trigger-Name 头
    HxPrompt,       // HX-Prompt 头（用户输入）
};
```

---

## 前端核心模式

### 1. 主布局 (index.html)

```html
<!DOCTYPE html>
<html lang="zh-CN" data-theme="light">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Gewe Bot 配置管理</title>
  <!-- Tailwind + DaisyUI -->
  <link href="https://cdn.jsdelivr.net/npm/daisyui@5/dist/full.min.css" rel="stylesheet">
  <script src="https://cdn.tailwindcss.com"></script>
  <!-- Alpine.js -->
  <script defer src="https://unpkg.com/alpinejs@3/dist/cdn.min.js"></script>
  <!-- htmx -->
  <script src="https://unpkg.com/htmx.org@2.0.4"></script>
</head>
<body class="min-h-screen bg-base-200" x-data="{ currentPage: 'dashboard' }">

  <!-- 导航栏 -->
  <div class="navbar bg-base-100 shadow-sm">
    <div class="flex-1">
      <a class="btn btn-ghost text-xl">Gewe Bot</a>
    </div>
    <div class="flex-none">
      <ul class="menu menu-horizontal px-1">
        <li><a hx-get="/pages/dashboard" hx-target="#main" hx-push-url="true">概览</a></li>
        <li><a hx-get="/pages/bots" hx-target="#main" hx-push-url="true">Bots</a></li>
        <li><a hx-get="/pages/ai-profiles" hx-target="#main" hx-push-url="true">AI</a></li>
        <li><a hx-get="/pages/tools" hx-target="#main" hx-push-url="true">工具</a></li>
        <li><a hx-get="/pages/rules" hx-target="#main" hx-push-url="true">规则</a></li>
        <li><a hx-get="/pages/prompts" hx-target="#main" hx-push-url="true">Prompts</a></li>
        <li><a hx-get="/pages/simulator" hx-target="#main" hx-push-url="true">模拟器</a></li>
      </ul>
    </div>
  </div>

  <!-- 主内容区 -->
  <main id="main" class="container mx-auto p-4"
        hx-get="/pages/dashboard" hx-trigger="load">
    <!-- 初始内容由 htmx 加载 -->
    <div class="flex justify-center items-center h-64">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
  </main>

  <!-- 状态栏 -->
  <footer class="footer footer-center p-4 bg-base-100 text-base-content fixed bottom-0"
          x-data="{ etag: '', version: 0, hasChanges: false }">
    <div class="flex gap-4 items-center">
      <span class="badge badge-outline" x-text="'v' + version"></span>
      <span class="badge badge-ghost" x-text="'ETag: ' + etag.substring(0, 8) + '...'"></span>
      <button class="btn btn-sm btn-primary"
              hx-post="/api/config/save"
              hx-include="#config-form"
              :disabled="!hasChanges">
        保存
      </button>
      <button class="btn btn-sm btn-success"
              hx-post="/api/config/publish"
              hx-confirm="确定发布当前配置？">
        发布
      </button>
    </div>
  </footer>

  <!-- 模态框容器 -->
  <dialog id="modal" class="modal">
    <div class="modal-box" id="modal-content">
      <!-- 动态内容 -->
    </div>
    <form method="dialog" class="modal-backdrop">
      <button>关闭</button>
    </form>
  </dialog>

</body>
</html>
```

### 2. htmx 核心属性用法

```html
<!-- GET 请求，替换目标内容 -->
<button hx-get="/pages/bots" hx-target="#main" hx-swap="innerHTML">
  加载 Bots
</button>

<!-- POST 请求，追加到列表末尾 -->
<form hx-post="/api/bots" hx-target="#bot-list" hx-swap="beforeend">
  <input name="app_id" required />
  <button type="submit">添加</button>
</form>

<!-- DELETE 请求，删除整个元素 -->
<button hx-delete="/api/bots/123"
        hx-target="closest tr"
        hx-swap="delete"
        hx-confirm="确定删除？">
  删除
</button>

<!-- 带 JSON 参数的请求 -->
<button hx-post="/api/config/rollback"
        hx-vals='{"version": 1}'
        hx-target="#status">
  回滚到 v1
</button>
```

### 3. htmx 交换策略 (hx-swap)

| 值 | 说明 |
|----|------|
| `innerHTML` | 替换目标内部内容（默认） |
| `outerHTML` | 替换整个目标元素 |
| `beforeend` | 追加到目标内部末尾 |
| `afterend` | 追加到目标之后 |
| `delete` | 删除目标元素 |
| `none` | 不交换，仅触发事件 |

### 4. Alpine.js 状态管理

```html
<!-- 基础状态 -->
<div x-data="{ open: false }">
  <button @click="open = !open">切换</button>
  <div x-show="open" x-transition>
    下拉内容...
  </div>
</div>

<!-- 表单双向绑定 -->
<div x-data="{ name: '', model: 'gpt-4' }">
  <input type="text" x-model="name" placeholder="名称">
  <select x-model="model">
    <option value="gpt-4">GPT-4</option>
    <option value="gemini">Gemini</option>
  </select>
  <span x-text="'当前: ' + name + ' / ' + model"></span>
</div>

<!-- 复选框绑定 -->
<div x-data="{ enabled: true }">
  <input type="checkbox" x-model="enabled" class="toggle toggle-primary">
  <span x-text="enabled ? '已启用' : '已禁用'"></span>
</div>
```

### 5. DaisyUI 组件示例

#### 表格
```html
<div class="overflow-x-auto">
  <table class="table">
    <thead>
      <tr>
        <th><input type="checkbox" class="checkbox" /></th>
        <th>ID</th>
        <th>App ID</th>
        <th>操作</th>
      </tr>
    </thead>
    <tbody id="bot-list">
      <tr>
        <th><input type="checkbox" class="checkbox" /></th>
        <td>main</td>
        <td>wx_xxx</td>
        <td>
          <button class="btn btn-ghost btn-xs"
                  hx-get="/pages/bots/edit/main"
                  hx-target="#modal-content"
                  onclick="modal.showModal()">
            编辑
          </button>
        </td>
      </tr>
    </tbody>
  </table>
</div>
```

#### 卡片
```html
<div class="card bg-base-100 shadow-sm">
  <div class="card-body">
    <h2 class="card-title">配置概览</h2>
    <p>当前版本：v1</p>
    <div class="card-actions justify-end">
      <button class="btn btn-primary">发布</button>
    </div>
  </div>
</div>
```

#### 模态框
```html
<button class="btn" onclick="my_modal.showModal()">打开模态框</button>
<dialog id="my_modal" class="modal">
  <form method="dialog" class="modal-box">
    <h3 class="font-bold text-lg">编辑 Bot</h3>
    <div class="py-4">
      <input type="text" name="app_id" class="input input-bordered w-full" />
    </div>
    <div class="modal-action">
      <button class="btn">取消</button>
      <button class="btn btn-primary" hx-post="/api/bots">保存</button>
    </div>
  </form>
</dialog>
```

#### 标签页
```html
<div class="tabs tabs-box">
  <input type="radio" name="my_tabs" class="tab" aria-label="模板" checked />
  <div class="tab-content bg-base-100 p-6">
    规则模板列表...
  </div>

  <input type="radio" name="my_tabs" class="tab" aria-label="实例" />
  <div class="tab-content bg-base-100 p-6">
    规则实例列表...
  </div>
</div>
```

#### 表单控件
```html
<!-- 文本输入 -->
<label class="form-control w-full">
  <div class="label"><span class="label-text">App ID</span></div>
  <input type="text" class="input input-bordered" name="app_id" required />
</label>

<!-- 下拉选择 -->
<label class="form-control w-full">
  <div class="label"><span class="label-text">Provider</span></div>
  <select class="select select-bordered" name="provider">
    <option value="openai">OpenAI</option>
    <option value="gemini">Gemini</option>
    <option value="anthropic">Anthropic</option>
  </select>
</label>

<!-- 开关 -->
<label class="label cursor-pointer">
  <span class="label-text">启用</span>
  <input type="checkbox" class="toggle toggle-primary" name="enabled" />
</label>
```

### 6. htmx 事件处理

```html
<script>
// 请求前添加认证头
document.body.addEventListener('htmx:configRequest', (evt) => {
  evt.detail.headers['X-CSRF-Token'] = getCsrfToken();
});

// 内容交换后执行初始化
document.body.addEventListener('htmx:afterSwap', (evt) => {
  console.log('内容已更新:', evt.detail.target);
  // 初始化新加载的组件
});

// 处理验证错误（422 响应）
document.body.addEventListener('htmx:beforeSwap', (evt) => {
  if (evt.detail.xhr.status === 422) {
    evt.detail.shouldSwap = true;
    evt.detail.target = document.getElementById('errors');
  }
});

// 服务端触发的事件
document.body.addEventListener('configSaved', (evt) => {
  // 服务端通过 HX-Trigger 头触发
  alert('配置已保存: ' + evt.detail.etag);
});
</script>
```

### 7. 服务端触发事件

```rust
use axum::response::Response;
use axum::http::HeaderValue;

// 保存成功后触发前端事件
async fn save_config() -> Response {
    // ... 保存逻辑 ...

    Response::builder()
        .header("HX-Trigger", r#"{"configSaved": {"etag": "abc123"}}"#)
        .body("保存成功".into())
        .unwrap()
}
```

---

## 新增 API 端点（HTML 片段）

在 `api/pages.rs` 中实现：

| 端点 | 方法 | 功能 |
|------|------|------|
| `/pages/dashboard` | GET | 概览页面 |
| `/pages/bots` | GET | Bots 列表 |
| `/pages/bots/edit/{id}` | GET | Bot 编辑表单 |
| `/pages/ai-profiles` | GET | AI Profiles 列表 |
| `/pages/ai-profiles/edit/{id}` | GET | Profile 编辑表单 |
| `/pages/tools` | GET | Tools 列表 |
| `/pages/tools/edit/{id}` | GET | Tool 编辑表单 |
| `/pages/rules` | GET | Rules 页面（含模板和实例） |
| `/pages/rule-templates/edit/{id}` | GET | 模板编辑表单 |
| `/pages/rule-instances/edit/{id}` | GET | 实例编辑表单 |
| `/pages/prompts` | GET | Prompts 编辑页面 |
| `/pages/simulator` | GET | 模拟器页面 |

### pages.rs 示例结构

```rust
use axum::{
    extract::{Path, State},
    response::Html,
};
use axum_htmx::HxRequest;

pub async fn dashboard(
    State(state): State<ApiState>,
    HxRequest(is_htmx): HxRequest,
) -> Html<String> {
    let meta = state.get_meta().await;

    let content = format!(r#"
        <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
            <div class="card bg-base-100 shadow-sm">
                <div class="card-body">
                    <h2 class="card-title">配置状态</h2>
                    <p>版本: v{}</p>
                    <p>ETag: {}...</p>
                </div>
            </div>
            <!-- 更多卡片 -->
        </div>
    "#, meta.version, &meta.etag[..8]);

    Html(content)
}

pub async fn bots_list(State(state): State<ApiState>) -> Html<String> {
    // 读取配置，渲染 bot 列表
    // ...
}

pub async fn bot_edit_form(Path(id): Path<String>) -> Html<String> {
    // 渲染编辑表单
    // ...
}
```

---

## 实现任务清单

### Phase 2.1 - 基础框架 ⬜
- [ ] 创建 `static/index.html` 主框架
- [ ] 引入 htmx、Alpine.js、DaisyUI CDN
- [ ] 实现侧边/顶部导航
- [ ] 添加状态栏（版本、ETag）
- [ ] 添加模态框容器
- [ ] 添加 `axum-htmx` 依赖

### Phase 2.2 - Dashboard ⬜
- [ ] 创建 `api/pages.rs` 模块
- [ ] 实现 `/pages/dashboard` 端点
- [ ] 配置概览卡片（版本、状态）
- [ ] 备份列表和回滚功能

### Phase 2.3 - Bots 管理 ⬜
- [ ] 实现 `/pages/bots` 列表
- [ ] 实现 `/pages/bots/edit/{id}` 表单
- [ ] 添加/编辑/删除功能
- [ ] 表单校验提示

### Phase 2.4 - AI Profiles ⬜
- [ ] 实现 AI Profiles 列表
- [ ] 实现编辑表单
- [ ] Prompt 文件选择（下拉）
- [ ] 工具多选（checkbox）

### Phase 2.5 - Tools ⬜
- [ ] 实现 Tools 列表
- [ ] 实现编辑表单
- [ ] Parameters JSON 编辑（textarea）

### Phase 2.6 - Rules ⬜
- [ ] 实现双栏布局（标签页）
- [ ] 规则模板列表和编辑
- [ ] 规则实例列表和编辑
- [ ] 优先级显示

### Phase 2.7 - Prompts ⬜
- [ ] 文件列表
- [ ] 文本编辑器（textarea）
- [ ] 新建/保存/删除

### Phase 2.8 - Simulator ⬜
- [ ] 模拟器表单
- [ ] 调用 `/api/config/simulate`
- [ ] 显示匹配结果

### Phase 2.9 - 完善 ⬜
- [ ] 全局错误处理
- [ ] 加载状态指示（htmx indicator）
- [ ] 表单校验提示（422 处理）
- [ ] 主题切换（DaisyUI theme-controller）

---

## 关键技术要点

### 1. JSON 请求发送

htmx 默认发送 form-urlencoded，发送 JSON 需要使用扩展或 `hx-vals`：

```html
<!-- 方式1: hx-vals 直接传 JSON -->
<button hx-post="/api/config/rollback"
        hx-vals='{"version": 1}'>
  回滚
</button>

<!-- 方式2: 表单 + JSON 编码扩展 -->
<script src="https://unpkg.com/htmx.org@2.0.4/dist/ext/json-enc.js"></script>
<form hx-post="/api/config/save" hx-ext="json-enc">
  <!-- 表单会以 JSON 格式提交 -->
</form>
```

### 2. 乐观锁处理

```html
<form hx-post="/api/config/save"
      hx-vals='js:{"expected_etag": document.querySelector("[data-etag]").dataset.etag}'>
  <!-- 表单内容 -->
</form>
```

### 3. 刷新列表

```html
<!-- 保存后刷新列表 -->
<form hx-post="/api/bots"
      hx-target="#bot-list"
      hx-swap="innerHTML"
      hx-on::after-request="this.reset()">
```

### 4. 加载指示器

```html
<button hx-get="/pages/bots" hx-indicator="#spinner">
  加载
</button>
<span id="spinner" class="loading loading-spinner htmx-indicator"></span>
```

---

## 注意事项

1. **状态同步** - 每次保存后更新 ETag，防止并发覆盖
2. **安全性** - 密钥字段只显示 `_env` 后缀的环境变量名
3. **错误处理** - 422 响应需要特殊处理，显示校验错误
4. **性能** - 数据量小，无需分页或虚拟滚动
5. **主题** - DaisyUI 支持多主题，可添加切换功能
