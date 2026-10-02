//! End-to-end smoke test: real migrations, settings, an offline profile, a real
//! Minecraft install and launch. Run it after every upstream merge:
//!
//! ```sh
//! THESEUS_CONFIG_DIR=<empty folder> cargo test -p theseus --lib smoke -- --ignored --nocapture
//! ```

use crate::State;
use crate::api::instance::QuickPlayType;
use crate::install::InstallJobStatus;
use crate::state::{InstanceLink, ModLoader, Settings, elyby_auth};
use std::time::{Duration, Instant};

#[tokio::test(flavor = "multi_thread")]
#[ignore = "downloads Minecraft; needs THESEUS_CONFIG_DIR pointing at a throwaway folder"]
async fn smoke_offline_launch_1_16_5() {
    let dir = std::env::var("THESEUS_CONFIG_DIR").expect(
        "set THESEUS_CONFIG_DIR so the test never touches real launcher data",
    );
    assert!(
        !dir.contains("app.batxrinth.launcher"),
        "THESEUS_CONFIG_DIR must not be the real launcher folder"
    );

    State::init("BatxRinthSmokeTest".to_string()).await.unwrap();
    let state = State::get().await.unwrap();
    println!("state initialised, migrations applied");

    let mut settings = Settings::get(&state.pool).await.unwrap();
    assert!(!settings.telemetry && !settings.personalized_ads);
    settings.discord_rpc_show_play_time = false;
    settings.update(&state.pool).await.unwrap();
    assert!(
        !Settings::get(&state.pool)
            .await
            .unwrap()
            .discord_rpc_show_play_time
    );
    println!("settings round-trip ok");

    let jar = elyby_auth::authlib_injector_path(&state).await.unwrap();
    assert!(jar.exists());
    println!("authlib-injector verified at {}", jar.display());

    let credentials = crate::api::minecraft_auth::login_offline("SmokeTest")
        .await
        .unwrap();
    assert!(credentials.is_offline() && !credentials.is_microsoft());

    let job = crate::install::create_instance(
        "Smoke 1.16.5".to_string(),
        "1.16.5".to_string(),
        ModLoader::Vanilla,
        None,
        None,
        None,
        InstanceLink::Unmanaged,
    )
    .await
    .unwrap();
    let job_id = job.job_id.parse().unwrap();
    let started = Instant::now();
    let job = loop {
        let job = crate::install::get_job(job_id).await.unwrap();
        if job.status.is_finished() {
            break job;
        }
        assert!(
            started.elapsed() < Duration::from_secs(30 * 60),
            "install timed out"
        );
        tokio::time::sleep(Duration::from_secs(5)).await;
    };
    assert_eq!(job.status, InstallJobStatus::Succeeded, "{:?}", job.error);
    let instance_id = job.instance_id.unwrap();
    println!("installed 1.16.5 in {:?}", started.elapsed());

    crate::api::instance::run(&instance_id, QuickPlayType::None)
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_secs(45)).await;

    let running = crate::api::process::get_by_instance_id(&instance_id)
        .await
        .unwrap();
    assert!(!running.is_empty(), "Minecraft exited during startup");
    println!("Minecraft still running after 45s");

    #[cfg(windows)]
    {
        let output = std::process::Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "Get-CimInstance Win32_Process -Filter \"name like 'java%'\" | Select-Object -ExpandProperty CommandLine",
            ])
            .output()
            .unwrap();
        let command_lines = String::from_utf8_lossy(&output.stdout);
        let game = command_lines
            .lines()
            .find(|line| line.contains("SmokeTest"))
            .expect("game command line not found");
        assert!(
            game.contains("-Dminecraft.api.services.host=https://nope.invalid")
        );
        assert!(!game.contains("authlib-injector"));
        println!(
            "offline 1.16.5 multiplayer fix present on the JVM command line"
        );
    }

    crate::api::instance::kill(&instance_id).await.unwrap();
}
