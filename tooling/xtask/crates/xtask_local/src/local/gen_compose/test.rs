use super::*;

/// Uses the generated service in an isolated Compose project, never an app instance.
#[test]
#[ignore = "requires Docker and the pinned local AWS image"]
fn local_aws_data_survives_restart_and_container_replacement() {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let project = format!("macro-storage-regression-{suffix}");
    let dir = std::env::temp_dir().join(&project);
    fs::create_dir(&dir).unwrap();
    let compose_path = dir.join("compose.json");
    let instance = Instance::derive(Some("storage-regression"), None).unwrap();
    let mut services = IndexMap::new();
    add_localstack_service(&mut services, &instance);
    let service = services.get_mut("localstack").unwrap().as_mut().unwrap();
    service.ports = dct::Ports::default();
    service.networks = dct::Networks::default();
    fs::write(
        &compose_path,
        serde_json::to_vec(&serde_json::json!({
            "services": services,
            "volumes": {"localstack_data": {}}
        }))
        .unwrap(),
    )
    .unwrap();

    let compose = |args: &[&str]| {
        Command::new("docker")
            .args(["compose", "-p", &project, "-f"])
            .arg(&compose_path)
            .args(args)
            .output()
            .unwrap()
    };
    let checked = |args: &[&str]| {
        let output = compose(args);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        output.stdout
    };
    let result = std::panic::catch_unwind(|| {
        checked(&["up", "-d", "--wait", "--wait-timeout", "180"]);
        let manifest = checked(&[
            "exec",
            "-T",
            "localstack",
            "/opt/code/localstack/.venv/bin/python",
            "-c",
            include_str!("persistence_probe.py"),
            "seed",
        ]);
        let manifest = String::from_utf8(manifest).unwrap();
        for replace in [false, true] {
            checked(&["stop", "--timeout", "60", "localstack"]);
            if replace {
                checked(&[
                    "up",
                    "-d",
                    "--force-recreate",
                    "--renew-anon-volumes",
                    "--wait",
                    "--wait-timeout",
                    "180",
                ]);
            } else {
                checked(&["up", "-d", "--wait", "--wait-timeout", "180"]);
            }
            checked(&[
                "exec",
                "-T",
                "localstack",
                "/opt/code/localstack/.venv/bin/python",
                "-c",
                include_str!("persistence_probe.py"),
                "verify",
                manifest.trim(),
            ]);
        }
    });
    let cleanup = compose(&["down", "--volumes", "--timeout", "60"]);
    fs::remove_dir_all(&dir).unwrap();
    assert!(cleanup.status.success(), "isolated test cleanup failed");
    if let Err(error) = result {
        std::panic::resume_unwind(error);
    }
}

#[test]
fn generated_services_have_bounded_console_logs_in_every_mode() {
    for mode in [Mode::Local, Mode::Dev] {
        for name in [None, Some("logging-test")] {
            let instance = Instance::derive(name, None).unwrap();
            let mut services = IndexMap::new();
            for svc in services_for_mode(mode) {
                services.insert(svc.compose_name.to_string(), Some(dct::Service::default()));
            }
            add_localstack_service(&mut services, &instance);
            add_proxy_service(&mut services, &instance, false, false);
            let compose = dct::Compose {
                services: dct::Services(services),
                ..Default::default()
            };
            let mut value = serde_yaml::to_value(compose).unwrap();
            apply_tags(&mut value, mode, &instance);

            for service in value["services"].as_mapping().unwrap().values() {
                assert_eq!(service["logging"]["driver"], "json-file");
                assert_eq!(service["logging"]["options"]["max-size"], "20m");
                assert_eq!(service["logging"]["options"]["max-file"], "3");
            }
        }
    }
}

#[test]
fn fusionauth_kickstart_is_privately_relabelled_and_replaces_inherited_mounts() {
    for name in [None, Some("selinux-test")] {
        let instance = Instance::derive(name, None).unwrap();
        let mut services = IndexMap::new();
        add_local_infra(&mut services, &instance, false);
        let compose = dct::Compose {
            services: dct::Services(services),
            ..Default::default()
        };
        let mut value = serde_yaml::to_value(compose).unwrap();
        apply_tags(&mut value, Mode::Local, &instance);

        let Value::Tagged(volumes) = &value["services"]["fusionauth"]["volumes"] else {
            panic!("FusionAuth mounts must replace the inherited mount list");
        };
        assert_eq!(volumes.tag, Tag::new("!override"));
        assert_eq!(
            volumes.value,
            serde_yaml::to_value(vec![
                "fusionauth_config:/usr/local/fusionauth/config".to_string(),
                format!(
                    "{}:/usr/local/fusionauth/kickstart:ro,Z",
                    kickstart_dir(&instance).display()
                ),
            ])
            .unwrap()
        );
    }
}

#[test]
fn every_local_harness_publishes_the_api_port_used_by_the_sdk() {
    for name in [None, Some("harness-ports")] {
        let instance = Instance::derive(name, None).unwrap();
        let service = services_for_mode(Mode::Local)
            .find(|service| service.compose_name == EGRESS_SERVICE)
            .unwrap();
        let ports = published_ports(Mode::Local, &instance, service);
        assert!(ports.contains(&format!("{}:8080", instance.port(Port::AgentHarness))));
        assert!(ports.contains(&format!(
            "{}:{EGRESS_CONTAINER_PORT}",
            instance.port(Port::AgentHarnessEgress)
        )));
        assert!(ports.contains(&format!("{}:2222", instance.port(Port::PreviewSsh))));
    }
}
