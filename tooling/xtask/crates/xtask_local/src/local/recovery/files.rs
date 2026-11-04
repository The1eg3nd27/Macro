//! Private recovery artifacts and network-disabled archive helpers.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};
use std::path::Path;
use std::process::Command;

use anyhow::{Result, ensure};
use sha2::{Digest, Sha256};

use super::output;

pub(super) const HELPER: &str =
    "alpine@sha256:be2656061b536c1883c4034790e21d0a09c5aaaf51968b89368a3c0371ea2c9a";

pub(super) fn private_dir_all(path: &Path) -> Result<()> {
    fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(path)?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
    Ok(())
}

pub(super) fn private_new_dir(path: &Path) -> Result<()> {
    fs::DirBuilder::new().mode(0o700).create(path)?;
    Ok(())
}

pub(super) fn write_private(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(path)?;
    file.set_permissions(fs::Permissions::from_mode(0o600))?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}

pub(super) fn checksum(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut digest = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        digest.update(&buffer[..count]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

pub(super) fn helper() -> Command {
    let mut command = Command::new("docker");
    command.args([
        "run",
        "--rm",
        "--pull",
        "never",
        "--network",
        "none",
        "--read-only",
    ]);
    command
}

pub(super) fn mount(kind: &str, source: &str, target: &str, readonly: bool) -> Result<String> {
    ensure!(
        !source.contains([',', '\n', '\r']),
        "unsupported Docker mount source"
    );
    let mut value = format!("type={kind},source={source},target={target}");
    if readonly {
        value.push_str(",readonly");
    }
    Ok(value)
}

/// Archive an entire volume/directory, or a selected runtime file/subtree.
pub(super) fn archive(
    kind: &str,
    source: &str,
    entries: &[String],
    directory: &Path,
    file: &str,
) -> Result<()> {
    let mut command = helper();
    command
        .args(["--mount", &mount(kind, source, "/source", true)?, "--mount"])
        .arg(mount(
            "bind",
            &directory.to_string_lossy(),
            "/backup",
            false,
        )?)
        .args([
            HELPER,
            "tar",
            "-czf",
            &format!("/backup/{file}"),
            "-C",
            "/source",
        ]);
    if entries == ["services/analytics-proxy"] {
        command.arg("--exclude=node_modules");
    }
    output(command.arg("--").args(entries), "archiving recovery data")?;
    let path = directory.join(file);
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
    File::open(&path)?.sync_all()?;
    Ok(())
}

/// Only trusted local backups are accepted. Extraction stays inside a helper
/// container, with the destination and read-only backup as its only mounts.
pub(super) fn extract(kind: &str, destination: &str, directory: &Path, file: &str) -> Result<()> {
    output(
        helper()
            .args([
                "--mount",
                &mount(kind, destination, "/restore", false)?,
                "--mount",
            ])
            .arg(mount(
                "bind",
                &directory.to_string_lossy(),
                "/backup",
                true,
            )?)
            .args([
                HELPER,
                "tar",
                "-xzf",
                &format!("/backup/{file}"),
                "-C",
                "/restore",
            ]),
        "extracting recovery data",
    )?;
    Ok(())
}
