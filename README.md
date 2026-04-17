# WinCleaner Pro（Windows 磁盘清理与系统整理工具）

> Tauri + React + TypeScript。目标是可运行、可打包、可 PR 的真实桌面工程，而不是演示壳子。

## 1. 项目特性

- **磁盘总览**：读取真实磁盘分区（总容量/已用/可用）。
- **垃圾扫描与清理**：临时文件、缩略图、日志、Edge/Chrome 缓存、回收站。
- **大文件查找**：按路径与阈值扫描，支持打开目录、加入待处理、安全删除。
- **重复文件检测**：`size + name + sha256` 判断重复项，支持“保留最新，勾选其余”。
- **启动项管理**：读取 HKCU/HKLM Run + 启动文件夹，支持可修改项启停。
- **存储分析**：目录树占用分析，快速定位大目录。
- **额外工具**：清空回收站、最近下载聚合、桌面整理建议、聊天缓存识别、常用目录跳转。

## 2. 安全策略（默认启用）

- 关键目录保护：`Windows`、`Program Files`、`ProgramData` 等目录禁止危险删除。
- 危险操作必须确认。
- 删除文件优先进入应用恢复区：`%USERPROFILE%\\.wincleaner_recovery`。
- 异常场景有可读错误提示（权限不足、占用、路径不存在）。

## 3. 目录结构

```text
.
├─ index.html
├─ package.json
├─ tsconfig.json
├─ tsconfig.node.json
├─ vite.config.ts
├─ src
│  ├─ main.tsx
│  ├─ App.tsx
│  ├─ styles.css
│  ├─ types.ts
│  ├─ stores/appStore.ts
│  ├─ services/tauriApi.ts
│  ├─ components
│  │  ├─ ShellLayout.tsx
│  │  ├─ DiskCard.tsx
│  │  ├─ ProgressLine.tsx
│  │  └─ Section.tsx
│  └─ pages
│     ├─ DashboardPage.tsx
│     ├─ JunkScanPage.tsx
│     ├─ LargeFilesPage.tsx
│     ├─ DuplicatePage.tsx
│     ├─ StartupPage.tsx
│     ├─ StoragePage.tsx
│     ├─ ToolsPage.tsx
│     └─ SettingsPage.tsx
└─ src-tauri
   ├─ Cargo.toml
   ├─ build.rs
   ├─ tauri.conf.json
   ├─ icons
   └─ src
      ├─ main.rs
      ├─ models.rs
      ├─ protect.rs
      ├─ scanner.rs
      ├─ cleanup.rs
      ├─ startup.rs
      └─ storage.rs
```

## 4. 在 VSCode 直接运行

### 4.1 环境准备（Windows）

1. 安装 **Node.js 20+**
2. 安装 **Rust stable**（`rustup`）
3. 安装 **Visual Studio 2022 Build Tools**（勾选 *Desktop development with C++*）
4. 确认系统存在 **WebView2 Runtime**（Win11 一般已内置）

### 4.2 启动开发

```bash
npm install
npm run dev
```

## 5. 打包成 EXE（详细教程）

### 5.1 一次性安装构建目标

```bash
rustup default stable
rustup target add x86_64-pc-windows-msvc
```

### 5.2 本地构建前端

```bash
npm run build
```

### 5.3 打包桌面安装包/可执行文件

```bash
npm run package
```

或：

```bash
npm run dist
```

### 5.4 产物位置

打包完成后，主要产物位于：

```text
src-tauri/target/release/
src-tauri/target/release/bundle/
```

常见包含：

- `bundle/nsis/*.exe`（安装程序）
- `release/wincleaner-pro.exe`（可执行文件）

> 如果你的机器策略禁用签名或安装器创建，Tauri 仍会保留 release 可执行文件。

## 6. 脚本说明

- `npm run frontend:dev`：仅启动 Vite
- `npm run dev`：启动 Tauri 开发模式
- `npm run build`：构建前端
- `npm run package`：打包桌面应用
- `npm run dist`：同 package（便于 CI）
- `npm run lint`：TypeScript 类型检查

## 7. 功能降级说明

- HKLM 启动项修改通常需要管理员权限。
- 回收站清空依赖 PowerShell `Clear-RecycleBin`，被策略限制时会失败并提示。
- 聊天缓存与浏览器缓存路径可能因用户安装方式不同而有差异（已做基础兼容）。

## 8. 可 PR 说明

本项目按模块拆分并保持前后端职责明确，适合直接 PR：

- 前端：页面、组件、状态、API 层分离。
- 后端：扫描、清理、启动项、存储分析、安全保护分模块。
- 文档：README 提供完整运行与打包步骤。


## 9. 关于 PR 显示二进制导致失败（已处理）

已从仓库移除 `src-tauri/icons` 下的二进制图标文件（png），改为文本占位说明，避免在某些仓库策略下因为二进制文件而导致 PR 检查失败。

如需发布版本，请在本地补充图标后再打包（图标不必提交到仓库）。

