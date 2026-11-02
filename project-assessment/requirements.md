# Project brief and accepted decisions

Updated: 2026-11-02. Consolidates the original project brief and later decisions;
this is a scope record, not evidence that all requirements are implemented.

## Goal

A privately hosted, distinctly branded workspace for one team of two or three
people. Reuse Macro's existing behavior wherever practical: team chat, projects,
tasks, documents, storage, search, AI models, external MCP and permission-aware
shared knowledge. Future automation and extensibility remain goals.

## Accepted direction

- Keep most of Macro and build on it. The local preview was positively reviewed;
  product name and visual identity remain open.
- The first bounded proof ran locally on macOS after an explicit change from the
  earlier remote-only preference. The application is now stopped as requested.
- Future development must work from Mac and Windows using GitHub and cloud coding
  sessions. Final hosting is intended for a dedicated server; no host is selected.
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

Start with [the roadmap](roadmap.md) for the next discussion and implementation.
