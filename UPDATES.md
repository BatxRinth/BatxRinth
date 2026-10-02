# BatxRinth Release & Update Guide

## How users get updates

Release builds include Tauri's signed updater when the `TAURI_SIGNING_PRIVATE_KEY` repository secret is set. Once an hour the app reads:

```
https://github.com/BatxRinth/BatxRinth/releases/latest/download/latest.json
```

When a newer version is listed, the app downloads it, checks its signature against the public key in `apps/app/tauri-release.conf.json`, and offers to restart into it. An update signed with any other key is rejected.

Builds made without the secret fall back to an hourly lookup of `https://api.github.com/repos/BatxRinth/BatxRinth/releases/latest` and a notification with a **Download** button.

Either way, users can turn update checks off in **Settings → Privacy → Check for updates**.

## Publishing a release

Every push to `main` that changes more than Markdown or the website runs `.github/workflows/release.yml`:

1. The release tag is `v` + the `version` in `apps/app-frontend/package.json`, so bump the version (also in `apps/app/Cargo.toml` and `packages/app-lib/Cargo.toml`) before pushing a new release. Pushing again without a bump rebuilds and replaces that version's files.
2. Windows, macOS and Linux build in parallel and upload installers with stable names, so `releases/latest/download/<name>` links on the website keep working.
3. Signed builds also upload `.sig` files and the macOS `.app.tar.gz`, and a final job writes `latest.json` from those signatures.

| Platform | Installer                                                                 | Used by the updater                                          |
| :------- | :------------------------------------------------------------------------ | :----------------------------------------------------------- |
| Windows  | `BatxRinth_x64-setup.exe`                                                 | `BatxRinth_x64-setup.exe` + `.sig`                           |
| macOS    | `BatxRinth_universal.dmg` (Apple Silicon and Intel, not notarized)        | `BatxRinth_universal.app.tar.gz` + `.sig`                    |
| Linux    | `BatxRinth_amd64.AppImage`, `BatxRinth_amd64.deb`, `BatxRinth_x86_64.rpm` | `BatxRinth_amd64.AppImage` + `.sig` (AppImage installs only) |

Pushing a `v*` tag also builds and uploads under that tag.

## The signing key

Whoever holds the private key can ship code that installs itself on every BatxRinth user's machine, so keep it out of the repository and back it up somewhere safe. If it's lost, existing installs can't verify new updates and users have to reinstall manually; if it leaks, rotate it by putting a new public key in `tauri-release.conf.json` and shipping that build as a manual download.

To create a new key: `pnpm --filter=@batxrinth/app exec tauri signer generate -w <path outside the repo>`, then store the private key file's contents as the `TAURI_SIGNING_PRIVATE_KEY` secret.
