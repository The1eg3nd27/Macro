//! Cold, complete backups of a selected fully local runtime. Init snapshots
//! intentionally serve a different purpose and are never used for recovery.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::net::TcpListener;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail, ensure};
use clap::Args;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::{
    Container, ResumeArgs, actual_environment, compose, files, inspect, load_config, lock, output,
    resume_selected, selected_services, services, stop_containers, waves, write_compose,
};
use crate::local::{instance::Instance, repo_root};

#[derive(Args, Clone)]
pub struct BackupArgs {
    #[command(flatten)]
    pub runtime: ResumeArgs,
    /// New directory beneath an existing, gitignored parent. Contains secrets.
    #[arg(long)]
    pub output: PathBuf,
}

#[derive(Args, Clone)]
pub struct RestoreArgs {
    /// Trusted backup created by `stack backup` on the same architecture.
    #[arg(long)]
    pub backup: PathBuf,
    /// Fresh destination instance. Existing instances are never overwritten.
    #[arg(long)]
    pub instance: String,
}

#[derive(Serialize, Deserialize)]
struct Manifest {
    version: u32,
    source_project: String,
    architecture: String,
    configuration_sha256: String,
    archives: Vec<Archive>,
    excluded: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Archive {
    key: String,
    sha256: String,
    /// `None` is a volume; `Some("")` a bind directory; otherwise a bind file.
    bind_file: Option<String>,
}

struct Capture {
    archive: Archive,
    kind: String,
    source: String,
    entries: Vec<String>,
}

fn bind_paths(captures: &[Capture]) -> Vec<PathBuf> {
    captures
        .iter()
        .filter(|c| c.kind == "bind")
        .flat_map(|c| {
            c.entries
                .iter()
                .map(|entry| Path::new(&c.source).join(entry))
        })
        .collect()
}

fn architecture() -> Result<String> {
    Ok(String::from_utf8(output(
        Command::new("docker").args(["info", "--format", "{{.Architecture}}"]),
        "reading Docker architecture",
    )?)?
    .trim()
    .to_owned())
}

fn one_component(value: &str) -> bool {
    let mut components = Path::new(value).components();
    matches!(components.next(), Some(Component::Normal(_))) && components.next().is_none()
}

fn image_present(image: &str) -> Result<()> {
    output(
        Command::new("docker").args(["image", "inspect", image, "--format", "{{.Id}}"]),
        "required pinned image is not available locally",
    )?;
    Ok(())
}

fn runtime_bind(
    service: &str,
    mount: &super::Mount,
    container: &Container,
) -> Result<(String, Vec<String>, String)> {
    let source = mount
        .source
        .canonicalize()
        .context("runtime bind is missing")?;
    ensure!(
        source.starts_with(repo_root()),
        "runtime binds must be inside this checkout"
    );
    if source.is_file() {
        let file = source
            .file_name()
            .context("missing bind filename")?
            .to_string_lossy()
            .into_owned();
        return Ok((
            source
                .parent()
                .context("missing bind parent")?
                .to_string_lossy()
                .into_owned(),
            vec![file.clone()],
            file,
        ));
    }
    ensure!(source.is_dir(), "unsupported runtime bind type");
    if source == repo_root() {
        ensure!(
            service == "analytics_proxy",
            "whole-checkout binds are only supported for the analytics runtime"
        );
        return Ok((
            source.to_string_lossy().into_owned(),
            vec!["services/analytics-proxy".into()],
            String::new(),
        ));
    }
    let entries = if mount.destination == "/app/out" {
        let binaries: BTreeSet<_> = container
            .config
            .cmd
            .iter()
            .flatten()
            .chain(container.config.entrypoint.iter().flatten())
            .filter_map(|arg| arg.strip_prefix("/app/out/"))
            .map(String::from)
            .collect();
        ensure!(
            !binaries.is_empty(),
            "cannot identify runtime binary for {service}"
        );
        for binary in &binaries {
            ensure!(
                one_component(binary) && source.join(binary).is_file(),
                "missing or invalid runtime binary for {service}"
            );
        }
        binaries.into_iter().collect()
    } else {
        vec![".".into()]
    };
    Ok((
        source.to_string_lossy().into_owned(),
        entries,
        String::new(),
    ))
}

/// Replace every mount with an archive reference, and capture the environment,
/// command and image actually used by the container rather than today's build.
fn capture_plan(
    config: &Value,
    containers: &BTreeMap<String, Container>,
    selected: &BTreeSet<String>,
) -> Result<(Value, Vec<Capture>)> {
    let mut frozen = config.clone();
    let all = frozen["services"]
        .as_object_mut()
        .context("missing services")?;
    all.retain(|name, _| selected.contains(name));
    let mut captures: Vec<Capture> = Vec::new();
    let mut volumes: BTreeMap<String, String> = BTreeMap::new();
    for (name, service) in all {
        let container = containers
            .get(name)
            .with_context(|| format!("missing container {name}"))?;
        let object = service.as_object_mut().context("invalid service")?;
        for key in ["build", "container_name", "env_file", "profiles", "develop"] {
            object.remove(key);
        }
        ensure!(
            !object.contains_key("volumes_from"),
            "volumes_from is unsupported"
        );
        object.insert("image".into(), json!(container.image));
        object.insert("environment".into(), actual_environment(container)?);
        object.insert("command".into(), json!(container.config.cmd));
        object.insert("entrypoint".into(), json!(container.config.entrypoint));
        let mut mounts = Vec::new();
        for mount in &container.mounts {
            let key = if mount.kind == "volume" {
                let source = mount.name.as_ref().context("volume has no name")?;
                if let Some(key) = volumes.get(source) {
                    key.clone()
                } else {
                    let key = format!("a{:04}", captures.len());
                    captures.push(Capture {
                        archive: Archive {
                            key: key.clone(),
                            sha256: String::new(),
                            bind_file: None,
                        },
                        kind: "volume".into(),
                        source: source.clone(),
                        entries: vec![".".into()],
                    });
                    volumes.insert(source.clone(), key.clone());
                    key
                }
            } else {
                ensure!(mount.kind == "bind", "unsupported mount type for {name}");
                let (source, entries, file) = runtime_bind(name, mount, container)?;
                let key = format!("a{:04}", captures.len());
                captures.push(Capture {
                    archive: Archive {
                        key: key.clone(),
                        sha256: String::new(),
                        bind_file: Some(file),
                    },
                    kind: "bind".into(),
                    source,
                    entries,
                });
                key
            };
            mounts.push(json!({"type": mount.kind, "source": key, "target": mount.destination, "read_only": !mount.writable}));
        }
        object.insert("volumes".into(), json!(mounts));
    }
    frozen["volumes"] = json!({});
    Ok((frozen, captures))
}

fn reject_foreign_writers(project: &str, captures: &[Capture]) -> Result<()> {
    for capture in captures.iter().filter(|c| c.kind == "volume") {
        let rows = output(
            Command::new("docker").args([
                "ps",
                "--filter",
                &format!("volume={}", capture.source),
                "--format",
                "{{.Label \"com.docker.compose.project\"}}",
            ]),
            "checking shared-volume writers",
        )?;
        ensure!(
            String::from_utf8(rows)?.lines().all(|p| p == project),
            "a foreign running container shares a source volume"
        );
    }
    let ids = String::from_utf8(output(
        Command::new("docker").args(["ps", "-q"]),
        "checking runtime bind writers",
    )?)?;
    if !ids.trim().is_empty() {
        let rows: Vec<Value> = serde_json::from_slice(&output(
            Command::new("docker")
                .arg("inspect")
                .args(ids.split_whitespace()),
            "inspecting runtime bind writers",
        )?)?;
        let captured = bind_paths(captures);
        for row in rows {
            if row["Config"]["Labels"][super::PROJECT_LABEL] == project {
                continue;
            }
            for mount in row["Mounts"].as_array().into_iter().flatten() {
                if mount["Type"] != "bind" || mount["RW"] != true {
                    continue;
                }
                let source = Path::new(
                    mount["Source"]
                        .as_str()
                        .context("missing foreign bind source")?,
                );
                ensure!(
                    !captured
                        .iter()
                        .any(|path| path.starts_with(source) || source.starts_with(path)),
                    "a foreign running container can write a captured runtime bind"
                );
            }
        }
    }
    Ok(())
}

pub fn backup(args: &BackupArgs) -> Result<()> {
    let instance = Instance::derive(
        args.runtime.instance.instance.as_deref(),
        args.runtime.instance.port_base,
    )?;
    let _lock = lock(&instance)?;
    let config = load_config(&instance)?;
    let selected = selected_services(&config, &args.runtime.exclude)?;
    waves(&config, &selected)?;
    for name in super::INFRA
        .iter()
        .chain(["fusionauth", "sync_service"].iter())
    {
        ensure!(
            selected.contains(*name),
            "complete local backup requires {name}"
        );
    }
    let containers = inspect(instance.project_name())?;
    ensure!(
        containers.get("localstack").is_some_and(|c| c
            .mounts
            .iter()
            .any(|m| m.kind == "volume" && m.destination == "/persisted-data")),
        "upgrade local AWS persistence before using complete backups"
    );
    ensure!(
        containers
            .iter()
            .all(|(name, c)| !c.state.running || selected.contains(name)),
        "stop excluded or unknown services explicitly before backing up"
    );
    let running: BTreeSet<_> = containers
        .iter()
        .filter(|(_, c)| c.state.running)
        .map(|(name, _)| name.clone())
        .collect();
    if !running.is_empty() {
        waves(&config, &running)?;
    }
    let (frozen, mut captures) = capture_plan(&config, &containers, &selected)?;
    reject_foreign_writers(instance.project_name(), &captures)?;
    image_present(files::HELPER)?;
    let architecture = architecture()?;
    let parent = args
        .output
        .parent()
        .context("backup output needs an existing parent directory")?
        .canonicalize()?;
    let directory = parent.join(args.output.file_name().context("invalid backup output")?);
    ensure!(
        !directory.exists(),
        "backup output already exists; choose a new directory"
    );
    ensure!(
        !bind_paths(&captures)
            .iter()
            .any(|path| directory.starts_with(path)),
        "backup output cannot be inside a captured runtime bind"
    );
    let ignored = Command::new("git")
        .current_dir(repo_root())
        .args(["check-ignore", "-q", "--"])
        .arg(&directory)
        .status()?;
    ensure!(
        ignored.success(),
        "backup must be beneath a gitignored directory in this checkout"
    );
    files::private_new_dir(&directory)?;
    println!(
        "Cold backup: stopping writers; the previously running services will resume afterward."
    );
    let saved = (|| -> Result<()> {
        stop_containers(&instance, &config, &containers, &selected)?;
        reject_foreign_writers(instance.project_name(), &captures)?;
        for capture in &mut captures {
            println!("Saving {} ({})", capture.archive.key, capture.kind);
            let filename = format!("{}.tar.gz", capture.archive.key);
            files::archive(
                &capture.kind,
                &capture.source,
                &capture.entries,
                &directory,
                &filename,
            )?;
            capture.archive.sha256 = files::checksum(&directory.join(filename))?;
        }
        let configuration = directory.join("runtime.json");
        files::write_private(&configuration, &serde_json::to_vec_pretty(&frozen)?)?;
        let manifest = Manifest {
            version: 1,
            source_project: instance.project_name().into(),
            architecture,
            configuration_sha256: files::checksum(&configuration)?,
            archives: captures.into_iter().map(|c| c.archive).collect(),
            excluded: args.runtime.exclude.clone(),
        };
        // Completion marker is written last. An interrupted/partial backup is rejected.
        files::write_private(
            &directory.join("manifest.json"),
            &serde_json::to_vec_pretty(&manifest)?,
        )?;
        Ok(())
    })();
    let resumed = if running.is_empty() {
        Ok(())
    } else {
        resume_selected(&instance, &config, &running)
    };
    match (saved, resumed) {
        (Ok(()), Ok(())) => {
            println!(
                "Backup complete: {} (private; contains credentials and keys)",
                directory.display()
            );
            Ok(())
        }
        (Err(save), Ok(())) => Err(save).context("backup incomplete; source resumed"),
        (Ok(()), Err(resume)) => {
            Err(resume).context("backup completed; source resume failed; run stack resume")
        }
        (Err(save), Err(resume)) => {
            bail!("backup failed: {save:#}; source resume also failed: {resume:#}")
        }
    }
}

fn validate_backup(directory: &Path) -> Result<(Manifest, Value)> {
    let manifest: Manifest = serde_json::from_slice(
        &fs::read(directory.join("manifest.json")).context("missing complete backup manifest")?,
    )?;
    ensure!(manifest.version == 1, "unsupported backup format");
    ensure!(
        files::checksum(&directory.join("runtime.json"))? == manifest.configuration_sha256,
        "runtime configuration checksum mismatch"
    );
    let mut keys = BTreeSet::new();
    for archive in &manifest.archives {
        ensure!(
            archive.key.len() == 5
                && archive.key.starts_with('a')
                && archive.key[1..].bytes().all(|b| b.is_ascii_digit()),
            "invalid archive key"
        );
        ensure!(keys.insert(&archive.key), "duplicate archive key");
        if let Some(file) = &archive.bind_file {
            ensure!(
                file.is_empty() || one_component(file),
                "invalid bind filename"
            );
        }
        ensure!(
            files::checksum(&directory.join(format!("{}.tar.gz", archive.key)))? == archive.sha256,
            "archive checksum mismatch: {}",
            archive.key
        );
    }
    Ok((
        manifest,
        serde_json::from_slice(&fs::read(directory.join("runtime.json"))?)?,
    ))
}

fn resource_names(kind: &str) -> Result<BTreeSet<String>> {
    Ok(String::from_utf8(output(
        Command::new("docker").args([kind, "ls", "--format", "{{.Name}}"]),
        "checking destination resources",
    )?)?
    .lines()
    .map(String::from)
    .collect())
}

/// Rewrite every volume, network and bind to the destination. Published ports
/// and origins intentionally stay unchanged for the frozen frontend/auth setup.
fn destination_config(
    instance: &Instance,
    manifest: &Manifest,
    mut config: Value,
) -> Result<Value> {
    config["name"] = json!(instance.project_name());
    let mut volume_definitions = serde_json::Map::new();
    let archives: BTreeMap<_, _> = manifest
        .archives
        .iter()
        .map(|a| (a.key.as_str(), a))
        .collect();
    for service in config["services"]
        .as_object_mut()
        .context("missing services")?
        .values_mut()
    {
        for mount in service["volumes"]
            .as_array_mut()
            .context("missing service mounts")?
        {
            let archive = archives
                .get(
                    mount["source"]
                        .as_str()
                        .context("missing archive reference")?,
                )
                .context("unknown archive reference")?;
            if let Some(file) = &archive.bind_file {
                ensure!(mount["type"] == "bind", "mount type mismatch");
                mount["source"] = json!(
                    instance
                        .artifact_dir()
                        .join("runtime")
                        .join(&archive.key)
                        .join(file)
                );
                mount["bind"] = json!({"create_host_path": false});
            } else {
                ensure!(mount["type"] == "volume", "mount type mismatch");
                volume_definitions.insert(archive.key.clone(), json!({"name": format!("{}_{}", instance.project_name(), archive.key), "external": true}));
            }
        }
    }
    config["volumes"] = json!(volume_definitions);
    for (key, network) in config["networks"]
        .as_object_mut()
        .context("missing networks")?
    {
        ensure!(one_component(key), "invalid network key");
        *network = json!({"name": format!("{}_{}", instance.project_name(), key), "internal": network["internal"].as_bool().unwrap_or(false)});
    }
    Ok(config)
}

pub fn restore(args: &RestoreArgs) -> Result<()> {
    let directory = args.backup.canonicalize()?;
    let (manifest, original) = validate_backup(&directory)?;
    let instance = Instance::derive(Some(&args.instance), None)?;
    let _lock = lock(&instance)?;
    ensure!(
        instance.project_name() != manifest.source_project,
        "restore requires a new instance name"
    );
    ensure!(
        !instance.artifact_dir().exists(),
        "destination instance directory already exists"
    );
    ensure!(
        inspect(instance.project_name())?.is_empty(),
        "destination containers already exist"
    );
    ensure!(
        manifest.architecture == architecture()?,
        "backup architecture differs from Docker; rebuild/migration is required"
    );
    ensure!(
        inspect(&manifest.source_project)?
            .values()
            .all(|c| !c.state.running),
        "stop the source instance first: restore reuses its ports and URLs"
    );
    image_present(files::HELPER)?;
    let config = destination_config(&instance, &manifest, original)?;
    let selected = services(&config)?.keys().cloned().collect();
    waves(&config, &selected)?;
    for service in services(&config)?.values() {
        image_present(service["image"].as_str().context("missing pinned image")?)?;
        for port in service["ports"].as_array().into_iter().flatten() {
            if port["protocol"] == "udp" {
                continue;
            }
            let published: u16 = port["published"]
                .as_str()
                .context("invalid published port")?
                .parse()?;
            let host = port["host_ip"].as_str().unwrap_or("127.0.0.1");
            TcpListener::bind((host, published))
                .with_context(|| format!("restore port {published} is occupied"))?;
        }
    }
    for kind in ["volume", "network"] {
        let existing = resource_names(kind)?;
        let plural = format!("{kind}s");
        for value in config[&plural]
            .as_object()
            .context("missing resources")?
            .values()
        {
            ensure!(
                !existing.contains(value["name"].as_str().context("missing resource name")?),
                "destination {kind} already exists"
            );
        }
    }
    files::private_new_dir(&instance.artifact_dir())?;
    // A failure leaves only the fresh destination for inspection; never delete
    // or reuse it automatically, and never touch the source resources.
    files::write_private(
        &instance.artifact_dir().join("RESTORE-INCOMPLETE"),
        b"Do not resume until restore finishes.\n",
    )?;
    let result = (|| -> Result<()> {
        files::private_dir_all(&instance.artifact_dir().join("runtime"))?;
        for archive in &manifest.archives {
            let filename = format!("{}.tar.gz", archive.key);
            if archive.bind_file.is_some() {
                let destination = instance.artifact_dir().join("runtime").join(&archive.key);
                files::private_new_dir(&destination)?;
                files::extract(
                    "bind",
                    &destination.to_string_lossy(),
                    &directory,
                    &filename,
                )?;
            } else {
                let destination = format!("{}_{}", instance.project_name(), archive.key);
                output(
                    Command::new("docker").args([
                        "volume",
                        "create",
                        "--label",
                        &format!("com.docker.compose.project={}", instance.project_name()),
                        &destination,
                    ]),
                    "creating fresh restore volume",
                )?;
                files::extract("volume", &destination, &directory, &filename)?;
            }
            println!("Restored {}", archive.key);
        }
        link_linux_dependencies(&config)?;
        let path = instance.artifact_dir().join("restore-compose.json");
        write_compose(&path, &config)?;
        output(
            compose(instance.project_name(), &path).args([
                "create",
                "--no-build",
                "--pull",
                "never",
            ]),
            "creating stopped restore containers",
        )?;
        files::write_private(
            &instance.artifact_dir().join("stack.json"),
            br#"{"mode":"local","frontend":"static","binaries_dir":null}"#,
        )?;
        fs::remove_file(instance.artifact_dir().join("RESTORE-INCOMPLETE"))?;
        Ok(())
    })();
    result.context("restore incomplete; source is unchanged; inspect the fresh destination before retrying with another name")?;
    println!(
        "Restore complete, still stopped. Run: cargo x stack resume --instance {}",
        instance.name()
    );
    println!(
        "This restore reuses the source URLs and ports. Do not run both copies together. Excluded services: {}",
        manifest.excluded.join(", ")
    );
    Ok(())
}

/// Wrangler's own binary is installed outside the checkout in the analytics
/// image, but its source imports also need that image's dependency directory.
/// Only the fresh private runtime is changed; never use host node_modules.
fn link_linux_dependencies(config: &Value) -> Result<()> {
    if let Some(mount) = config["services"]["analytics_proxy"]["volumes"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|mount| mount["type"] == "bind" && mount["target"] == "/app")
    {
        let root = Path::new(
            mount["source"]
                .as_str()
                .context("missing analytics runtime source")?,
        );
        let link = root.join("services/analytics-proxy/node_modules");
        let expected = Path::new("/opt/analytics-proxy/node_modules");
        if link.symlink_metadata().is_ok() {
            ensure!(
                fs::read_link(&link).is_ok_and(|target| target == expected),
                "unexpected existing analytics dependency path; refusing to overwrite it"
            );
        } else {
            std::os::unix::fs::symlink(expected, link)
                .context("linking the restored analytics source to its Linux image dependencies")?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test;
