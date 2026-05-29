# T2Bucket - Cloud Bucket Manager Code Wiki

## 📋 项目概述

**项目名称**: T2Bucket  
**项目类型**: 桌面应用程序 (Desktop Application)  
**版本**: 0.1.0  
**技术栈**: Tauri 2.x + Vue 3 + TypeScript + Rust  
**功能定位**: 云存储桶管理器,支持腾讯云COS等云存储服务的文件管理

### 核心特性
- ✅ 多云存储连接管理 (目前支持腾讯云COS,预留S3/OSS/OBS/MinIO接口)
- ✅ 存储桶(Bucket)浏览与管理
- ✅ 对象(Object)上传、下载、删除、预览
- ✅ 文件夹创建与导航
- ✅ 连接信息AES-256-GCM加密存储
- ✅ 暗色/亮色主题切换
- ✅ 跨平台支持 (macOS/Windows/Linux)

---

## 🏗️ 项目架构

### 整体架构图

```
┌─────────────────────────────────────────────────────────────┐
│                    T2Bucket Desktop App                      │
├─────────────────────────────────────────────────────────────┤
│  Frontend (Vue 3 + TypeScript)                              │
│  ┌───────────┬───────────┬───────────┬───────────┐         │
│  │   Pages   │Components │Composables│  Router   │         │
│  │ (Views)   │ (UI组件)  │(逻辑复用) │(路由管理) │         │
│  └─────┬─────┴─────┬─────┴─────┬─────┴─────┬─────┘         │
│        │           │           │           │               │
│        └───────────┴───────────┴───────────┘               │
│                           │                                 │
│              Tauri IPC Bridge (@tauri-apps/api)             │
├─────────────────────────────────────────────────────────────┤
│  Backend (Rust - Tauri 2.x)                                │
│  ┌───────────┬───────────┬───────────┐                     │
│  │   lib.rs  │  cos.rs   │ store.rs  │                     │
│  │(命令处理) │(COS客户端) │(数据持久化)│                    │
│  └─────┬─────┴─────┬─────┴─────┬─────┘                     │
│        │           │           │                            │
│  ┌─────┴───────────┴───────────┴─────┐                     │
│  │            crypto.rs             │                     │
│  │      (AES-256-GCM 加密模块)       │                     │
│  └───────────────────────────────────┘                     │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
              ┌────────────────────────┐
              │   Tencent Cloud COS    │
              │   (Object Storage)     │
              └────────────────────────┘
```

### 技术分层说明

| 层级 | 技术 | 职责 |
|------|------|------|
| **表现层** | Vue 3 + CSS | 用户界面渲染、交互逻辑 |
| **业务层** | TypeScript | 前端业务逻辑、状态管理 |
| **通信层** | Tauri IPC | 前后端进程间通信桥梁 |
| **核心层** | Rust | 加密、HTTP请求、数据持久化 |
| **基础设施** | Tauri Runtime | 窗口管理、系统API调用 |

---

## 📁 项目目录结构

```
t2bucket/
├── src/                          # 前端源代码 (Vue 3)
│   ├── main.ts                   # 应用入口,初始化Vue实例和路由
│   ├── App.vue                   # 根组件,包含全局布局和主题切换
│   ├── vite-env.d.ts             # Vite环境类型声明
│   │
│   ├── router/
│   │   └── index.ts              # Vue Router路由配置
│   │
│   ├── pages/                    # 页面组件
│   │   ├── HomePage.vue          # 首页 - 连接列表
│   │   ├── ConnectionForm.vue    # 连接表单 - 新建/编辑连接
│   │   ├── BucketBrowser.vue     # 存储桶浏览器 - 核心页面
│   │   └── ObjectPreview.vue     # 对象预览页 - 文件预览
│   │
│   ├── components/               # 可复用UI组件
│   │   ├── AppIcon.vue           # SVG图标组件
│   │   ├── ConfirmDialog.vue     # 确认对话框组件
│   │   ├── AppToast.vue          # Toast提示组件
│   │   ├── AppSkeleton.vue       # 骨架屏加载组件
│   │   └── EmptyState.vue        # 空状态占位组件
│   │
│   ├── composables/              # 组合式函数
│   │   └── useTheme.ts           # 主题管理逻辑
│   │
│   ├── types/                    # TypeScript类型定义
│   │   └── index.ts              # 全局接口和常量定义
│   │
│   └── assets/                   # 静态资源
│       └── vue.svg
│
├── src-tauri/                    # 后端源代码 (Rust/Tauri)
│   ├── Cargo.toml                # Rust依赖配置
│   ├── tauri.conf.json           # Tauri应用配置
│   ├── build.rs                  # 构建脚本
│   ├── capabilities/
│   │   └── default.json          # 权限配置
│   │
│   ├── src/
│   │   ├── main.rs               # Rust程序入口
│   │   ├── lib.rs                # Tauri命令注册与处理
│   │   ├── cos.rs                # COS API客户端实现
│   │   ├── crypto.rs             # AES-256-GCM加密模块
│   │   └── store.rs              # 本地数据持久化(JSON文件)
│   │
│   ├── icons/                    # 应用图标资源
│   │   ├── icon.png
│   │   ├── icon.ico
│   │   ├── icon.icns
│   │   └── android/ / ios/       # 移动端图标
│   │
│   └── target/                   # Rust编译输出
│
├── public/                       # 公共静态资源
│   ├── tauri.svg
│   └── vite.svg
│
├── dist/                         # 前端构建输出
│
├── package.json                  # Node.js依赖与脚本
├── vite.config.ts                # Vite构建配置
├── tsconfig.json                 # TypeScript编译配置
├── tsconfig.node.json            # Node.js专用TS配置
├── pnpm-lock.yaml                # pnpm锁定文件
├── pnpm-workspace.yaml           # pnpm工作区配置
├── index.html                    # HTML入口
├── README.md                     # 项目说明文档
└── .gitignore                    # Git忽略规则
```

