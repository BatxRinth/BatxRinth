-- Disable telemetry, ads, and discord RPC by default in BatxRinth
UPDATE settings SET telemetry = 0, personalized_ads = 0, discord_rpc = 0 WHERE id = 0;

ALTER TABLE settings ADD COLUMN discord_rpc_show_launcher_activity INTEGER NOT NULL DEFAULT 1;
ALTER TABLE settings ADD COLUMN discord_rpc_show_instance_name INTEGER NOT NULL DEFAULT 1;
ALTER TABLE settings ADD COLUMN discord_rpc_show_version INTEGER NOT NULL DEFAULT 1;
ALTER TABLE settings ADD COLUMN discord_rpc_show_modloader INTEGER NOT NULL DEFAULT 1;
ALTER TABLE settings ADD COLUMN discord_rpc_show_play_time INTEGER NOT NULL DEFAULT 1;
ALTER TABLE settings ADD COLUMN discord_rpc_show_afk INTEGER NOT NULL DEFAULT 1;
ALTER TABLE settings ADD COLUMN discord_rpc_hide_private_instances INTEGER NOT NULL DEFAULT 0;
ALTER TABLE settings ADD COLUMN discord_rpc_clear_on_exit INTEGER NOT NULL DEFAULT 1;
