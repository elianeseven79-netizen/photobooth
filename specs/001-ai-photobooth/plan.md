# Implementation Plan: AI自助拍照机桌面应用

**Branch**: `001-ai-photobooth` | **Date**: 2026-02-17 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-ai-photobooth/spec.md`

## Summary

开发一款AI自助拍照机桌面应用，支持用户选择拍照模式（卡通、电影海报、二次元等），通过MiniMax AI API将用户人像与选定场景合成，支持微信支付下载照片。应用采用Tauri框架构建，前端使用React + TypeScript，后端使用Rust处理业务逻辑和API集成。

## Technical Context

**Language/Version**: Rust 1.75+ (Tauri 2.x), TypeScript 5.x, React 18.x
**Primary Dependencies**: Tauri 2.x, React, MiniMax API, 微信支付API, SQLite
**Storage**: SQLite (本地会话和订单管理)
**Testing**: Vitest (前端), Rust测试 (后端)
**Target Platform**: Windows/Linux 桌面设备 (自助拍照机)
**Project Type**: 桌面应用 - 单项目结构
**Performance Goals**: AI合成处理 ≤30秒, 支付响应 <5秒
**Constraints**: 需要网络连接，支持触摸屏交互
**Scale/Scope**: 单机部署，100+效果组合

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

项目宪法文件 (.specify/memory/constitution.md) 尚未配置，跳过此检查。

## Project Structure

### Documentation (this feature)

```text
specs/001-ai-photobooth/
├── plan.md              # This file
├── research.md          # 技术调研文档
├── data-model.md        # 数据模型设计
├── quickstart.md        # 快速开始指南
├── contracts/           # API合约定义
└── tasks.md             # 任务列表 (/speckit.tasks 输出)
```

### Source Code (repository root)

```text
src/                      # Tauri 应用根目录
├── src/                  # 前端 React 代码
│   ├── components/       # UI 组件
│   ├── pages/            # 页面组件
│   ├── services/         # API 服务
│   ├── hooks/             # React Hooks
│   └── styles/            # 样式文件
├── src-tauri/           # Rust 后端代码
│   ├── src/
│   │   ├── commands/     # Tauri 命令
│   │   ├── services/     # 业务服务
│   │   ├── models/       # 数据模型
│   │   └── main.rs       # 入口
│   ├── Cargo.toml        # Rust 依赖
│   └── tauri.conf.json   # Tauri 配置
├── public/               # 静态资源
│   └── effects/          # 效果预览图
├── package.json          # Node 依赖
└── SPEC.md               # 应用规格文档
```

**Structure Decision**: 单项目结构，前端使用React + TypeScript，后端使用Rust通过Tauri命令处理MiniMax API和微信支付集成。

## Phase 0: Research

### 技术选型决策

**Framework**: Tauri 2.x
- 轻量级 (10-15MB)，适合自助机部署
- 内存占用低，启动快速
- 良好的安全沙箱，适合支付处理
- 对中文操作系统兼容性良好

**Frontend**: React 18 + TypeScript
- 成熟稳定，生态丰富
- 适合构建复杂的UI交互
- TypeScript 提供类型安全

**Backend**: Rust (via Tauri commands)
- 高性能，适合图像处理
- 安全的内存管理
- 方便调用本地系统API

**Storage**: SQLite
- 轻量级，无需额外服务
- 支持本地会话和订单管理
- 适合单机部署

**AI Integration**: MiniMax API (图生图)
- 使用 subject_reference 上传用户照片
- 结合 prompt 生成指定风格图片

**Payment**: 微信支付API
- 需要商户资质
- 扫码支付流程

### 关键集成点

1. **摄像头**: 使用WebRTC通过浏览器API访问
2. **MiniMax API**: Rust后端调用，图片Base64编解码
3. **微信支付**: 生成二维码 -> 用户扫码 -> 回调验证
4. **文件存储**: 本地文件系统存储合成的照片

## Phase 1: Design

### 核心模块设计

1. **模式选择模块**
   - 显示6种预设模式
   - 加载对应效果预览图

2. **拍照模块**
   - WebRTC 摄像头捕获
   - 倒计时拍照
   - 照片预览和确认

3. **AI合成模块**
   - 调用MiniMax图生图API
   - 进度显示
   - 错误处理和重试

4. **支付模块**
   - 微信支付二维码生成
   - 支付状态轮询
   - 订单创建

5. **下载模块**
   - 订单查询
   - 图片下载

### 待设计文档

- data-model.md - 数据模型定义
- contracts/ - API合约
- quickstart.md - 开发快速开始
