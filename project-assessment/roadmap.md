# Roadmap and shared engineering workflow

Updated: 2026-11-08. Scope: one privately hosted team of two or three people.
Current checkpoint: local file persistence and complete local restore verified;
ordered stop/resume implemented. Local AI startup and three-account access checked.
Source/history handoff is complete. The KEY Team name and Black Studio design
direction are confirmed. The interactive concept has not been applied to Macro.
This document records future work, not authorization to start services or deploy.

## Next work, in order

| Stage | Work | Completion evidence |
| --- | --- | --- |
| 1. Repository handoff | Publish upstream source/history and the reviewed project record; keep project and upstream remotes distinct | Default branch and assessment branch visible remotely; no generated secrets or runtime data |
| 2. Product identity | Name and Black Studio direction selected on 2026-11-02 | Decision recorded; concept remains separate from application implementation |
| 3. Durable foundation | Correct file-storage persistence, restart ordering and local logout URLs; establish backups | New file retains identical bytes after restart; databases, sync state, objects and keys restore into a separate instance |
| 4. Private server pilot | Assess Linux resources; prepare verified, restricted onboarding, private networking and durable services; resolve shutdown and independent recovery gaps | Two or three accounts use chat/documents/files; access and revocation pass; encrypted off-host backup restores on a separate machine; see [pilot gates](production-feasibility.md#earlier-private-team-pilot) |
| 5. Black Studio shell | Apply branding, theme, narrow navigation rail and responsive shell through existing components; connect the Today overview to existing records | Real navigation works; existing functionality preserved; keyboard, contrast and desktop/mobile checks pass |
| 6. Complete integration proof | Verify model-backed actions and gated MCP access; connect Google mail, add primary iCloud calendar support and optional Google Calendar independently | Two-user access, chat, documents, files, tasks, search, email, calendar sync, AI and MCP tested; Apple Calendar changes round-trip without duplicates |
| 7. Team knowledge implementation | Owner approval and 14-day review agreed; choose the storage representation after reuse assessment, then implement revisions, permissions and export | Cross-user/cross-model knowledge scenario passes; correction, revocation, archive and restore demonstrated |

Stage 3 is partly complete: new files survive full-stack restart, isolated
container replacement and restoration into fresh volumes. Dependency-ordered
stop/resume and missing-state guards are implemented. The local logout destination is fixed. See [team access](team-access-and-ai.md),
[storage repair](local-storage-persistence.md) and [recovery](backup-and-recovery.md).
The 37 rich-text fixture failures are resolved: all 498 ordinary worker tests pass.
Next, prepare the private pilot in stage 4: exercise verified, restricted onboarding
and revocation, investigate the intermittent Kafka stop and prove independent
recovery. The first hosted milestone is a durable workspace for the small team;
the selected Black Studio shell follows. Detailed knowledge decisions can be
discussed alongside those foundation checks. A provisional remote build target
is described in [remote setup](remote-setup.md); no machine has been purchased,
provisioned or validated. Final sizing follows measurement.
Netcup remains a hosting candidate. Claude and GPT will use APIs; optional local
inference and the possible 27B model are deferred and do not block the core pilot.

See [the product blueprint](product-blueprint.md) and [visual comparison](design/README.md)
for this design checkpoint. The mockups use synthetic data and do not change the app.
The [mail/calendar decision](product-blueprint.md#email-and-calendar-connections)
adds iCloud integration work; it does not change the durability-first priority.

## Where things belong

| Concern | Source of truth |
| --- | --- |
| Product decisions, findings, next work | `project-assessment/` |
| Application and service code | `apps/`, `packages/`, `services/`, `crates/` |
| Infrastructure and development tooling | `infra/`, `docker/`, `tooling/` |
| Shared coding-agent instructions | Root `CLAUDE.md`, linked by `AGENTS.md`, plus the relevant directory guides |
| Setup, tests and observed limitations | `project-assessment/local-setup.md`, `remote-setup.md`, `test-results.md` |
| Personal project tracking | A concise project note in the existing personal vault; no engineering log dump |
| Team knowledge and application data | Separate persistent stores with access controls and tested backups; not Git |
| Credentials and local generated configuration | Ignored environment files or an approved secret manager; not Git |

## Codex and Claude Code workflow

1. Read the root agent guide, relevant area guide, project record and latest tests.
2. Inspect Git status and the actual code before relying on a previous note.
3. Use a feature branch; use separate worktrees when agents work concurrently.
   Do not concurrently edit the same files or mutate the same running instance.
4. Keep changes small, preserve upstream attribution, and avoid unrelated rewrites.
5. Run the affected checks, record what passed/failed, and obtain the required
   independent audit before handoff. Commit using the repository's Git skill.
6. Push to the project remote only when authorized; review changes before merging.
7. Update the personal project note only for a milestone, important decision or
   meaningful status change. Link to the engineering record for details.

Record branch, commit, checks, remaining blockers and instance state at every
handoff. Keep runtime data on persistent storage with backups; Git commits do not
preserve Docker volumes or uploaded file bytes.

## GitHub Actions

The repository owner confirmed on 2026-11-02 that Actions was disabled in the
project repository before the initial import. Upstream workflows are preserved
as source, but include deployment jobs, schedules, private runners and provider
assumptions. Keep Actions disabled until a project-specific CI/deployment review;
enable only the checks intended for this deployment. No workflow was dispatched.

## Publication boundary

Publish tracked source, upstream history and notices, curated engineering docs,
and synthetic screenshots. Exclude local environment files, caches, generated
certificates, application databases/objects, private vault content and raw logs.
Upstream's already tracked development fixtures remain identifiable as upstream
material. Review changes and scan for secrets before each public push.
