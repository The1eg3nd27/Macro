# Verification results

Updated: 2026-10-30. Initial tooling checks: 2026-10-29.
Baseline: `3953a658d877ddfcd8c4724e0b5d66f02de008f3`.
Branch: `assessment/local-smoke-test`.

## Execution boundary

Source inspection and lightweight tests ran on macOS arm64. Nix 2.35.4 is now
installed and Docker is running. Local runtime preparation is in progress;
no application startup, database initialization, deployment, model request or
browser walkthrough has completed.

Python: 3.13.0. Node.js: 22.23.1. These tests used existing host runtimes, outside
the pinned Nix shell. Their results are limited to the selected tooling suites.
The first application trial now targets this Mac. No remote Linux machine is
connected. Cloud development and dedicated hosting remain later stages.

## Local environment checks

| Result | Check | Evidence |
| --- | --- | --- |
| PASS | Nix installation | `nix --version`: 2.35.4 |
| PASS | Docker daemon and Compose | Engine 28.0.4; Compose 2.34.0-desktop.1; ARM64; 12 CPUs; about 7.65 GiB container RAM |
| PASS | Registry access and ARM container execution | Pulled and ran `alpine:3.22` |
| PASS | Repository bind mount | A read-only container mount could read `Cargo.toml` and `flake.lock` |
| PASS | Loopback port forwarding | Temporary TCP echo container at `127.0.0.1:31039` returned the probe bytes |
| PASS | Pinned Nix shell | Cargo 1.94.0, Bun 1.3.13 and Just 1.57.0 resolve from the pinned development shell |

These checks establish prerequisites only, not application health.

## Executed tests

| Result | Command from repository root | Coverage |
| --- | --- | --- |
| PASS: 2 tests | `python3 -m unittest discover -s tooling/scripts -p test_consolidate_agent_config.py -v` | Preserves existing/empty config values; rejects a missing signer reference; uses mocked uploads, no live service writes |
| PASS: 3 tests | `node --test apps/web/scripts/write-bundle-manifest.test.mjs` | Bundle manifest compatibility and invalid minimum-version rejection |
| PASS: 12 tests | `python3 -m unittest discover -s tooling/pii -p test_scan.py -v` | Added-line privacy/secret scanner and Git error handling |

Total: 17 passed, 0 failed. These are tooling regression tests, not application
integration tests. No claim is made that chat, file storage or memory works.

## Pending application verification

| Check | Status | Blocker or required evidence |
| --- | --- | --- |
| Remote CPU/RAM/disk and network preflight | NOT RUN | Remote host/access not supplied |
| `just doctor-local` | IN PROGRESS | First compilation of the Rust orchestration tool |
| Full application build/start | NOT RUN | Local preflight and dependencies |
| Browser login and three-user workspace | NOT RUN | Running private application |
| Chat, documents, uploads/downloads and tasks | NOT RUN | Running private application |
| Search and cross-user access boundaries | NOT RUN | Synthetic fixtures and permission matrix |
| Content survival after restart | NOT RUN | Existing instance restarted without initialization/reset |
| Restore into an isolated instance | NOT RUN | Complete backup of databases, document sync state and file objects |
| Claude/OpenAI requests | NOT RUN | Provider configuration and running harness |
| External MCP access | NOT RUN | Harness, identity-provider configuration and private routing |
| Shared team knowledge and memory | NOT RUN | End-to-end multi-user evidence; inspected memory path is per-user and skips local automatic generation |

## Documentation checks

The changed files contain assessment documentation only. The repository-required
independent audit checked the findings and command definitions. Its two update
procedure findings were corrected and the corrections independently verified:
valid existing stack state is required, and original configuration flags must
be retained.

| Result | Check | Evidence or limitation |
| --- | --- | --- |
| PASS | Source findings and command definitions | Independent source review, including local-memory gating and update fallback |
| PASS | Documentation links and whitespace | Independent review plus `git diff --cached --check` |
| PASS | Added-line privacy scan | `python3 tooling/pii/scan.py --changed --skip-gitleaks`; gitleaks is not available, so this is not a full secret scan |
| PASS | `just check`, `just pr-privacy` wrappers | Independent audit using realized Just 1.57.0; earlier missing-tool blocker resolved |
| NOT RUN | Rust/TypeScript builds and browser checks | Local setup in progress; no application source changes in this assessment |

The underlying commit-message privacy script also passed after the initial
assessment commit. Repeat documentation and privacy checks after final runtime
results are recorded. Tooling checks are not substitutes for application
integration tests.
