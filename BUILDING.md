# Building BatxRinth

This document provides instructions for compiling and packaging **BatxRinth** from source code.

## Prerequisites

### 1. Core Tooling
* **Node.js:** v20.x or v24.x LTS ([nodejs.org](https://nodejs.org/))
* **pnpm:** v10.x (`npm install -g pnpm`)
* **Rust:** Stable toolchain (`rustup default stable`)

### 2. Platform Build Dependencies

#### Windows
* C++ Build Tools (via Visual Studio Build Tools with C++ Desktop Development workload)
* WebView2 Runtime (pre-installed on Windows 10/11)

#### macOS
* Xcode Command Line Tools (`xcode-select --install`)

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
# Compile Vue frontend + Tauri production binary
pnpm turbo run build --filter=theseus_gui
```

Output artifacts:
* **Windows:** `apps/app/target/release/bundle/nsis/BatxRinth_x64-setup.exe`
* **macOS:** `apps/app/target/release/bundle/dmg/BatxRinth.dmg`
* **Linux:** `apps/app/target/release/bundle/deb/batxrinth_amd64.deb`
