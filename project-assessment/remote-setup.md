# Remote Linux verification setup

Updated 2026-11-08. Use project branch `assessment/rich-text-and-server-pilot`,
which includes the local storage, recovery and startup fixes. The original
upstream baseline is `3953a658d877ddfcd8c4724e0b5d66f02de008f3`.
These steps are prepared and have not been executed on a remote machine.
They create a synthetic development preview. Complete the separate
[private pilot gates](production-feasibility.md#earlier-private-team-pilot)
before using real team data; that pilot now precedes branding and integrations.

## Proposed test machine

Use a normal x86-64 Linux VM with permission to run a Docker daemon, initially
Ubuntu 24.04 LTS, at least 8 vCPUs, 32 GB RAM and approximately 320 GB SSD storage.
Start Cargo with two build jobs. This is a conservative engineering starting
point, not a measured minimum or a production sizing claim. Shared vCPUs are
acceptable for a first test, with less predictable build duration.
This application build target excludes local inference; assess the optional
27B model's hardware separately after selecting the model and workload.

The Rust guide warns about memory exhaustion when linking multiple large test
binaries. The local binary builder honors `CARGO_BUILD_JOBS`; without it, it uses
available CPU parallelism. A large CPU count should not imply unrestricted builds.

A remote machine has not been selected or provisioned. Verify the actual CPU,
RAM, disk, Docker privileges and network limits before installing or building.
If capacity is insufficient, keep the work to source analysis and targeted tests.
The macOS build and browser trial are documented in [Local trial](local-setup.md).
It demonstrated core workflows and exposed object loss on LocalStack restart,
now repaired for graceful local restarts; see [storage repair](local-storage-persistence.md).
The local emulator configuration must not become the persistent team data store.
These capacity checks still apply before moving development to a cloud machine.

## Preflight

Run on the selected Linux machine, recording sanitized output:

```bash
uname -m
nproc
free -h
df -h .
df -i .
command -v git nix docker
```

If running in a container, also inspect the actual cgroup CPU and memory limits
(commonly `/sys/fs/cgroup/cpu.max`, `memory.max` and `memory.current` on cgroup v2),
the container's writable disk quota and whether a Docker daemon can operate.
Host-reported resources alone can overstate the resources available to the build.

Check outbound DNS and HTTPS to GitHub, `cache.nixos.org`, container registries,
the package registries referenced by the lockfiles, and the chosen model APIs.
An unauthenticated API returning 401/405 demonstrates reachability only. It does
not verify credentials or integration behavior. Do not assume access to Macro's
private build cache, Doppler projects or hosted development services.

Use the official Nix installation instructions linked from
`docs/development/running-locally.md`. On Linux the pinned shell supplies the
Docker tooling, but a usable daemon and sufficient host privileges still need
verification. Record the resulting Nix and Docker versions and resource limits.

## Private networking and storage

Before startup, apply a provider firewall that allows SSH only from authorized
administration addresses and denies other public inbound traffic. Docker-published
ports must remain unreachable publicly; do not rely solely on UFW rules.
Use a private network or a correctly configured SSH tunnel for browser access.

Browser hostname, TLS certificate, redirects and WebSocket origins must agree.
Follow the remote access section of the upstream development guide and inspect
the generated instance URLs. Validate login, uploads, downloads and WebSockets
through the actual private browser path before declaring the preview usable.

Keep the checkout, build caches, generated instance configuration and named test
volumes on persistent server storage. Use synthetic data. Export anything worth
keeping before retiring a server; remote disks and Docker volumes are not backups.
Production data must never depend on ephemeral cloud coding containers.

A complete backup must cover application and authentication databases, the
collaborative document `sync_state` volume, actual file objects, and encryption
keys needed to decrypt stored credentials or content. The repaired local setup
stores AWS resource state, objects and keys in its Compose-scoped
`localstack_data` volume; include the complete volume in a consistent backup. Preserve required generated
configuration securely outside Git. Recreating buckets, queues or tables is not
a content restore. A complete local cold backup has now been restored into fresh
volumes on the same Docker engine and architecture; see the [recovery runbook](backup-and-recovery.md).
That does not prove recovery after losing the host: exact images are prerequisites,
and encrypted off-machine retention plus an independent Linux restore remain open.

## First startup of a fresh, disposable instance

After preflight and the Nix installation, from the project branch above:

```bash
nix develop
export CARGO_BUILD_JOBS=2
bun install --frozen-lockfile
just doctor-local --instance assessment --port-base 31000
just stack up --no-doppler --instance assessment --port-base 31000 --traces off
just stack status --json --instance assessment --port-base 31000
```

The headless stack builds the frontend and serves it with the backend behind
the proxy. This recipe is for a regular Linux development VM, not Cursor Cloud.
The `.cursor/` scripts have separate environment-specific assumptions.

Record failed services separately. An agent-harness failure caused by missing
integration configuration does not prove chat, document or search failures.
Supply approved provider/OAuth credentials through an ignored environment file
or secret manager after inspecting the configuration requirements. Never commit
them, paste them into logs, or reuse Macro's hosted-development credentials.

Apply synthetic multi-user fixtures, using the same instance and port base:

```bash
just seed-scenario --instance assessment --port-base 31000 apply --file seed/scenarios/team-perms.json
just seed-scenario --instance assessment --port-base 31000 status --file seed/scenarios/team-perms.json
just seed-scenario --instance assessment --port-base 31000 matrix --file seed/scenarios/team-perms.json
```

The scenario can contain more than three personas; use three relevant accounts
for the product walkthrough. The matrix supplies broader permission checks.
Applying the scenario again recreates its marked rows; do not reapply after
creating content that should survive. Treat printed seed links as inputs to configure the private preview, not proof
that those hostnames resolve from another computer.

## Preserve-data restart and update

Identify the exact generated Compose project and containers for this instance.
Stop it with `just stack stop --instance assessment --port-base 31000`; resume
with `just stack resume --instance assessment --port-base 31000`. Add explicit
`--exclude` flags for optional services deliberately left stopped. This local
recovery path starts existing containers only, waits for infrastructure before
dependents and starts the proxy last. See [backup and recovery](backup-and-recovery.md).
If LocalStack lost its resources, use `cargo x localstack-provision --instance
assessment --port-base 31000` to restore missing resource definitions. This does
not recover lost file bytes or keys. The repaired local image preserves new
objects and keys during graceful restart; preserve its `localstack_data` volume
and 60-second shutdown grace. Never reprovision between persistence checks.
Keep the generated configuration, project name and volumes. Inspect health and
run the content checks again. Handle any required helper/proxy service according
to the upstream recovery guide, without rebuilding a fresh instance.

For source changes, first verify that
`infra/local/generated/assessment/stack.json` exists, is readable, and contains
valid recorded state for the existing instance. Inspect the recorded status and
containers. `stack update` now rejects absent or corrupt state instead of falling
back to initialization. Resolve the state problem before attempting an update.
Frozen restored instances also reject updates until an explicit upgrade is planned.

With valid existing state, use `just stack update --no-doppler --instance
assessment --port-base 31000`; add `--frontend` for the headless frontend bundle
when needed. Repeat the original `--env-file` if one was supplied. Updates resolve
configuration again and must not silently fall back to Doppler or lose required
integration overrides. Preserve any other required startup overrides as well.

Do not use `stack up`, `run_local`, `stack down`, `destroy_local`, `reset_local`,
or `docker compose down -v` as a persistence test or recovery shortcut. Some of
these recreate or remove data. Any destructive reset requires explicit approval.

## Completion evidence

Record actual resource peaks, container health, logs and results in
`test-results.md`. Verify login for three accounts, messaging, document editing,
file upload/download, tasks, search, allowed/denied access, a normal restart,
and a restore into a separate disposable instance. Verify AI, MCP and memory
separately after their configuration and local-memory constraints are resolved.

Use Git branches, small commits and a project-owned remote for source changes.
At handoff record the branch, commit, instance name, test results and blockers.
The same instructions and source revision should be usable from either client
computer and by either coding agent.
