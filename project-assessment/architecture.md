# Preliminary architecture findings

Baseline: `3953a658d877ddfcd8c4724e0b5d66f02de008f3`, inspected 2026-10-29.
Runtime assessment updated 2026-11-08. See [verification results](test-results.md)
for the limited browser proof and unresolved deployment gaps.

## Reusable foundation

Macro is a reusable starting point for a single small team. The local browser
trial demonstrated login, chat, collaborative document editing and file transfer.
The local file-persistence failure now has a tested development fix, and both
AI services start. Model-backed operations remain unverified. Preserve the upstream
service structure and measure it before replacing infrastructure. Product features
and branding remain unchanged.

| Responsibility | Existing implementation or configuration |
| --- | --- |
| Browser interface | `apps/web`, SolidJS and TypeScript |
| Shared Rust logic | `crates/` |
| Application services | `services/`, composed in `docker/docker-compose.yml` |
| Authentication | Authentication service and FusionAuth |
| Relational data | PostgreSQL; local image is `pgvector/pgvector:pg18` |
| Search | OpenSearch and search processing service |
| Events and caching | Kafka and Redis |
| Local AWS-compatible dependencies | Pinned LocalStack persistence wrapper with instance-scoped volume; development only |
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
  identity provider. It now omits public MCP locally when that provider is absent;
  hosted configuration still requires the provider. Model and external MCP
  requests remain unverified.
- The AI editing worker failed because its Linux container received the host's
  macOS `workerd` package through the whole-repository bind mount. Container
  dependencies are now installed inside the Linux image; only local Wrangler
  state is mounted. Both startup fixes are covered in [team access](team-access-and-ai.md).
- The current MCP routes live in the combined agent harness. Do not assume an
  older issue about a missing standalone MCP container describes this revision.
- Fresh `run_local` / `stack up` operations initialize clean state and can restore
  initialization snapshots. They are not the restart path for valuable data.
- The `sync_state` volume stores local collaborative document state. Database-only
  backups are not sufficient evidence of complete document recovery.
- The first restart lost LocalStack resources and file bytes. The
  [storage repair](local-storage-persistence.md) now preserves new files across
  graceful restart and container replacement. It does not recover old missing
  bytes or establish crash resilience or production durability. A subsequent
  [recovery pass](backup-and-recovery.md) verified full local restoration into
  fresh volumes, including object bytes, sync state and encryption keys.
- Starting all stopped containers together caused dependency-readiness races;
  `stack resume` now waits for infrastructure, then dependent services, with the
  proxy last. It never provisions or initializes. `stack update` rejects missing
  saved state instead of falling back to a fresh stack.
- Logout redirected to an unused local development origin. The headless preview
  now returns to its own welcome URL. Verified onboarding and revocation still
  need a complete pilot test.

## A distinct identity with a small maintenance burden

The existing theme implementation is in `apps/web/src/features/theme/`, including
named themes and token editors. The application shell and layout are in
`apps/web/src/components/app/`.

The confirmed name is The KEY Team. The [product blueprint](product-blueprint.md)
defines the sidebar and proposed work/knowledge flows.
[Black Studio](design/black-variants.md) is the selected visual direction; earlier
concepts remain available as references.
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

Continue from verified local file persistence, complete cold restore, ordered
resume and AI startup. Prioritize the [private server pilot](production-feasibility.md#earlier-private-team-pilot)
before branding and extended integrations. Independent-host recovery, verified
restricted onboarding and production service configuration remain necessary.
Size application hosting from build peaks, stable idle usage, sample workloads
and recovery results. API-hosted models do not require local weights; optional
local inference is a separate capacity decision.
