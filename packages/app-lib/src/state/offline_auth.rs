use crate::state::minecraft_auth::{Credentials, MinecraftProfile};
use chrono::Utc;
use thiserror::Error;
use uuid::Uuid;

/// Access token stored for offline local profiles.
///
/// Minecraft never validates this value for singleplayer or LAN play, and it lets
/// the rest of the launcher recognise credentials that must never reach Mojang.
pub const OFFLINE_ACCESS_TOKEN: &str = "OFFLINE_LOCAL_TOKEN";

#[derive(Error, Debug)]
pub enum OfflineProfileError {
    #[error("Username must be between 3 and 16 characters long.")]
    InvalidLength,
    #[error("Username may only contain letters, numbers, and underscores.")]
    InvalidCharacters,
    #[error(
        "User must acknowledge legal ownership responsibility before creating an offline local profile."
    )]
    AcknowledgementRequired,
    #[error(
        "This action requires a Microsoft account. Offline local profiles cannot use Mojang online services."
    )]
    OnlineAccountRequired,
}

pub fn sanitize_offline_username(
    username: &str,
) -> Result<String, OfflineProfileError> {
    let trimmed = username.trim();
    if trimmed.len() < 3 || trimmed.len() > 16 {
        return Err(OfflineProfileError::InvalidLength);
    }
    if !trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err(OfflineProfileError::InvalidCharacters);
    }
    Ok(trimmed.to_string())
}

pub fn generate_offline_player_uuid(username: &str) -> Uuid {
    let key = format!("OfflinePlayer:{username}");
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
        access_token: OFFLINE_ACCESS_TOKEN.to_string(),
        refresh_token: String::new(),
        expires: Utc::now() + chrono::Duration::days(365 * 10),
        active: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Offline UUIDs must match vanilla's `UUID.nameUUIDFromBytes("OfflinePlayer:<name>")`,
    /// otherwise worlds and LAN sessions created by other launchers lose their player data.
    #[test]
    fn offline_uuids_match_vanilla() {
        for (username, expected) in [
            ("Notch", "b50ad385-829d-3141-a216-7e7d7539ba7f"),
            ("jeb_", "a762f560-4fce-3236-812a-b80efff0b62b"),
            ("Player", "a01e3843-e521-3998-958a-f459800e4d11"),
        ] {
            assert_eq!(
                generate_offline_player_uuid(username)
                    .hyphenated()
                    .to_string(),
                expected,
                "offline UUID mismatch for {username}"
            );
        }
    }

    #[test]
    fn usernames_are_validated() {
        assert_eq!(sanitize_offline_username("  Player  ").unwrap(), "Player");
        assert!(sanitize_offline_username("ab").is_err());
        assert!(sanitize_offline_username("a".repeat(17).as_str()).is_err());
        assert!(sanitize_offline_username("has space").is_err());
        assert!(sanitize_offline_username("émoji").is_err());
    }

    #[test]
    fn offline_credentials_are_recognisable_as_offline() {
        let credentials = create_offline_credentials("Player", true).unwrap();
        assert_eq!(credentials.access_token, OFFLINE_ACCESS_TOKEN);
        assert!(credentials.is_offline());
        assert!(create_offline_credentials("Player", false).is_err());
    }
}
