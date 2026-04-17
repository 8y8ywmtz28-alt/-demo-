# Icons Placeholder (No Binary in Git)

当前仓库为避免 PR 因二进制文件被拦截，**不提交任何 png/ico/icns 图标文件**。

如果你要发布安装包，请在本地准备图标并放入本目录，然后在 `src-tauri/tauri.conf.json` 的 `bundle.icon` 中配置路径。

建议至少准备：

- `icon-32.png`
- `icon-128.png`
- `icon.ico`（Windows）

示例配置：

```json
"bundle": {
  "active": true,
  "targets": "all",
  "icon": ["icons/icon-32.png", "icons/icon-128.png", "icons/icon.ico"]
}
```
