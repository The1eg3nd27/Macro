# Local trial

Baseline: `3953a658d877ddfcd8c4724e0b5d66f02de008f3`.
Branch: `assessment/local-smoke-test`.
Updated: 2026-11-01. Status: built and exercised; persistence and AI gaps remain.

The first trial uses the upstream macOS Nix and Docker workflow. It does not
change the eventual cloud development and dedicated-server hosting objectives.
Keep synthetic trial content separate from any personal knowledge vault.

## Measured prerequisites

The test host has an ARM M3 Pro processor, 12 CPU cores, 18 GB RAM and about
242 GiB free disk before dependency installation. Docker Desktop exposes 12 CPUs
and about 7.65 GiB RAM to containers. Its sparse virtual disk capacity does not
represent free host storage. Leave the Docker allocation unchanged initially;
limit Cargo builds to two jobs and monitor host memory and disk during the build.

Nix 2.35.4 is installed. Docker Engine 28.0.4 and Compose 2.34.0-desktop.1
respond. ARM container execution, an actual read-only repository bind mount,
and loopback port forwarding passed. The pinned Nix shell is ready, providing
Cargo 1.94.0, Bun 1.3.13 and Just 1.57.0. Its first realization downloads the
complete upstream development toolchain.

The instance preflight passed all checks, including all 33 host ports. The
default startup also built `macro-agent-harness:latest` (about 15.1 GB) and
`macro-local-runtime:dev` (about 331 MB). The agent sandbox contains a separate
Linux Nix toolchain; its image is larger than the application runtime image.
All 16 Linux ARM service binaries compiled successfully: the unified build took
14m49s and the isolated search worker took 10m30s. All eight browser WebAssembly
components and the static frontend bundle built successfully. Initial database
migrations, LocalStack resources, Kafka topics, FusionAuth and search indices were
initialized. The stack reported 31 containers; 29 services ran while the agent
harness restarted and the AI editing worker exited.

One later runtime sample reported about 4.8 GiB of container memory and about
202% aggregate Docker CPU (roughly two CPU cores), with 180.5 GiB free host disk.
This was a sample during troubleshooting, not a stable idle benchmark or peak
measurement. The full build fits this host with constrained build parallelism;
the complete integration suite is not proven. See [results](test-results.md).

## Commands

From this checkout, activate Nix in a new terminal. If its binary is not found:

```bash
. /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh
```

Enter the pinned environment and use the same instance and port base throughout:

```bash
nix --extra-experimental-features 'nix-command flakes' develop
export CARGO_BUILD_JOBS=2
export CARGO_PROFILE_DEV_DEBUG=0
export WASM_PACK_CACHE="$PWD/.cache/wasm-pack"
bun install --frozen-lockfile
just doctor-local --instance mac-smoke --port-base 31000
```

Debug symbols are disabled for this constrained trial to reduce build output
and linker pressure. The sandboxed run keeps Cargo, Bun, sccache and XDG caches
under the checkout's ignored `.cache/` directory, with an 8 GiB sccache cap.
Those cache locations are optional outside a sandbox; keep the same Cargo
profile settings on subsequent builds to reuse artifacts.

On macOS, cargo-zigbuild also writes to `~/Library/Caches/cargo-zigbuild`.
A sandbox must allow that specific directory; setting `XDG_CACHE_HOME` alone
did not redirect it. An earlier build attempt stopped at this restriction
before any instance database or application container was initialized.
`WASM_PACK_CACHE` explicitly keeps wasm-pack's downloads inside the checkout;
without that override, frontend preparation hit another sandbox write denial.

Only for the first initialization of a fresh disposable instance:

```bash
just stack up --no-doppler --instance mac-smoke --port-base 31000 --traces off
just stack status --json --instance mac-smoke --port-base 31000
```

The generated instance directory is `infra/local/generated/mac-smoke` and the
Compose project is `macro-mac-smoke`. Check actual status output before sharing
a browser URL. The expected proxy port is 31009. Do not use hosted development
services as a substitute for this private trial or enable a public tunnel.

