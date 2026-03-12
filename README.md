**`cfgwow`**

*Opinionated terminal UI config editor with type-aware widgets and user-defined schemas*

---

[![Build Status](https://img.shields.io/github/actions/workflow/status/yourname/cfgwow/ci.yml?branch=main&style=flat-square&logo=github&label=ci)](https://github.com/yourname/cfgwow/actions)
[![Crates.io](https://img.shields.io/crates/v/cfgwow?style=flat-square&logo=rust&color=orange)](https://crates.io/crates/cfgwow)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](./LICENSE)
[![Rust: 1.75+](https://img.shields.io/badge/rust-1.75%2B-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Built with Ratatui](https://img.shields.io/badge/built%20with-ratatui-blueviolet?style=flat-square)](https://ratatui.rs)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen?style=flat-square)](./CONTRIBUTING.md)

</div>

---

> **⚠️ Early Development** — `cfgwow` is pre-release. Schemas and CLI interface may change between versions.

---

## What is cfgwow?

`cfgwow` is a terminal UI for editing tool configs without touching raw files. Instead of memorizing syntax, hunting docs, and hand-editing key-value pairs, you get a schema-driven interface with **type-aware widgets** — toggles for booleans, dropdowns for enums, bounded inputs for numbers.

Officially ships with schemas for **Ghostty**, **tmux**, and **Fish shell**. Infinitely extensible via user-authored `.toml` schema files — no plugin system, no scripting, just drop a file in `~/.config/cfgwow/schemas/`.

```
┌──────────────────────────────────────────────────────────────┐
│ cfgwow                                    [?] help  [q] quit │
├─────────────┬────────────────────────────────────────────────┤
│ Tools       │ ▸ Font                                         │
│             │                                                │
│ ▶ Ghostty   │   font-size          [  13  ] ▲ ▼              │
│   tmux      │   font-family        [ JetBrains Mono       ]  │
│   fish      │   font-style         [ Regular      ▼ ]        │
│   ─────     │                                                │
│   + custom  │ ▸ Appearance                                   │
│             │                                                │
│             │   theme              [ catppuccin-frappe     ] │
│             │   background-opacity [  0.95  ] ▲ ▼            │
│             │   cursor-style       [ block         ▼ ]       │
├─────────────┴────────────────────────────────────────────────┤
│ Font size in points. Range: 6–72   Default: 13   Type: int   │
└──────────────────────────────────────────────────────────────┘
```

---

## Features

| | Feature |
|---|---|
| 🎛️ | **Type-aware widgets** — toggles, dropdowns, steppers, text fields per option type |
| 📐 | **Schema-driven** — options, types, ranges, and descriptions defined in plain TOML |
| 🔌 | **User extensibility** — add any tool by dropping a schema into `~/.config/cfgwow/schemas/` |
| 💾 | **Safe write-back** — preserves comments, blank lines, and unknown keys in your config files |
| ⌨️ | **Keyboard-first** — full vim-style navigation, no mouse required |
| 📦 | **Batteries included** — ships with schemas for Ghostty, tmux, and Fish shell |

---

## Demo

> 🎬 *Animated demo coming soon — tracked in [#12](https://github.com/yourname/cfgwow/issues/12)*

---

## Installation

### From crates.io *(once published)*

```bash
cargo install cfgwow
```

### Build from source

```bash
git clone https://github.com/yourname/cfgwow
cd cfgwow
cargo build --release
./target/release/cfgwow
```

**Requirements:** Rust 1.75+ · macOS or Linux · A terminal with true color support

---

## Usage

```bash
cfgwow                  # launch with tool picker
cfgwow ghostty          # open directly to Ghostty schema
cfgwow tmux             # open directly to tmux schema
cfgwow fish             # open directly to Fish shell schema
cfgwow --schema path/to/custom.toml   # load a one-off user schema
```

### Keybindings

| Key | Action |
|---|---|
| `j` / `k` | Navigate options |
| `h` / `l` | Switch between tool list and options panel |
| `Enter` | Edit focused option |
| `Space` | Toggle boolean |
| `s` | Save changes to config file |
| `r` | Reload config file from disk |
| `?` | Help overlay |
| `q` | Quit |

---

## Officially Supported Tools

| Tool | Config Path | Format | Schema |
|---|---|---|---|
| [Ghostty](https://ghostty.org) | `~/.config/ghostty/config` | `key = value` | [`schemas/ghostty.toml`](./schemas/ghostty.toml) |
| [tmux](https://github.com/tmux/tmux) | `~/.config/tmux/tmux.conf` | `set key value` | [`schemas/tmux.toml`](./schemas/tmux.toml) |
| [Fish shell](https://fishshell.com) | `~/.config/fish/config.fish` | `set -U key value` | [`schemas/fish.toml`](./schemas/fish.toml) |

> Fish support is scoped to `set -U` (universal) and `set -g` (global) variable declarations in `config.fish`. Arbitrary fish scripting is out of scope.

---

## User Schemas

Any tool can be supported by writing a `.toml` schema file and placing it in:

```
~/.config/cfgwow/schemas/your-tool.toml
```

`cfgwow` loads all schemas from that directory at startup alongside built-ins.

### Schema Format

A schema file describes the tool, its config file location, the file format, and each available option.

```toml
# ~/.config/cfgwow/schemas/alacritty.toml

[tool]
name        = "Alacritty"
config_path = "~/.config/alacritty/alacritty.toml"
format      = "toml"          # hint to the parser: "key = value" | "set key value" | "toml"
description = "GPU-accelerated terminal emulator"

# ── Options ───────────────────────────────────────────────────

[[options]]
key         = "font.size"
type        = "float"
default     = 12.0
min         = 6.0
max         = 72.0
description = "Font size in points"
section     = "Font"

[[options]]
key         = "window.opacity"
type        = "float"
default     = 1.0
min         = 0.0
max         = 1.0
description = "Background opacity (0.0 = transparent, 1.0 = opaque)"
section     = "Window"

[[options]]
key         = "cursor.style"
type        = "enum"
default     = "Block"
values      = ["Block", "Underline", "Beam"]
description = "Cursor shape"
section     = "Cursor"

[[options]]
key         = "live_config_reload"
type        = "bool"
default     = true
description = "Automatically reload config on file change"
section     = "Misc"
```

### Supported Option Types

| Type | Widget | Extra Fields |
|---|---|---|
| `bool` | Toggle | — |
| `string` | Text input | — |
| `int` | Numeric stepper | `min`, `max` (optional) |
| `float` | Numeric stepper | `min`, `max` (optional) |
| `enum` | Dropdown | `values` (required) |
| `color` | Text input + hex validation | — |

---

## Project Architecture

```
cfgwow/
├── schemas/                        # bundled schemas (compiled in via include_str!)
│   ├── ghostty.toml
│   ├── tmux.toml
│   └── fish.toml
│
└── src/
    ├── main.rs                     # entry point, arg parsing, app init
    ├── schema.rs                   # schema loader, type definitions, validation
    ├── config_file.rs              # read/write actual config files
    ├── state.rs                    # in-memory representation of pending edits
    │
    ├── parsers/
    │   ├── mod.rs                  # parser trait + dispatcher
    │   ├── key_equals_value.rs     # ghostty-style  (key = value)
    │   ├── key_space_value.rs      # tmux-style     (set key value)
    │   └── fish_set.rs             # fish-style     (set -U/-g key value)
    │
    └── tui/
        ├── mod.rs
        ├── app.rs                  # top-level event loop, input handling
        ├── tool_list.rs            # left panel widget
        ├── option_list.rs          # center panel widget
        └── widgets.rs              # toggle, dropdown, text input, numeric stepper
```

---

## Roadmap

This roadmap tracks the full MVP build from zero to a usable v0.1.0 release. Items are roughly ordered by dependency — each phase should be completable and testable before moving to the next.

---

### Phase 1 — Schema & Data Model

> Goal: round-trip a Ghostty config with zero TUI code.

- [x] **1.1** Define `Schema` and `OptionDef` types in `schema.rs`
  - [x] 1.1.1 — Represent all supported types: `bool`, `string`, `int`, `float`, `enum`, `color`
  - [x] 1.1.2 — Optional fields: `min`, `max`, `values`, `default`, `description`, `section`
  - [x] 1.1.3 — Tool metadata: `name`, `config_path`, `format`, `description`
- [x] **1.2** TOML schema loader
  - [x] 1.2.1 — Parse a single `.toml` schema file into `Schema` struct
  - [x] 1.2.2 — Emit useful errors for malformed or missing required fields
  - [x] 1.2.3 — Load bundled schemas via `include_str!` at compile time
  - [x] 1.2.4 — Scan `~/.config/cfgwow/schemas/` and merge with built-ins
- [x] **1.3** Config file reader — `key = value` parser (Ghostty format)
  - [x] 1.3.1 — Parse into `Vec<Line>` (comment, key-value, blank, unknown)
  - [x] 1.3.2 — Build `HashMap<key, raw_value>` for schema overlay
  - [x] 1.3.3 — Handle `~` expansion in config paths
- [x] **1.4** In-memory state (`state.rs`)
  - [x] 1.4.1 — `AppState`: loaded schema + parsed config lines + pending edits map
  - [x] 1.4.2 — `apply_edit(key, value)` — stage a change without writing to disk
  - [x] 1.4.3 — `resolve_value(key)` — return edit if staged, else parsed, else default
- [x] **1.5** Config file writer
  - [x] 1.5.1 — Walk `Vec<Line>`, substitute staged edits in-place
  - [x] 1.5.2 — Append any new keys not previously in the file
  - [x] 1.5.3 — Preserve comments, ordering, and unknown keys verbatim
  - [x] 1.5.4 — Write to a temp file then rename (atomic write)
- [x] **1.6** Integration test: read → mutate → write → re-read Ghostty config and assert round-trip

---

### Phase 2 — Parser Coverage

> Goal: support all three official config formats.

- [ ] **2.1** tmux parser — `set [-g|-s] key value` format
  - [ ] 2.1.1 — Handle optional flags (`-g`, `-s`, `-w`) without breaking key extraction
  - [ ] 2.1.2 — Write-back preserves `set` prefix and flags
- [ ] **2.2** Fish parser — `set [-U|-g] key value` format
  - [ ] 2.2.1 — Only target `set -U` and `set -g` declarations
  - [ ] 2.2.2 — Skip and preserve all other fish scripting verbatim
  - [ ] 2.2.3 — Write-back does not alter non-`set` lines
- [ ] **2.3** Parser trait / dispatcher (`parsers/mod.rs`)
  - [ ] 2.3.1 — Define `ConfigParser` trait with `read` and `write` methods
  - [ ] 2.3.2 — Select parser at runtime based on schema `format` field
- [ ] **2.4** Integration tests for tmux and Fish round-trips

---

### Phase 3 — TUI Skeleton

> Goal: navigable three-panel layout rendering hardcoded data, no real schema yet.

- [ ] **3.1** Project setup
  - [ ] 3.1.1 — Add `ratatui` and `crossterm` to `Cargo.toml`
  - [ ] 3.1.2 — Terminal init/teardown boilerplate (raw mode, alternate screen, panic hook)
- [ ] **3.2** Layout (`tui/app.rs`)
  - [ ] 3.2.1 — Three-zone layout: left tool list, center options panel, bottom status bar
  - [ ] 3.2.2 — Focus management: track which panel is active
  - [ ] 3.2.3 — Event loop: keyboard input → state mutation → re-render
- [ ] **3.3** Tool list panel (`tui/tool_list.rs`)
  - [ ] 3.3.1 — Render tool names, highlight selected
  - [ ] 3.3.2 — Navigate with `j`/`k`, select with `Enter` or `l`
- [ ] **3.4** Options panel (`tui/option_list.rs`)
  - [ ] 3.4.1 — Render options grouped under section headers
  - [ ] 3.4.2 — Scrollable list with cursor tracking
  - [ ] 3.4.3 — Highlight focused option row
- [ ] **3.5** Status bar
  - [ ] 3.5.1 — Show description + type + default for focused option
  - [ ] 3.5.2 — Show context-sensitive key hint strip
- [ ] **3.6** Global keybindings: `q` to quit, `?` help overlay

---

### Phase 4 — Widget Layer

> Goal: each option type renders and edits correctly.

- [ ] **4.1** Toggle widget (`bool`)
  - [ ] 4.1.1 — Render `[✓]` / `[ ]`, toggle with `Space`
- [ ] **4.2** Text input widget (`string`, `color`)
  - [ ] 4.2.1 — Inline edit mode, cursor movement, backspace
  - [ ] 4.2.2 — `color` type validates hex on confirm
- [ ] **4.3** Numeric stepper widget (`int`, `float`)
  - [ ] 4.3.1 — `▲`/`▼` keys increment/decrement by step
  - [ ] 4.3.2 — Clamp to `min`/`max` if defined
  - [ ] 4.3.3 — Allow direct text entry as fallback
- [ ] **4.4** Dropdown widget (`enum`)
  - [ ] 4.4.1 — Open a small popup list of `values` on `Enter`
  - [ ] 4.4.2 — Navigate and confirm with `Enter`, dismiss with `Esc`
- [ ] **4.5** Widget dispatch — select widget type from `OptionDef.type` at render time

---

### Phase 5 — Wire Schema into TUI

> Goal: TUI renders live data from real schema + config files.

- [ ] **5.1** Load schemas at startup, populate tool list
- [ ] **5.2** On tool selection, load the corresponding config file and build `AppState`
- [ ] **5.3** Option list renders from `AppState` — live values, not hardcoded
- [ ] **5.4** Edits stage into `AppState.pending_edits`
- [ ] **5.5** `s` key triggers atomic write-back via the correct parser
- [ ] **5.6** `r` key reloads config from disk, discarding unsaved edits (with confirmation prompt)
- [ ] **5.7** Unsaved changes indicator in status bar

---

### Phase 6 — User Schema Support

> Goal: drop a `.toml` in `~/.config/cfgwow/schemas/` and it just works.

- [ ] **6.1** Scan user schema directory at startup
- [ ] **6.2** Merge user schemas into tool list after built-ins
- [ ] **6.3** Deduplicate: user schema with same `tool.name` overrides built-in
- [ ] **6.4** Graceful error display for malformed user schemas (don't crash, show inline warning)
- [ ] **6.5** `cfgwow --schema <path>` flag to load a one-off schema without installing it

---

### Phase 7 — Polish & Release

> Goal: v0.1.0 — stable enough to daily-drive on your own machine.

- [ ] **7.1** Error handling audit — replace all `unwrap()` with proper error propagation
- [ ] **7.2** Config path edge cases — missing file, wrong permissions, `~` expansion on all platforms
- [ ] **7.3** Write full built-in schemas
  - [ ] 7.3.1 — Ghostty: ~20 options across Font, Appearance, Cursor, Window sections
  - [ ] 7.3.2 — tmux: ~20 options across Display, Behavior, Keys, Mouse sections
  - [ ] 7.3.3 — Fish: ~10 universal variables across Prompt, Editor, History sections
- [ ] **7.4** README animated demo (record with `vhs` or `asciinema`)
- [ ] **7.5** `cargo publish` to crates.io
- [ ] **7.6** GitHub Actions CI — `cargo test`, `cargo clippy`, `cargo fmt --check`

---

### Backlog (Post-MVP)

> Not committed to any milestone. Ideas to revisit after v0.1.0.

- [ ] Search / fuzzy filter across options (`/` key)
- [ ] Undo/redo stack for in-session edits
- [ ] Config file diff view before saving
- [ ] Option to open raw config file in `$EDITOR`
- [ ] Multiple config profiles per tool
- [ ] Live file watching — detect external edits and prompt reload
- [ ] Mouse support
- [ ] Windows / WSL testing

---

## Contributing

`cfgwow` is early-stage and contributions are very welcome — especially new built-in schemas and parser improvements.

### Getting Started

```bash
git clone https://github.com/yourname/cfgwow
cd cfgwow
cargo build
cargo test
```

### Areas Where Help is Wanted

- 📋 **New schemas** — written a schema for a tool you use? Open a PR to add it to `schemas/`
- 🐛 **Parser edge cases** — config files are messy in the wild; bug reports with real config snippets are gold
- 🎨 **TUI improvements** — ratatui widget ideas, layout feedback, color theme tweaks
- 📖 **Documentation** — improving schema format docs, usage examples, or this README

### Submitting a Schema

The fastest way to contribute is a new built-in schema. To add one:

1. Create `schemas/your-tool.toml` following the [schema format](#schema-format) above
2. Add it to the `schemas/` directory and reference it in `schema.rs` via `include_str!`
3. Open a PR with a brief description of the tool and which options you've covered
4. Include a sample config snippet so the parser can be tested against real input

### PR Guidelines

- Keep PRs focused — one feature or fix per PR
- Run `cargo fmt` and `cargo clippy` before submitting
- Add tests for any new parser logic
- Schema-only PRs don't need tests, but do need a sample config in the PR description

### Opening Issues

When filing a bug, please include:
- Your OS and terminal emulator
- The config file snippet that caused the issue
- The schema being used (built-in name or your custom file)
- Output of `cfgwow --version`

---

## License

MIT — see [LICENSE](./LICENSE)

---

<div align="center">

built with [ratatui](https://ratatui.rs) · written in [Rust](https://www.rust-lang.org) · crafted for terminal people

</div>
