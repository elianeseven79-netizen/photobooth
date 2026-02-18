---

description: "Task list for AI自助拍照机桌面应用"
---

# Tasks: AI自助拍照机桌面应用

**Input**: Design documents from `/specs/001-ai-photobooth/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), data-model.md, contracts/

**Tests**: 测试任务暂未要求，仅包含实现任务

**Organization**: 任务按用户故事分组，实现每个用户故事的独立实现和测试

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 可并行运行（不同文件，无依赖）
- **[Story]**: 所属用户故事 (如 US1, US2, US3)
- 描述中包含具体文件路径

---

## Phase 1: Setup (项目初始化)

**Purpose**: 项目初始化和基础结构

- [x] T001 Create Tauri project with React + TypeScript frontend
- [x] T002 [P] Configure Rust backend dependencies (reqwest, rusqlite, serde)
- [x] T003 [P] Setup frontend dependencies (react, react-router, qrcode.react)
- [x] T004 Create project directory structure per plan.md
- [x] T005 Configure Tauri for window management and permissions
- [x] T006 [P] Setup environment configuration (.env example files)

---

## Phase 2: Foundational (核心基础设施)

**Purpose**: 所有用户故事实现前必须完成的核心里程碑

**⚠️ CRITICAL**: 基础阶段未完成前无法开始用户故事实现

- [x] T007 Setup SQLite database schema in src-tauri/src/db/
- [x] T008 [P] Implement data models (PhotoMode, Effect, PhotoSession, Order)
- [x] T009 [P] Create Tauri commands infrastructure
- [x] T010 Setup MiniMax API client in src-tauri/src/services/
- [x] T011 [P] Setup WeChat Pay API client in src-tauri/src/services/
- [x] T012 Implement session state management
- [x] T013 Configure error handling and logging

**Checkpoint**: 基础设施就绪 - 用户故事实现可以开始

---

## Phase 3: User Story 1 - 选择拍照模式 (Priority: P1) 🎯 MVP

**Goal**: 用户打开应用后可以看到所有可选的拍照模式，点击后进入效果预览

**Independent Test**: 用户打开应用能看到模式列表，选择模式后跳转到效果选择页面

### Implementation for User Story 1

- [x] T014 [P] [US1] Create ModeService for mode data in src-tauri/src/services/mode_service.rs
- [x] T015 [P] [US1] Implement get_modes Tauri command in src-tauri/src/commands/mode.rs
- [x] T016 [US1] Create ModeSelect component in src/components/ModeSelect.tsx
- [x] T017 [US1] Create mode card UI with icons in src/components/ModeCard.tsx
- [x] T018 [US1] Add mode selection navigation in src/App.tsx
- [x] T019 [US1] Add mode data (6 modes: cartoon, movie, anime, cyberpunk, traditional, age) in src-tauri/src/data/modes.rs

**Checkpoint**: US1 应该可以完整功能并独立测试

---

## Phase 4: User Story 2 - 预览并选择效果 (Priority: P1)

**Goal**: 用户选择模式后可以看到该模式下的所有效果预览图，选择效果后进行拍照

**Independent Test**: 用户选择模式后能看到效果列表，点击效果可以选中

### Implementation for User Story 2

- [x] T020 [P] [US2] Create EffectService for effect data in src-tauri/src/services/effect_service.rs
- [x] T021 [P] [US2] Implement get_effects Tauri command in src-tauri/src/commands/effect.rs
- [x] T022 [US2] Create EffectSelect component in src/components/EffectSelect.tsx
- [x] T023 [US2] Create effect thumbnail grid in src/components/EffectGrid.tsx
- [x] T024 [US2] Add effect selection state management in src/hooks/usePhotoSession.ts

**Checkpoint**: US1 + US2 应该可以完整功能并独立测试

---

## Phase 5: User Story 3 - 拍照与AI合成 (Priority: P1)

**Goal**: 用户确认效果后进行拍照，系统调用MiniMax API生成AI照片，用户可以预览或重新生成

**Independent Test**: 用户拍照后可以看到AI合成的成品，不满意可以重新生成

### Implementation for User Story 3

- [x] T025 [P] [US3] Create PhotoSession model in src-tauri/src/models/session.rs
- [x] T026 [P] [US3] Implement session creation command in src-tauri/src/commands/session.rs
- [x] T027 [US3] Create Camera component with WebRTC in src/components/Camera.tsx
- [x] T028 [US3] Implement photo capture and countdown in src/components/Camera.tsx
- [x] T029 [US3] Implement MiniMax API integration in src-tauri/src/services/minimax.rs
- [x] T030 [US3] Create generate_photo Tauri command in src-tauri/src/commands/generate.rs
- [x] T031 [US3] Create PhotoPreview component in src/components/PhotoPreview.tsx
- [x] T032 [US3] Add progress indicator for AI processing in src/components/ProcessingIndicator.tsx
- [x] T033 [US3] Add retry and regenerate functionality in src/hooks/usePhotoSession.ts

**Checkpoint**: US1 + US2 + US3 应该可以完整功能并独立测试 - MVP完成！

---

## Phase 6: User Story 4 - 付费下载 (Priority: P1)

**Goal**: 用户对AI合成效果满意后，可以通过微信扫码支付后下载照片

**Independent Test**: 用户可以通过扫描二维码完成微信支付，支付成功后可以下载照片

### Implementation for User Story 4

- [x] T034 [P] [US4] Create Order model in src-tauri/src/models/order.rs
- [x] T035 [P] [US4] Implement order creation command in src-tauri/src/commands/order.rs
- [x] T036 [US4] Implement WeChat Pay integration in src-tauri/src/services/wechat.rs
- [x] T037 [US4] Create Payment component with QR code display in src/components/Payment.tsx
- [x] T038 [US4] Implement payment status polling in src/components/Payment.tsx
- [x] T039 [US4] Create OrderList component for order history in src/components/OrderList.tsx
- [x] T040 [US4] Implement photo download in src/components/DownloadButton.tsx

**Checkpoint**: US1-US4 全部完成，应用完整可用

---

## Phase 7: User Story 5 - 现场打印 (Priority: P2) ⚠️ 暂不开发

**Goal**: 用户可以选择现场打印照片

**Note**: 此功能标记为后期集成，暂不实现

### Implementation for User Story 5 (Deferred)

- [ ] T041 [US5] Add print-specific pricing logic (stub only)
- [ ] T042 [US5] Reserve print command slot for future SDK integration

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: 改进和跨用户故事的功能

- [ ] T043 [P] Add network error handling across all API calls
- [ ] T044 [P] Implement session timeout and cleanup
- [ ] T045 Add loading states and transitions
- [ ] T046 Add responsive layout for different screen sizes
- [ ] T047 Security hardening (API key protection)
- [ ] T048 Performance optimization (image caching)
- [ ] T049 [P] Update SPEC.md with final implementation details
- [ ] T050 Build and verify production executable

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 无依赖 - 可以立即开始
- **Foundational (Phase 2)**: 依赖Setup - 阻塞所有用户故事
- **User Stories (Phase 3-7)**: 全部依赖Foundational阶段完成
  - 用户故事可以按优先级并行进行 (P1 → P2)
- **Polish (Phase 8)**: 依赖所有用户故事完成

### User Story Dependencies

- **US1 选择拍照模式 (P1)**: Foundational完成后即可开始 - 无需依赖其他故事
- **US2 预览并选择效果 (P1)**: 依赖Foundational - 集成US1但应独立测试
- **US3 拍照与AI合成 (P1)**: 依赖Foundational - 集成US1, US2
- **US4 付费下载 (P1)**: 依赖Foundational - 集成US3完成后的支付流程
- **US5 现场打印 (P2)**: 延期，暂不开发

### Within Each User Story

- Models → Services → Commands → UI Components
- 核心实现 → 集成
- 故事完成后再进入下一个优先级

### Parallel Opportunities

- Phase 1 Setup 标记[P]的任务可以并行
- Phase 2 Foundational 标记[P]的任务可以并行
- Foundational完成后，所有P1用户故事可以并行进行
- 同一用户故事内标记[P]的model可以并行

---

## Parallel Example: Phase 1 Setup

```bash
# 并行执行 Setup 任务:
Task: "Configure Rust backend dependencies"
Task: "Setup frontend dependencies"
Task: "Create project directory structure"
```

---

## Implementation Strategy

### MVP First (US1-US3)

1. 完成 Phase 1: Setup
2. 完成 Phase 2: Foundational
3. 完成 Phase 3: US1 - 选择拍照模式
4. 完成 Phase 4: US2 - 预览并选择效果
5. 完成 Phase 5: US3 - 拍照与AI合成
6. **STOP and VALIDATE**: 独立测试MVP
7. 部署/演示

### Incremental Delivery

1. Setup + Foundational → 基础设施就绪
2. 添加 US1 → 测试独立 → 部署/演示 (MVP!)
3. 添加 US2 → 测试独立 → 部署/演示
4. 添加 US3 → 测试独立 → 部署/演示
5. 添加 US4 → 测试独立 → 部署/演示
6. 每个故事增加价值且不破坏之前功能

---

## Notes

- [P] 任务 = 不同文件，无依赖
- [Story] 标签将任务映射到特定用户故事以便追踪
- 每个用户故事应独立完成和测试
- 完成每个任务或逻辑组后提交
- 在任何检查点停止以独立验证故事
- 避免: 模糊任务、同一文件冲突、破坏独立性的跨故事依赖
