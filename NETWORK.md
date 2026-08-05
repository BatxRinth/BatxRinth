# BatxRinth Outbound Network Request Inventory

This document enumerates every outbound host contacted by **BatxRinth**, the exact user action triggering the connection, and privacy guarantees.

## Network Request Inventory

| Host / Endpoint Domain | Category | Trigger Event | Can Be Disabled? |
| :--- | :--- | :--- | :--- |
| `api.modrinth.com` | Content Metadata | Searching/browsing projects or fetching instance updates | N/A (Core browsing feature) |
| `cdn.modrinth.com` | File Downloads | Downloading mods, modpacks, shaders, or resource packs | N/A (Core download feature) |
| `piston-meta.mojang.com` | Game Manifest | Fetching official Minecraft version manifest | N/A (Core launcher feature) |
| `launchermeta.mojang.com` | Game Manifest | Fetching historical Minecraft manifests | N/A (Core launcher feature) |
| `resources.download.minecraft.net` | Game Assets | Downloading Minecraft sound effects and assets | N/A (Core game launch requirement) |
| `libraries.minecraft.net` | Library Jars | Downloading Minecraft Java dependency libraries | N/A (Core game launch requirement) |
| `textures.minecraft.net` | Player Textures | Rendering player skin textures | N/A (Core UI feature) |
| `login.live.com` | Microsoft OAuth | Initiating Microsoft account sign-in | Yes (Use offline profile mode) |
| `device.auth.xboxlive.com` | Xbox Auth | Authenticating device token during sign-in | Yes (Use offline profile mode) |
| `sisu.xboxlive.com` | Xbox Auth | Authenticating Xbox user token | Yes (Use offline profile mode) |
| `xsts.auth.xboxlive.com` | Xbox Auth | Authenticating XSTS authorization token | Yes (Use offline profile mode) |
| `api.minecraftservices.com` | Minecraft Profile | Fetching user profile, skins, and game license entitlement | Yes (Use offline profile mode) |
| `meta.fabricmc.net` | Modloader Metadata | Resolving Fabric loader versions | N/A (When Fabric instance selected) |
| `maven.neoforged.net` | Modloader Metadata | Resolving NeoForge loader versions | N/A (When NeoForge instance selected) |
| `files.minecraftforge.net` | Modloader Metadata | Resolving Forge loader versions | N/A (When Forge instance selected) |
| `meta.quiltmc.org` | Modloader Metadata | Resolving Quilt loader versions | N/A (When Quilt instance selected) |
| `api.azul.com` | JRE Resolution | Automated detection & download of Java Runtime Environments | Yes (Specify manual Java path) |
| `github.com` | Updates | Checking for BatxRinth app releases on GitHub Releases | Yes (Disable automatic update checks) |
