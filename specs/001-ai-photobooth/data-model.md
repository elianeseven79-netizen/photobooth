# Data Model: AI自助拍照机

## 实体定义

### 1. 拍照模式 (PhotoMode)

| 字段 | 类型 | 说明 |
|------|------|------|
| id | String | 模式唯一标识 (如: cartoon, movie, anime) |
| name | String | 模式名称 (中文) |
| description | String | 模式描述 |
| icon | String | 图标路径 |
| effects | Vec<Effect> | 效果列表 |

### 2. 效果配置 (Effect)

| 字段 | 类型 | 说明 |
|------|------|------|
| id | String | 效果唯一标识 |
| mode_id | String | 所属模式ID |
| name | String | 效果名称 |
| prompt | String | AI生成用的提示词 |
| thumbnail | String | 预览图路径 |
| price_download | i32 | 下载价格 (分) |
| price_print | i32 | 打印价格 (分) |

### 3. 拍照会话 (PhotoSession)

| 字段 | 类型 | 说明 |
|------|------|------|
| id | String | 会话唯一标识 (UUID) |
| mode_id | String | 选择的模式ID |
| effect_id | String | 选择的效果ID |
| original_photo | Vec<u8> | 原始照片 (Base64) |
| generated_photo | Vec<u8> | 生成的AI照片 (Base64) |
| status | SessionStatus | 会话状态 |
| created_at | i64 | 创建时间戳 |
| updated_at | i64 | 更新时间戳 |

**SessionStatus 枚举**:
- `SelectingMode` - 选择模式中
- `SelectingEffect` - 选择效果中
- `Capturing` - 拍照中
- `Processing` - AI处理中
- `Previewing` - 预览中
- `Completed` - 完成

### 4. 订单 (Order)

| 字段 | 类型 | 说明 |
|------|------|------|
| id | String | 订单唯一标识 (UUID) |
| session_id | String | 关联的会话ID |
| order_type | OrderType | 订单类型 |
| amount | i32 | 金额 (分) |
| status | OrderStatus | 订单状态 |
| wechat_order_id | String | 微信订单号 |
| payment_time | i64 | 支付时间戳 |
| created_at | i64 | 创建时间戳 |

**OrderType 枚举**:
- `Download` - 下载
- `Print` - 打印 (暂不支持)

**OrderStatus 枚举**:
- `Pending` - 待支付
- `Paid` - 已支付
- `Cancelled` - 已取消
- `Refunded` - 已退款

### 5. 用户会话状态 (UserSession)

| 字段 | 类型 | 说明 |
|------|------|------|
| session_id | String | 会话ID |
| current_step | Step | 当前步骤 |
| mode_id | Option<String> | 已选模式 |
| effect_id | Option<String> | 已选效果 |
| expires_at | i64 | 过期时间 |

**Step 枚举**:
- `Home` - 首页
- `SelectMode` - 选择模式
- `SelectEffect` - 选择效果
- `Capture` - 拍照
- `Preview` - 预览
- `Payment` - 支付
- `Download` - 下载

---

## 关系图

```
PhotoMode (1) ─────< (N) Effect
PhotoSession (1) ─────< (1) Order
PhotoSession (N) ─────< (1) UserSession
```

---

## 数据流

1. 用户打开应用 -> 创建 UserSession
2. 选择模式 -> 更新 UserSession.mode_id, PhotoSession.mode_id
3. 选择效果 -> 更新 UserSession.effect_id, PhotoSession.effect_id
4. 拍照 -> 存储 PhotoSession.original_photo
5. AI生成 -> 存储 PhotoSession.generated_photo
6. 确认效果 -> PhotoSession.status = Completed
7. 支付 -> 创建 Order，记录 PhotoSession.id
8. 支付成功 -> Order.status = Paid
9. 下载 -> 根据 Order.session_id 查找 PhotoSession.generated_photo
