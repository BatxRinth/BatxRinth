# BatxRinth Privacy Policy & Guarantees

**BatxRinth** is built with privacy as a foundational principle.

## Summary Guarantees

1. **No Analytics or Telemetry:** BatxRinth contains zero tracking SDKs, event capturing code, usage statistics, background beacons, or advertising identifiers.
2. **Local Logs Stay Local:** System and game logs are stored strictly on your local disk. Log outputs are scrubbed to prevent accidental token leakage.
3. **No Central Profiling:** BatxRinth does not assign or transmit a persistent installation ID or hardware fingerprint.
4. **Transparent Outbound Traffic:** Every network request made by BatxRinth is explicitly user-initiated (e.g. downloading a modpack, fetching game metadata, or authenticating via Microsoft).

## What Data Remains Local

* Game profiles, installed mods, resource packs, and instance configurations.
* Local SQLite database (`state.db`).
* Account session cache (encrypted in system keychain / secure store where available).
* Application log files in your operating system's standard app data path.

## User-Initiated Outbound Connections

When performing specific user actions, BatxRinth connects directly to relevant external endpoints:
* **Modrinth API & CDN:** For fetching project information, mod search results, and downloading mod files.
* **Mojang & Minecraft Servers:** For fetching official game version manifests, asset libraries, client jars, and skin textures.
* **Microsoft OAuth:** Directly with official Microsoft endpoints (`login.live.com`, `xboxlive.com`) during account sign-in.
* **GitHub Releases:** For optional application update checking.

Refer to [`NETWORK.md`](./NETWORK.md) for a complete host-by-host breakdown.
