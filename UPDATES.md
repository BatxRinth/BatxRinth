# BatxRinth Release & Update Guide

BatxRinth utilizes Tauri v2's signature-verified auto-updater plugin, configured to pull releases directly from **GitHub Releases**.

## Release Manifest Architecture

Auto-update checks query:
```
https://github.com/BatxRinth/BatxRinth/releases/latest/download/latest.json
```

### Manifest Schema (`latest.json`)

```json
{
  "version": "1.0.0",
  "notes": "BatxRinth Release 1.0.0",
  "pub_date": "2026-08-05T12:00:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "dW50cnVzdGVkIGNvbW1lbnQ6...",
      "url": "https://github.com/BatxRinth/BatxRinth/releases/download/v1.0.0/BatxRinth_1.0.0_x64-setup.nsis.zip"
    },
    "darwin-aarch64": {
      "signature": "dW50cnVzdGVkIGNvbW1lbnQ6...",
      "url": "https://github.com/BatxRinth/BatxRinth/releases/download/v1.0.0/BatxRinth_1.0.0_aarch64.app.tar.gz"
    },
    "linux-x86_64": {
      "signature": "dW50cnVzdGVkIGNvbW1lbnQ6...",
      "url": "https://github.com/BatxRinth/BatxRinth/releases/download/v1.0.0/BatxRinth_1.0.0_amd64.AppImage.tar.gz"
    }
  }
}
```

---

## Generating Release Signing Keys

Tauri updater uses `minisign` key pairs:

```bash
# Generate key pair using Tauri CLI
npx tauri signer generate -w ~/.tauri/batxrinth.key
```

* **Public Key:** Save in `apps/app/tauri-release.conf.json` under `plugins.updater.pubkey`.
* **Private Key:** Set as environment variable `TAURI_SIGNING_PRIVATE_KEY` during CI/CD build workflows. **NEVER COMMIT PRIVATE KEYS TO GIT**.

---

## Publishing a Release to GitHub

1. Tag the repository release:
   ```bash
   git tag -a v1.0.0 -m "Release v1.0.0"
   git push origin v1.0.0
   ```
2. Build production artifacts with `TAURI_SIGNING_PRIVATE_KEY` set.
3. Upload installer packages (`.nsis.zip`, `.app.tar.gz`, `.AppImage.tar.gz`) along with `latest.json` to the GitHub Release.
