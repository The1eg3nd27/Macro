# Small-team self-hosting assessment

Assessment date: 2026-10-29.
Upstream baseline: `3953a658d877ddfcd8c4724e0b5d66f02de008f3`.

Status: preliminary source assessment and lightweight tooling checks complete;
the application has not been built, started, or verified in a browser.
No remote Linux environment is connected yet. The full stack must run remotely.

Scope: one team with two or three users, chat, documents, file storage, search,
tasks, AI access, and a separate team knowledge system. Reuse existing behavior
before adding features. Give the workspace a distinct identity through its
existing theme and interface architecture after establishing a working baseline.

- [Architecture findings](architecture.md): evidence, limitations, customization.
- [Remote setup](remote-setup.md): capacity gate, private access, first boot,
  and preservation of test data.
- [Test results](test-results.md): executed commands and pending integration checks.

Keep engineering documentation in this repository. Personal project tracking
and the team's actual knowledge store are separate concerns. Application data,
credentials, generated local configuration, and backups do not belong in Git.

This branch is a local assessment checkout. Its origin points at upstream;
it must not be used to push project-specific work to upstream. A project-owned
GitHub destination has not been created. Until it exists, preserve the assessment
commit and its exported patch; no cloud workspace should be the only copy.
