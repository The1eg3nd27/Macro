# AI and MCP integration findings

Updated 2026-11-08. Both local services start. Harness access and editing-worker
validation were exercised with synthetic inputs. Live model and external MCP
requests are pending. See [verification and limits](team-access-and-ai.md).

## Desired integration

The workspace should use Anthropic and OpenAI models against the same authorized
team knowledge. External compatible clients should be able to search and retrieve
permitted documents, projects, tasks and knowledge, and perform allowed writes.
Keep the model API, a hosted chat product and a coding client conceptually separate;
one working connection does not establish support for all three.

## Historical findings at the original assessed revision

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
  containers were stopped on 2026-11-02. The 29 core containers resumed for the
  2026-11-04 storage repair; both AI containers were stopped at that checkpoint. No MCP endpoint
  was exposed publicly.

Evidence and prior browser results are in [test results](test-results.md).

## Local model and connector direction

Claude and GPT use provider APIs; their model weights are not installed on the
application server. The accepted direction includes an optional local model and
MCP tool connections. Only the local inference service needs locally stored model
weights. A roughly 27B model is being considered; selection and resource sizing
are deferred until after the core private pilot is prepared.

`crates/agent/src/model/router.rs` has a registry for OpenAI-compatible
providers; `crates/agent/src/test/test_router.rs` includes routing coverage for a
registered local provider. This is source evidence, not a test run or working
Ollama integration. Model configuration, tool calling and shared authorized
retrieval still need end-to-end proof.

Higgsfield publishes an MCP endpoint. Lovable documents consuming external MCP
connectors; that does not establish a reverse control API for Lovable. Internal
tasks are canonical, so an external Notion workspace is not required. See the
[blueprint](product-blueprint.md#models-and-mcp) for scope and primary references.

## Next verification

1. Trace existing provider selection, credential handling and usage accounting.
2. Retain the verified local startup and Linux worker packaging fixes; configure
   and verify the hosted harness's Google identity-provider requirement separately.
3. Supply approved credentials through ignored configuration or a secret manager.
4. Run a request through each chosen model provider; record the actual model,
   authorized source retrieval and outcome without logging secrets.
5. Exercise external MCP authentication, search/read, permitted writes, denied
   access, revocation and source references through the private endpoint.
6. Run the cross-model knowledge scenario after its curation design is agreed.

Repository references: `docs/guides/consolidate-agent-services.md`,
`docs/development/running-locally.md`, `docker/docker-compose.yml`,
`docker/ai-editing-worker.Dockerfile`, `crates/memory/src/domain/service.rs`.
