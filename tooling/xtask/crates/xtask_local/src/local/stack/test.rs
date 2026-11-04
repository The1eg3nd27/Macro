use super::*;

#[test]
fn update_missing_or_corrupt_state_never_bootstraps() {
    let name = format!("missing-update-{}", std::process::id());
    let instance = Instance::derive(Some(&name), None).unwrap();
    let args = UpdateArgs {
        instance: InstanceArgs {
            instance: Some(name),
            port_base: None,
        },
        ..Default::default()
    };
    assert!(
        update(&args)
            .unwrap_err()
            .to_string()
            .contains("will not initialize")
    );
    assert!(!instance.artifact_dir().exists());
    instance.ensure_artifact_dir().unwrap();
    std::fs::write(state_path(&instance), "broken json").unwrap();
    assert!(
        update(&args)
            .unwrap_err()
            .to_string()
            .contains("will not initialize")
    );
    assert_eq!(
        std::fs::read_to_string(state_path(&instance)).unwrap(),
        "broken json"
    );
    std::fs::remove_dir_all(instance.artifact_dir()).unwrap();
}

/// `up` writes stack.json and `update`/`status` read it back — the record and
/// the mode labels must stay in agreement.
#[test]
fn stack_state_roundtrips() {
    let state = StackState {
        mode: "local".to_string(),
        frontend: "static".to_string(),
        binaries_dir: Some(PathBuf::from("/tmp/binaries")),
    };
    let json = serde_json::to_string(&state).unwrap();
    let back: StackState = serde_json::from_str(&json).unwrap();
    assert_eq!(back.mode, "local");
    assert_eq!(back.frontend, "static");
    assert_eq!(back.binaries_dir, Some(PathBuf::from("/tmp/binaries")));
    assert!(mode_from_label(&back.mode).is_ok());
    assert!(mode_from_label("nonsense").is_err());
}

#[test]
fn legacy_stack_state_has_no_binaries_dir() {
    let state: StackState =
        serde_json::from_str(r#"{"mode":"local","frontend":"static"}"#).unwrap();
    assert_eq!(state.binaries_dir, None);
}

#[test]
fn clearing_state_invalidates_a_previous_headless_stack() {
    let instance =
        Instance::derive(Some(&format!("state-clear-{}", std::process::id())), None).unwrap();
    write_state(
        &instance,
        &StackState {
            mode: "local".to_string(),
            frontend: "static".to_string(),
            binaries_dir: None,
        },
    )
    .unwrap();
    assert!(read_state(&instance).is_some());
    clear_state(&instance).unwrap();
    assert!(read_state(&instance).is_none());
    clear_state(&instance).unwrap();
    let _ = std::fs::remove_dir_all(instance.artifact_dir());
}
