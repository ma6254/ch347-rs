use std::fs::File;
use std::io::Write;
use std::process::Command;

use shadow_rs::{Format, SdResult};

fn main() -> SdResult<()> {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();

    let output = Command::new("git")
        .args(&["describe", "--tags", &git_hash])
        .output()
        .unwrap();
    let build_version = String::from_utf8(output.stdout).unwrap();

    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=BUILD_VERSION={}", build_version);

    println!(r"cargo:rustc-link-lib=static=CH347DLLA64");
    println!(r"cargo:rustc-link-search=static_lib");

    shadow_rs::new_hook(hook)
}

fn hook(file: &File) -> SdResult<()> {
    append_write_const(file)?;
    Ok(())
}

fn append_write_const(mut file: &File) -> SdResult<()> {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();

    let output = Command::new("git")
        .args(&["describe", "--tags", &git_hash])
        .output()
        .unwrap();
    let build_version = String::from_utf8(output.stdout).unwrap();

    // let hook_const: &str = &format!(r#"pub const GIT_HASH: &str = "{}";"#, git_hash);
    // writeln!(file, "{}", hook_const)?;

    // let hook_const: &str = &format!(r#"pub const BUILD_VERSION: &str = "{}";"#, build_version);
    // writeln!(file, "{}", hook_const)?;

    let about_message: &str = &format!(
        "build_date: {}\r\ngit_hash: {}\r\nbuild_version: {}",
        shadow_rs::DateTime::now().human_format(),
        git_hash.trim(),
        build_version.trim()
    );
    writeln!(
        file,
        r#"pub const {}: &str = "{}";"#,
        "ABOUT_MESSABE", about_message
    )?;

    Ok(())
}
