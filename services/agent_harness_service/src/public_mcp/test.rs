use mcp_auth_proxy::domain::redirect_uri::RedirectUriPolicy;

use super::*;

#[test]
fn default_redirect_hosts_permit_cursor_web_agent_callbacks() {
    let policy = RedirectUriPolicy::new(DEFAULT_MCP_ALLOWED_REDIRECT_HOSTS);

    assert!(policy.permits("https://cursor.com/agents/mcp/oauth/callback"));
    assert!(policy.permits("https://www.cursor.com/agents/mcp/oauth/callback"));
}

#[test]
fn only_an_absent_local_provider_can_disable_public_mcp() {
    use macro_env::Environment::{Develop, Local, Production};

    let missing =
        anyhow::Error::new(fusionauth::error::FusionAuthClientError::NoIdentityProviderFound)
            .context("provider lookup");
    assert!(super::local_provider_absent(Local, &missing));
    assert!(!super::local_provider_absent(Develop, &missing));
    assert!(!super::local_provider_absent(Production, &missing));
    assert!(!super::local_provider_absent(
        Local,
        &anyhow::anyhow!("connection failed")
    ));
}
