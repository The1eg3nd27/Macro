# Local backup and recovery

Updated: 2026-11-04. Branch: `assessment/backup-restore-startup`.

## Purpose and limits

`stack backup` saves a cold copy of a selected fully local instance, including
all its named and anonymous volumes and required runtime binds. It stops writers
before stores and resumes the services that were running before the backup.
`stack restore` creates a separate, stopped instance using fresh volumes,
networks and private copies of the runtime files. It never overwrites an instance.

This is local development recovery, not a production backup service or an
architecture migration. Use synthetic data. The existing initialization snapshot
is a clean-start cache and remains separate from these backups.

The backup includes application/authentication databases, search, Kafka, Redis,
local AWS objects/resource metadata/keys, collaborative sync state, runtime
environment, certificates, static frontend, selected executable files and mounted
service configuration/source. The analytics runtime captures only its own subtree
from its checkout-wide mount, excluding host `node_modules`; its Linux dependencies
are supplied by the existing image. Restore links the private analytics source
to `/opt/analytics-proxy/node_modules` inside that image so imports resolve
without the original host checkout. Unsupported whole-checkout mounts and sockets
are rejected. An explicitly excluded stopped service is not captured.

Container image layers are **not** exported. Keep the exact image IDs and pinned
archive-helper image available on the same Docker architecture. Container writable
layers, host developer processes, SSH reverse tunnels, external providers and
external storage are outside the backup. Production recovery needs portable image
retention, encrypted off-machine backups, retention policy and an independent
machine restore. A same-disk copy does not protect against losing the Mac.

## Preserve an existing instance

From the repository root in its pinned Nix shell, with Docker running:

```bash
cargo x stack stop --instance mac-smoke --port-base 31000
cargo x stack resume --instance mac-smoke --port-base 31000 \
  --exclude agent_harness_service --exclude ai_editing_worker
```

The exclusions describe the two known failing AI services in this trial, not a
general default. Resume refuses missing containers or state and never initializes,
builds, pulls, recreates or seeds. It waits for infrastructure health before
dependents and starts the proxy last. Services without a healthcheck are checked
for running state only. A failure returns an error; some earlier waves may already
be running and can be inspected before retrying.

`stack update` now refuses missing or invalid `stack.json`, rather than falling
back to initialization. Frozen restored instances also reject updates until an
explicit upgrade is planned. `stack up`, `run_local`, reset commands and destructive
`down` remain clean-state operations: do not use them to resume preserved data.
Host SDK reverse tunnels need separate setup; container readiness does not certify
external MCP/AI/mail/calendar integrations.

## Save and restore

Pause editing runtime files during the backup. The destination must be new, under
an existing gitignored parent, and outside captured bind folders. The command
rejects running excluded services and foreign containers writing shared volumes
or bind folders. A forced-kill exit from a storage service invalidates the backup;
forced termination of a stateless writer is reported before stores are captured.

```bash
mkdir -p .cache/backups
cargo x stack backup --instance mac-smoke --port-base 31000 \
  --exclude agent_harness_service --exclude ai_editing_worker \
  --output .cache/backups/local-trial-01

# Backup resumed the original. Stop it before restoring at the same URLs.
cargo x stack stop --instance mac-smoke --port-base 31000
cargo x stack restore --backup .cache/backups/local-trial-01 \
  --instance restore-trial-01
cargo x stack resume --instance restore-trial-01

# After validating the restored copy, return to the original.
cargo x stack stop --instance restore-trial-01
cargo x stack resume --instance mac-smoke --port-base 31000 \
  --exclude agent_harness_service --exclude ai_editing_worker
```

The frozen restore deliberately retains the source hostname and published ports:
authentication callbacks, frontend URLs and certificates depend on them. Run only
one copy at a time. Restore refuses a running source or occupied TCP ports. Use
the original verified HTTPS URL, not a URL inferred from the new instance name.
For `stack status`, repeat the source's explicit `--port-base` (31000 for this
trial); that existing command otherwise derives endpoints from the instance name.

The destination must have no existing instance directory, containers, matching
volumes or networks. There is no overwrite flag. Failed restoration retains only
the new partial destination for inspection and leaves an incomplete marker that
blocks resume. Do not remove source resources to resolve a restore failure.

Backup directories are mode 0700 and archives/configuration are 0600. They contain
credentials and encryption keys: keep them private and out of Git, logs, public
artifacts and personal project notes. Checksums detect corruption, not malicious
replacement. Restore only trusted local backups; archives and the local AWS
serialization are not safe inputs from strangers. The manifest is written last;
a partial backup is not accepted. After interrupting backup, inspect the source
and use `stack resume` if it is still stopped.

## Reproducible checks

With `SQLX_OFFLINE` unset, run `cargo test -p xtask_local`. The existing SSH test
uses port 31023, so run the whole suite when that port is free. The Docker-specific
interpolation regression is:

```bash
cargo test -p xtask_local captured_environment_and_shell_variables -- --ignored
```

For integration verification, record original container IDs/mounts and file hashes;
create unique synthetic S3/DynamoDB/SQS/KMS fixtures; back up; restore to a fresh
instance; verify object bytes and decrypt the original ciphertext. In a fresh
browser tab verify authentication, old chat/document markers and downloaded
file hashes, then upload/download a new fixture. Stop/resume the restored instance
and repeat retained-content checks without provisioning, reseeding or reuploading.
Finally stop the restored copy, resume the source and compare its original IDs and
mounts. Keep private fixture state and operational logs outside Git.

## Verification record

Executed on 2026-11-04 against the synthetic `mac-smoke` instance:

| Result | Check |
| --- | --- |
| PASS | 11 volumes and 23 runtime archives captured; approximately 414 MiB compressed; 0700 directory and 0600 artifact files |
| PASS | Original source resumed automatically after successful backup and after an earlier rejected attempt |
| PASS | Fresh restore has 29 running core services, configured healthchecks healthy, and no mount sources shared with the original |
| PASS | Fresh passwordless login, retained chat marker and collaborative document edit |
| PASS | Original 188-byte file downloaded with unchanged SHA-256 |
| PASS | New 127-byte upload opened/downloaded; retained identical bytes after ordered stop/resume of the restored copy |
| PASS | S3 bytes/metadata/presigned GET, DynamoDB record, retained queue message, notification configuration and original KMS ciphertext decryption before and after restored restart |
| PASS | Original instance name, running source, corrupt configuration and nested backup output rejected before destination mutation or source shutdown |
| PASS | All 138 crate tests, including Docker regressions; scoped strict Clippy, formatting, changed-file and privacy checks |

The first backup attempt exposed forced termination of the stateless lexical and
analytics processes. It correctly resumed the source after rejection. The final
shutdown path reports these terminations while rejecting forced termination of
storage, including a storage container already stopped by a previous failed stop.
The first restored analytics runtime exposed missing imports from the host
checkout; the Linux dependency link corrected that in a fresh second restore.
Regression tests cover repeat restore and refusal to overwrite another dependency
path. These fixes do not repair the separate agent harness/AI editing failures.

Private artifacts remain under `.cache/backup-recovery/`, including the complete
`full-20261104-02` backup, fixture state and operational logs. The first restore is
`restore-check-20261104`; the verified copy is `restore-verified-20261104`. These
copies are test destinations, not the active team instance. Retain the original
`mac-smoke` data and do not use clean-state commands on it. Full execution evidence
and the remaining gaps are in [test results](test-results.md).

At handoff, both restore copies are stopped and `mac-smoke` is running its original
29 core containers with the two AI services stopped. All original container IDs
and mounts were compared with the pre-backup record and are unchanged.
