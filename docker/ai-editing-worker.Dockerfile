FROM node:22-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
	&& rm -rf /var/lib/apt/lists/*
RUN npm install -g bun@1.3.13

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

WORKDIR /app
COPY . .
RUN bun install --frozen-lockfile --filter ai-editing-worker

WORKDIR /app/services/ai-editing-worker
RUN bun scripts/generate-sandbox.ts
EXPOSE 8933

ENV PATH="/app/node_modules/.bin:${PATH}"
ENV WRANGLER_SEND_METRICS=false
COPY docker/start-ai-editing-worker.sh /usr/local/bin/start-ai-editing-worker
CMD ["sh", "/usr/local/bin/start-ai-editing-worker"]
