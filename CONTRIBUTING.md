# Contributing

感谢你考虑为 NetEase Music Shell 做贡献。

## 提交问题

- 功能建议和普通缺陷请使用 GitHub Issues。
- 安全问题不要公开提交 Issue，请按 [SECURITY.md](SECURITY.md) 私密报告。
- 请说明发行版、桌面环境、Wayland/X11、WebKitGTK 版本及复现步骤。
- 窗口或渲染问题建议附截图，但请先清理账号、歌单等个人信息。

## 本地开发

安装 README 中列出的系统依赖后运行：

```bash
npm ci
npm run dev
```

提交前至少执行：

```bash
cargo check --locked --manifest-path src-tauri/Cargo.toml
npm run build
```

涉及窗口外观的改动，还应在 Wayland 或 X11 实机检查：

- 四角透明和抗锯齿
- 深色、浅色主题
- 弹窗、菜单和播放条
- 悬浮标题栏和窗口控制
- Dock 图标及 MPRIS 媒体控制

## Pull Request

- 一个 Pull Request 聚焦一个问题。
- 说明改动动机、验证方式和受影响的平台。
- 不要提交构建产物、登录数据、密钥或未经授权的第三方素材。
- 保持远程页面权限最小化；新增 Tauri 权限时请解释必要性。

提交代码即表示你同意按本项目的 MIT License 授权你的贡献。
