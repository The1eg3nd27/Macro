# Roadmap and shared engineering workflow

Updated: 2026-11-02. Scope: one privately hosted team of two or three people.
Current checkpoint: local build and core browser trial completed; preview stopped.
This document records future work, not authorization to start services or deploy.

## Next work, in order

| Stage | Work | Completion evidence |
| --- | --- | --- |
| 1. Repository handoff | Publish upstream source/history and the reviewed project record; keep project and upstream remotes distinct | Default branch and assessment branch visible remotely; no generated secrets or runtime data |
| 2. Product identity | Choose a name, original visual identity and a small initial navigation focused on chat, files, documents and team knowledge | Agreed name and interface direction grounded in real Macro screens |
| 3. Durable foundation | Correct file-storage persistence, restart ordering and local logout URLs; establish backups | New file retains identical bytes after restart; databases, sync state, objects and keys restore into a separate instance |
| 4. Complete integration proof | Fix AI worker dependency isolation and harness identity-provider requirements; configure approved providers | Two-user allowed/denied access, chat, documents, upload/download, tasks, search, AI and MCP tested with recorded results |
| 5. Team knowledge design | Decide the canonical shared knowledge store and its curation, permissions, citations and export workflow | One agreed source of truth; useful material enters deliberately; no automatic dumping into the personal vault |
| 6. Cloud development and hosting | Assess a remote Linux machine, then a persistent dedicated deployment | CPU/RAM/disk, Docker and network preflight; reproducible build; private access; durable data; successful restore |

The next implementation priority is stage 3. Identity and knowledge decisions can
be discussed while the application is stopped. A provisional remote build target
is described in [remote setup](remote-setup.md); no machine has been purchased,
provisioned or validated. Final sizing follows measurement.

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
