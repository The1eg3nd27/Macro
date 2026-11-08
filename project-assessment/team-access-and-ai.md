# Local team access and AI services

Local verification on 2026-11-05 used the existing synthetic workspace on Apple
Silicon. Development login returns the verification code to the client. Keep
this instance for synthetic testing. An invite-only, inbox-verified login and
independent recovery are prerequisites for a real team pilot.

## What the three accounts could do

The requests went through the HTTPS proxy with a separate session cookie jar for
each seeded account. “Denied” below means the application returned HTTP 401.
Each signed-in identity was verified before testing its permissions.

| Operation | Team owner (Alice) | Teammate (Bob) | Outsider (Eve) |
| --- | --- | --- | --- |
| Read shared Q3 Plan and the team channel | 200 | 200 | Denied |
| Open the earlier upload owned by Bob | 200 | 200 | Denied |
| Open Alice and Dave's direct conversation | 200 | Denied | Denied |
| Rename the view-only Q3 Plan | — | Denied | Denied |
| Read the team upload with the same cookie jar after logout | Denied | Denied | Denied |

The upload is readable by another member of this team with its link. Treat it
as team-visible. The rename probe submitted the existing name. Identity checks,
document reads, channel reads and logout used normal application endpoints.
Search ranking, invitation acceptance, membership removal and already-issued
download links are separate checks for the pilot.

To repeat, use the existing `team-perms` scenario and the HTTPS proxy port printed
by `stack status`. On an existing instance, skip `seed ... apply` since it
replaces marked rows. The seeded Q3 Plan ID is `5eed2c0d-5520-8fb6-abdc-dc3a75b45645`,
acme-hq is `5eed2c0d-0a22-84b8-9233-4d5b366e1309`, and dm-alice-dave is
`5eed2c0d-6f8f-871c-85a8-974faa0a5eb6`. Copy the upload ID from its Drive URL. For each persona, sign in through the welcome page, check the
account identity and request the following paths using that account's session.

| Method | Proxy path | Fixture |
| --- | --- | --- |
| GET | `/auth/user/legacy_user_permissions` | Signed-in persona |
| GET | `/dss/documents/{id}` | Q3 Plan, then the retained upload |
| GET | `/dss/channels/{id}` | acme-hq, then dm-alice-dave |
| PATCH | `/dss/documents/{id}` | Q3 Plan with JSON body `{"documentName":"Q3 Plan"}` |
| POST | `/auth/logout` | Repeat a previously permitted read using the same cookies |

## Services and limits

- The local app clears its session through the auth service and returns the
  browser to `/app/welcome` on the same origin. Verify via Settings → Account →
  Log out, then sign in as another seeded user. Use a disposable browser profile
  for this check because signing out clears local drafts. The browser reached
  its draft-removal prompt for the synthetic teammate. Confirmation was held
  because the client could not inspect local drafts. HTTP session teardown and
  the redirect configuration were tested separately.
- The harness serves requests on the passwordless stack. A missing
  `google_gmail` provider disables the entire public MCP/OAuth router locally.
  Other startup errors still propagate. Hosted configurations require the
  provider. Locally, `GET /agent-harness/health` returned 200. Coding preferences
  returned 200 with a valid session and 401 without one. Both public MCP routes
  returned 404 without the provider.
- The editing worker installs its locked Linux dependencies inside its image.
  Rebuild it after editing worker or shared package source. The only host mount
  is the worker's existing Wrangler state directory, which holds local D1 traces.
  Startup applies local migrations. Empty provider keys allow operations that
  use no model. A request selecting a model still requires its provider key.
  An invalid edit payload returned 400, an invalid Word edit token returned 403,
  and unauthenticated trace reads returned 401. A direct service request read
  and cleaned up an empty synthetic trace ID (200). Sending the same service
  credential through the public proxy was denied (401).
- A Linux rebuild exposed an in-place overwrite of the running search worker
  executable. The local build now copies the replacement to a temporary file in
  the destination directory and renames it over the old path. Repeating the full
  cross-build kept the original search process and its healthy status.
- Model-generated replies, remote MCP clients, Gmail and calendar connections
  need separate end-to-end trials with approved accounts. The local backup and
  restart commands are in [backup and recovery](backup-and-recovery.md).

## Regression checks

The web checks passed, including 15 focused tests for sign-out and server URLs.
The worker type-check and 14 focused worker tests passed. The broader worker
suite had 461 passes and 37 failures in the rich-text table/list cell-ID fixture.
The same 37 failures reproduced on the parent branch with the same dependencies.
On 2026-11-08 the stale list-type lookup in that fixture was repaired. All 498
ordinary worker tests now pass, along with the worker type-check; see the
[rich-text repair and reproduction commands](test-results.md#2026-11-08-rich-text-fixture-repair).

The harness library and binary passed 178 unit tests with three ignored using
isolated PostgreSQL and Redis. Strict Clippy passed for both changed Rust
packages with the migrated test database. The dependency check, formatting,
changed-file and privacy checks passed. The ordinary local tooling suite passed
137 tests in a single run. Concurrent runs of one existing snapshot test can
collide on a fixed test path. Its isolated retry passed.

Both Docker-specific tooling regressions passed separately. The new cold
backup and restore also passed the three-account checks and worker trace
authorization. All 31 original containers resumed. One later source Kafka stop
returned exit 137. The completed backup predated that failure. See the
[restore record](backup-and-recovery.md#2026-11-05-restore-with-the-editing-worker)
for recovery evidence and the remaining shutdown issue.
