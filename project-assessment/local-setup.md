# Local trial

Baseline: `3953a658d877ddfcd8c4724e0b5d66f02de008f3`.
Branch: `assessment/local-smoke-test`.
Status: setup in progress; application verification is not complete.

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
just doctor-local --instance mac-smoke --port-base 31000
```

Debug symbols are disabled for this constrained trial to reduce build output
and linker pressure. The sandboxed run keeps Cargo, Bun, sccache and XDG caches
under the checkout's ignored `.cache/` directory, with an 8 GiB sccache cap.
Those cache locations are optional outside a sandbox; keep the same Cargo
profile settings on subsequent builds to reuse artifacts.

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

For a later restart, inspect containers belonging to `macro-mac-smoke` and start
those exact existing containers with `docker start`. Preserve generated instance
configuration and named volumes. Do not use `stack down`, `destroy_local`,
`reset_local` or Compose `down -v` to recover from a failure.

Before `stack update`, confirm that this instance has valid existing `stack.json`
state; the missing-state fallback can initialize a fresh stack. Keep the original
`--no-doppler`, instance, port and environment-file options when updating.
`stack update` does not accept `--traces`; it infers tracing from the existing
collector state.
See [the remote runbook](remote-setup.md) for backup scope, including collaborative
document sync state and actual file objects. Git preserves code and documentation,
not application content, credentials or Docker volumes.
