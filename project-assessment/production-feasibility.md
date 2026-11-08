# Production feasibility checkpoint

Updated 2026-11-08. Local cold recovery has passed. Production deployment,
host selection and independent off-machine recovery remain open.

## Earlier private team pilot

The current priority is a private server pilot for two or three users before the
complete visual customization, calendar integrations and shared-knowledge workflow.
Access from Mac and Windows should reach the same persistent workspace over a
private network with HTTPS. This is a deployment target, not an existing service.
No Linux server has been provisioned or assessed; Netcup remains a candidate.

Start with the demonstrated chat, document and file workflows. The rich-text
fixture failures are [resolved](test-results.md#2026-11-08-rich-text-fixture-repair),
with 498 ordinary worker tests passing. Before accepting real team data:

| Work | Required evidence |
| --- | --- |
| Linux host and reproducible build | CPU, RAM, disk, Docker and network preflight from [remote setup](remote-setup.md); pinned source and images; measured resource use |
| Private access and authentication | Private ingress, valid HTTPS and fresh secrets; verified account ownership; restricted onboarding; teammate invitation/removal and session revocation tests |
| Persistent services | Explicit production configuration for databases, authentication, sync state, objects and AWS dependencies; no reliance on the local emulator for valuable team data |
| Recovery and updates | Resolve the intermittent Kafka stop; encrypted off-host backup; restore on a separate Linux host with files, messages, documents and keys intact; repeat after a controlled update |
| Team acceptance | Two or three separate accounts exercise chat, document collaboration, file transfer, search and allowed/denied access through the actual private client path |

The local build explicitly enables
`authentication_service/return_passwordless_code`; changing its hostname or
environment file does not remove that compiled behavior. The hosted build must
omit it and verify that login responses never reveal the code. Inspect the
existing signup policy as well: a small team still needs actual authorization,
even when the network is private. Hosted harness startup also currently requires
the configured Google identity provider; the missing-provider bypass is local only.

Do not copy the development Compose configuration into a public deployment.
Its credentials, certificates, service ports and AWS emulator are development
fixtures. The initial remote recipe is suitable for synthetic integration tests;
the gates above distinguish that preview from a workspace for real team data.

## Model hosting boundary

Claude and GPT run through their providers' APIs, so their model weights are not
installed on the application server. API keys and application data remain subject
to normal storage, access and backup controls. Actual provider requests still
need verification before enabling AI for the team.

Only the optional local model requires locally stored model weights and inference
resources. A roughly 27B model is under consideration, with selection, precision,
context length, concurrency and hardware sizing deferred. Do not include a model
download or GPU purchase in the first pilot. Keep inference a separate service
boundary so it can later run on suitable hardware without moving application data.
The current provider extension point is documented in [AI integrations](ai-integrations.md).

## What the trial establishes

The assessed source built on an 18 GiB ARM Mac with two Cargo build jobs. Core
chat/document workflows and initial file transfer were demonstrated for synthetic
users. This supports continued reuse for a two-to-three-person team. It does not
establish production reliability, concurrency capacity or the minimum server size.

The original development stack lost uploaded objects on a LocalStack restart.
A [local persistence repair](local-storage-persistence.md) passed graceful
restart and container-replacement tests on 2026-11-04. It remains a development
emulator with unverified crash recovery. Complete cold backup restoration on the
same Docker engine and dependency-ordered resume are now verified in the
[recovery record](backup-and-recovery.md). Independent host recovery and model-backed integration tests remain.
Both local AI processes now start; see [local access tests](team-access-and-ai.md). Moving this development
configuration to a dedicated server does not establish production readiness.

## Proposed deployment direction

Keep development sessions disposable, source in Git, and valuable data on
persistent managed storage with verified backups. Start remote integration tests
on a normal Linux machine that can run Docker. The provisional 8-vCPU/32-GB/320-GB
build target and required resource/network checks are in
[remote setup](remote-setup.md); this is a conservative starting point, not a
measured runtime minimum or a purchased machine.

Before production, define and verify:

- Durable relational/authentication stores, collaborative sync state and file
  objects, with compatible encryption-key retention and separate backup copies.
- Private team authentication, access boundaries, TLS, routing and inbound
  firewall rules. Development credentials and publicly reachable service ports
  are not the production access model.
- Replacements or production configurations for AWS emulators and required
  external services; identify each actual AWS/Cloudflare dependency before
  removing or substituting it.
- Model-provider credentials, data handling, logging, monitoring and cost limits.
- A coordinated update/migration procedure and restore into an isolated instance.
- Container/source pinning, including the agent sandbox's nested source checkout.
- Third-party service licensing and AGPL obligations; see
  [licensing and branding](licensing-and-branding.md).

EU hosting remains a possible requirement. Server location alone does not prove
that all data stays in that region: model APIs, OAuth integrations, telemetry,
backups and other processors must be assessed individually. No residency or
compliance claim is made by this checkpoint.

## Reuse decision

| Approach | Current assessment |
| --- | --- |
| Retain most of Macro and customize | Selected direction after the source audit and positively reviewed local preview; fix bounded gaps first |
| Simplify Macro | Consider after dependency and runtime evidence identifies components that can actually be removed |
| Build a new lightweight app | Not selected; would require reimplementing verified collaboration features and a new maintenance/security assessment |

Costs and relative effort remain to be scoped against specific designs. No claim
is made that a rewrite or a smaller service count would be cheaper or safer.
