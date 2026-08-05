# BatxRinth Desktop Launcher

**BatxRinth** is an independently branded, privacy-first, advertisement-free Minecraft launcher built with Rust, Tauri v2, Vue 3, and TypeScript.

> [!IMPORTANT]
> **Legal & Non-Affiliation Disclaimer**
> BatxRinth is an independent community fork and is not affiliated with, sponsored by, or endorsed by Modrinth, Rinth, Inc., Microsoft, Mojang, or Discord.

---

## Features

* **Advertisement-Free Interface:** All paid placements, ad webviews, tracking pixels, and consent banners have been completely removed.
* **Privacy by Default:** Zero analytics, telemetry, usage statistics, background tracking, or persistent device fingerprinting.
* **Official Microsoft Authentication:** Full support for legitimate Microsoft/Minecraft accounts using system-browser OAuth 2.0 with PKCE.
* **Offline Local Testing Profiles:** Optional offline profile mode designed for local development, testing, demos, and offline-compatible environments.
* **Granular Discord Rich Presence:** Optional activity integration disabled by default with granular privacy controls.
* **GitHub Releases Auto-Updater:** Integrated release update verification against configured GitHub Releases endpoints.
* **Performance-Oriented Engine:** High-performance Rust core backend for fast instance management, JRE resolution, and parallel modpack processing.

---

## Supported Platforms

* **Windows:** Windows 10 / 11 (x64) via NSIS Installer & standalone binary.
* **macOS:** macOS 11+ (Apple Silicon & Intel) via DMG package.
* **Linux:** Debian/Ubuntu (.deb), AppImage, and tarball distributions.

---

## Quick Start & Building

For comprehensive environment setup and build instructions across all platforms, see [`BUILDING.md`](./BUILDING.md).

```bash
# Install dependencies
pnpm install

# Run frontend + Tauri desktop dev app
pnpm app:dev

# Build production bundle
pnpm --filter=@batxrinth/app build
```

---

## Documentation Index

* [`BUILDING.md`](./BUILDING.md) — Build prerequisites, environment setup, and packaging guide.
* [`PRIVACY.md`](./PRIVACY.md) — Privacy guarantees and local data handling commitments.
* [`NETWORK.md`](./NETWORK.md) — Complete outbound network request inventory.
* [`UPDATES.md`](./UPDATES.md) — Auto-updater design, release key signing, and GitHub Releases distribution.
* [`SECURITY.md`](./SECURITY.md) — Security model, token storage practices, and vulnerability reporting.
* [`MIGRATION.md`](./MIGRATION.md) — Upstream compatibility, schema migration, and data import tooling.
* [`THIRD_PARTY_NOTICES.md`](./THIRD_PARTY_NOTICES.md) — Open source licenses and legal attributions.

---

## License

BatxRinth is licensed under the **GNU General Public License v3.0 (GPL-3.0-only)**. Refer to `LICENSE` and `COPYING.md` for terms.
