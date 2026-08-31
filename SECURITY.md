# Security Policy

## 支持范围

安全修复面向最新发布版本和 `main` 分支。旧版本可能不会单独回补。

## 报告漏洞

请使用 GitHub 的
[Private vulnerability reporting](https://github.com/Leon00x/netease-music-shell/security/advisories/new)
私密报告安全问题，不要创建公开 Issue。

报告中请包含：

- 受影响版本和运行环境
- 可复现步骤或最小示例
- 潜在影响
- 已知的缓解方式（如有）

维护者会尽快确认报告。在修复发布前，请不要公开漏洞细节或可直接利用的代码。

## 安全边界

本项目加载网易云音乐的远程网页，网页内容和服务端行为不由本项目维护。与远程服务自身
有关的问题应报告给对应服务提供方。本项目负责的范围主要包括桌面壳、Tauri 权限、窗口
控制、数据目录和打包流程。
