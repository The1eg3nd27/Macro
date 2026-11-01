# Small-team self-hosting assessment

Updated: 2026-11-01; initial source assessment: 2026-10-29.
Upstream baseline: `3953a658d877ddfcd8c4724e0b5d66f02de008f3`.

Status: the local stack built and started. Browser login, team chat, document
editing, and a byte-for-byte file upload/download passed. Chat and the document
edit survived a stop/start; the uploaded file did not survive the local storage
emulator restart. AI startup and further integration checks remain blocked.
See the results for the exact coverage and limitations. Cloud development and
dedicated-server hosting remain later stages.

Scope: one team with two or three users, chat, documents, file storage, search,
tasks, AI access, and a separate team knowledge system. Reuse existing behavior
before adding features. Give the workspace a distinct identity through its
existing theme and interface architecture after establishing a working baseline.

- [Architecture findings](architecture.md): evidence, limitations, customization.
- [Local trial](local-setup.md): named instance, build limits and safe restarts.
- [Remote setup](remote-setup.md): capacity gate, private access, first boot,
  and preservation of test data.
- [Test results](test-results.md): executed commands and pending integration checks.

Keep engineering documentation in this repository. Personal project tracking
and the team's actual knowledge store are separate concerns. Application data,
credentials, generated local configuration, and backups do not belong in Git.

This branch is a local assessment checkout. Its origin points at upstream;
it must not be used to push project-specific work to upstream. A project-owned
GitHub destination has not been created. Until it exists, preserve the assessment
commits and their exported patch; no cloud workspace should be the only copy.
