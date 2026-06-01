# T2Bucket — 项目开发指南

## 概述

T2Bucket 是一个跨平台云存储桶管理桌面应用，基于 **Tauri 2 + Vue 3 + TypeScript**，支持 Windows / macOS / Linux。已实现 **Tencent COS** 和 **AWS S3**（含中国区域），预留了 Aliyun OSS / Huawei OBS / MinIO 的扩展位。

## 技术栈

| 层 | 技术 | 版本 |
|---|------|------|
| 桌面框架 | Tauri | 2.x |
| 前端框架 | Vue 3 + TypeScript | ^3.5 / ~5.6 |
| 前端路由 | vue-router | ^4.5 |
| 构建工具 | Vite | ^6.0 |
| 包管理器 | pnpm | |
| Rust 加密 | aes-gcm + rand | 0.10 / 0.8 |
| Rust HTTP | reqwest (blocking) | 0.12 |
| Rust COS 签名 | hmac + sha1 | 0.12 / 0.10 |
| Rust S3 签名 | hmac + sha2 (SigV4) | 0.12 / 0.10 |
| 序列化 | serde + serde_json | 1.x |

## 项目结构

```
t2bucket/
├── index.html                    # 入口 HTML
├── package.json                  # 前端依赖
├── vite.config.ts                # Vite 配置（dev 端口 1420）
├── tsconfig.json / tsconfig.node.json
├── src/                          # 前端源码
│   ├── main.ts                   # Vue 入口，挂载 Router
│   ├── App.vue                   # 根组件（header + router-view + toast/dialog 注入）
│   ├── router/index.ts           # 6 条路由
│   ├── types/index.ts            # TypeScript 类型 + providerLabels 映射
│   ├── composables/useTheme.ts   # 主题切换（system/light/dark）
│   ├── components/
│   │   ├── AppIcon.vue           # SVG 图标组件（22+ 图标，Heroicons 风格）
│   │   ├── AppSkeleton.vue       # 骨架屏（list/card/text 三种）
│   │   ├── AppToast.vue          # Toast 通知组件（error/success/info）
│   │   ├── ConfirmDialog.vue     # 自定义确认/输入弹窗（替代 confirm/prompt）
│   │   └── EmptyState.vue        # 空状态展示组件
│   └── pages/
│       ├── HomePage.vue          # 连接列表（增删改）
│       ├── ConnectionForm.vue    # 新建/编辑连接（含表单校验）
│       ├── BucketBrowser.vue     # 桶列表 + 文件浏览 + 操作 + 右键菜单
│       └── ObjectPreview.vue     # 文件预览（图片/文本/二进制）
└── src-tauri/                    # Rust 后端
    ├── Cargo.toml                # Rust 依赖
    ├── tauri.conf.json           # Tauri 配置
    ├── build.rs                  # Tauri 构建脚本
    ├── capabilities/default.json # Tauri 权限
    ├── icons/                    # 应用图标（自动生成）
    └── src/
        ├── main.rs               # Rust 入口
        ├── lib.rs                # Tauri 命令注册（10 个命令）
        ├── store.rs              # 连接持久化（JSON 文件 + 加密）
        ├── crypto.rs             # AES-256-GCM 加解密模块（#[cfg(unix)] 条件编译）
        └── providers/            # 存储服务商实现
            ├── mod.rs            # StorageProvider trait 与工厂函数
            ├── cos.rs            # 腾讯云 COS（V5 签名）
            └── s3.rs             # AWS S3（SigV4 签名，含中国区域 endpoint）
```

## 路由表

| 路径 | 名称 | 组件 | 说明 |
|------|------|------|------|
| `/` | home | HomePage | 连接列表 |
| `/connection/new` | new-connection | ConnectionForm | 新建连接 |
| `/connection/:id/edit` | edit-connection | ConnectionForm | 编辑连接 |
| `/browse/:connectionId` | browse-buckets | BucketBrowser | 桶列表 |
| `/browse/:connectionId/:bucket` | browse-objects | BucketBrowser | 文件/文件夹列表（通过 query `prefix` 实现子目录） |
| `/preview/:connectionId/:bucket/:key` | preview | ObjectPreview | 文件预览 |

