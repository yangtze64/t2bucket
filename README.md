# T2Bucket

[English](#english) | [中文](#中文)

---

## English

T2Bucket is a cross-platform cloud storage bucket management desktop application built with **Tauri 2 + Vue 3 + TypeScript**. It provides a unified interface to manage buckets and objects across multiple cloud storage providers.

> Currently supports **Tencent COS** and **AWS S3**, with planned support for Aliyun OSS, Huawei OBS, and MinIO.

### Features

- 🔐 **Encrypted Credential Storage** — AK/SK encrypted with AES-256-GCM, keys stored locally
- 🪣 **Bucket Management** — List, browse, upload, download, and delete objects
- 👁️ **File Preview** — Preview images, text, and binary files inline
- 🖱️ **Right-Click Context Menu** — Upload, create folder, copy path, preview, download, delete
- 🌙 **Theme Switching** — System / Light / Dark mode with CSS variables
- 🌍 **Multi-Provider** — Unified interface with factory pattern, easy to extend
- 🖥️ **Cross-Platform** — Supports Windows, macOS, and Linux
- 🔄 **CI/CD** — GitHub Actions multi-platform build with auto-release

### Supported Providers

| Provider | Status |
|----------|--------|
| Tencent COS | ✅ Supported |
| AWS S3 | ✅ Supported (including China regions) |
| Aliyun OSS | 🔜 Planned |
| Huawei OBS | 🔜 Planned |
| MinIO | 🔜 Planned |

### Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop Framework | Tauri 2 |
| Frontend | Vue 3 + TypeScript |
| Routing | vue-router 4 |
| Build Tool | Vite 6 |
| Package Manager | pnpm |
| Rust Crypto | aes-gcm + rand |
| Rust HTTP | reqwest (blocking) |
| COS Signing | hmac + sha1 |
| S3 Signing | hmac + sha2 (SigV4) |

### Getting Started

#### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Tauri 2 Prerequisites](https://v2.tauri.app/start/prerequisites/)

#### Install & Run

```bash
pnpm install
pnpm tauri dev
```

#### Build

```bash
pnpm tauri build
```

The built installer can be found in `src-tauri/target/release/bundle/`.

> **Note:** Cross-platform builds are not supported. To build for a specific platform, run the build command on that platform.

### Project Structure

```
t2bucket/
├── src/                          # Frontend (Vue 3)
│   ├── pages/                    # Page components
│   │   ├── HomePage.vue          # Connection list
│   │   ├── ConnectionForm.vue    # Create/Edit connection
│   │   ├── BucketBrowser.vue     # Bucket & object browser
│   │   └── ObjectPreview.vue     # File preview
│   ├── components/               # Shared components
│   ├── composables/              # Vue composables
│   ├── router/                   # Vue Router config
│   └── types/                    # TypeScript types
└── src-tauri/                    # Backend (Rust)
    └── src/
        ├── lib.rs                # Tauri command registration
        ├── store.rs              # Connection persistence (JSON + encryption)
        ├── crypto.rs             # AES-256-GCM encryption module
        └── providers/            # Storage provider implementations
            ├── mod.rs            # StorageProvider trait & factory
            ├── cos.rs            # Tencent COS (V5 signing)
            └── s3.rs             # AWS S3 (SigV4 signing)
```

### License

MIT

---

## 中文

T2Bucket 是一个跨平台云存储桶管理桌面应用，基于 **Tauri 2 + Vue 3 + TypeScript** 构建，提供统一的界面管理多个云存储服务商的存储桶和对象。

> 目前已支持 **腾讯云 COS** 和 **AWS S3**，计划支持阿里云 OSS、华为云 OBS 和 MinIO。

### 功能特性

- 🔐 **加密凭证存储** — AK/SK 使用 AES-256-GCM 加密，密钥本地保存
- 🪣 **存储桶管理** — 列出、浏览、上传、下载和删除对象
- 👁️ **文件预览** — 内联预览图片、文本和二进制文件
- 🖱️ **右键上下文菜单** — 上传文件、新建文件夹、复制路径、预览、下载、删除
- 🌙 **主题切换** — 跟随系统 / 浅色 / 深色模式，基于 CSS 变量
- 🌍 **多云支持** — 统一接口 + 工厂模式，易于扩展
- 🖥️ **跨平台** — 支持 Windows、macOS 和 Linux
- 🔄 **CI/CD** — GitHub Actions 多平台构建 + 自动发布

### 支持的服务商

| 服务商 | 状态 |
|--------|------|
| 腾讯云 COS | ✅ 已支持 |
| AWS S3 | ✅ 已支持（含中国区域） |
| 阿里云 OSS | 🔜 计划中 |
| 华为云 OBS | 🔜 计划中 |
| MinIO | 🔜 计划中 |

### 技术栈

| 层 | 技术 |
|---|------|
| 桌面框架 | Tauri 2 |
| 前端 | Vue 3 + TypeScript |
| 路由 | vue-router 4 |
| 构建工具 | Vite 6 |
| 包管理器 | pnpm |
| Rust 加密 | aes-gcm + rand |
| Rust HTTP | reqwest (blocking) |
| COS 签名 | hmac + sha1 |
| S3 签名 | hmac + sha2 (SigV4) |

### 快速开始

#### 环境要求

- [Node.js](https://nodejs.org/)（v18+）
- [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/tools/install)（stable）
- [Tauri 2 前置依赖](https://v2.tauri.app/start/prerequisites/)

#### 安装与运行

```bash
pnpm install
pnpm tauri dev
```

#### 打包构建

```bash
pnpm tauri build
```

构建产物位于 `src-tauri/target/release/bundle/`。

> **注意：** 不支持交叉编译，需要在目标平台上执行构建命令。

### 项目结构

```
t2bucket/
├── src/                          # 前端 (Vue 3)
│   ├── pages/                    # 页面组件
│   │   ├── HomePage.vue          # 连接列表
│   │   ├── ConnectionForm.vue    # 新建/编辑连接
│   │   ├── BucketBrowser.vue     # 存储桶与对象浏览
│   │   └── ObjectPreview.vue     # 文件预览
│   ├── components/               # 公共组件
│   ├── composables/              # Vue 组合式函数
│   ├── router/                   # Vue Router 配置
│   └── types/                    # TypeScript 类型定义
└── src-tauri/                    # 后端 (Rust)
    └── src/
        ├── lib.rs                # Tauri 命令注册
        ├── store.rs              # 连接持久化（JSON + 加密）
        ├── crypto.rs             # AES-256-GCM 加解密模块
        └── providers/            # 存储服务商实现
            ├── mod.rs            # StorageProvider trait 与工厂函数
            ├── cos.rs            # 腾讯云 COS（V5 签名）
            └── s3.rs             # AWS S3（SigV4 签名）
```

### 许可证

MIT
