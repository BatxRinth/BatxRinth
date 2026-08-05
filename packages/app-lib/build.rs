use std::ffi::OsString;
use std::path::PathBuf;
use std::process::{Command, exit};
use std::{env, fs};

fn main() {
    println!("cargo::rerun-if-changed=.env");
    println!("cargo::rerun-if-changed=java/gradle");
    println!("cargo::rerun-if-changed=java/src");
    println!("cargo::rerun-if-changed=java/build.gradle.kts");
    println!("cargo::rerun-if-changed=java/settings.gradle.kts");
    println!("cargo::rerun-if-changed=java/gradle.properties");

    set_env();
    build_java_jars();
}

fn set_env() {
    let mut vars = std::collections::HashMap::new();

    // 1. Try loading .env or .env.prod
    if let Ok(iter) = dotenvy::dotenv_iter() {
        for (k, v) in iter.flatten() {
            vars.insert(k, v);
        }
    }
    if let Ok(iter) = dotenvy::from_filename_iter(".env.prod") {
        for (k, v) in iter.flatten() {
            vars.entry(k).or_insert(v);
        }
    }

    // 2. Also incorporate environment variables passed from process environment
    for (k, v) in env::vars() {
        if k.starts_with("MODRINTH_") || k.starts_with("SHARED_INSTANCES_") {
            vars.insert(k, v);
        }
    }

    // 3. Fallback defaults if still missing
    vars.entry("MODRINTH_URL".to_string()).or_insert_with(|| "https://modrinth.com/".to_string());
    vars.entry("MODRINTH_API_BASE_URL".to_string()).or_insert_with(|| "https://api.modrinth.com/".to_string());
    vars.entry("SHARED_INSTANCES_API_BASE_URL".to_string()).or_insert_with(|| "https://shared-instances.modrinth.com/".to_string());
    vars.entry("MODRINTH_ARCHON_BASE_URL".to_string()).or_insert_with(|| "https://archon.modrinth.com/".to_string());
    vars.entry("MODRINTH_API_URL".to_string()).or_insert_with(|| "https://api.modrinth.com/v2/".to_string());
    vars.entry("MODRINTH_API_URL_V3".to_string()).or_insert_with(|| "https://api.modrinth.com/v3/".to_string());
    vars.entry("MODRINTH_SOCKET_URL".to_string()).or_insert_with(|| "wss://api.modrinth.com/".to_string());
    vars.entry("MODRINTH_LAUNCHER_META_URL".to_string()).or_insert_with(|| "https://launcher-meta.modrinth.com/".to_string());

    for (var_name, var_value) in vars {
        if var_name == "DATABASE_URL" {
            continue;
        }
        println!("cargo::rustc-env={var_name}={var_value}");
    }
}

fn build_java_jars() {
    let out_dir =
        dunce::canonicalize(PathBuf::from(env::var_os("OUT_DIR").unwrap()))
            .unwrap();

    println!(
        "cargo::rustc-env=JAVA_JARS_DIR={}",
        out_dir.join("java/libs").display()
    );

    let gradle_path = fs::canonicalize(
        #[cfg(target_os = "windows")]
        "java\\gradlew.bat",
        #[cfg(not(target_os = "windows"))]
        "java/gradlew",
    )
    .unwrap();

    let mut build_dir_str = OsString::from("-Dorg.gradle.project.buildDir=");
    build_dir_str.push(out_dir.join("java"));
    let exit_status = Command::new(gradle_path)
        .arg(build_dir_str)
        .arg("build")
        .arg("--no-daemon")
        .arg("--console=rich")
        .current_dir(dunce::canonicalize("java").unwrap())
        .status()
        .expect("Failed to wait on Gradle build");

    if !exit_status.success() {
        println!("cargo::error=Gradle build failed with {exit_status}");
        exit(exit_status.code().unwrap_or(1));
    }
}