## Rust 命令 (Tauri Commands)

| 命令 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `list_connections` | — | `Vec<Connection>` | 列出所有连接（自动解密） |
| `add_connection` | name, secretId, secretKey, region, provider | `Result<String, String>` | 新增连接，后端校验必填 |
| `update_connection` | id, name, secretId, secretKey, region, provider | `Result<(), String>` | 更新连接，后端校验必填 |
| `delete_connection` | id | `Result<(), String>` | 删除连接 |
| `test_connection` | secretId, secretKey, region, provider | `Result<Vec<String>>` | 直接用 AK/SK 测试连接（支持 COS/S3） |
| `list_buckets` | connectionId | `Result<Vec<String>>` | 获取桶列表（支持 COS/S3） |
| `list_objects` | connectionId, bucket, prefix?, delimiter? | `Result<Value>` | 列出对象（返回 prefixes + items） |
| `get_object` | connectionId, bucket, key | `Result<Vec<u8>>` | 下载对象内容 |
| `put_object` | connectionId, bucket, key, content | `Result<(), String>` | 上传对象 |
| `delete_object` | connectionId, bucket, key | `Result<(), String>` | 删除对象 |

## 命名约定

- **前端 ↔ Tauri**：invoke 参数名使用 **camelCase**（如 `secretId`、`secretKey`），Tauri 2 默认接受驼峰命名
- **Rust 内部**：struct 字段使用 **snake_case**（如 `secret_id`、`secret_key`）
- **JSON 存储**：使用 snake_case，与 Rust struct 字段名一致
- **组件 props/refs**：前端 Vue 组件内部 ref 使用 camelCase

⚠️ 不要给 `Connection` struct 加 `#[serde(rename_all = "camelCase")]`——这会导致无法读取以 snake_case 存储的 JSON 数据。

## 加密存储架构

```
用户输入 AK/SK
  ↓
ConnectionForm → invoke → Rust add_connection
  ↓
store.rs: Connection → StoredConnection { enc_secret_id, enc_secret_key }
  ↓
crypto.rs: encrypt() → AES-256-GCM(nonce + 密文) → base64
  ↓
JSON 文件: ~/.t2bucket/connections.json
```

- **密钥文件**：`~/.t2bucket/.key`（32 字节 hex 编码，首次运行时随机生成）
- **数据文件**：`~/.t2bucket/connections.json`（secret_id / secret_key 存储为 base64 密文）
- **读取流程**：JSON → base64 解码 → AES-GCM 解密 → 返回明文给前端
- **密钥与数据绑定**：删除 `.key` 会导致所有已加密数据无法解密。删除密钥时必须同时删除 `connections.json`

### 跨平台数据路径

