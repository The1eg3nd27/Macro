# Roadmap and shared engineering workflow

Updated: 2026-11-02. Scope: one privately hosted team of two or three people.
Current checkpoint: local build and core browser trial completed; preview stopped.
Source/history handoff is complete. The KEY Team name and Black Studio design
direction are confirmed. The interactive concept has not been applied to Macro.
This document records future work, not authorization to start services or deploy.

## Next work, in order

| Stage | Work | Completion evidence |
| --- | --- | --- |
| 1. Repository handoff | Publish upstream source/history and the reviewed project record; keep project and upstream remotes distinct | Default branch and assessment branch visible remotely; no generated secrets or runtime data |
| 2. Product identity | Name and Black Studio direction selected on 2026-11-02 | Decision recorded; concept remains separate from application implementation |
| 3. Durable foundation | Correct file-storage persistence, restart ordering and local logout URLs; establish backups | New file retains identical bytes after restart; databases, sync state, objects and keys restore into a separate instance |
| 4. Black Studio shell | Apply branding, theme, narrow navigation rail and responsive shell through existing components; connect the Today overview to existing records | Real navigation works; existing functionality preserved; keyboard, contrast and desktop/mobile checks pass |
| 5. Complete integration proof | Fix AI worker dependency isolation and harness identity-provider requirements; configure approved providers | Two-user allowed/denied access, chat, documents, upload/download, tasks, search, AI and MCP tested with recorded results |
| 6. Team knowledge implementation | Owner approval and 14-day review agreed; choose the storage representation after reuse assessment, then implement revisions, permissions and export | Cross-user/cross-model knowledge scenario passes; correction, revocation, archive and restore demonstrated |
| 7. Cloud development and hosting | Assess a remote Linux machine, then a persistent dedicated deployment | CPU/RAM/disk, Docker and network preflight; reproducible build; private access; durable data; successful restore |

The next implementation priority is stage 3, followed by the selected shell in
stage 4. The first build milestone is a durable, branded workspace: uploaded files
survive restart, a separate restore succeeds, and the Black Studio shell preserves
working chat/document/file access. Detailed knowledge decisions can be discussed
while the application is stopped. A provisional remote build target
is described in [remote setup](remote-setup.md); no machine has been purchased,
provisioned or validated. Final sizing follows measurement.

See [the product blueprint](product-blueprint.md) and [visual comparison](design/README.md)
for this design checkpoint. The mockups use synthetic data and do not change the app.

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
