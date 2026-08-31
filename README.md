# NetEase Music Shell 🎵

> 把 [网易云音乐 Web 播放器](https://music.163.com/st/webplayer) 包装成轻量 Linux 桌面应用。
> 一个 ~4MB 的原生壳，基于 **Tauri 2 + 系统 WebKitGTK**，无 Electron。

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Tauri](https://img.shields.io/badge/Tauri-2.x-orange)](https://tauri.app)
[![Platform](https://img.shields.io/badge/Platform-Linux%20x64-green)]()

## 为什么不用浏览器 / Electron？

| 方案 | 二进制体积 | 内存占用 |
|---|---|---|
| Chrome `--app` 模式 | 0（蹭浏览器） | ~250MB+ |
| Electron 包装 | ~100MB+ | ~700MB+ |
| **本项目（Tauri 2）** | **~4MB** | 壳本身 ~160MB |

数据来源：[web-to-desktop-framework-comparison](https://github.com/Elanis/web-to-desktop-framework-comparison)（Linux x64 实测）+ 本机实测。

## 功能特性

- ✅ **无边框沉浸窗口**——内容铺满全窗，鼠标移到顶部浮现毛玻璃标题条
- ⭕ **原生透明圆角**——16px 全四角裁剪，深色/浅色主题均无需额外适配
- 🖐️ **全宽拖拽**，双击标题条最大化/还原；右上角悬浮 最小化 / 最大化 / 关闭 按钮（关闭按钮 hover 变红）
- 🔒 **独立数据目录**，登录状态持久化，不污染系统浏览器
- 🪶 **原生体验**：GTK 原生窗口、显式应用图标、独立的 Wayland/X11 进程

## 安装

### 方式一：下载预编译包（推荐）

到 [Releases](../../releases) 页面下载：

| 文件 | 适合场景 |
|---|---|
| `NeteaseMusic_*_amd64.deb` | Debian / Ubuntu 系，`sudo apt install ./NeteaseMusic_*.deb` |
| `NeteaseMusic_*_amd64.AppImage` | 任意发行版，`chmod +x` 后直接运行 |

### 方式二：从源码编译

**1. 安装系统依赖**（Ubuntu / Debian）：

```bash
sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl wget file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  pkg-config
```

<details>
<summary>其他发行版依赖名</summary>

- **Fedora**: `sudo dnf install webkit2gtk4.1-devel gcc gcc-c++ make openssl-devel librsvg2-devel`
- **Arch**: `sudo pacman -S webkit2gtk-4.1 base-devel librsvg`

详见 [Tauri 官方文档](https://v2.tauri.app/start/prerequisites/)。
</details>

**2. 安装 Rust 工具链**：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
source ~/.cargo/env
```

**3. 编译**：

```bash
npm install
npm run build          # 产出 deb + AppImage + 二进制
```

产物位置：

```
src-tauri/target/release/netease-music            # 裸二进制 (~4MB)
src-tauri/target/release/bundle/deb/*.deb
src-tauri/target/release/bundle/appimage/*.AppImage
```

**开发调试**：

```bash
npm run dev
```

## 技术实现

把现代 Web 应用跑在 WebKitGTK 上有三个坑，本项目逐一解决：

1. **浏览器检测**：网易云会校验 UA 并拦截不认识的浏览器
   → 窗口 `user_agent` 伪装成 Chrome 126

2. **`requestIdleCallback` 缺失**：伪装 Chrome 后，网页的"现代浏览器"代码路径会调用
   `requestIdleCallback`——WebKitGTK 尚未实现，导致 **ReferenceError → 白屏**
   → `initialization_script` 在页面脚本执行前注入 polyfill：

   ```js
   window.requestIdleCallback = function(cb, opts) {
     return setTimeout(function(){ cb({ didTimeout:false, timeRemaining:()=>50 }); }, 1);
   };
   window.cancelIdleCallback = function(id){ clearTimeout(id); };
   ```

3. **远程页面无法调用 IPC**：Tauri 默认禁止远程域访问 IPC，窗口控制按钮会失效
   → 通过 `src-tauri/capabilities/netease-remote.json` 精确授权
   （仅 `music.163.com` 域、仅窗口四项权限）

4. **无边框窗口圆角**：`body` 的背景默认会传播到 WebView 的方形画布，而且页面含有
   `position: fixed` 的全屏元素
   → 给根元素设置完全透明的渐变以阻止背景传播，再通过
   `body { transform: translateZ(0); overflow: hidden; border-radius: 16px; }`
   统一裁剪普通内容和固定定位内容。该方案不移动页面 DOM，也不扫描或复制主题背景。

5. **窗口图标**：Linux 桌面环境可能缓存桌面图标，而 Tauri 窗口也有独立的运行时图标
   → 打包配置提供 32/128/256px PNG，同时在创建窗口时通过 `Image::from_bytes`
   显式设置图标。替换图标后需要重新编译二进制。

## 故障排查

| 现象 | 解决 |
|---|---|
| 窗口空白 | 尝试 `WEBKIT_DISABLE_DMABUF_RENDERER=1 netease-music`（NVIDIA/Wayland 显卡兼容问题） |
| 提示浏览器不兼容 | 确认用的是本项目编译版本（UA + polyfill 都在壳里） |
| 无法登录 / 登录丢失 | 数据目录在 `~/.local/share/com.leon.netease-music/`，删除后重试 |

## 项目结构

```
netease-music-shell/
├── package.json                    # Tauri CLI 入口
├── src/                            # 占位页（实际加载远程 URL）
├── release/                        # 本地编译产物（不入库）
└── src-tauri/
    ├── src/main.rs                 # 窗口、圆角、标题栏及图标设置
    ├── tauri.conf.json             # 窗口 / UA / 打包配置
    ├── capabilities/
    │   └── netease-remote.json     # 远程页面 IPC 白名单
    └── icons/                      # 应用图标
```

## 自动发布（CI）

仓库自带 GitHub Actions 工作流 (`.github/workflows/release.yml`)：
推送 `v*` 标签即可自动编译 deb + AppImage 并发布到 Release：

```bash
git tag v1.0.1 && git push origin v1.0.1
```

## 免责声明

本项目仅是第三方 Web 播放器的**桌面壳**，与 [网易云音乐](https://music.163.com) / 杭州乐读科技有限公司**无任何 affiliation**。
网易云音乐及全部内容版权归官方所有，使用本项目产生的一切行为请遵守官方服务条款。

## License

[MIT](LICENSE) © 2026 Leon
