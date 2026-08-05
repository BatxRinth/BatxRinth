use crate::state::minecraft_auth::{Credentials, MinecraftProfile};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum OfflineProfileError {
    #[error("Username must be between 3 and 16 characters long.")]
    InvalidLength,
    #[error("Username may only contain letters, numbers, and underscores.")]
    InvalidCharacters,
    #[error("User must acknowledge legal ownership responsibility before creating an offline local profile.")]
    AcknowledgementRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineLocalProfileNotice {
    pub title: &'static str,
    pub disclaimer: &'static str,
    pub limitations: &'static str,
}

impl Default for OfflineLocalProfileNotice {
    fn default() -> Self {
        Self {
            title: "Offline Local Testing Profile",
            disclaimer: "This profile mode is intended strictly for lawful local development, testing, demos, and use with locally controlled servers configured to permit offline profiles. This mode does NOT prove game ownership.",
            limitations: "Offline profiles cannot access official Mojang/Microsoft authenticated multiplayer servers, paid online services, or Mojang profile customization APIs.",
        }
    }
}

pub fn sanitize_offline_username(username: &str) -> Result<String, OfflineProfileError> {
    let trimmed = username.trim();
    if trimmed.len() < 3 || trimmed.len() > 16 {
        return Err(OfflineProfileError::InvalidLength);
    }
    if !trimmed.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(OfflineProfileError::InvalidCharacters);
    }
    Ok(trimmed.to_string())
}

pub fn generate_offline_player_uuid(username: &str) -> Uuid {
    let key = format!("OfflinePlayer:{}", username);
    let mut hash = md5::compute(key.as_bytes()).0;
    hash[6] = (hash[6] & 0x0f) | 0x30; // Version 3 MD5
    hash[8] = (hash[8] & 0x3f) | 0x80; // Variant RFC 4122
    Uuid::from_bytes(hash)
}

pub fn create_offline_credentials(
    username: &str,
    acknowledged_lawful_access: bool,
) -> Result<Credentials, OfflineProfileError> {
    if !acknowledged_lawful_access {
        return Err(OfflineProfileError::AcknowledgementRequired);
    }

    let sanitized_name = sanitize_offline_username(username)?;
    let player_uuid = generate_offline_player_uuid(&sanitized_name);

    let profile = MinecraftProfile {
        id: player_uuid,
        name: sanitized_name,
        skins: Vec::new(),
        capes: Vec::new(),
        fetch_time: None,
    };

    Ok(Credentials {
        offline_profile: profile,
        access_token: "OFFLINE_LOCAL_TOKEN".to_string(),
        refresh_token: String::new(),
        expires: Utc::now() + chrono::Duration::days(365 * 10),
        active: true,
    })
}
