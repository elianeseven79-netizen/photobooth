# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

AI-powered photobooth desktop application built with Tauri 2.x (Rust backend + React frontend). Users select photo styles/effects, take photos, and AI generates composite images that can be paid for and downloaded via WeChat Pay.

## Development Commands

```bash
# Frontend development
npm run dev          # Start Vite dev server (port 1420)
npm run build        # Build TypeScript and Vite production bundle
npm run preview      # Preview production build

# Full Tauri app
npm run tauri dev    # Start Tauri dev mode (hot reload)
npm run tauri build # Build production Tauri app
```

## Architecture

**Frontend** (`src/`): React 19 + TypeScript + React Router + Vite
- `src/components/` - UI components (Camera, PhotoPreview, Payment, OrderList, ModeSelect, EffectSelect)
- `src/services/api.ts` - API service layer
- `src/types/` - TypeScript type definitions

**Backend** (`src-tauri/`): Rust + Tauri 2.x
- Uses rusqlite (bundled SQLite) for local session/order storage
- Integrates with MiniMax AI API for image generation
- WeChat Pay API integration for payments

**Speckit Workflow**: Feature-driven development using `.specify/` templates. Features are developed in feature branches with specs stored in `specs/[number]-[feature-name]/` directories.
