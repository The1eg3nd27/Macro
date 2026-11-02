# Production feasibility checkpoint

Updated: 2026-11-02. Status: provisional; no production deployment, provider
selection, cost quotation or complete recovery test has been performed.

## What the trial establishes

The assessed source built on an 18 GiB ARM Mac with two Cargo build jobs. Core
chat/document workflows and initial file transfer were demonstrated for synthetic
users. This supports continued reuse for a two-to-three-person team. It does not
establish production reliability, concurrency capacity or the minimum server size.

The development stack lost uploaded objects on a LocalStack restart. It also
exposed startup ordering, proxy recovery, logout redirect and AI dependency gaps.
Moving the same configuration to a dedicated server would preserve those risks.

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
