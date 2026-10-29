# Preliminary architecture findings

Baseline: `3953a658d877ddfcd8c4724e0b5d66f02de008f3`, inspected 2026-10-29.
Source inspection is evidence of implementation, not proof of working deployment.

## Reusable foundation

Macro is a credible starting point for a single small team. Preserve the upstream
service structure initially and measure it before removing services or designing
replacement infrastructure. No application code has been changed in this assessment.

| Responsibility | Existing implementation or configuration |
| --- | --- |
| Browser interface | `apps/web`, SolidJS and TypeScript |
| Shared Rust logic | `crates/` |
| Application services | `services/`, composed in `docker/docker-compose.yml` |
| Authentication | Authentication service and FusionAuth |
| Relational data | PostgreSQL; local image is `pgvector/pgvector:pg18` |
| Search | OpenSearch and search processing service |
| Events and caching | Kafka and Redis |
| Local AWS-compatible dependencies | LocalStack; production substitution needs assessment |
| Collaborative documents | Sync, WebSocket, lexical and editing services |
| Agents and workspace MCP | `agent_harness_service`; consult `docs/guides/consolidate-agent-services.md` |
| Development lifecycle | `tooling/xtask/crates/xtask_local`, Nix, and documented just recipes |

The deployed upstream infrastructure uses AWS and Cloudflare components. Its local
emulators, test credentials, passwordless authentication and exposed service ports
must not be treated as production deployment settings.

## Memory: a concrete limitation to verify

The inspected `crates/memory/src/outbound/pg_memory_repo.rs` stores records in
PostgreSQL's `memory` table keyed by `user_id`. The domain service researches
the user's accessible documents, projects, emails and channels to generate a
personal summary. This can incorporate team context, but does not by itself prove
one canonical, editable, shared team knowledge store.

`crates/memory/src/domain/service.rs`, in `get_or_generate_memory`, explicitly
skips background generation when `Environment::Local` is active. Existing records
can still be read. Therefore a successful local login or AI conversation does
not demonstrate working automatic memory generation. The CLI also invokes this
method, so it is not an established workaround for that local-environment gate.

The same source uses a quality judge and can reject insufficient source material.
Memory testing needs substantive synthetic content as well as provider credentials.
Do not change the environment to production merely to bypass the gate. Assess a
targeted local configuration/code change after establishing the core stack.

Trace team-wide retrieval, permission changes, source references, export and
deletion separately. Reuse existing retrieval before considering another index or
knowledge engine. Keep the new team brain independent of personal project notes.

## Known startup and persistence constraints

- The current development guide says the agent harness's public MCP routes need
  a real Google Gmail identity provider. Credential-free stubs do not make this
  integration functional.
- The current MCP routes live in the combined agent harness. Do not assume an
  older issue about a missing standalone MCP container describes this revision.
- Fresh `run_local` / `stack up` operations initialize clean state and can restore
  initialization snapshots. They are not the restart path for valuable data.
- The `sync_state` volume stores local collaborative document state. Database-only
  backups are not sufficient evidence of complete document recovery.
- LocalStack resource reprovisioning can recreate buckets/queues but cannot
  recover lost file objects. Verify object durability and backups explicitly.

## A distinct identity with a small maintenance burden

The existing theme implementation is in `apps/web/src/features/theme/`, including
named themes and token editors. The application shell and layout are in
`apps/web/src/components/app/`.

After the baseline smoke test, choose a name, logo, palette and typography;
adapt a theme and the relevant branding entry points; then refine the initial
navigation and landing view around chat, files, documents and team knowledge.
Use the real running screens as references and retain the existing interaction
patterns. No visual changes or feature removals have been made yet.

## Evidence

- `README.md`
- `docker/docker-compose.yml` and `docker/docker-compose-databases.yml`
- `docs/development/running-locally.md`
- `docs/development/infrastructure.md`
- `docs/development/rust.md`
- `docs/guides/consolidate-agent-services.md`
- `crates/memory/src/domain/service.rs`
- `crates/memory/src/outbound/pg_memory_repo.rs`
- `tooling/xtask/crates/xtask_local/src/local/build/zigbuild.rs`

## Open decision

Proceed with the first bounded proof of concept locally on macOS, then assess
cloud development and dedicated hosting. The baseline remains a candidate,
not a verified production configuration. Size final hosting from measured build
peaks, idle usage, sample workloads, and recovery results.
