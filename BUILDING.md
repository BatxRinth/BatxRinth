# Building BatxRinth

This document provides instructions for compiling and packaging **BatxRinth** from source code.

## Prerequisites

### 1. Core Tooling

- **Node.js:** v20.x or v24.x LTS ([nodejs.org](https://nodejs.org/))
- **pnpm:** v10.x (`npm install -g pnpm`)
- **Rust:** Stable toolchain (`rustup default stable`)

### 2. Platform Build Dependencies

#### Windows

- C++ Build Tools (via Visual Studio Build Tools with C++ Desktop Development workload)
- WebView2 Runtime (pre-installed on Windows 10/11)

#### macOS

- Xcode Command Line Tools (`xcode-select --install`)

#### Linux (Debian/Ubuntu)

```bash
sudo apt update
sudo apt install -y build-essential libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev webkit2gtk-4.1
```

---

## Development Setup

```bash
# Clone repository
git clone https://github.com/BatxRinth/BatxRinth.git
cd BatxRinth

# Install monorepo dependencies
pnpm install

# Run frontend dev server + Tauri window
pnpm app:dev
```

---

## Production Packaging

To compile release binaries and generate platform installers:

```bash
# Compile Vue frontend + Tauri production binary and installers
pnpm --filter=@batxrinth/app exec tauri build
```

Output artifacts:

- **Windows:** `target/release/bundle/nsis/BatxRinth_<version>_x64-setup.exe`
- **macOS:** `target/release/bundle/dmg/BatxRinth_<version>_<arch>.dmg`
- **Linux:** `target/release/bundle/{appimage,deb,rpm}/`

## Verifying a Build End to End

After merging upstream changes, run the smoke test. It applies every database migration, round-trips settings, verifies authlib-injector, creates an offline profile, installs Minecraft 1.16.5 and launches it, then checks the game is still running and was launched with the offline multiplayer fix. It uses only the folder you point it at, never your real launcher data, and opens a Minecraft window for about 45 seconds.

```bash
THESEUS_CONFIG_DIR=/path/to/empty/folder cargo test -p theseus --lib smoke -- --ignored --nocapture
```
