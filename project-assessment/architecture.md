# Preliminary architecture findings

Baseline: `3953a658d877ddfcd8c4724e0b5d66f02de008f3`, inspected 2026-10-29.
Runtime assessment updated 2026-11-01. See [verification results](test-results.md)
for the limited browser proof and unresolved deployment gaps.

## Reusable foundation

Macro is a reusable starting point for a single small team. The local browser
trial demonstrated login, chat, collaborative document editing and file transfer.
It also exposed file durability and optional-service startup failures. Preserve the upstream
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

## Shared knowledge

The inspected memory path is per-user and skips automatic generation locally.
A shared, editable team brain has not been demonstrated. See
[Second Brain findings](second-brain.md) for the source evidence, boundaries and
cross-model acceptance scenario.

## Known startup and persistence constraints

- The agent harness failed at startup because FusionAuth had no Google Gmail
  identity provider. This confirms the integration dependency described in the
  development guide. Model and external MCP requests remain unverified.
- The AI editing worker failed because its Linux container received the host's
  macOS `workerd` package through the whole-repository bind mount. Container
  dependencies need isolation before testing AI document edits.
- The current MCP routes live in the combined agent harness. Do not assume an
  older issue about a missing standalone MCP container describes this revision.
- Fresh `run_local` / `stack up` operations initialize clean state and can restore
  initialization snapshots. They are not the restart path for valuable data.
- The `sync_state` volume stores local collaborative document state. Database-only
  backups are not sufficient evidence of complete document recovery.
- The local restart actually lost LocalStack resources and the uploaded test
  object's bytes. Reprovisioning restored empty resources, not file contents.
  Durable object storage and restore testing are prerequisites for team use.
- Starting all stopped containers together caused dependency-readiness races;
  start infrastructure first, provision missing resources, then start services.
- Logout redirected to an unused local development origin. The headless preview
  needs consistent welcome/logout URLs before a polished team onboarding flow.

## A distinct identity with a small maintenance burden

The existing theme implementation is in `apps/web/src/features/theme/`, including
named themes and token editors. The application shell and layout are in
`apps/web/src/components/app/`.

The confirmed name is The KEY Team. The [product blueprint](product-blueprint.md)
defines a proposed sidebar and work/knowledge flows; [two visual concepts](design/README.md)
compare Dark glass and Minimal black. A final visual direction is still open.
After the baseline issues are resolved, adapt the existing theme and branding
entry points, then refine navigation and the landing view.
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

Continue from the demonstrated core workflows. First make file storage durable,
correct restart ordering and preview redirects, then resolve AI service startup
and test multi-user access and backup restoration. Apply branding after those
baseline issues are understood. Cloud development and dedicated hosting follow;
the local emulator setup is not a production configuration. Size final hosting
from build peaks, stable idle usage, sample workloads, and recovery results.