---

## 🔧 核心模块详解

### 1. 后端核心模块 (Rust)

#### 1.1 `lib.rs` - Tauri命令处理器

**文件路径**: [lib.rs](src-tauri/src/lib.rs)  
**职责**: 注册所有Tauri IPC命令,作为前后端通信的中枢

##### 主要函数说明

| 函数名 | 类型 | 参数 | 返回值 | 功能描述 |
|--------|------|------|--------|----------|
| `list_connections` | Command | 无 | `Vec<Connection>` | 获取所有保存的连接列表 |
| `add_connection` | Command | name, secret_id, secret_key, region, provider | `Result<String>` | 新建连接,返回新连接ID |
| `update_connection` | Command | id, name, secret_id, secret_key, region, provider | `Result<()>` | 更新指定连接信息 |
| `delete_connection` | Command | id: String | `Result<()>` | 删除指定连接 |
| `list_cos_buckets` | Command | connection_id: String | `Result<Vec<String>>` | 列出指定连接的所有存储桶 |
| `list_cos_objects` | Command | connection_id, bucket, prefix?, delimiter? | `Result<serde_json::Value>` | 列出存储桶中的对象(支持分页和文件夹模拟) |
| `get_cos_object` | Command | connection_id, bucket, key | `Result<Vec<u8>>` | 下载对象内容(二进制) |
| `put_cos_object` | Command | connection_id, bucket, key, content: Vec\<u8\> | `Result<()>` | 上传对象到存储桶 |
| `delete_cos_object` | Command | connection_id, bucket, key | `Result<()>` | 删除指定对象 |

##### 关键代码片段

```rust
#[tauri::command]
fn list_cos_objects(
    connection_id: String,
    bucket: String,
    prefix: Option<String>,
    delimiter: Option<String>,
) -> Result<serde_json::Value, String> {
    let conn = store::get_connection(&connection_id).ok_or("连接不存在")?;
    let client = CosClient::new(&conn.secret_id, &conn.secret_key, &conn.region);
    let (prefixes, objects) = client.list_objects(
        &bucket,
        &prefix.unwrap_or_default(),
        &delimiter.unwrap_or_else(|| "/".to_string()),
    )?;
    
    Ok(serde_json::json!({
        "prefixes": prefixes,
        "items": objects.iter().map(|o| serde_json::json!({
            "key": o.key,
            "size": o.size,
            "lastModified": o.last_modified,
            "isDir": false,
        })).collect::<Vec<_>>(),
    }))
}
```

#### 1.2 `cos.rs` - 腾讯云COS客户端

**文件路径**: [cos.rs](src-tauri/src/cos.rs)  
**职责**: 实现腾讯云COS RESTful API调用,包括签名算法和CRUD操作

##### 核心结构体: `CosClient`

```rust
pub struct CosClient {
    pub secret_id: String,
    pub secret_key: String,
    pub region: String,
}
```

##### 方法列表

| 方法名 | 参数 | 返回值 | 说明 |
|--------|------|--------|------|
| `new(secret_id, secret_key, region)` | 凭证信息 | `CosClient` | 创建客户端实例 |
| `list_buckets()` | 无 | `Result<Vec<String>>` | 获取所有Bucket名称列表 |
| `list_objects(bucket, prefix, delimiter)` | 桶名,前缀,分隔符 | `Result<(Vec<String>, Vec<CosObject>)>` | 列出对象,返回文件夹前缀和对象列表 |
| `get_object(bucket, key)` | 桶名,对象键 | `Result<Vec<u8>>` | 下载对象二进制内容 |
| `put_object(bucket, key, content)` | 桶名,键,内容字节 | `Result<()>` | 上传对象 |
| `delete_object(bucket, key)` | 桶名,键 | `Result<()>` | 删除对象 |
| `sign(method, path, params, headers)` | HTTP方法,路径,参数,头 | `String` | 生成COS鉴权签名(私有方法) |

##### 数据结构: `CosObject`

```rust
pub struct CosObject {
    pub key: String,           // 对象键(完整路径)
    pub size: u64,             // 对象大小(字节)
    pub last_modified: String, // 最后修改时间(ISO 8601格式)
}
```

##### 签名算法实现细节

项目实现了**腾讯云COS V5签名算法**,流程如下:

1. **生成KeyTime**: 当前时间戳前后各取一定范围(如 `now-60;now+3600`)
2. **计算SignKey**: 使用HMAC-SHA1对SecretKey和KeyTime进行签名
3. **构造HttpString**: 格式为 `method\npath\nparams\nheaders`
4. **计算StringToSign**: 格式为 `sha1\nkey_time\nhashed_httpstring`
5. **生成最终签名**: 使用HMAC-SHA1对SignKey和StringToSign签名
6. **拼接Authorization头**: 包含算法、AK、时间范围、签名等信息

