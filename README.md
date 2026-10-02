# BatxRinth Desktop Launcher

**BatxRinth** is an independently branded, privacy-first, advertisement-free Minecraft launcher built with Rust, Tauri v2, Vue 3, and TypeScript.

> [!IMPORTANT]
> **Legal & Non-Affiliation Disclaimer**
> BatxRinth is an independent community fork and is not affiliated with, sponsored by, or endorsed by Modrinth, Rinth, Inc., Microsoft, Mojang, or Discord.

---

## Features

- **Advertisement-Free Interface:** All paid placements, ad webviews, tracking pixels, and consent banners have been completely removed.
- **Privacy by Default:** Zero analytics, telemetry, usage statistics, background tracking, or persistent device fingerprinting.
- **Official Microsoft Authentication:** Full support for legitimate Microsoft/Minecraft accounts using system-browser OAuth 2.0 with PKCE.
- **Ely.by Accounts:** Sign in with Ely.by (2FA supported). Games launch through a pinned, checksum-verified authlib-injector, so Ely.by skins and Ely.by-enabled servers just work.
- **Offline Local Testing Profiles:** Optional offline profile mode for local development, testing, demos, and offline-compatible environments, including working multiplayer on Minecraft 1.16.4 and 1.16.5.
- **Granular Discord Rich Presence:** Optional activity integration disabled by default with granular privacy controls.
- **Update Notifications:** An hourly check of BatxRinth's GitHub releases with a one-click download, which you can switch off in Settings → Privacy.
- **Kept Current With Modrinth:** Regularly merged with upstream Modrinth, with a weekly check that flags any tracking or ads code before it can land.
- **Performance-Oriented Engine:** High-performance Rust core backend for fast instance management, JRE resolution, and parallel modpack processing.

---

## Supported Platforms

- **Windows:** Windows 10 / 11 (x64) via NSIS Installer & standalone binary.
- **macOS:** macOS 11+ (Apple Silicon & Intel) via DMG package.
- **Linux:** AppImage, Debian/Ubuntu (.deb), and Fedora/openSUSE (.rpm) packages.

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

- [`BUILDING.md`](./BUILDING.md) — Build prerequisites, environment setup, and packaging guide.
- [`PRIVACY.md`](./PRIVACY.md) — Privacy guarantees and local data handling commitments.
- [`NETWORK.md`](./NETWORK.md) — Complete outbound network request inventory.
- [`UPDATES.md`](./UPDATES.md) — Auto-updater design, release key signing, and GitHub Releases distribution.
- [`SECURITY.md`](./SECURITY.md) — Security model, token storage practices, and vulnerability reporting.
- [`MIGRATION.md`](./MIGRATION.md) — Upstream compatibility, schema migration, and data import tooling.
- [`THIRD_PARTY_NOTICES.md`](./THIRD_PARTY_NOTICES.md) — Open source licenses and legal attributions.

---

## License

BatxRinth is licensed under the **GNU General Public License v3.0 (GPL-3.0-only)**. Refer to `LICENSE` and `COPYING.md` for terms.
