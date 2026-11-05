use super::*;

#[test]
fn previously_killed_storage_still_blocks_a_later_backup() {
    assert!(ensure_storage_shutdown("localstack", 137).is_err());
    assert!(ensure_storage_shutdown("postgres", 137).is_err());
    assert!(ensure_storage_shutdown("sync_service", 137).is_err());
    assert!(ensure_storage_shutdown("ai_editing_worker", 137).is_err());
    assert!(ensure_storage_shutdown("localstack", 0).is_ok());
    assert!(ensure_storage_shutdown("lexical_service", 137).is_ok());
}

#[test]
fn readiness_orders_implicit_local_aws_dependency_and_proxy_last() {
    let config = json!({"services": {
        "proxy": {}, "document_upload_finalizer": {}, "localstack": {}, "postgres": {},
        "fusionauth": {"depends_on": {"postgres": {}}},
        "authentication-service": {"depends_on": {"fusionauth": {}}}
    }});
    let selected = services(&config).unwrap().keys().cloned().collect();
    let plan = waves(&config, &selected).unwrap();
    let position = |name: &str| {
        plan.iter()
            .position(|wave| wave.iter().any(|n| n == name))
            .unwrap()
    };
    assert!(position("localstack") < position("document_upload_finalizer"));
    assert!(position("postgres") < position("fusionauth"));
    assert!(position("fusionauth") < position("authentication-service"));
    assert_eq!(position("proxy"), plan.len() - 1);
}

#[test]
fn dependency_exclusion_and_cycle_fail_before_starting() {
    let config = json!({"services": {"worker": {"depends_on": {"store": {}}}, "store": {}}});
    let selected = selected_services(&config, &["store".into()]).unwrap();
    assert!(waves(&config, &selected).is_err());
    assert!(selected_services(&config, &["typo".into()]).is_err());
    let cyclic =
        json!({"services": {"a": {"depends_on": {"b": {}}}, "b": {"depends_on": {"a": {}}}}});
    assert!(
        waves(
            &cyclic,
            &services(&cyclic).unwrap().keys().cloned().collect()
        )
        .is_err()
    );
}

#[test]
fn incomplete_restore_cannot_start() {
    let instance =
        Instance::derive(Some(&format!("incomplete-{}", std::process::id())), None).unwrap();
    files::private_new_dir(&instance.artifact_dir()).unwrap();
    files::write_private(
        &instance.artifact_dir().join("RESTORE-INCOMPLETE"),
        b"interrupted",
    )
    .unwrap();
    let error = load_config(&instance).unwrap_err();
    assert!(error.to_string().contains("restore is incomplete"));
    std::fs::remove_dir_all(instance.artifact_dir()).unwrap();
}

#[test]
#[ignore = "requires local Docker; creates an isolated stopped container to check literal values"]
fn captured_environment_and_shell_variables_survive_compose_roundtrip() {
    let dir = std::env::temp_dir().join(format!("recovery-compose-{}", std::process::id()));
    files::private_new_dir(&dir).unwrap();
    let path = dir.join("compose.json");
    let original = json!({"name":"literal-test","services":{"test":{"image":files::HELPER,
        "environment":{"SECRET":"literal$VALUE$$more${NOT_AN_ENV}"},
        "command":["sh","-c","echo \"$SECRET\""]}}});
    write_compose(&path, &original).unwrap();
    let mut parsed: Value = serde_json::from_slice(
        &output(
            compose("literal-test", &path).args(["config", "--format", "json"]),
            "checking interpolation",
        )
        .unwrap(),
    )
    .unwrap();
    map_strings(&mut parsed, &|s| s.replace("$$", "$"));
    assert_eq!(
        parsed["services"]["test"]["environment"],
        original["services"]["test"]["environment"]
    );
    let project = format!("literal-test-{}", std::process::id());
    write_compose(&path, &parsed).unwrap();
    output(
        compose(&project, &path).args(["create", "--pull", "never"]),
        "creating interpolation probe",
    )
    .unwrap();
    let captured = inspect(&project).unwrap();
    output(
        compose(&project, &path).args(["down"]),
        "removing interpolation probe",
    )
    .unwrap();
    assert_eq!(
        actual_environment(&captured["test"]).unwrap()["SECRET"],
        original["services"]["test"]["environment"]["SECRET"]
    );
    assert_eq!(
        json!(captured["test"].config.cmd),
        original["services"]["test"]["command"]
    );
    std::fs::remove_dir_all(dir).unwrap();
}