##### XML解析工具函数

由于COS API返回XML格式响应,项目实现了轻量级XML解析器:

- `parse_xml_list(xml, parent, child)`: 解析XML列表节点
- `parse_cos_objects(xml)`: 解析Contents节点提取对象信息
- `extract_text(s, tag)`: 提取XML标签文本内容

#### 1.3 `crypto.rs` - 加密模块

**文件路径**: [crypto.rs](src-tauri/src/crypto.rs)  
**职责**: 提供AES-256-GCM对称加密,用于保护敏感凭证(SecretId/SecretKey)

##### 核心函数

| 函数名 | 参数 | 返回值 | 说明 |
|--------|------|--------|------|
| `encrypt(plaintext)` | 明文字符串 | `String`(Base64编码) | AES-256-GCM加密 |
| `decrypt(encoded)` | Base64密文 | `String` | AES-256-GCM解密 |

##### 加密方案详情

- **算法**: AES-256-GCM (Galois/Counter Mode)
- **密钥长度**: 256位 (32字节)
- **Nonce长度**: 96位 (12字节),每次随机生成
- **密钥存储位置**: `~/.t2bucket/.key` (Unix) 或 `%USERPROFILE%\.t2bucket\.key` (Windows)
- **密钥权限**: Unix系统设置为 `0o600` (仅所有者可读写)
- **输出格式**: Base64(nonce + ciphertext)

##### 密钥管理策略

```rust
fn load_or_create_key() -> Vec<u8> {
    let path = key_path();
    if path.exists() {
        return fs::read(&path).unwrap_or_else(|_| generate_and_save_key());
    }
    generate_and_save_key()
}

fn generate_and_save_key() -> Vec<u8> {
    let mut key = vec![0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut key);  // 使用操作系统安全随机数生成器
    fs::write(&path, &key).ok();
    // Unix系统设置严格文件权限
    #[cfg(unix)]
    { fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).ok(); }
    key
}
```

#### 1.4 `store.rs` - 数据持久化层

**文件路径**: [store.rs](src-tauri/src/store.rs)  
**职责**: 管理本地JSON文件存储,负责连接信息的CRUD操作

##### 核心结构体: `Connection`

```rust
pub struct Connection {
    pub id: String,           // UUID v4
    pub name: String,         // 用户自定义名称
    pub secret_id: String,    // 加密存储
    pub secret_key: String,   // 加密存储
    pub region: String,       // 如 ap-guangzhou
    pub provider: String,     // cos/s3/oss/obs/minio
    pub created_at: i64,      // Unix时间戳
}
```

##### 序列化钩子 (Serialization Hooks)

使用Serde的自定义序列化器实现透明加密:

```rust
#[serde(serialize_with = "encrypt_serialize", deserialize_with = "decrypt_deserialize")]
pub secret_id: String,

fn encrypt_serialize<S>(val: &str, s: S) -> Result<S::Ok, S::Error> {
    let encrypted = crate::crypto::encrypt(val);  // 写入时自动加密
    s.serialize_str(&encrypted)
}

fn decrypt_deserialize<'de, D>(d: D) -> Result<String, D::Error> {
    let s: String = serde::Deserialize::deserialize(d)?;
    let decrypted = crate::crypto::decrypt(&s);   // 读取时自动解密
    // 向后兼容:如果解密失败且非Base64,视为明文(旧数据)
    Ok(decrypted)
}
```

##### 存储机制

- **存储位置**: `~/.t2bucket/connections.json`
- **存储格式**: JSON数组,每项为一个Connection对象
- **并发安全**: 采用"读-改-写"全量更新模式(适合单用户桌面应用)

---

### 2. 前端核心模块 (Vue 3 + TypeScript)

#### 2.1 路由系统 (`router/index.ts`)

**文件路径**: [router/index.ts](src/router/index.ts)

##### 路由配置表

| 路径 | 名称 | 组件 | 参数 | 用途 |
|------|------|------|------|------|
| `/` | home | `HomePage` | 无 | 连接列表首页 |
| `/connection/new` | new-connection | `ConnectionForm` | 无 | 新建连接 |
| `/connection/:id/edit` | edit-connection | `ConnectionForm` | id | 编辑连接 |
| `/browse/:connectionId` | browse-buckets | `BucketBrowser` | connectionId | 浏览存储桶列表 |
| `/browse/:connectionId/:bucket` | browse-objects | `BucketBrowser` | connectionId, bucket, query.prefix? | 浏览对象列表 |
| `/preview/:connectionId/:bucket/*` | preview | `ObjectPreview` | connectionId, bucket, key | 预览对象内容 |

##### 路由特点

- 使用HTML5 History模式 (`createWebHistory()`)
- 支持动态路由参数 (connectionId, bucket)
- 支持通配符路径 (key可能包含 `/`)
- 通过query传递可选参数 (prefix)

#### 2.2 页面组件详解

##### 2.2.1 `HomePage.vue` - 连接管理首页

**文件路径**: [HomePage.vue](src/pages/HomePage.vue)  
**功能**: 展示已保存的云存储连接列表,支持新建、编辑、删除操作

