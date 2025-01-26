use std::{borrow::Cow, process::Command};

pub fn generate_cargo_keys() {
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output();
        
    let commit = match output {
        Ok(output) if output.status.success() => {
            let sha = String::from_utf8_lossy(&output.stdout).trim().to_owned();
            Cow::from(sha)
        },
        Ok(output) => {
            println!("cargo::warning=Git command failed with status: {}", output.status);
            Cow::from("unknown")
        },
        Err(err) => {
            println!("cargo::warning=Failed to run git command: {}", err);
            Cow::from("unknown")
        },
    };

    println!("cargo:rustc-env=QA_WEB_VERSION={}", get_version(&commit));
}

fn get_version(impl_commit: &str) -> String {
    let commit_dash = if impl_commit.is_empty() { "" } else { "-" };

    format!(
           "{}{}{}-{}",
           std::env::var("CARGO_PKG_VERSION").unwrap_or_default(),
           commit_dash,
           impl_commit,
           get_platform(),
    )
}

fn get_platform() -> String {
    let target_os = std::env::consts::OS;
    let target_arch = std::env::consts::ARCH;

    let target_env = std::env::var("TARGET_ENV").ok();

    let env_dash = if target_env.is_some() { "-" } else { "" };

    format!(
        "{}-{}{}{}",
        target_arch,
        target_os,
        env_dash,
        target_env.unwrap_or_default()
    )
}

fn main() {
    generate_cargo_keys();
}
