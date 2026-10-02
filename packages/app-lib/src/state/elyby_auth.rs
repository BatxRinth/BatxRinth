//! Ely.by accounts: signed in through Ely.by's Yggdrasil-compatible auth server
//! and launched through authlib-injector, so skins and online-mode servers that
//! trust Ely.by work in game.
//!
//! Ely.by credentials reuse the regular `minecraft_users` row. The refresh token
//! column holds [`ELYBY_REFRESH_PREFIX`] followed by the Yggdrasil client token,
//! which marks the credentials as Ely.by without a schema change.

use crate::State;
use crate::state::minecraft_auth::{Credentials, MinecraftProfile};
use crate::util::fetch::REQWEST_CLIENT;
use crate::util::io;
use chrono::{Duration, Utc};
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use uuid::Uuid;

pub const ELYBY_REFRESH_PREFIX: &str = "elyby:";
const AUTH_SERVER: &str = "https://authserver.ely.by/auth";

// Pinned so a compromised download host can't swap the agent that runs inside the game.
const AUTHLIB_INJECTOR_VERSION: &str = "1.2.8";
const AUTHLIB_INJECTOR_URL: &str = "https://github.com/yushijinhun/authlib-injector/releases/download/v1.2.8/authlib-injector-1.2.8.jar";
const AUTHLIB_INJECTOR_SHA256: &str =
    "9c7f4343e6c82034958ffb48c14a2cb0c85928be7283103ce17da00c6d5a7b10";

/// How long an Ely.by access token is trusted before it is refreshed.
const TOKEN_REFRESH_INTERVAL: Duration = Duration::hours(12);

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthResponse {
    access_token: String,
    client_token: String,
    selected_profile: Option<Profile>,
}

#[derive(Deserialize)]
struct Profile {
    id: Uuid,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ErrorResponse {
    error_message: Option<String>,
}

fn elyby_error(message: impl Into<String>) -> crate::Error {
    crate::ErrorKind::OtherError(format!("Ely.by: {}", message.into())).into()
}

async fn post(
    endpoint: &str,
    body: serde_json::Value,
) -> crate::Result<AuthResponse> {
    let response = REQWEST_CLIENT
        .post(format!("{AUTH_SERVER}/{endpoint}"))
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        let message = response
            .json::<ErrorResponse>()
            .await
            .ok()
            .and_then(|e| e.error_message)
            .unwrap_or_else(|| "sign-in failed".to_string());
        return Err(elyby_error(message));
    }

    Ok(response.json().await?)
}

fn client_token(credentials: &Credentials) -> Option<&str> {
    credentials.refresh_token.strip_prefix(ELYBY_REFRESH_PREFIX)
}

/// Signs in with an Ely.by username or email. Accounts with two-factor
/// authentication need the current code, which Ely.by expects as `password:code`.
pub async fn authenticate(
    username: &str,
    password: &str,
    totp: Option<&str>,
) -> crate::Result<Credentials> {
    let password = match totp.map(str::trim).filter(|code| !code.is_empty()) {
        Some(code) => format!("{password}:{code}"),
        None => password.to_string(),
    };

    let response = post(
        "authenticate",
        json!({
            "username": username.trim(),
            "password": password,
            "clientToken": Uuid::new_v4().simple().to_string(),
            "requestUser": true,
        }),
    )
    .await?;

    let profile = response
        .selected_profile
        .ok_or_else(|| elyby_error("this account has no Minecraft profile"))?;

    Ok(Credentials {
        offline_profile: MinecraftProfile {
            id: profile.id,
            name: profile.name,
            skins: Vec::new(),
            capes: Vec::new(),
            fetch_time: None,
        },
        access_token: response.access_token,
        refresh_token: format!(
            "{ELYBY_REFRESH_PREFIX}{}",
            response.client_token
        ),
        expires: Utc::now() + TOKEN_REFRESH_INTERVAL,
        active: true,
    })
}

/// Exchanges the stored access token for a fresh one.
pub async fn refresh(credentials: &mut Credentials) -> crate::Result<()> {
    let client_token = client_token(credentials)
        .ok_or_else(|| elyby_error("credentials are not from Ely.by"))?
        .to_string();

    let response = post(
        "refresh",
        json!({
            "accessToken": credentials.access_token,
            "clientToken": client_token,
        }),
    )
    .await?;

    credentials.access_token = response.access_token;
    credentials.expires = Utc::now() + TOKEN_REFRESH_INTERVAL;
    Ok(())
}

/// Returns the authlib-injector jar, downloading and verifying it on first use.
pub async fn authlib_injector_path(state: &State) -> crate::Result<PathBuf> {
    let path = state
        .directories
        .libraries_dir()
        .join("moe/yushi/authlib-injector")
        .join(AUTHLIB_INJECTOR_VERSION)
        .join(format!("authlib-injector-{AUTHLIB_INJECTOR_VERSION}.jar"));

    if let Ok(existing) = io::read(&path).await
        && sha256_hex(&existing) == AUTHLIB_INJECTOR_SHA256
    {
        return Ok(path);
    }

    let bytes = REQWEST_CLIENT
        .get(AUTHLIB_INJECTOR_URL)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    if sha256_hex(&bytes) != AUTHLIB_INJECTOR_SHA256 {
        return Err(elyby_error(
            "downloaded authlib-injector failed its checksum check",
        ));
    }

    if let Some(parent) = path.parent() {
        io::create_dir_all(parent).await?;
    }
    io::write(&path, &bytes).await?;
    Ok(path)
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_token_is_read_from_marker() {
        let credentials = Credentials {
            offline_profile: MinecraftProfile {
                id: Uuid::nil(),
                name: "Player".into(),
                skins: Vec::new(),
                capes: Vec::new(),
                fetch_time: None,
            },
            access_token: "token".into(),
            refresh_token: format!("{ELYBY_REFRESH_PREFIX}abc123"),
            expires: Utc::now(),
            active: true,
        };
        assert!(credentials.is_elyby());
        assert_eq!(client_token(&credentials), Some("abc123"));
    }

    #[test]
    fn profile_ids_accept_undashed_uuids() {
        let profile: Profile = serde_json::from_str(
            r#"{"id":"ffc8fdc95824509e8a57c99b940fb996","name":"ErickSkrauch"}"#,
        )
        .unwrap();
        assert_eq!(
            profile.id.hyphenated().to_string(),
            "ffc8fdc9-5824-509e-8a57-c99b940fb996"
        );
    }
}
