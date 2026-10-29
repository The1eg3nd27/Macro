# Verification results

Date: 2026-10-29.
Baseline: `3953a658d877ddfcd8c4724e0b5d66f02de008f3`.
Branch: `assessment/remote-linux-smoke-test`.

## Execution boundary

Only source inspection and lightweight tests ran on macOS arm64. No Nix
installation, Docker startup, application build, database initialization,
deployment, model request or browser walkthrough was performed.

Python: 3.13.0. Node.js: 22.23.1. These tests used existing host runtimes, outside
the pinned Nix shell. Their results are limited to the selected tooling suites.
The full application must run on a remote Linux machine; none is connected yet.

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
| `just doctor-local` | NOT RUN | Requires selected remote machine and Nix/Docker setup |
| Full application build/start | NOT RUN | Remote preflight and dependencies |
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
| BLOCKED | `just check`, `just pr-privacy` wrappers | Both attempted; exit 127 because `just` is absent |
| NOT RUN | Rust/TypeScript builds and browser checks | No remote runtime yet; no application source changes in this assessment |

Run the underlying commit-message privacy script after creating the assessment
commit. Full Nix/Rust/TypeScript checks remain unavailable on this planning machine
and are not substitutes for remote application tests.
