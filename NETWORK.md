# BatxRinth Outbound Network Request Inventory

This document enumerates every outbound host contacted by **BatxRinth**, the exact user action triggering the connection, and privacy guarantees.

## Network Request Inventory

| Host / Endpoint Domain             | Category           | Trigger Event                                               | Can Be Disabled?                      |
| :--------------------------------- | :----------------- | :---------------------------------------------------------- | :------------------------------------ |
| `api.modrinth.com`                 | Content Metadata   | Searching/browsing projects or fetching instance updates    | N/A (Core browsing feature)           |
| `cdn.modrinth.com`                 | File Downloads     | Downloading mods, modpacks, shaders, or resource packs      | N/A (Core download feature)           |
| `piston-meta.mojang.com`           | Game Manifest      | Fetching official Minecraft version manifest                | N/A (Core launcher feature)           |
| `launchermeta.mojang.com`          | Game Manifest      | Fetching historical Minecraft manifests                     | N/A (Core launcher feature)           |
| `resources.download.minecraft.net` | Game Assets        | Downloading Minecraft sound effects and assets              | N/A (Core game launch requirement)    |
| `libraries.minecraft.net`          | Library Jars       | Downloading Minecraft Java dependency libraries             | N/A (Core game launch requirement)    |
| `textures.minecraft.net`           | Player Textures    | Rendering player skin textures                              | N/A (Core UI feature)                 |
| `login.live.com`                   | Microsoft OAuth    | Initiating Microsoft account sign-in                        | Yes (Use offline profile mode)        |
| `device.auth.xboxlive.com`         | Xbox Auth          | Authenticating device token during sign-in                  | Yes (Use offline profile mode)        |
| `sisu.xboxlive.com`                | Xbox Auth          | Authenticating Xbox user token                              | Yes (Use offline profile mode)        |
| `xsts.auth.xboxlive.com`           | Xbox Auth          | Authenticating XSTS authorization token                     | Yes (Use offline profile mode)        |
| `api.minecraftservices.com`        | Minecraft Profile  | Fetching user profile, skins, and game license entitlement  | Yes (Use offline profile mode)        |
| `meta.fabricmc.net`                | Modloader Metadata | Resolving Fabric loader versions                            | N/A (When Fabric instance selected)   |
| `maven.neoforged.net`              | Modloader Metadata | Resolving NeoForge loader versions                          | N/A (When NeoForge instance selected) |
| `files.minecraftforge.net`         | Modloader Metadata | Resolving Forge loader versions                             | N/A (When Forge instance selected)    |
| `meta.quiltmc.org`                 | Modloader Metadata | Resolving Quilt loader versions                             | N/A (When Quilt instance selected)    |
| `api.azul.com`                     | JRE Resolution     | Automated detection & download of Java Runtime Environments | Yes (Specify manual Java path)        |
| `github.com`                       | Updates            | Checking for BatxRinth app releases on GitHub Releases      | Yes (Disable automatic update checks) |

Every request above is a direct consequence of an action you took: browsing, downloading, launching, signing in, or checking for updates. Nothing is sent on a timer, on startup, or in the background for measurement purposes.

## Removed Upstream Reporting

Upstream Modrinth builds report usage back to `api.modrinth.com`. BatxRinth removes these entirely; the code paths are deleted rather than disabled behind a setting:

| Upstream Endpoint                               | What It Reported                                                                                               | Status in BatxRinth                                       |
| :---------------------------------------------- | :------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------- |
| `analytics/playtime`                            | Seconds played per installed project version, loader, and game version, uploaded when an instance closes       | Removed. Playtime is only totalled in the local database. |
| `analytics/minecraft-server-play`               | Your username and the project ID of a server you joined, preceded by a `sessionserver.mojang.com` session join | Removed, along with the session join it depended on.      |
| Sentry crash reporting                          | Stack traces and app context                                                                                   | Removed.                                                  |
| Ad webviews, Tally surveys, and consent banners | Impressions and survey responses                                                                               | Removed.                                                  |

## Request Metadata

Content downloads from `api.modrinth.com` and `cdn.modrinth.com` carry a `modrinth-download-meta` header describing the download itself: whether it is a new install, modpack, or update, plus the target game version, loader, and parent pack. This is how Modrinth attributes downloads to the authors whose files you are fetching. It contains no account identifier, device identifier, or persistent ID, and it is only ever attached to a file transfer you started.

## Offline Local Profiles

An account created through **Offline Local Profile** never contacts Mojang or Microsoft. It has no valid token, so BatxRinth skips profile lookups, skin and cape operations, and session joins for it instead of sending requests that would be rejected. Launching with an offline profile makes no authentication request of any kind.