**主要状态变量**:
- `connections`: 连接数组
- `loading`: 加载状态
- `errorMsg`: 错误信息

**关键交互**:
1. 页面加载时自动调用 `invoke("list_connections")` 获取连接列表
2. 点击连接卡片跳转到存储桶浏览页
3. 悬停显示编辑/删除按钮
4. 删除操作需二次确认

**UI特性**:
- 卡片式布局,悬停动画效果
- 空状态提示引导用户新建连接
- 显示提供商标签和区域信息

##### 2.2.2 `ConnectionForm.vue` - 连接表单

**文件路径**: [ConnectionForm.vue](src/pages/ConnectionForm.vue)  
**功能**: 新建或编辑云存储连接配置

**表单字段**:

| 字段 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| provider | 选择器 | 是 | cos | 存储提供商(目前仅COS可用) |
| name | 文本输入 | 是 | - | 自定义连接名称 |
| secretId | 文本输入 | 是 | - | API密钥ID |
| secretKey | 密码输入 | 是 | - | API密钥Key(支持显示/隐藏切换) |
| region | 下拉选择 | 是 | ap-guangzhou | 地域区域 |

**验证逻辑**:
- 实时字段验证(touched状态跟踪)
- 表单提交前完整性检查
- 错误提示内联显示

**区域选项分组**:
- 中国大陆: 北京、南京、上海、广州、成都、重庆、香港
- 亚太: 新加坡、孟买、首尔、东京
- 北美: 硅谷、阿什本
- 欧洲: 法兰克福、莫斯科

##### 2.2.3 `BucketBrowser.vue` - 存储桶浏览器 ⭐

**文件路径**: [BucketBrowser.vue](src/pages/BucketBrowser.vue)  
**功能**: 核心页面,提供完整的文件管理界面

**双模式视图**:
1. **存储桶列表模式** (无bucket参数): 网格展示所有Bucket
2. **对象浏览模式** (有bucket参数): 表格展示文件/文件夹列表

**面包屑导航**:
```
[bucket] / [folder1] / [folder2] / ... / [current]
```
- 点击任意层级可快速跳转
- 支持"返回上级"操作

**工具栏功能**:
- 📤 **上传文件**: 触发文件选择器,读取ArrayBuffer并调用put接口
- 📁 **新建文件夹**: 弹出输入对话框,创建以 `/` 结尾的空对象
- 🔄 **刷新**: 重新加载当前目录列表

**文件操作**:
| 操作 | 触发方式 | 功能 |
|------|----------|------|
| 预览 | 点击眼睛图标 | 跳转到预览页(图片/文本/二进制) |
| 下载 | 点击下载图标 | 触发浏览器下载(通过Blob URL) |
| 删除 | 点击垃圾桶图标 | 二次确认后删除 |

**数据展示列**:
- 文件名(带图标区分文件/文件夹)
- 大小(智能格式化 B/KB/MB/GB/TB)
- 最后修改日期
- 操作按钮组(悬停显示)

**性能优化**:
- 使用computed缓存路径拆分结果
- watch监听路由变化自动刷新
- 骨架屏加载态提升感知速度

##### 2.2.4 `ObjectPreview.vue` - 对象预览

**文件路径**: [ObjectPreview.vue](src/pages/ObjectPreview.vue)  
**功能**: 在线预览文件内容,支持多种格式

**预览类型判断逻辑**:

```typescript
const imageExts = ["png", "jpg", "jpeg", "gif", "webp", "svg", "bmp", "ico"];
const textExts = ["txt", "md", "json", "xml", "yaml", "yml", "toml", ...];

if (imageExts.includes(ext)) {
    // 图片预览模式: 使用Blob URL渲染<img>
} else if (textExts.includes(ext) || bytes.length < 512KB) {
    // 尝试文本解码(UTF-8 strict mode)
    // 成功 → 文本预览模式
    // 失败 → 二进制模式
} else {
    // 二进制文件提示(不支持在线预览)
}
```

**三种预览模式**:

1. **图片预览**:
   - 居中显示,自适应缩放
   - 最大高度75vh防止溢出
   - 支持常见图片格式

2. **文本预览**:
   - 使用 `<pre><code>` 保持格式
   - 显示字符数和文件大小
   - 最大高度70vh可滚动
   - 使用Fira Code等宽字体
   - 自动换行和单词断行

3. **二进制预览**:
   - 显示文件图标和大小
   - 提示"不支持在线预览"
   - 引导用户下载查看

**下载功能**:
- 复用get_cos_object接口获取完整数据
- 创建临时Blob URL触发浏览器下载
- 自动提取文件名作为下载名称

#### 2.3 可复用组件

##### 2.3.1 `AppIcon.vue` - 图标系统

**文件路径**: [AppIcon.vue](src/components/AppIcon.vue)  
**类型**: 内联SVG图标库

**Props**:
- `name: string` - 图标名称(必填)
- `size?: number` - 尺寸像素(默认18)

**可用图标清单** (共20个):

