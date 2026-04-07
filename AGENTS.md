# AGENTS.md — AI Agent Guidance

> **Canonical guidance lives in [`Bevy/AGENTS.md`](./Bevy/AGENTS.md).**
> Read that file before making any changes.
> The active project root is `Bevy/`. All paths below are relative to it.
> Naming rule: inside `Bevy/src`, every file and folder must use Title Case except the root `src` folder itself.

---

## What this project is

A **Bevy 0.18 / Rust game template** with out-of-the-box builds for:
- Native (Windows, Linux, macOS) via `cargo run`
- Web (WASM) via `trunk serve`
- Mobile (Android / iOS) via the `mobile/` workspace crate

---

## Quick layout reference

```
Bevy/
├── src/
│   ├── Runtime/
│   │   ├── Client/
│   │   │   ├── Lib.rs              ← GamePlugin, GameState
│   │   │   ├── Main.rs             ← binary + web entrypoint
│   │   │   ├── Components/         ← *Component.rs files
│   │   │   ├── Plugins/            ← *Plugin.rs files (feature-level plugins)
│   │   │   ├── Resources/          ← *Resource.rs files
│   │   │   └── Systems/            ← *System.rs files (system-heavy plugins)
│   │   └── Server/                 ← future headless scaffold
│   └── Tests/                      ← headless in-crate test modules
├── assets/
├── build/
├── mobile/
└── Cargo.toml
```

See [`Bevy/AGENTS.md`](./Bevy/AGENTS.md) for the full file listing, conventions, and workflow.