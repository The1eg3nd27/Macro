# Project brief and accepted decisions

Updated: 2026-11-08. Consolidates the original project brief and later decisions;
this is a scope record, not evidence that all requirements are implemented.

## Goal

A privately hosted, distinctly branded workspace for one team of two or three
people. Reuse Macro's existing behavior wherever practical: team chat, projects,
tasks, documents, storage, search, AI models, external MCP and permission-aware
shared knowledge. Future automation and extensibility remain goals.

## Accepted direction

- Keep most of Macro and build on it. The local preview was positively reviewed.
  The confirmed name is **The KEY Team**. The initial Dark glass and Minimal black
  concepts remain available for comparison.
- **Black Studio is selected** (2026-11-02): flat black surfaces, square controls,
  narrow labeled left navigation, white logo in the top bar and a desktop agenda
  beside the work. Other concepts remain as references. The application theme
  is not implemented yet.
- Use the supplied company logo. Prefer a restrained working interface over the
  initial generic dashboard treatment: compact rows, plain headings and limited
  glass. Review 21st.dev references without installing unassessed components.
- Keep left-side navigation. Tasks live inside the workspace, not an external
  Notion task system. Connect owners, status, deadlines and calendar presentation.
- Connect Google business email. Use iCloud as the primary calendar, with Google
  Calendar as an optional alternative. Email and calendar connections are
  independent; connecting Google mail must not select or enable Google Calendar.
  Calendar work should appear in Apple Calendar on macOS and iOS. These
  connections remain planned and unverified in this deployment.
- The owner approves durable team-knowledge changes. Suggest worthwhile context
  and tasks from authorized sources; review stale knowledge every 14 days.
  Task completion updates immediately. No review automation is scheduled yet.
- Shared company context must work across Claude, GPT and a potential local model.
  Assess external MCP integrations, including Higgsfield and Lovable, by direction
  and permission scope. None of these provider/tool workflows is verified yet.
- Claude and GPT use remote APIs; their weights are not installed on the server.
  Only the optional local model needs local model storage. A roughly 27B model
  is under consideration; selection and inference hardware sizing are deferred.
- The first bounded proof ran locally on macOS after an explicit change from the
  earlier remote-only preference. After the planning pause, local storage fixes
  and another local test were authorized on 2026-11-04.
- Future development must work from Mac and Windows using GitHub and cloud coding
  sessions. Final hosting is intended for a dedicated server; no host is selected.
- Prioritize a private server pilot before the complete branding, calendar and
  shared-knowledge implementation. Core access, durable storage and independent
  recovery remain prerequisites; see [pilot readiness](production-feasibility.md#earlier-private-team-pilot).
- Inspect actual CPU, RAM, disk, Docker support and network restrictions before
  each cloud build. Use source review/lightweight tests if capacity is insufficient.
- The personal Obsidian vault contains concise project tracking only. The shared
  team brain is a separate design decision; do not fill either with raw logs or
  uncurated conversational residue.
- Codex and Claude Code share setup/architecture/test documentation and preserve
  progress through branches and commits. Persistent application data and backups
  must not depend on ephemeral development containers.
- Project source is public for the initial handoff. Application access and data
  remain separate from repository visibility.

## Required evidence still outstanding

The intended complete proof covers task creation/assignment, search, workspace
creation, permission denial and revocation, stable restart, full backup/restore,
Claude and OpenAI requests, external MCP reads/writes, and cross-user/cross-model
shared knowledge with source references. See [test results](test-results.md) for
what actually passed and failed; do not convert an untested feature into a claim.

The architecture assessment must distinguish local components, external services
and upstream-hosted dependencies. The shared knowledge design must cover storage,
updates, manual editing, permission separation, correction/deletion and export.
The production assessment must cover authentication, durable storage, recovery,
networking, provider configuration, updates, monitoring, costs and possible EU
hosting. These later stages remain open.

## Working boundaries

Use synthetic data; preserve existing functionality and upstream notices. Keep
credentials out of Git. Do not publicly expose the development stack or deploy or
purchase infrastructure without approval. No database/volume reset or destructive
recovery is authorized by the roadmap. Inspect current code before proposing
replacement services, another vector database or a new retrieval pipeline.

See [the product blueprint](product-blueprint.md) for accepted decisions, proposed
workflows and remaining choices, and [the roadmap](roadmap.md) for implementation.