| 图标名 | 用途场景 |
|--------|----------|
| plus | 新建/添加操作 |
| edit | 编辑操作 |
| trash | 删除操作 |
| folder / folder-open | 文件夹图标 |
| file | 通用文件图标 |
| refresh | 刷新/重载 |
| eye / eyeOff | 显示/隐藏密码 |
| download | 下载操作 |
| upload | 上传操作 |
| chevron-right | 导航箭头 |
| arrow-left | 返回按钮 |
| home | 主页入口 |
| bucket | 存储桶图标 |
| spinner | 加载旋转动画 |
| sun / moon | 主题切换 |
| info | 信息提示 |
| check / xmark | 成功/错误标识 |

**技术实现**:
- 使用v-html动态注入SVG path
- 继承父元素颜色(currentColor)
- stroke-width统一为1.8保证清晰度

##### 2.3.2 `ConfirmDialog.vue` - 对话框组件

**文件路径**: [ConfirmDialog.vue](src/components/ConfirmDialog.vue)  
**功能**: 模态确认/输入对话框

**暴露方法** (通过defineExpose):
- `confirm(msg: string): Promise<boolean>` - 确认对话框,返回布尔值
- `prompt(msg: string, defaultValue?: string): Promise<string \| null>` - 输入对话框,返回用户输入或null

**技术亮点**:
- 使用Teleport传送到body层级避免z-index问题
- 基于Promise的异步交互模式
- 支持键盘Enter快捷提交
- 点击遮罩层取消操作

##### 2.3.3 `AppToast.vue` - Toast通知组件

**文件路径**: [AppToast.vue](src/components/AppToast.vue)  
**功能**: 全局消息提示(错误/成功/信息)

**Props接口**:
- `show(message, type?)` - type可选: `"error"` | `"success"` | `"info"`

**行为特征**:
- 固定在右下角显示
- 4秒自动消失
- 支持手动点击关闭
- 多条消息堆叠显示(反向排列)
- 进入/退出动画(滑入+淡出)

