use std::sync::{Arc, atomic::AtomicBool};

use chrono::{DateTime, Utc};
use discord_rich_presence::{
    DiscordIpc, DiscordIpcClient,
    activity::{Activity, Assets, Timestamps},
};
use tokio::sync::RwLock;

use crate::State;

pub struct DiscordGuard {
    client: Arc<RwLock<DiscordIpcClient>>,
    connected: Arc<AtomicBool>,
}

impl DiscordGuard {
    /// Initialize discord IPC client, and attempt to connect to it
    /// If it fails, it will still return a DiscordGuard, but the client will be unconnected
    pub fn init() -> crate::Result<DiscordGuard> {
        let dipc = DiscordIpcClient::new("1555641295739953283");

        Ok(DiscordGuard {
            client: Arc::new(RwLock::new(dipc)),
            connected: Arc::new(AtomicBool::new(false)),
        })
    }

    /// If the client failed connecting during init(), this will check for connection and attempt to reconnect
    /// This MUST be called first in any client method that requires a connection, because those can PANIC if the client is not connected
    /// (No connection is different than a failed connection, the latter will not panic and can be retried)
    pub async fn retry_if_not_ready(&self) -> bool {
        let mut client = self.client.write().await;
        if !self.connected.load(std::sync::atomic::Ordering::Relaxed) {
            if client.connect().is_ok() {
                self.connected
                    .store(true, std::sync::atomic::Ordering::Relaxed);
                return true;
            }
            return false;
        }
        true
    }

    /// Shows what the launcher is doing in Discord, honouring the user's Rich
    /// Presence settings. With no game passed in, the most recently started running
    /// game is shown, or the idle status when nothing is running.
    pub async fn update_presence(
        &self,
        playing: Option<(&str, DateTime<Utc>)>,
    ) -> crate::Result<()> {
        let state = State::get().await?;
        let settings = crate::state::Settings::get(&state.pool).await?;
        if !settings.discord_rpc {
            return self.clear_activity(true).await;
        }

        let running = state
            .process_manager
            .get_all()
            .into_iter()
            .max_by_key(|process| process.start_time);
        let playing = playing.or_else(|| {
            running.as_ref().map(|process| {
                (process.instance_name.as_str(), process.start_time)
            })
        });

        let options = PresenceOptions {
            show_instance_name: settings.discord_rpc_show_instance_name,
            show_play_time: settings.discord_rpc_show_play_time,
            show_launcher_activity: settings.discord_rpc_show_launcher_activity,
        };
        match presence_for(&options, playing) {
            Some((text, started)) => {
                self.force_set_activity(&text, started, true).await
            }
            None => self.clear_activity(true).await,
        }
    }

    /// Sets the activity to the given message, regardless of if discord is disabled or offline
    /// Should not be used except for in the above method, or if it is already known that discord is enabled (specifically for state initialization) and we are connected to the internet
    pub async fn force_set_activity(
        &self,
        msg: &str,
        started: Option<i64>,
        reconnect_if_fail: bool,
    ) -> crate::Result<()> {
        // Attempt to connect if not connected. Do not continue if it fails, as the client.set_activity can panic if it never was connected
        if !self.retry_if_not_ready().await {
            return Ok(());
        }

        let mut activity = Activity::new().state(msg).assets(
            Assets::new()
                .large_image("batxrinth_logo")
                .large_text("BatxRinth Launcher"),
        );
        if let Some(started) = started {
            activity = activity.timestamps(Timestamps::new().start(started));
        }

        // Attempt to set the activity
        // If the existing connection fails, attempt to reconnect and try again
        let mut client: tokio::sync::RwLockWriteGuard<'_, DiscordIpcClient> =
            self.client.write().await;
        let res = client.set_activity(activity.clone());

        if reconnect_if_fail {
            if let Err(_e) = res {
                client.reconnect()?;
                return Ok(client.set_activity(activity)?); // try again, but don't reconnect if it fails again
            }
        } else {
            res?;
        }

        Ok(())
    }

    /// Clear the activity entirely ('disabling' the RPC until the next update_presence)
    pub async fn clear_activity(
        &self,
        reconnect_if_fail: bool,
    ) -> crate::Result<()> {
        // Attempt to connect if not connected. Do not continue if it fails, as the client.clear_activity can panic if it never was connected
        if !self.retry_if_not_ready().await {
            return Ok(());
        }

        // Attempt to clear the activity
        // If the existing connection fails, attempt to reconnect and try again
        let mut client = self.client.write().await;
        let res = client.clear_activity();

        if reconnect_if_fail {
            if res.is_err() {
                client.reconnect()?;
                return Ok(client.clear_activity()?); // try again, but don't reconnect if it fails again
            }
        } else {
            res?;
        }
        Ok(())
    }
}

/// Which parts of the activity the user agreed to share.
struct PresenceOptions {
    show_instance_name: bool,
    show_play_time: bool,
    show_launcher_activity: bool,
}

/// The status text and start timestamp to show, or `None` to show nothing.
fn presence_for(
    options: &PresenceOptions,
    playing: Option<(&str, DateTime<Utc>)>,
) -> Option<(String, Option<i64>)> {
    match playing {
        Some((instance_name, started)) => {
            let text = if options.show_instance_name {
                format!("Playing {instance_name}")
            } else {
                "Playing Minecraft".to_string()
            };
            Some((text, options.show_play_time.then(|| started.timestamp())))
        }
        None if options.show_launcher_activity => {
            Some(("Idling...".to_string(), None))
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options(name: bool, time: bool, idle: bool) -> PresenceOptions {
        PresenceOptions {
            show_instance_name: name,
            show_play_time: time,
            show_launcher_activity: idle,
        }
    }

    #[test]
    fn presence_respects_privacy_options() {
        let started = DateTime::from_timestamp(1_700_000_000, 0).unwrap();
        let playing = Some(("Secret Pack", started));

        assert_eq!(
            presence_for(&options(true, true, true), playing),
            Some(("Playing Secret Pack".into(), Some(1_700_000_000)))
        );
        assert_eq!(
            presence_for(&options(false, false, true), playing),
            Some(("Playing Minecraft".into(), None))
        );
        assert_eq!(
            presence_for(&options(true, true, true), None),
            Some(("Idling...".into(), None))
        );
        assert_eq!(presence_for(&options(true, true, false), None), None);
    }
}
