# 维护说明

本文记录 NetEase Music Shell 中容易重复踩坑的实现细节。

## 当前实现

- Tauri 2.11 + WebKitGTK，加载 `https://music.163.com/st/webplayer`。
- 无边框透明窗口，顶部 30px 悬停显示自定义拖拽标题栏。
- 16px 全四角圆角已经在 GNOME Wayland 下人工验证，深色和浅色主题均正常。
- 应用图标包含 32、128、256px 三种 PNG，并由窗口构建器显式加载。
- WebKitGTK 会自动通过 MPRIS 暴露页面媒体控制，无需额外播放器依赖。

## 圆角原理

站点会把 `body` 背景传播到 WebView 的 canvas，canvas 会以矩形覆盖透明窗口；同时，
页面存在覆盖全屏的 `position: fixed` 元素。当前实现只注入一段静态 CSS：

```css
html {
  background-color: transparent !important;
  background-image: linear-gradient(transparent, transparent) !important;
  overflow: hidden !important;
}

body {
  position: fixed !important;
  inset: 0 !important;
  margin: 0 !important;
  border-radius: 16px !important;
  overflow: hidden !important;
  transform: translateZ(0) !important;
}
```

透明渐变使根元素拥有背景图，阻止 `body` 背景传播到方形 canvas；`transform` 使
`body` 成为 fixed 后代的 containing block，因此 `overflow: hidden` 可以统一裁剪页面。

不要恢复旧的 wrapper 方案。旧方案会移动整个页面 DOM、扫描元素背景并轮询主题，
不仅复杂，而且容易破坏站点更新、弹窗层级和深浅主题切换。

## 图标

- `src-tauri/icons/32x32.png`：32×32
- `src-tauri/icons/128x128.png`：128×128
- `src-tauri/icons/icon.png`：256×256

`Cargo.toml` 启用了 Tauri 的 `image-png` 特性，`main.rs` 使用 `include_bytes!` 和
`Image::from_bytes` 显式设置窗口图标。因此修改图标后必须重新编译，单独替换 PNG
不会改变已经生成的二进制。

## 发布检查

```bash
cargo check --locked --manifest-path src-tauri/Cargo.toml
npm ci
npm run build
```

发布前还需在真实应用中检查：四角透明、深浅主题切换、弹窗/菜单、播放条、自定义标题栏、
Dock 图标和 MPRIS。离屏 WebKit 测试不包含登录态，只能作为辅助验证。

发布版本必须在 `package.json`、`src-tauri/Cargo.toml` 和 `src-tauri/tauri.conf.json`
保持一致。图标是编译期嵌入资源，替换 PNG 后要确保 Rust 目标确实重新编译。
