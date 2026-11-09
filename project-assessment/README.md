# The KEY Team — project record

Updated 2026-11-09. Upstream baseline `3953a658d877ddfcd8c4724e0b5d66f02de008f3`.

The local workspace passed [file persistence](local-storage-persistence.md),
[backup and recovery](backup-and-recovery.md) and
[three-account access checks](team-access-and-ai.md). The two AI processes now
start without a Google sign-in provider or a model connection. The local logout
destination uses the active workspace origin. Real team onboarding, model
responses, calendar connections and independent host recovery are still open.
The 37 rich-text fixture failures are resolved: all 498 ordinary worker tests
pass. The next milestone is an [earlier private server pilot](production-feasibility.md#earlier-private-team-pilot),
before complete branding and integrations. Claude and GPT use APIs; optional
local inference remains a later hardware decision.

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
- [Backup and recovery](backup-and-recovery.md): private cold backups, fresh-volume restore, readiness ordering and recovery proof.
- [Remote setup](remote-setup.md): capacity gate, private access, first boot,
  and preservation of test data.
- [Server setup](server-setup.md): current host proposal, provisioning sequence,
  read-only inspection command and remaining private deployment configuration.
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
