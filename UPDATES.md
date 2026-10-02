# BatxRinth Release & Update Guide

## How users get updates today

Release builds do not include Tauri's signed in-app updater. Instead, BatxRinth asks GitHub once an hour for the latest release:

```
https://api.github.com/repos/BatxRinth/BatxRinth/releases/latest
```

When the release's version is newer than the running app, a notification offers a **Download** button that opens the release page. Nothing is downloaded or installed automatically. Users can turn the check off in **Settings → Privacy → Check for updates**.

## Publishing a release

Every push to `main` runs `.github/workflows/release.yml`:

1. The release tag is `v` + the `version` in `apps/app-frontend/package.json`, so bump the version (also in `apps/app/Cargo.toml` and `packages/app-lib/Cargo.toml`) before pushing a new release. Pushing again without a bump rebuilds and replaces that version's files.
2. Windows, macOS and Linux build in parallel and upload installers with stable names, so `releases/latest/download/<name>` links on the website keep working:

| Platform | File                                                                      |
| :------- | :------------------------------------------------------------------------ |
| Windows  | `BatxRinth_x64-setup.exe`                                                 |
| macOS    | `BatxRinth_universal.dmg` (Apple Silicon and Intel, not notarized)        |
| Linux    | `BatxRinth_amd64.AppImage`, `BatxRinth_amd64.deb`, `BatxRinth_x86_64.rpm` |

Pushing a `v*` tag also builds and uploads under that tag.

## Enabling one-click signed updates (optional)

`apps/app/tauri-release.conf.json` holds the configuration for Tauri's signed updater, but it still contains upstream settings and is not used by release builds. Turning it on is a maintainer decision, because whoever holds the signing key can push code that installs itself on every user's machine. To enable it:

1. Generate a key pair with `pnpm --filter=@batxrinth/app exec tauri signer generate -w <path outside the repo>`. Never commit the private key.
2. Put the public key in `plugins.updater.pubkey` in `tauri-release.conf.json`, remove the upstream Windows `signCommand`, and set `bundle.createUpdaterArtifacts` to `true`.
3. Add the private key (and its password, if any) as the `TAURI_SIGNING_PRIVATE_KEY` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` repository secrets.
4. Build with `--config tauri-release.conf.json` (adding the `app` bundle on macOS) and publish a `latest.json` manifest with each release's `.sig` files at the endpoint in `plugins.updater.endpoints`.
