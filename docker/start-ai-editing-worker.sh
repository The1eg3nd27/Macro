#!/bin/sh
set -eu

# Wrangler reads local secrets from this private container file. Expose only
# worker bindings, keeping the shared Compose environment out of the sandbox.
umask 077
node -e '
  const keys = ["OPENAI_API_KEY", "ANTHROPIC_API_KEY", "CEREBRAS_API_KEY",
    "GOOGLE_GENERATIVE_AI_API_KEY", "TRACE_ADMIN_KEY", "INTERNAL_API_KEY"];
  for (const key of keys) {
    if (process.env[key]) console.log(key + "=" + JSON.stringify(process.env[key]));
  }
' > .dev.vars
CI=true wrangler d1 migrations apply TRACES_DB --local --env local
exec wrangler dev --local --env local --ip 0.0.0.0
