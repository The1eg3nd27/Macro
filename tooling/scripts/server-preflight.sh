#!/usr/bin/env bash
# Read-only Linux host inspection. Does not install, build, pull or start anything.
set -uo pipefail

network=false
if [[ ${1:-} == --network ]]; then
  network=true
  shift
fi
if [[ $# -gt 1 || ${1:-} == -* ]]; then
  printf 'Usage: bash tooling/scripts/server-preflight.sh [--network] [existing-directory]\n' >&2
  exit 2
fi
inspect_path=${1:-.}
if [[ $(uname -s) != Linux || ! -d "$inspect_path" ]]; then
  printf 'Run on Linux with an existing directory on the intended build disk.\n' >&2
  exit 2
fi

incomplete=0
printf 'Read-only inspection; a successful exit is not permission to build or deploy.\n'
printf 'Architecture: %s\n' "$(uname -m)"
printf 'Kernel: %s\n' "$(uname -r)"
printf 'Available CPU count (affinity): '
nproc || incomplete=1
printf 'Host memory (kB; cgroup limits below may be smaller):\n'
awk '/^(MemTotal|MemAvailable|SwapTotal|SwapFree):/ {print}' /proc/meminfo || incomplete=1
printf 'Build disk bytes and inodes:\n'
df -B1 "$inspect_path" || incomplete=1
df -i "$inspect_path" || incomplete=1

if [[ -r /sys/fs/cgroup/cgroup.controllers ]]; then
  relative_group=$(awk -F: '$1 == "0" {print $3}' /proc/self/cgroup)
  process_group=$(readlink -f "/sys/fs/cgroup${relative_group}" 2>/dev/null || true)
  if [[ -z "$relative_group" || ! -d "$process_group" ||
        ( "$process_group" != /sys/fs/cgroup && "$process_group" != /sys/fs/cgroup/* ) ]]; then
    printf 'Cgroups: process path unavailable; inspect the cgroup mount before building.\n'
    incomplete=1
  else
    level=0
    while :; do
      printf 'Cgroup v2 level %s (0 is this process; inspect every visible ancestor):\n' "$level"
      for limit in cpu.max cpuset.cpus.effective memory.max memory.current memory.swap.max; do
        if [[ -r "$process_group/$limit" ]]; then
          printf '  %s: %s\n' "$limit" "$(cat "$process_group/$limit")"
        fi
      done
      [[ "$process_group" == /sys/fs/cgroup ]] && break
      process_group=${process_group%/*}
      level=$((level + 1))
    done
  fi
else
  printf 'Cgroups: v2 unavailable; inspect v1 limits manually before building.\n'
  incomplete=1
fi

printf 'Tool availability:\n'
for tool in git nix docker curl timeout; do
  if command -v "$tool" >/dev/null 2>&1; then
    printf '  %s: available\n' "$tool"
  else
    printf '  %s: missing\n' "$tool"
    incomplete=1
  fi
done

if command -v docker >/dev/null 2>&1 && command -v timeout >/dev/null 2>&1; then
  printf 'Docker daemon (selected CLI context; no containers started):\n'
  daemon_info=$(timeout 15 docker info --format \
    '{{.ServerVersion}} {{.Architecture}} {{.NCPU}} {{.MemTotal}} {{.Driver}}' 2>/dev/null) || daemon_info=''
  read -r daemon_version daemon_arch daemon_cpus daemon_memory daemon_driver <<< "$daemon_info"
  if [[ -n "$daemon_version" && -n "$daemon_arch" &&
        "$daemon_cpus" =~ ^[1-9][0-9]*$ && "$daemon_memory" =~ ^[1-9][0-9]*$ ]]; then
    printf '  server=%s architecture=%s CPUs=%s memory_bytes=%s storage_driver=%s\n' \
      "$daemon_version" "$daemon_arch" "$daemon_cpus" "$daemon_memory" "$daemon_driver"
  else
    printf '  Daemon unavailable, inaccessible or timed out.\n'
    incomplete=1
  fi
  compose_version=$(timeout 15 docker compose version --short 2>/dev/null) || compose_version=''
  if [[ -n "$compose_version" ]]; then
    printf 'Compose version: %s\n' "$compose_version"
  else
    printf 'Compose: unavailable or timed out.\n'
    incomplete=1
  fi
  printf 'Confirm the Docker context is this Linux host and inspect its data-disk capacity separately.\n'
fi

if "$network"; then
  if command -v curl >/dev/null 2>&1; then
    printf 'Outbound HTTPS (no credentials; response alone does not prove downloads or API access):\n'
    for host in github.com cache.nixos.org registry.npmjs.org index.crates.io \
      static.crates.io static.rust-lang.org registry-1.docker.io ghcr.io \
      download.docker.com pypi.org files.pythonhosted.org api.openai.com api.anthropic.com; do
      if status=$(curl --disable --head --silent --output /dev/null --write-out '%{http_code}' \
        --connect-timeout 3 --max-time 4 "https://$host/"); then
        printf '  %s: HTTP %s\n' "$host" "$status"
      else
        printf '  %s: DNS, TLS, proxy or connection check failed\n' "$host"
        incomplete=1
      fi
    done
  else
    printf 'Network checks unavailable: curl is missing.\n'
    incomplete=1
  fi
else
  printf 'Network checks skipped; rerun with --network after reviewing the destinations.\n'
fi

printf 'Review CPU quotas, usable RAM, free disk, Docker and network restrictions before a full build.\n'
printf 'Keep insufficient or unresolved hosts to repository review and lightweight tests.\n'
exit "$incomplete"
