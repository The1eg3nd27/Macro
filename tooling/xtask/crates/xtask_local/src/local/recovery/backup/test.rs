use super::*;

#[test]
fn restored_linux_dependencies_can_be_backed_up_and_restored_again() {
    let root = std::env::temp_dir().join(format!("recovery-links-{}", std::process::id()));
    let package = root.join("services/analytics-proxy");
    files::private_dir_all(&package).unwrap();
    let config = json!({"services":{"analytics_proxy":{"volumes":[{"type":"bind","target":"/app","source":root}]}}});
    link_linux_dependencies(&config).unwrap();
    link_linux_dependencies(&config).unwrap();
    let link = package.join("node_modules");
    fs::remove_file(&link).unwrap();
    fs::create_dir(&link).unwrap();
    fs::write(link.join("keep.txt"), "existing content").unwrap();
    assert!(link_linux_dependencies(&config).is_err());
    assert_eq!(
        fs::read_to_string(link.join("keep.txt")).unwrap(),
        "existing content"
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn restore_remaps_every_mount_and_network_away_from_source() {
    let instance = Instance::derive(Some("fresh-restore"), None).unwrap();
    let manifest = Manifest {
        version: 1,
        source_project: "source".into(),
        architecture: "test".into(),
        configuration_sha256: String::new(),
        excluded: vec![],
        archives: vec![
            Archive {
                key: "a0000".into(),
                sha256: String::new(),
                bind_file: None,
            },
            Archive {
                key: "a0001".into(),
                sha256: String::new(),
                bind_file: Some("Caddyfile".into()),
            },
        ],
    };
    let original = json!({"name":"source","services":{"service":{"volumes":[
        {"type":"volume","source":"a0000","target":"/data"},
        {"type":"bind","source":"a0001","target":"/etc/config"}
    ]}},"networks":{"private":{"name":"source-private","external":true,"internal":true}}});
    let restored = destination_config(&instance, &manifest, original).unwrap();
    assert_eq!(
        restored["volumes"]["a0000"]["name"],
        "macro-fresh-restore_a0000"
    );
    assert_eq!(
        restored["networks"]["private"]["name"],
        "macro-fresh-restore_private"
    );
    assert_eq!(restored["networks"]["private"]["internal"], true);
    assert_eq!(
        restored["services"]["service"]["volumes"][1]["source"],
        json!(instance.artifact_dir().join("runtime/a0001/Caddyfile"))
    );
}

#[test]
fn incomplete_corrupt_or_unsafe_backup_is_rejected_before_docker() {
    let dir = std::env::temp_dir().join(format!("recovery-integrity-{}", std::process::id()));
    files::private_new_dir(&dir).unwrap();
    assert!(validate_backup(&dir).is_err());
    files::write_private(&dir.join("runtime.json"), b"{}").unwrap();
    files::write_private(&dir.join("a0000.tar.gz"), b"original").unwrap();
    let mut manifest = Manifest {
        version: 1,
        source_project: "test".into(),
        architecture: "test".into(),
        configuration_sha256: files::checksum(&dir.join("runtime.json")).unwrap(),
        excluded: vec![],
        archives: vec![Archive {
            key: "a0000".into(),
            sha256: files::checksum(&dir.join("a0000.tar.gz")).unwrap(),
            bind_file: None,
        }],
    };
    files::write_private(
        &dir.join("manifest.json"),
        &serde_json::to_vec(&manifest).unwrap(),
    )
    .unwrap();
    assert!(validate_backup(&dir).is_ok());
    files::write_private(&dir.join("a0000.tar.gz"), b"corrupt").unwrap();
    assert!(validate_backup(&dir).is_err());
    manifest.archives[0].key = "../escape".into();
    files::write_private(
        &dir.join("manifest.json"),
        &serde_json::to_vec(&manifest).unwrap(),
    )
    .unwrap();
    assert!(validate_backup(&dir).is_err());
    fs::remove_dir_all(dir).unwrap();
}
