# AI and MCP integration findings

Updated: 2026-11-02. Status: startup dependencies identified; model and external
MCP requests have not been verified. No real model credentials were configured.

## Desired integration

The workspace should use Anthropic and OpenAI models against the same authorized
team knowledge. External compatible clients should be able to search and retrieve
permitted documents, projects, tasks and knowledge, and perform allowed writes.
Keep the model API, a hosted chat product and a coding client conceptually separate;
one working connection does not establish support for all three.

## Findings at the assessed revision

- The combined `services/agent_harness_service` contains the current agent/MCP
  path. The older report about an absent standalone MCP Compose service is not
  sufficient evidence about this revision's implementation.
- Startup failed while composing public MCP because FusionAuth had no Google
  Gmail identity provider. Determine whether approved OAuth configuration alone
  is appropriate or whether the provider coupling needs a targeted code change.
  Do not fabricate an identity provider to claim the integration works.
- The separate AI editing worker failed on Linux because the repository bind
  mount exposed the host's Darwin `workerd` dependency. Its dependencies need a
  container-specific installation boundary.
- Local automatic memory generation is gated off in the inspected path; see
  [Second Brain findings](second-brain.md). A working agent harness alone will
  not resolve that limitation.
- The harness was stopped after repeated failures. All other local preview
  containers were stopped on 2026-11-02. No MCP endpoint was exposed publicly.

Evidence and prior browser results are in [test results](test-results.md).

## Next verification

1. Trace existing provider selection, credential handling and usage accounting.
2. Resolve the harness startup dependency and Linux worker packaging.
3. Supply approved credentials through ignored configuration or a secret manager.
4. Run a request through each chosen model provider; record the actual model,
   authorized source retrieval and outcome without logging secrets.
5. Exercise external MCP authentication, search/read, permitted writes, denied
   access, revocation and source references through the private endpoint.
6. Run the cross-model knowledge scenario after its curation design is agreed.

Repository references: `docs/guides/consolidate-agent-services.md`,
`docs/development/running-locally.md`, `docker/docker-compose.yml`,
`docker/ai-editing-worker.Dockerfile`, `crates/memory/src/domain/service.rs`.
