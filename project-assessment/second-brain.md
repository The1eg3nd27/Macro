# Shared Second Brain: findings and agreed workflow

Updated: 2026-11-02. Status: source findings only; no end-to-end memory test passed.

## Inspected implementation

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

## Three separate responsibilities

1. The existing personal vault tracks project milestones and decisions only.
2. The project Git repository stores code, setup, architecture and engineering
   evidence for the coding agents.
3. The team's knowledge system holds curated, permission-aware shared context.
   Its final storage and editing interface are undecided. A separate Obsidian
   setup or a dedicated knowledge service remains an option, not an implemented
   component or a reason to duplicate Macro's existing retrieval.

## Required acceptance scenario

A first user discusses a synthetic product decision with a Claude-backed agent.
An approved, important decision enters the shared knowledge store with its source
and scope. A second user asks an OpenAI-backed agent about that decision and
receives the authorized current answer with a source reference. Removing access
must remove that user's retrieval access as well. Corrections, deletion, export
and restore must be demonstrable.

This scenario remains unverified. A passing chat/document smoke test does not
establish shared memory synthesis or reliable cross-model retrieval.

## Agreed workflow and open implementation

The owner approves lasting knowledge changes. AI proposes source-linked tasks and
knowledge updates from authorized content. Tasks live inside The KEY Team and
update completion immediately. A review every 14 days proposes stale/conflicting
material for correction or archival; it does not silently delete source history.
Company identity and writing guidance should be shared across all model providers.
No scheduled job or provider integration has been enabled. The detailed logical
design and acceptance gates are in [the product blueprint](product-blueprint.md).

- What is the authoritative record: curated documents, a dedicated knowledge
  interface, or a supported shared memory representation?
- Which existing document/knowledge representation best supports owner approval,
  versioning and source-linked updates without a parallel retrieval system?
- How are team-wide and personal information separated and permission changes
  propagated through retrieval, caches and generated summaries?
- How are conflicting/stale decisions resolved, cited, edited and forgotten?
- Which stored artifacts and keys must be backed up and exported?

Inspect the existing ingestion, retrieval and access paths first. Introduce a
separate vector database or RAG pipeline only if a measured gap requires it.
