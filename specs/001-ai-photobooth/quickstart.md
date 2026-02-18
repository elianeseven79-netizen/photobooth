# Quickstart: AI自助拍照机开发指南

## 环境要求

### 开发环境

- **Node.js**: 18.x 或更高
- **Rust**: 1.75 或更高
- **包管理器**: npm 或 yarn

### 安装依赖

```bash
# 安装 Node.js 依赖
npm install

# 安装 Rust 依赖 (在 src-tauri 目录)
cd src-tauri
cargo build
```

## 运行开发服务器

```bash
# 启动 Tauri 开发服务器
npm run tauri dev
```

这将同时启动前端开发服务器和 Tauri 后端。

## 项目结构

```
photobooth/
├── src/                    # 前端 React 代码
│   ├── App.tsx            # 主应用组件
│   ├── main.tsx           # 入口文件
│   ├── components/        # UI 组件
│   │   ├── ModeSelect.tsx
│   │   ├── EffectSelect.tsx
│   │   ├── Camera.tsx
│   │   ├── PhotoPreview.tsx
│   │   └── Payment.tsx
│   ├── pages/             # 页面
│   │   ├── HomePage.tsx
│   │   └── OrderPage.tsx
│   ├── services/          # API 服务
│   │   └── api.ts        # Tauri 命令封装
│   ├── hooks/            # React Hooks
│   │   └── usePhotoSession.ts
│   └── styles/          # 样式
├── src-tauri/           # Rust 后端
│   ├── src/
│   │   ├── main.rs      # 入口
│   │   ├── commands/    # Tauri 命令
│   │   │   ├── mode.rs
│   │   │   ├── session.rs
│   │   │   ├── generate.rs
│   │   │   └── payment.rs
│   │   ├── services/    # 业务服务
│   │   │   ├── minimax.rs
│   │   │   ├── wechat.rs
│   │   │   └── storage.rs
│   │   └── models/      # 数据模型
│   ├── Cargo.toml
│   └── tauri.conf.json
├── public/
│   └── effects/         # 效果预览图
└── package.json
```

## 核心API

### Tauri 命令

前端通过 `invoke` 调用 Rust 后端：

```typescript
import { invoke } from '@tauri-apps/api/core';

// 获取模式列表
const modes = await invoke<PhotoMode[]>('get_modes');

// 创建会话
const session = await invoke<CreateSessionResponse>('create_session', {
  mode_id: 'cartoon',
  effect_id: 'cartoon-01'
});

// 生成AI照片
const result = await invoke<GeneratePhotoResponse>('generate_photo', {
  session_id: session.session_id,
  photo_base64: '...'
});

// 创建支付订单
const order = await invoke<CreateOrderResponse>('create_order', {
  session_id: session.session_id,
  order_type: 'download'
});

// 查询订单
const status = await invoke<QueryOrderResponse>('query_order', {
  order_id: order.order_id
});
```

## 配置

### MiniMax API

在 `.env` 文件中配置：

```
MINIMAX_API_KEY=your_api_key
```

### 微信支付

```
WECHAT_MCH_ID=your_mch_id
WECHAT_API_KEY=your_api_key
```

## 构建发布版本

```bash
npm run tauri build
```

构建完成后，安装包位于 `src-tauri/target/release/bundle/`。

## 常见问题

### Q: 如何添加新的拍照模式？
A: 在 `src-tauri/src/data/modes.json` 中添加新的模式配置，包括模式ID、名称、效果列表等。

### Q: 如何测试支付功能？
A: 微信支付需要真实商户资质。开发阶段可以使用微信支付的沙箱环境。

### Q: 如何调试 Rust 后端？
A: 使用 `println!` 输出日志，或使用 `tracing` crate 进行结构化日志记录。