**样式差异**:
- Error: 红色背景 (#EF4444)
- Success: 绿色背景 (#22C55E)
- Info: 蓝色背景 (#60A5FA)

##### 2.3.4 `AppSkeleton.vue` - 骨架屏组件

**文件路径**: [AppSkeleton.vue](src/components/AppSkeleton.vue)  
**功能**: 数据加载时的占位动画

**Props**:
- `lines?: number` - 行数(默认4/3/3)
- `type?: "list" \| "card" \| "text"` - 骨架屏类型

**三种变体**:
1. **list**: 列表骨架(图标+文本行)
2. **card**: 卡片骨架(矩形块网格)
3. **text**: 文本骨架(渐短文本行)

**动画效果**: pulse脉冲动画(透明度30%-60%循环)

##### 2.3.5 `EmptyState.vue` - 空状态组件

**文件路径**: [EmptyState.vue](src/components/EmptyState.vue)  
**功能**: 数据为空时的友好提示

**Props**:
- `icon: string` - 图标名称
- `title: string` - 标题文本
- `description?: string` - 描述文本

**插槽**: 支持在描述下方插入自定义操作按钮

#### 2.4 组合式函数 (Composables)

##### `useTheme.ts` - 主题管理系统

**文件路径**: [useTheme.ts](src/composables/useTheme.ts)  
**功能**: 管理应用的暗色/亮色/跟随系统主题

**状态管理**:
- 使用localStorage持久化主题选择
- 监听系统prefers-color-scheme变化
- 支持三种模式: light / dark / system

**导出API**:

```typescript
export function useTheme() {
    return {
        theme: Ref<Theme>,           // 当前主题模式
        effectiveTheme: () => "light" | "dark",  // 计算实际生效的主题
        toggleTheme: () => void,     // 切换暗色/亮色
        setTheme: (t: Theme) => void, // 设置指定主题
    };
}
```

**实现原理**:
1. 初始化时从localStorage读取存储的主题偏好
2. 如果是system模式,读取matchMedia查询系统设置
3. 通过watchEffect响应式地应用data-theme属性到<html>
4. 监听系统主题变化事件,自动更新

#### 2.5 类型定义系统 (`types/index.ts`)

**文件路径**: [types/index.ts](src/types/index.ts)

**核心接口**:

```typescript
interface CosConnection {
    id: string;
    name: string;
    secret_id: string;
    secret_key: string;
    region: string;
    provider: string;       // cos/s3/oss/obs/minio
    created_at: number;     // Unix timestamp
}

interface ObjectItem {
    key: string;
    size: number;
    lastModified: string;
    isDir: boolean;         // 是否为虚拟文件夹
}

interface ObjectListResult {
    items: ObjectItem[];
    prefixes: string[];     // 文件夹前缀列表
    isTruncated: boolean;
    nextMarker: string;
}
```

**常量定义**:

```typescript
export const providerLabels: Record<string, string> = {
    cos: "Tencent COS",
    s3: "AWS S3",
    oss: "Aliyun OSS",
    obs: "Huawei OBS",
    minio: "MinIO",
};
```

---

## 🔗 依赖关系图

### 前端依赖 (package.json)

```
@tauri-apps/api ^2
├── core (IPC invoke)
└── plugin-opener (外部链接打开)

vue ^3.5.13
└── Composition API + SFC

vue-router ^4.5.1
└── SPA路由管理

开发依赖:
├── @vitejs/plugin-vue ^5.2.1    # Vite Vue插件
├── typescript ~5.6.2             # TS编译器
├── vite ^6.0.3                  # 构建工具
├── vue-tsc ^2.1.10              # Vue类型检查
└── @tauri-apps/cli ^2            # Tauri CLI
```

### 后端依赖 (Cargo.toml)

```
tauri ^2                          # 应用框架核心
├── tauri-plugin-opener ^2       # 打开外部链接插件
│
serde ^1 + serde_json ^1          # 序列化/反序列化框架
│
reqwest ^0.12                     # HTTP客户端
├── blocking feature              # 同步请求模式
└── json feature                  # JSON处理
│
hmac ^0.12                        # HMAC-SHA1签名
sha1 ^0.10                        # SHA1哈希
hex ^0.4                          # 十六进制编解码
│
aes-gcm ^0.10                     # AES-256-GCM加密
rand ^0.8                         # 安全随机数生成
│
uuid ^1 (v4 feature)              # UUID生成
chrono ^0.4 (serde feature)       # 时间处理
urlencoding ^2                    # URL编码
base64 ^0.22                      # Base64编解码
```

### 依赖用途分类

| 类别 | 依赖 | 用途 |
|------|------|------|
| **框架运行时** | tauri, vue, vue-router | 应用基础架构 |
| **网络通信** | reqwest | HTTP请求(COS API调用) |
| **加密安全** | aes-gcm, hmac, sha1, hex, rand | 凭证加密与API签名 |
| **数据处理** | serde, serde_json, chrono, uuid, urlencoding, base64 | 数据转换与编解码 |
| **构建工具** | vite, typescript, vue-tsc, @tauri-apps/cli | 开发与打包 |

---

## 🔄 数据流与交互流程

### 典型用户操作流程示例

#### 流程1: 新建连接并浏览文件

```
用户操作                    前端处理                         后端处理
─────────                  ────────                         ────────
点击"新建连接"         →   路由跳转 /connection/new
填写表单并提交         →   invoke("add_connection", {...})
                         ↓                              ↓
                                                    store::add_connection()
                                                    ├─ 生成UUID
                                                    ├─ 序列化Connection
                                                    │   └─ crypto::encrypt()  ← AES加密secret_id/key
                                                    └─ 写入JSON文件
                         ←                              返回新ID
路由跳转首页            →   invoke("list_connections")
                         ↓                              ↓
                                                    store::list_connections()
                                                    ├─ 读取JSON文件
                                                    │   └─ crypto::decrypt()  ← AES解密
                                                    └─ 反序列化为Vec<Connection>
                         ←                              返回连接列表
点击连接卡片           →   路由跳转 /browse/{id}
                         invoke("list_cos_buckets", {id})
                         ↓                              ↓
                                                    lib::list_cos_buckets()
                                                    ├─ store::get_connection(id)
                                                    └─ CosClient::list_buckets()
                                                        ├─ sign("get", "/", "", "")
                                                        └─ HTTP GET https://service.cos.myqcloud.com/
                                                         └─ 解析XML提取<Name>
                         ←                              返回Bucket列表
点击Bucket             →   路由跳转 /browse/{id}/{bucket}
                         invoke("list_cos_objects", {...})
                         ↓                              ↓
                                                    CosClient::list_objects()
                                                        ├─ 构造URL含prefix/delimiter参数
                                                        ├─ 生成签名
                                                        └─ HTTP GET
                                                         └─ 解析XML分离CommonPrefixes和Contents
                         ←                              返回{prefixes, items}
```

#### 流程2: 文件上传

```
选择文件                 →   FileReader.readAsArrayBuffer()
                         ↓
转换为Uint8Array         →   Array.from(bytes)
                         invoke("put_cos_object", {content})
                         ↓                              ↓
                                                    CosClient::put_object()
                                                        ├─ 生成PUT请求签名
                                                        └─ HTTP PUT (body=bytes)
                         ←                              返回OK
重新加载列表             →   loadObjects()
```

#### 流程3: 文件预览

```
点击预览图标           →   路由跳转 /preview/{id}/{bucket}/{key}
                         invoke("get_cos_object", {...})
                         ↓                              ↓
                                                    CosClient::get_object()
                                                        ├─ 生成GET签名
                                                        └─ HTTP GET → Vec<u8>
                         ←                              返回字节数组
判断文件类型             →   扩展名匹配 + UTF-8解码尝试
                         ↓
渲染预览界面             ├─ 图片: Blob URL + <img>
                         ├─ 文本: <pre><code>
                         └─ 二进制: 占位提示
```

---

## 🎨 设计系统与UI规范

### 色彩体系

#### 暗色主题 (Default)
```css
--color-bg: #0F172A;              /* 主背景 - 深蓝黑 */
--color-surface: #1E293B;         /* 卡片/表面背景 */
--color-surface-hover: #273549;   /* 悬停态表面 */
--color-border: #334155;          /* 边框颜色 */
--color-primary: #60A5FA;         /* 主色调 - 亮蓝 */
--color-cta: #22C55E;             /* 行动按钮 - 绿色 */
--color-danger: #EF4444;          /* 危险操作 - 红色 */
--color-text: #F8FAFC;            /* 主文本 - 近白 */
--color-text-secondary: #94A3B8;  /* 次要文本 - 灰蓝 */
--color-text-muted: #64748B;      /* 弱化文本 - 暗灰 */
```

#### 亮色主题
```css
--color-bg: #F1F5F9;              /* 主背景 - 浅灰 */
--color-surface: #FFFFFF;         /* 表面背景 - 纯白 */
--color-primary: #2563EB;         /* 主色调 - 深蓝 */
/* 其他色彩保持语义一致性 */
```

### 字体规范

```css
--font-heading: 'Fira Code', monospace;  /* 标题/代码字体 */
--font-body: 'Fira Sans', system-ui;      /* 正文字体 */
```

**字号阶梯**:
- 页面标题: 22px (font-weight: 600)
- 区块标题: 20px / 18px / 16px
- 正文: 14px
- 辅助文本: 13px / 12px / 11px

### 间距与圆角

- **卡片圆角**: 8px / 10px / 12px
- **按钮圆角**: 6px / 8px
- **内边距**: 8px基准单位,按4px递增(8/12/16/20/24/28px)
- **间隙**: 6px / 8px / 10px / 12px / 16px / 20px

### 动效规范

- **过渡时长**: 0.15s(微交互) / 0.2s(常规) / 0.3s(强调)
- **缓动函数**: ease(默认)
- **页面切换**: fade + translateY(6px)
- **悬停效果**: translateX/Y位移 + 背景色变化
- **加载动画**: pulse脉冲(1.8s周期)

---

## 🛠️ 开发指南

### 环境要求

| 工具 | 版本要求 | 用途 |
|------|----------|------|
| Node.js | >= 18.x | 前端运行时 |
| pnpm | >= 8.x | 包管理器(推荐) |
| Rust | >= 1.70 | 后端编译 |
| Tauri CLI | ^2 | 应用构建工具链 |

### 安装步骤

```bash
# 1. 克隆仓库
git clone <repository-url>
cd t2bucket

# 2. 安装前端依赖
pnpm install

# 3. (可选)安装Tauri系统依赖
# macOS: 已内置支持
# Ubuntu: sudo apt install libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev
# Windows: 安装 Microsoft Visual Studio C++ Build Tools

# 4. 启动开发服务器
pnpm tauri dev
```

### 开发脚本

```bash
# 开发模式(热重载)
pnpm dev              # 仅启动Vite前端开发服务器 (端口1420)
pnpm tauri dev        # 启动完整Tauri应用(前端+后端)

# 生产构建
pnpm build            # 构建前端资源到dist/
pnpm tauri build      # 构建完整桌面应用安装包

# 其他
pnpm preview          # 预览生产构建的前端
pnpm tauri <command>  # 直接调用Tauri CLI
```

### Vite配置要点 ([vite.config.ts](vite.config.ts))

```typescript
{
    port: 1420,              // 固定端口(Tauri要求)
    strictPort: true,        // 端口占用则失败
    clearScreen: false,      // 不清屏(保留Rust错误输出)
    ignored: ["**/src-tauri/**"],  // 忽略后端目录变更
    hmr: {                   // 热更新配置
        protocol: "ws",
        host: TAURI_DEV_HOST,
        port: 1421
    }
}
```

### IDE推荐配置

**VS Code扩展**:
- Vue - Official (Vue Language Features)
- Tauri (Tauri支持)
- rust-analyzer (Rust语言服务)

**TypeScript配置** ([tsconfig.json](tsconfig.json)):
- 目标: ES2020
- 模块: ESNext (bundler模式)
- 严格模式开启 (strict: true)
- 未使用变量/参数检测

---

## 🔒 安全机制

### 1. 凭证加密存储

**威胁模型**: 防止本地文件泄露导致云服务凭据被盗

**防护措施**:
- SecretId和SecretKey使用AES-256-GCM加密后存储
- 加密密钥独立存储于 `.key` 文件(Unix权限0o600)
- 每次加密使用随机Nonce,防止重放攻击
- 使用操作系统安全随机数生成器(OsRng)

### 2. API签名认证

**实现标准**: 腾讯云COS Signature V5

**安全特性**:
- 时间戳有效期限制 (keyTime窗口约1小时)
- HMAC-SHA1防篡改签名
- 完整请求参与签名(method/path/params/headers)

### 3. 前端安全

- CSP (Content Security Policy): 当前配置为null(开发阶段)
- 输入验证: 所有表单字段必填校验和trim处理
- XSS防护: Vue模板自动转义,v-html仅用于受信任的SVG图标

### 4. 权限控制

**Tauri Capabilities** ([capabilities/default.json](src-tauri/capabilities/default.json)):

```json
{
  "permissions": [
    "core:default",      // 核心API(窗口/事件/文件系统基础)
    "opener:default"     // 外部链接打开能力
  ]
}
```

**最小权限原则**: 仅授予必要API访问权限

---

## 📊 性能优化策略

### 前端优化

1. **路由懒加载**: 页面组件按需加载(可通过import()进一步优化)
2. **Computed缓存**: 路径拆分、表单验证等高频计算使用computed
3. **骨架屏**: 异步操作期间显示加载态,提升感知性能
4. **Transition动画**: 页面切换平滑过渡,减少视觉跳跃
5. **事件防抖**: 未明显使用debounce(桌面应用场景网络延迟非主要瓶颈)

### 后端优化

1. **同步HTTP客户端**: 使用reqwest::blocking简化异步复杂度(桌面应用可接受)
2. **内存效率**: 大文件传输使用Vec<u8>避免多次拷贝
3. **XML轻量解析**: 手写行解析器,无需引入重型XML库
4. **密钥缓存**: 加密密钥一次生成后持久化,重复使用

### 网络优化

1. **条件请求**: 未实现ETag/Last-Modified(可后续优化)
2. **分页支持**: COS API原生支持marker分页(当前未启用)
3. **并发限制**: 未实施请求队列(桌面应用通常单用户操作)

---

## 🚀 未来扩展方向

### 已规划但未实现的功能

根据代码中的预留设计,未来可能支持:

1. **多云厂商适配**
   - AWS S3 (已有provider选项,标记为不可用)
   - Aliyun OSS
   - Huawei OBS
   - MinIO (自托管兼容S3)

   *需要实现*: 各厂商API签名算法适配层

2. **高级文件操作**
   - 批量上传/下载
   - 文件移动/重命名
   - 复制/Copy操作
   - 分片上传(大文件支持)

3. **用户体验增强**
   - 拖拽上传
   - 键盘快捷键
   - 文件搜索
   - 收藏夹/常用路径
   - 多标签/多窗口浏览

4. **安全性增强**
   - 主密码保护(加密密钥二次加密)
   - 会话超时自动锁定
   - 操作审计日志
   - 导出/导入连接配置(加密备份)

5. **企业级功能**
   - 多账户管理
   - 团队共享配置
   - CDN预热/刷新
   - 存储用量统计可视化

---

## ❓ 常见问题 (FAQ)

### Q1: 为什么选择Tauri而非Electron?
**A**: 
- 包体大小: Tauri应用约5-10MB vs Electron 100MB+
- 内存占用: 共享系统WebView vs 内置Chromium
- 性能: Rust后端天然高性能
- 安全性: 更小的攻击面

### Q2: 如何添加新的云存储服务商?
**A**:
1. 在`cos.rs`中新增对应Client结构体(或抽象出trait)
2. 实现该厂商的签名算法
3. 在`lib.rs`中注册新的Command
4. 前端ConnectionForm添加provider选项
5. 更新`providerLabels`映射

### Q3: 加密的凭证能被破解吗?
**A**: 
- AES-256-Gcm是目前最安全的对称加密算法之一
- 密钥存储在本地文件系统,权限受限
- *注意*: 如果攻击者获得物理访问权且系统未锁定,理论上可提取密钥和解密数据
- 建议: 未来增加主密码保护层

### Q4: 支持哪些操作系统?
**A**:
- ✅ macOS (Intel + Apple Silicon)
- ✅ Windows (10/11)
- ✅ Linux (主流发行版)
- 📱 移动端(iOS/Android) - Tauri 2支持,但需额外配置

### Q5: 如何调试后端Rust代码?
**A**:
```bash
# 方法1: VSCode + rust-analyzer
# 设置断点,使用 "Run Debug" 配置

# 方法2: 日志输出
# 在Rust代码中使用 println!() 或 log::info!()

# 方法3: 单独测试
cargo test --manifest-path src-tauri/Cargo.toml
```

---

## 📝 开发注意事项

### 代码风格约定

1. **Rust后端**:
   - 使用 `Result<T, String>` 作为错误返回类型(简化错误处理)
   - 中文错误信息(面向中文用户)
   - 函数命名遵循snake_case
   - 模块划分按职责(cos/crypto/store)

2. **Vue前端**:
   - 全部使用 `<script setup lang="ts">` 语法
   - 组合式API (Composition API)
   - Props接口明确标注
   - 使用inject/provide进行跨组件通信(toast/dialog)

3. **CSS样式**:
   - CSS Variables实现主题化
   - Scoped Styles避免污染
   - BEM变体命名(.btn-primary, .btn-danger)
   - 响应式设计优先考虑桌面端(固定宽度1100px容器)

### Git提交建议

```
feat: 新增MinIO存储支持
fix: 修复大文件上传内存溢出问题
docs: 更新README安装说明
style: 统一按钮间距规范
refactor: 重构XML解析为流式处理
perf: 优化列表加载性能
test: 添加加密模块单元测试
chore: 升级Tauri到2.1版本
```

---

## 📚 参考资源

### 官方文档
- [Tauri 2 Documentation](https://v2.tauri.app/)
- [Vue 3 Documentation](https://vuejs.org/)
- [Vue Router 4 Documentation](https://router.vuejs.org/)
- [腾讯云COS API文档](https://cloud.tencent.com/document/product/436)
- [COS 签名算法说明](https://cloud.tencent.com/document/product/436/7778)

### 关键依赖库
- [Reqwest](https://docs.rs/reqwest/) - Rust HTTP Client
- [AES-GCM](https://docs.rs/aes-gcm/) - AES-GCM加密实现
- [Serde](https://docs.rs/serde/) - 序列化框架
- [Vite](https://vitejs.dev/) - 下一代前端构建工具

---

## 📄 许可证

本项目采用开源许可证(具体见项目根目录LICENSE文件,如有)

---

**文档版本**: 1.0.0  
**最后更新**: 2026-05-29  
**维护者**: 项目开发团队  
**适用版本**: T2Bucket v0.1.0
