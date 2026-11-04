# The KEY Team — project record

Updated: 2026-11-04; initial source assessment: 2026-10-29.
Upstream baseline: `3953a658d877ddfcd8c4724e0b5d66f02de008f3`.

Status: core browser workflows passed the initial trial. The file loss found
then has a local development fix: a new upload now survives full-stack stop/start
with identical downloaded bytes. An isolated regression also passed container
replacement. See the [storage repair](local-storage-persistence.md) for scope and
limits. The 29 core containers are running after the 2026-11-04 retest; the two AI
services remain stopped. Logout, automated restart ordering, full backup restore
and further integration checks remain open. Cloud development and dedicated
hosting remain later stages. Existing databases and generated configuration were
preserved; runtime data is not stored in Git.

Scope: one team with two or three users, chat, documents, file storage, search,
tasks, AI access, and a separate team knowledge system. Reuse existing behavior
before adding features. Give the workspace a distinct identity through its
existing theme and interface architecture after establishing a working baseline.

- [Product blueprint](product-blueprint.md): accepted identity, workflows and proposed architecture.
- [Design direction](design/README.md): selected Black Studio concept and retained alternatives.
- [Consolidated brief](requirements.md): original goals and accepted scope changes.
- [Second Brain findings](second-brain.md): inspected memory and open curation design.
- [AI integrations](ai-integrations.md): provider/MCP blockers and remaining proof.
- [Production feasibility](production-feasibility.md): provisional deployment assessment.
- [Roadmap and agent workflow](roadmap.md): priorities, ownership and handoff.
- [Licensing and branding](licensing-and-branding.md): private use and rebranding.
- [Architecture findings](architecture.md): evidence, limitations, customization.
- [Local trial](local-setup.md): named instance, build limits and safe restarts.
- [Remote setup](remote-setup.md): capacity gate, private access, first boot,
  and preservation of test data.
- [Test results](test-results.md): executed commands and pending integration checks.

Keep engineering documentation in this repository. Personal project tracking
and the team's actual knowledge store are separate concerns. Application data,
credentials, generated local configuration, and backups do not belong in Git.

The project-owned repository is the publication destination. Keep its remote
separate from upstream: `origin` is the project repository and `upstream` is
`https://github.com/macro-inc/macro.git`. Verify URLs before pushing; never push
project-specific changes to upstream. The initial import preserves the upstream
history plus the assessment commits. Do not publish generated environment files,
credentials, runtime logs, application content or build caches.
