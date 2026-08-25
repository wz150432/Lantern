# Lantern（阅读器）

跨平台离线小说阅读器：Windows / macOS / Linux。
纯本地阅读，不联网、不爬虫、不收集数据。TXT / EPUB（EPUB 二期）。

- 设计文档：docs/superpowers/specs/2026-08-25-reader-design.md
- 实施计划：docs/superpowers/plans/2026-08-25-reader-v0.1.md

## v0.1 已完成功能

- 书架：TXT 导入（链接 / 复制进书库）、直接打开（Ctrl+O / 拖拽）、最近阅读、搜索过滤、删除记录
- 阅读：多编码自动识别（UTF-8 / BOM / UTF-16LE / UTF-16BE / GBK）、章节目录、翻页/滚动双模式、自动翻页、进度记忆恢复
- 辅助：书签（Ctrl+M）、全文查找（Ctrl+F）、百分比跳转（Ctrl+G）、字体缩放（Ctrl+= / Ctrl+-）
- 设置：主题（极简沉浸 / 书卷质感）、排版参数实时预览与持久化、翻页与点击方式、窗口置顶 / 透明度 / 沉浸模式
- 窗口：全屏（F11）、沉浸模式（F12）、窗口置顶（Ctrl+T）、透明度调节（Ctrl+滚轮）
- 稳定性：文件缺失不崩溃、library.db 删除后自动重建空库

## 开发

```bash
npm install
npm run tauri dev
```

## 测试

```bash
cd src-tauri && cargo test   # Rust 单测
npm run test                 # 前端单测
```

## 冒烟测试

v0.1 冒烟测试清单见 [docs/QA-v0.1.md](docs/QA-v0.1.md)。
