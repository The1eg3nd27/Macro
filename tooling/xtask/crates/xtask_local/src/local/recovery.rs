//! Preserve-data lifecycle operations. Recovery never initializes application state.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, ensure};
use clap::Args;
use serde::Deserialize;
use serde_json::{Value, json};

use super::cli::InstanceArgs;
use super::instance::Instance;
use super::{gen_compose, repo_root, sdk_webhook};

pub mod backup;
mod files;
#[cfg(test)]
mod test;

const PROJECT_LABEL: &str = "com.docker.compose.project";
const SERVICE_LABEL: &str = "com.docker.compose.service";
const INFRA: &[&str] = &[
    "postgres",
    "db",
    "redis",
    "search",
    "kafka",
    "localstack",
    "mailpit",
];
const STOP_SECONDS: &str = "60";

#[derive(Args, Clone, Default)]
pub struct ResumeArgs {
    #[command(flatten)]
    pub instance: InstanceArgs,
    /// Explicitly leave a service stopped. Required dependencies cannot be excluded.
    #[arg(long)]
    pub exclude: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(super) struct Container {
    id: String,
    image: String,
    config: ContainerConfig,
    state: ContainerState,
    mounts: Vec<Mount>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ContainerConfig {
    labels: BTreeMap<String, String>,
    env: Vec<String>,
    cmd: Option<Vec<String>>,
    entrypoint: Option<Vec<String>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ContainerState {
    running: bool,
    exit_code: i32,
    #[serde(default)]
    restarting: bool,
    health: Option<Health>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Health {
    status: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Mount {
    #[serde(rename = "Type")]
    kind: String,
    name: Option<String>,
    source: PathBuf,
    destination: String,
    #[serde(rename = "RW")]
    writable: bool,
}

pub(super) fn output(command: &mut Command, label: &str) -> Result<Vec<u8>> {
    let out = command.output().with_context(|| label.to_string())?;
    ensure!(
        out.status.success(),
        "{label} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    Ok(out.stdout)
}

pub(super) fn inspect(project: &str) -> Result<BTreeMap<String, Container>> {
    let ids = output(
        Command::new("docker").args([
            "ps",
            "-aq",
            "--filter",
            &format!("label={PROJECT_LABEL}={project}"),
        ]),
        "listing instance containers",
    )?;
    let ids = String::from_utf8(ids)?;
    if ids.trim().is_empty() {
        return Ok(BTreeMap::new());
    }
    let rows: Vec<Container> = serde_json::from_slice(&output(
        Command::new("docker")
            .arg("inspect")
            .args(ids.split_whitespace()),
        "inspecting instance containers",
    )?)?;
    let mut containers = BTreeMap::new();
    for row in rows {
        ensure!(
            row.config.labels.get(PROJECT_LABEL).map(String::as_str) == Some(project),
            "container project mismatch"
        );
        let name = row
            .config
            .labels
            .get(SERVICE_LABEL)
            .context("missing service label")?
            .clone();
        ensure!(
            containers.insert(name.clone(), row).is_none(),
            "scaled service {name} is not supported by local recovery"
        );
    }
    Ok(containers)
}

pub(super) fn lock(instance: &Instance) -> Result<File> {
    let dir = repo_root().join("infra/local/generated/.recovery-locks");
    files::private_dir_all(&dir)?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(dir.join(instance.name()))?;
    file.try_lock()
        .context("another recovery operation owns this instance")?;
    Ok(file)
}

pub(super) fn load_config(instance: &Instance) -> Result<Value> {
    ensure!(
        !instance.artifact_dir().join("RESTORE-INCOMPLETE").exists(),
        "restore is incomplete; refusing to start it"
    );
    let restored = instance.artifact_dir().join("restore-compose.json");
    let mut config: Value = if restored.exists() {
        serde_json::from_slice(&output(
            compose(instance.project_name(), &restored).args(["config", "--format", "json"]),
            "reading restored configuration",
        )?)?
    } else {
        let state: Value = serde_json::from_slice(
            &std::fs::read(instance.artifact_dir().join("stack.json"))
                .context("missing stack.json; recovery will not initialize an instance")?,
        )?;
        ensure!(
            state["mode"] == "local",
            "recovery supports fully local instances only"
        );
        let env = instance.artifact_dir().join("local.generated.env");
        ensure!(env.is_file(), "missing saved instance environment");
        let files = gen_compose::compose_files(instance);
        serde_json::from_slice(&output(
            gen_compose::docker_compose(instance, &files, &env)
                .args(["config", "--format", "json"]),
            "reading saved Compose configuration",
        )?)?
    };
    ensure!(
        config["name"] == instance.project_name(),
        "saved Compose project does not match the requested instance"
    );
    // `compose config` emits a reusable Compose file and escapes literal dollars.
    // Internally keep resolved values; write_compose escapes exactly once.
    map_strings(&mut config, &|s| s.replace("$$", "$"));
    config
        .as_object_mut()
        .context("invalid Compose object")?
        .remove("version");
    Ok(config)
}

pub(super) fn services(config: &Value) -> Result<&serde_json::Map<String, Value>> {
    config["services"]
        .as_object()
        .context("missing Compose services")
}

fn selected_services(config: &Value, exclude: &[String]) -> Result<BTreeSet<String>> {
    let all = services(config)?;
    for name in exclude {
        ensure!(all.contains_key(name), "unknown excluded service {name}");
    }
    let selected: BTreeSet<_> = all
        .keys()
        .filter(|name| !exclude.contains(name))
        .cloned()
        .collect();
    ensure!(!selected.is_empty(), "no services selected");
    Ok(selected)
}

/// Topological waves wait for health before any dependent is started. Local
/// AWS/search readiness is required even where legacy Compose omitted an edge.
pub(super) fn waves(config: &Value, selected: &BTreeSet<String>) -> Result<Vec<Vec<String>>> {
    let all = services(config)?;
    let mut dependencies = BTreeMap::new();
    for name in selected {
        let service = all
            .get(name)
            .with_context(|| format!("missing configuration for {name}"))?;
        let mut deps: BTreeSet<String> = service["depends_on"]
            .as_object()
            .map(|d| d.keys().cloned().collect())
            .unwrap_or_default();
        for dependency in &deps {
            ensure!(
                selected.contains(dependency),
                "{name} requires excluded or missing service {dependency}"
            );
        }
        if !INFRA.contains(&name.as_str()) {
            deps.extend(
                INFRA
                    .iter()
                    .filter(|n| selected.contains(**n))
                    .map(|n| (*n).to_string()),
            );
        }
        if name == "proxy" {
            deps.extend(selected.iter().filter(|n| *n != name).cloned());
        }
        dependencies.insert(name.clone(), deps);
    }
    let mut done = BTreeSet::new();
    let mut result = Vec::new();
    while !dependencies.is_empty() {
        let ready: Vec<_> = dependencies
            .iter()
            .filter(|(_, d)| d.is_subset(&done))
            .map(|(n, _)| n.clone())
            .collect();
        ensure!(
            !ready.is_empty(),
            "dependency cycle in recovery startup plan"
        );
        for name in &ready {
            dependencies.remove(name);
            done.insert(name.clone());
        }
        result.push(ready);
    }
    Ok(result)
}

pub(super) fn compose(project: &str, path: &Path) -> Command {
    let mut command = Command::new("docker");
    command
        .args(["compose", "--project-directory"])
        .arg(repo_root())
        .args(["-p", project, "-f"])
        .arg(path);
    command
}

/// Preserve literal dollar signs in captured environment and command values.
pub(super) fn write_compose(path: &Path, config: &Value) -> Result<()> {
    let mut escaped = config.clone();
    map_strings(&mut escaped, &|s| s.replace('$', "$$"));
    files::write_private(path, &serde_json::to_vec_pretty(&escaped)?)
}

fn map_strings(value: &mut Value, map: &impl Fn(&str) -> String) {
    match value {
        Value::String(s) => *s = map(s),
        Value::Array(a) => a.iter_mut().for_each(|v| map_strings(v, map)),
        Value::Object(o) => o.values_mut().for_each(|v| map_strings(v, map)),
        _ => {}
    }
}

pub(super) fn resume_selected(
    instance: &Instance,
    config: &Value,
    selected: &BTreeSet<String>,
) -> Result<()> {
    let plan = waves(config, selected)?;
    let before = inspect(instance.project_name())?;
    for name in selected {
        ensure!(
            before.contains_key(name),
            "missing container {name}; resume will not recreate or initialize it"
        );
    }
    let path = instance.artifact_dir().join("resume-compose.json");
    write_compose(&path, config)?;
    for wave in plan {
        println!("Starting and waiting: {}", wave.join(", "));
        output(
            compose(instance.project_name(), &path)
                .args([
                    "up",
                    "-d",
                    "--no-recreate",
                    "--no-build",
                    "--no-deps",
                    "--pull",
                    "never",
                    "--wait",
                    "--wait-timeout",
                    "180",
                ])
                .args(&wave),
            "waiting for startup dependencies",
        )?;
    }
    let after = inspect(instance.project_name())?;
    for name in selected {
        let container = after
            .get(name)
            .context("container disappeared during startup")?;
        ensure!(
            container.id == before[name].id,
            "container {name} was unexpectedly replaced"
        );
        ensure!(
            container.state.running && !container.state.restarting,
            "service {name} is not running steadily"
        );
        if let Some(health) = &container.state.health {
            ensure!(health.status == "healthy", "service {name} is not healthy");
        }
    }
    println!(
        "Ready: {} selected services; services without healthchecks are verified running only.",
        selected.len()
    );
    Ok(())
}

pub fn resume(args: &ResumeArgs) -> Result<()> {
    let instance = Instance::derive(args.instance.instance.as_deref(), args.instance.port_base)?;
    let _lock = lock(&instance)?;
    let config = load_config(&instance)?;
    let selected = selected_services(&config, &args.exclude)?;
    if !args.exclude.is_empty() {
        println!("Explicitly excluded: {}", args.exclude.join(", "));
    }
    resume_selected(&instance, &config, &selected)
}

pub(super) fn stop_containers(
    instance: &Instance,
    config: &Value,
    containers: &BTreeMap<String, Container>,
) -> Result<()> {
    let selected = services(config)?.keys().cloned().collect();
    let mut plan = waves(config, &selected)?;
    ensure!(
        containers
            .keys()
            .all(|name| services(config).is_ok_and(|s| s.contains_key(name))),
        "unknown containers exist in this project; inspect them before stopping"
    );
    plan.reverse();
    sdk_webhook::stop(instance);
    for wave in plan {
        let ids: Vec<_> = wave
            .iter()
            .filter_map(|n| containers.get(n))
            .filter(|c| c.state.running)
            .map(|c| c.id.as_str())
            .collect();
        if ids.is_empty() {
            continue;
        }
        println!("Stopping: {}", wave.join(", "));
        output(
            Command::new("docker")
                .args(["stop", "--timeout", STOP_SECONDS])
                .args(ids),
            "stopping instance writers",
        )?;
    }
    let stopped = inspect(instance.project_name())?;
    ensure!(
        stopped.values().all(|c| !c.state.running),
        "instance still has running writers"
    );
    for (name, container) in &stopped {
        ensure_storage_shutdown(name, container.state.exit_code)?;
        if containers.get(name).is_some_and(|c| c.state.running) && container.state.exit_code == 137
        {
            eprintln!(
                "Warning: {name} required forced termination; all writers are now stopped before storage capture."
            );
        }
    }
    Ok(())
}

fn ensure_storage_shutdown(name: &str, exit_code: i32) -> Result<()> {
    ensure!(
        exit_code != 137
            || (!INFRA.contains(&name) && name != "sync_service" && name != "fusionauth"),
        "{name} storage was killed instead of stopping gracefully; backup is unsafe"
    );
    Ok(())
}

pub fn stop(args: &InstanceArgs) -> Result<()> {
    let instance = Instance::derive(args.instance.as_deref(), args.port_base)?;
    let _lock = lock(&instance)?;
    let config = load_config(&instance)?;
    let containers = inspect(instance.project_name())?;
    ensure!(!containers.is_empty(), "instance has no containers");
    stop_containers(&instance, &config, &containers)
}

pub(super) fn actual_environment(container: &Container) -> Result<Value> {
    let mut values = serde_json::Map::new();
    for pair in &container.config.env {
        let (key, value) = pair
            .split_once('=')
            .context("invalid container environment")?;
        values.insert(key.to_string(), json!(value));
    }
    Ok(Value::Object(values))
}
