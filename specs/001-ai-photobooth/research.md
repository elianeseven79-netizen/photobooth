# Research: AI自助拍照机技术调研

**Date**: 2026-02-17
**Feature**: 001-ai-photobooth

## 技术选型

### 桌面框架选择

**Decision**: Tauri 2.x

**Rationale**:
- 轻量级 (10-15MB)，适合自助机部署
- 内存占用低 (~50MB)，适合长时间运行
- 启动快速，用户无需等待
- 安全沙箱，适合支付处理
- 对中文操作系统 (Windows) 兼容性良好

**Alternatives considered**:
- Electron: 体积大 (150MB+)，内存占用高
- Qt: 学习峭，曲线陡UI开发不够灵活

### 前端框架

**Decision**: React 18 + TypeScript

**Rationale**:
- 成熟稳定，生态丰富
- TypeScript 提供类型安全
- 适合构建复杂交互UI

### 后端架构

**Decision**: Rust (via Tauri commands)

**Rationale**:
- 高性能，适合图像处理
- 安全的内存管理
- 方便调用本地系统API

### 数据存储

**Decision**: SQLite

**Rationale**:
- 轻量级，无需额外服务
- 支持本地会话和订单管理
- 适合单机部署

---

## API集成

### MiniMax 图生图API

**Endpoint**: `https://api.minimaxi.com/v1/image_generation`

**Integration Approach**:
1. 前端通过摄像头捕获用户照片
2. 照片上传到Rust后端 (Base64编码)
3. Rust后端调用MiniMax API，使用 `subject_reference` 参数
4. 返回生成的图片 (Base64)
5. 前端显示合成结果

**Key Parameters**:
```json
{
  "model": "image-01",
  "prompt": "风格描述",
  "subject_reference": [
    {
      "type": "character",
      "image_file": "用户照片Base64"
    }
  ],
  "response_format": "base64"
}
```

### 微信支付API

**Integration Approach**:
1. 用户点击支付，Rust后端调用微信统一下单API
2. 获取支付二维码URL
3. 前端显示二维码
4. 用户扫码支付
5. 微信服务器回调 (需要公网暴露或使用轮询)
6. 订单状态更新

**Note**: 需要商户资质 (mch_id, api_key)

---

## 硬件考虑

### 摄像头
- 使用 WebRTC 通过浏览器API访问
- 支持多摄像头切换

### 触摸屏
- 响应式布局
- 大按钮适合触摸操作

### 自助机模式
- 全屏显示
- 禁用窗口控制
- 定时返回首页

---

## 结论

采用 Tauri + React + Rust 技术栈，可以快速开发轻量级、高性能的桌面应用，适合自助拍照机场景。MiniMax API 提供强大的图生图能力，微信支付满足国内支付需求。
