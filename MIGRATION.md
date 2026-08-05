# BatxRinth Migration & Upstream Compatibility

This document describes how **BatxRinth** preserves compatibility with existing user data while operating as an independent fork.

## Storage Directory Separation

* **BatxRinth Data Directory:**
  * Windows: `%APPDATA%\app.batxrinth.launcher`
  * macOS: `~/Library/Application Support/app.batxrinth.launcher`
  * Linux: `~/.config/app.batxrinth.launcher`

BatxRinth does not overwrite or take over existing installation files from other launchers.

## Database Migrations

* Database migrations are located in `packages/app-lib/migrations/`.
* Migrations use SQLite `PRAGMA user_version` tracking and run idempotently on startup.
* Migration `20260805120000_batxrinth_settings.sql` sets default settings for privacy (`telemetry = 0`, `personalized_ads = 0`, `discord_rpc = 0`).

## Summary of Upstream Feature Changes

| Feature | Upstream Status | BatxRinth Status | Reason |
| :--- | :--- | :--- | :--- |
| **PostHog Analytics** | Enabled | **Removed** | Privacy by default guarantee |
| **In-App Advertisements** | Enabled | **Removed** | Advertisement-free interface guarantee |
| **Modrinth Branding** | Default | **Removed** | Rebranded to BatxRinth per license & COPYING.md |
| **Microsoft Auth** | Supported | **Retained** | Official PKCE system-browser OAuth |
| **Offline Profiles** | N/A | **Added** | Added for lawful local testing/development |
| **Discord RPC** | Basic | **Enhanced** | Default-off with granular privacy controls |
| **Auto-Updater** | Modrinth Host | **Reconfigured** | GitHub Releases distribution |