| 平台 | 路径 |
|------|------|
| macOS | `~/.t2bucket/` |
| Linux | `~/.t2bucket/` |
| Windows | `%USERPROFILE%\.t2bucket\` |

## COS API 签名

`cos.rs` 实现了 COS V5 签名算法（HMAC-SHA1）：
1. `SignKey` = HMAC-SHA1(SecretKey, KeyTime)
2. `HttpString` = `{method}\n{path}\n{params}\n{headers}\n`
3. `StringToSign` = `sha1\n{KeyTime}\n{SHA1(HttpString)}\n`
4. `Signature` = HMAC-SHA1(SignKey, StringToSign)
5. Authorization header 组装

## S3 API 签名

`s3.rs` 实现了 AWS Signature Version 4 签名算法（HMAC-SHA256）：
1. `CanonicalRequest` = `{method}\n{path}\n{params}\n{headers}\n{signedHeaders}\n{payloadHash}`
2. `StringToSign` = `AWS4-HMAC-SHA256\n{timestamp}\n{scope}\n{SHA256(CanonicalRequest)}`
3. `SigningKey` = HMAC-SHA256(HMAC-SHA256(HMAC-SHA256(HMAC-SHA256("AWS4"+SecretKey, date), region), "s3"), "aws4_request")
4. `Signature` = HMAC-SHA256(SigningKey, StringToSign)
5. Authorization header: `AWS4-HMAC-SHA256 Credential={accessKey}/{scope}, SignedHeaders={signedHeaders}, Signature={signature}`
6. 中国区域 endpoint 使用 `amazonaws.com.cn`，其他区域使用 `amazonaws.com`

## 表单校验模式

ConnectionForm 使用三层校验：

1. **前端即时校验**：`touched` 状态追踪失焦，`isFormValid` computed 控制按钮 disabled 状态
2. **前端提交校验**：`save()` 函数开头再次检查 `isFormValid`
3. **后端校验**：Rust command 中检查必填字段，返回中文错误信息

## 主题系统

- `useTheme.ts` 管理三态：`system` / `light` / `dark`
- 默认 `system`（跟随系统 `prefers-color-scheme`）
- 手动切换后保存到 `localStorage`（key: `t2bucket-theme`）
- 切换按钮位于 App 顶栏右侧 ☀️/🌙
- CSS 变量定义在 `App.vue` 的 `:root` + `[data-theme="light"]`
- 所有组件通过 CSS 变量引用颜色，自动适配

## 常用命令

```bash
pnpm install                    # 安装前端依赖
pnpm dev                        # 仅启动 Vite 前端
pnpm build                      # 仅构建前端（vue-tsc + vite）
pnpm tauri dev                  # 启动 Tauri 开发模式（自动启动 Vite + 编译 Rust）
pnpm tauri build                # 生产构建
cd src-tauri && cargo build     # 仅编译 Rust 后端
cd src-tauri && cargo clean     # 清理 Rust 构建缓存（target/ 约 5GB）
```

## 沙箱环境注意事项

当 AI Agent 在沙箱中操作时：
- **网络请求被阻断**：`npm install` / `cargo build` 需要网络下载依赖时，使用 `pnpm install --offline` 或复制 pnpm store
- **端口绑定被阻断**：`pnpm tauri dev` 无法启动（需要监听 :1420 / :1421），需用户手动执行
- **文件删除受限制**：`rm -rf` 大目录（如 `target/`）需用 `cargo clean`
- npm registry 代理问题：检查 `HTTP_PROXY` / `ALL_PROXY` 环境变量，必要时 `unset` 后重试

## 图标生成

```bash
# 从 1024x1024 PNG 生成所有平台图标
npx tauri icon <source.png>

# 修改图标后强制重编（图标文件变化不触发 Cargo 重编）
touch src-tauri/build.rs && cd src-tauri && cargo build

# 清除 macOS 图标缓存
sudo rm -rf /Library/Caches/com.apple.iconservices.store && killall Dock
```

图标需要放在 `src-tauri/icons/`，Tauri 会自动生成 macOS (icns)、Windows (ico)、Linux (png) 及移动端图标。

## 待扩展功能

- [ ] Aliyun OSS / Huawei OBS / MinIO 后端实现
- [ ] 系统 keyring 集成（替代当前的 `.key` 文件方案）
- [ ] 文件拖拽上传
- [ ] 批量操作（批量删除/下载）
- [ ] 搜索/过滤文件

## 已完成功能（v0.1.1）

- [x] AWS S3 后端实现（SigV4 签名，含中国区域 endpoint 适配）
- [x] 连接测试按钮（支持 COS/S3）
- [x] 右键上下文菜单（上传文件、新建文件夹、复制路径、预览、下载、删除）
- [x] 返回按钮上下文感知导航（替代浏览器历史回退）
- [x] 版本号显示在顶栏
- [x] GitHub Actions CI/CD（多平台构建 + 自动发布 Draft Release）
- [x] 跨平台兼容（crypto.rs #[cfg(unix)] 条件编译）
