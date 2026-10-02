<p align="center">
  <img src="apps/app/icons/icon.png" width="128" alt="BatxRinth logo">
</p>

<h1 align="center">BatxRinth</h1>

<p align="center">
  The Modrinth App with the ads and tracking taken out, plus Ely.by and offline accounts.
</p>

<p align="center">
  <a href="https://github.com/BatxRinth/BatxRinth/releases/latest"><img src="https://img.shields.io/github/v/release/BatxRinth/BatxRinth?color=00E676&label=release" alt="Latest release"></a>
  <a href="https://github.com/BatxRinth/BatxRinth/releases"><img src="https://img.shields.io/github/downloads/BatxRinth/BatxRinth/total?color=00E676" alt="Downloads"></a>
  <a href="apps/app/LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-00E676" alt="License: GPL-3.0"></a>
</p>

## Download

| Platform              | File                                                                                                                                                                                                                                                                                            |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Windows 10/11         | [BatxRinth_x64-setup.exe](https://github.com/BatxRinth/BatxRinth/releases/latest/download/BatxRinth_x64-setup.exe)                                                                                                                                                                              |
| macOS (Apple + Intel) | [BatxRinth_universal.dmg](https://github.com/BatxRinth/BatxRinth/releases/latest/download/BatxRinth_universal.dmg)                                                                                                                                                                              |
| Linux                 | [AppImage](https://github.com/BatxRinth/BatxRinth/releases/latest/download/BatxRinth_amd64.AppImage), [.deb](https://github.com/BatxRinth/BatxRinth/releases/latest/download/BatxRinth_amd64.deb), [.rpm](https://github.com/BatxRinth/BatxRinth/releases/latest/download/BatxRinth_x86_64.rpm) |

The macOS build isn't notarized, so the first time you open it, right-click BatxRinth.app and choose Open. There's also a [download page](https://batxrinth.github.io/BatxRinth/).

## What's different from the Modrinth App

BatxRinth tracks upstream Modrinth closely. Browsing and installing mods and modpacks, instances, worlds, and importing from MultiMC, Prism, ATLauncher, GDLauncher and CurseForge all work the same way. The changes are about what the launcher sends home and which accounts it accepts.

Removed outright, not hidden behind a setting:

- Ads, the ad webview, and the ad-consent popup
- PostHog analytics and Sentry crash reporting
- Tally surveys and promo banners
- The playtime and server-join reports the app uploads to Modrinth's API

[NETWORK.md](NETWORK.md) lists every host BatxRinth talks to and what triggers each request.

Added:

- Ely.by accounts, with 2FA. Games launch through authlib-injector 1.2.8 (pinned and checked by SHA-256), so Ely.by skins and Ely.by servers work in game.
- Offline profiles for local play and testing. On 1.16.4 and 1.16.5 the Multiplayer button works too, where it's normally greyed out for offline accounts.
- Discord Rich Presence that's off until you turn it on. You choose whether it shows the instance name, your play time, and whether you're idle.
- An hourly check for new BatxRinth releases on GitHub, with an off switch in Settings → Privacy.
- Russian and Ukrainian translations for BatxRinth's own screens.

Microsoft accounts work exactly as they do in the Modrinth App. Offline profiles don't prove you own the game; if you play, buy Minecraft.

## Building from source

You need Node 24, pnpm 10, Rust (the version in `rust-toolchain.toml`) and a JDK 17+.

```bash
pnpm install
pnpm app:dev                                      # run the app in dev mode
pnpm --filter=@batxrinth/app exec tauri build     # build installers into target/release/bundle
```

[BUILDING.md](BUILDING.md) covers platform setup. After merging upstream changes, run the end-to-end smoke test described there: it installs and launches a real copy of Minecraft in a throwaway folder.

## More docs

[PRIVACY.md](PRIVACY.md) · [NETWORK.md](NETWORK.md) · [UPDATES.md](UPDATES.md) · [SECURITY.md](SECURITY.md) · [MIGRATION.md](MIGRATION.md) · [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)

## License and credits

BatxRinth is built on [Modrinth's open-source app](https://github.com/modrinth/code) and keeps its license, GPL-3.0-only ([apps/app/LICENSE](apps/app/LICENSE)). It's an independent fork and isn't affiliated with or endorsed by Modrinth, Microsoft, Mojang, or Ely.by.