The upstream development CA is checked in, including its private key. Do not
silently install it as a system-wide trust root. HTTPS browser trust is a
separate local setup step; certificate-verified command-line probes can use
the repository CA explicitly.

## Data preservation

`stack up` and `run_local` initialize clean state, including snapshot restoration;
they are not ordinary restart commands. Stop with:

```bash
just stop_local --instance mac-smoke --port-base 31000
```

A stop/start test preserved chat and collaborative document edits but lost the
uploaded test file from LocalStack. Preserving Docker volumes alone did not make
this emulator configuration durable. Use synthetic content only until object
storage persistence and backup restoration have been implemented and tested.

For a later restart, inspect containers belonging to `macro-mac-smoke` and start
those exact existing containers with `docker start`. Start infrastructure first
and wait for readiness before starting dependents. Three services exited on the
first simultaneous start because LocalStack/OpenSearch were not ready; starting
them again after readiness recovered them. Compose 2.34's `start` does not accept
`--wait`. Preserve generated instance configuration and named volumes. Do not use `stack down`, `destroy_local`,
`reset_local` or Compose `down -v` to recover from a failure.

Before `stack update`, confirm that this instance has valid existing `stack.json`
state; the missing-state fallback can initialize a fresh stack. Keep the original
`--no-doppler`, instance, port and environment-file options when updating.
`stack update` does not accept `--traces`; it infers tracing from the existing
collector state.
See [the remote runbook](remote-setup.md) for backup scope, including collaborative
document sync state and actual file objects. Git preserves code and documentation,
not application content, credentials or Docker volumes.

If LocalStack loses its resources, the documented non-reset recovery command is:

```bash
cargo x localstack-provision --instance mac-smoke --port-base 31000
```

This command completed in the trial. It recreates missing buckets, queues, tables
and keys; it does not restore lost objects or ciphertext encrypted by lost keys.
The uploaded fixture was not silently recreated or counted as a persistence pass.

## Browser and seed workflow

Use `https://<hostname>:31009/app/welcome` and the seed scenario's email personas.
The headless trial's printed persona links used a different HTTP frontend port;
use the verified HTTPS proxy origin instead. Email login completed for two seeded
accounts. The local flow auto-verified the code; Mailpit is also available at
`https://<hostname>:31009/mailpit/`. The browser certificate warning was handled
by the operator without installing the development CA globally.

Apply `seed/scenarios/team-perms.json` only to a fresh disposable instance:

```bash
just seed-scenario --instance mac-smoke --port-base 31000 apply --file seed/scenarios/team-perms.json
just seed-scenario --instance mac-smoke --port-base 31000 matrix --file seed/scenarios/team-perms.json
```

The existing trial is already seeded. Reapplying the scenario deletes and
recreates its marked rows, even without `--force`; do not reapply to preserve
edits. Never use `--force` as a recovery shortcut. The permission matrix checks
stored permissions, not every HTTP endpoint's enforcement.

Logout redirected to an unused `http://localhost:3000/` origin in this headless
build. Return to the HTTPS welcome page to sign in again. Browser controls needed
a fresh tab to leave the resulting connection-error page.

## Known optional-service failures

The agent harness cannot compose public MCP without a Google Gmail identity
provider in FusionAuth. The AI editing worker reads macOS `workerd` dependencies
from the repository bind mount inside Linux. No real provider credentials were
configured and no AI request completed. The harness was stopped after repeated
startup failures; both AI containers are stopped/exited at handoff. A later proxy
timeout was recovered by restarting only the proxy, after which the second
account could read both chat and document markers. These failures need separate fixes; a
healthy core backend does not establish that every feature works.

## Reproducibility limitation

The outer source checkout and Nix toolchain are pinned. The upstream agent
sandbox Dockerfile additionally clones the current default branch to bake a
developer environment, and several image tags are mutable. Consequently this
trial is not a fully pinned image build. Record the built image IDs/digests;
pin that nested source and container inputs before relying on identical rebuilds
on a remote server.
