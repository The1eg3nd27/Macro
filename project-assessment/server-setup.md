# Private server setup

Prepared 2026-11-09. Neither the Linux server nor a deployment has been set up.
The host proposal and inspection command are ready; real-team application
configuration still needs implementation. No purchase or subscription was made.

## Host proposal for review

Start the application pilot without local inference. The existing conservative
build target is at least 8 CPUs, 32 GB RAM and approximately 320 GB SSD, with two
Cargo build jobs. This is a provisional target, not a measured minimum. Keep
builds and synthetic integration tests off a host serving valuable team data once
the pilot is live; use separate disposable compute with the same Git revision.

The current Netcup VPS 4000 G12.5 provides 12 shared vCPUs, 32 GB RAM and 512 GB
SSD. Its live configurator, with a one-month contract, automatic European location
and IPv4/IPv6, displayed **EUR 53.92/month including 23% VAT**, with no setup fee.
The displayed tax selection is not a determination of the buyer's tax status.
Confirm the final tax, monthly price, billing and cancellation terms at checkout.
[Verified configuration](https://www.netcup.com/en/server/vps/vps-4000-g12.5-iv-24m-eu#vps-4000-g12.5-iv-1m-eu).

Prefer a monthly pilot while runtime requirements are still being measured.
The unselected base page defaults to 24 months; do not mistake that lower monthly
equivalent for a monthly contract. This is a virtual server with shared CPU;
Netcup's Root Server alternative has dedicated CPU/RAM allocations but is still
virtualized, not a whole physical machine. See the current
[VPS](https://www.netcup.com/en/server/vps) and
[Root Server](https://www.netcup.com/en/server/root-server) catalogs.

The proposal excludes off-host backup, model API usage, any additional storage
services and a future inference host. Budget approval and account provisioning
are outstanding. The older G12/1-TB marketing listings differ from the current
G12.5 order configuration; use the actual selected order details.

## Provisioning sequence

1. Confirm the monthly budget and provider configuration. Complete the provider
   account, billing and order. Keep the provider account under the project's
   ownership. No provider account or authenticated server access is available in
   this session.
2. Install a fresh Ubuntu 24.04 LTS x86-64 system on the new server. Use SSH keys
   and a dedicated administrator account. Keep private keys off Git and out of
   chat. Verify console/recovery access before changing SSH or firewall rules.
3. Inspect the host before installing the full toolchain or building. Run the
   command below; missing Docker/Nix on a fresh system is expected. Record CPU,
   memory, disk, cgroup and outbound-network limits. If resources are inadequate,
   stop at repository review and lightweight tests.
4. Set provider ingress rules for authorized administration and the chosen private
   VPN only. Apply both IPv4 and IPv6 restrictions. The app and backing services
   must remain unreachable on public interfaces. Install Docker Engine/Compose
   through the [official Ubuntu package instructions](https://docs.docker.com/engine/install/ubuntu/),
   then Nix using the repository's [development prerequisites](../docs/development/running-locally.md#shared-prerequisites).
   Record installed versions and rerun the inspection as the intended build user.
5. Use the pinned Nix shell and frozen dependencies on persistent build storage.
   Run `just doctor-local` and the existing [remote synthetic preview procedure](remote-setup.md).
   The development stack is for synthetic data. A passing preview does not make
   its credentials, authentication or LocalStack configuration suitable for the team.
6. Implement and verify the application gaps below, then restore a backup on a
   separate Linux machine. Only then admit real accounts and data. Exercise the
   actual Mac and Windows browser path, including HTTPS, redirects and WebSockets.

Netcup's [firewall documentation](https://www.netcup.com/en/helpcenter/documentation/server/firewall)
states that newly ordered servers initially allow traffic except SMTP. Configure
ingress explicitly; opening login-email delivery requires reviewing the SMTP
restriction too. Keep provider firewall controls alongside host/container rules:
Docker documents that [published container ports can bypass UFW](https://docs.docker.com/engine/network/packet-filtering-firewalls/).

## Read-only preflight

From the checkout on the Linux machine, before a full build:

```sh
bash tooling/scripts/server-preflight.sh --network .
```

The script reports architecture, affinity-visible CPUs, host memory, filesystem
bytes/inodes, the current cgroup v2 and its visible ancestors, required tools,
Docker daemon resources and Compose availability. `--network` makes unauthenticated
HTTPS HEAD requests to the fixed package/provider hosts listed in the script.
Omit it for an offline inspection. It never installs, builds, pulls or starts
containers, reads application secrets, or changes the host.

Exit `0` means the requested inspection completed; `1` means a tool, cgroup,
daemon or network inspection was incomplete; `2` rejects unsupported platforms
or arguments. No exit code certifies capacity or deployment readiness.

For CPU, divide each finite `cpu.max` quota by its period and use the smallest
visible quota alongside affinity/cpuset limits. For RAM, use the smallest visible
`memory.max` and the host memory limits, allowing for current usage. `max` means
that particular level imposes no limit; a parent can still limit it. Cgroup v1 or
an unresolved namespace requires manual inspection. Check provider disk quotas
and free space on both the checkout filesystem and Docker's actual data disk.
Filesystem totals inside a container may not represent the host's physical space.
Confirm that the selected Docker context targets this server, not another host.

HTTP status codes establish only that a response was received. Authentication
errors, proxy filtering, downloads, registry pulls, API credentials and mail
delivery need separate checks. Compare the results with the provisional target
in [remote setup](remote-setup.md); do not launch a full build when limits or
required connectivity are inadequate or unresolved.

## Application configuration still to implement

| Boundary | Source evidence and required change |
| --- | --- |
| Restricted signup | `authentication_service/src/config.rs` currently allows all Production/Local signups and forces the Develop bypass. Its existing `SignupPolicy` also admits the upstream staff domain. Add an explicit private deployment policy with exact approved identities and no domain exception; reuse the existing transactional FusionAuth webhook boundary. Test every creation path. |
| Verified login | Build without `authentication_service/return_passwordless_code`; verify code-free login responses and real account ownership through inbox verification or configured SSO. The private signup policy must not be bypassable by development flags. |
| Revocation | A signup allowlist alone does not remove existing users or revoke sessions, API tokens or issued object links. Verify those lifecycle boundaries separately. |
| Durable services | Supply production configurations for Postgres, FusionAuth, sync state, objects and the required AWS interfaces. Inventory S3/SQS/DynamoDB/KMS and related dependencies before selecting replacements. LocalStack remains limited to the synthetic preview. |
| Origins and secrets | Set the private HTTPS origin consistently across browser, auth, sync and workers; generate fresh credentials/certificates through a reviewed setup, outside Git. Do not reuse seed identities or development keys. |
| AI | Provider APIs require approved keys and real request tests. Hosted harness startup currently requires a Google identity provider. Optional local inference and the possible 27B model are deferred. |
| Recovery | Resolve or isolate the intermittent Kafka shutdown problem on Linux; retain encrypted off-host copies; prove restore and a controlled update without reinitializing data. |

These are open implementation gates, not completed configuration. The wider
[pilot acceptance criteria](production-feasibility.md#earlier-private-team-pilot)
remain the source of truth for allowing real team data.

## Verification of the preparation

`bash -n tooling/scripts/server-preflight.sh` passed. Running it on macOS correctly
returned `2`. An isolated, read-only Linux container with no network, one CPU and
256 MiB RAM reported `cpu.max = 100000 100000` and
`memory.max = 268435456`, despite larger host totals. Missing tools, an unavailable
Docker daemon/Compose and all blocked network destinations were reported; exit
`1` was expected. No existing application container, volume or browser session
was modified. A real Linux VM, successful network probes and a reachable remote
Docker daemon remain unverified until the host exists.
