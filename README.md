# TokenLedger

Enterprise-grade token management and pricing governance system for AI coding agents.

## Problem

Teams running multiple coding agents (`cursor`, `droid`, `codex`, `claude`) need one fast, real-time cost and token observability layer. Existing tooling is fragmented by provider and often too slow for live audits.

## Solution

TokenLedger provides a unified token and cost tracking system that:
- Tracks usage across all AI coding agents
- Calculates blended monthly costs (subscription + token/session)
- Surfaces `$ / MTok` per model and provider
- Emits optimization tips from measured telemetry

## Features

- **Real-time Usage Ingestion** - Fast streaming event processing
- **Multi-Provider Support** - OpenAI, Anthropic, Google, AWS, Azure, Ollama
- **Cost Analysis** - Per-model, per-provider, and blended cost views
- **Optimization Engine** - Actionable tips for cost reduction
- **Governance** - Usage policies and quota enforcement

## Quick Start

```bash
# Development
cargo run

# Testing
cargo test

# Linting
cargo clippy

# Build
cargo build --release
```

## Architecture

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│   Agents    │────▶│ TokenLedger  │────▶│   Storage   │
│ (cursor,    │     │  (ingest,    │     │ (SQLite,    │
│  droid,     │     │   track,     │     │  Postgres)  │
│  codex)     │     │   analyze)   │     │             │
└─────────────┘     └──────────────┘     └─────────────┘
                           │
                           ▼
                    ┌──────────────┐
                    │   Reports    │
                    │ (monthly,    │
                    │  alerts,     │
                    │  tips)       │
                    └──────────────┘
```

## Usage Tracking

```bash
# Run a report
cargo run -- report --monthly

# Check costs
cargo run -- costs --provider openai

# Optimize
cargo run -- optimize --dry-run
```

## Configuration

Edit `config.yaml`:

```yaml
providers:
  - name: openai
    api_key: $OPENAI_API_KEY
  - name: anthropic
    api_key: $ANTHROPIC_API_KEY

thresholds:
  monthly_limit: 5000
  per_model_limit: 1000
```

## Models Supported

| Provider | Models |
|----------|--------|
| OpenAI | gpt-4o, gpt-4o-mini, o1, o3-mini |
| Anthropic | claude-3-5-sonnet, claude-3-opus, claude-3-haiku |
| Google | gemini-1.5-pro, gemini-1.5-flash |
| AWS Bedrock | Claude, Titan, Llama |
| Azure OpenAI | gpt-4, gpt-35-turbo |

## Governance

TokenLedger integrates with the broader Kush ecosystem:

- **thegent** - Agent orchestration
- **agentapi** - Agent API routing
- **cliproxy** - LLM proxy with rate limiting

## Development Philosophy

### Extend, Never Duplicate
- NEVER create a v2 file. Refactor the original.
- NEVER create a new class if an existing one can be made generic.
- NEVER create custom implementations when an OSS library exists.

### Primitives First
- Build generic building blocks before application logic.
- A provider interface + registry is better than N isolated classes.

### Research Before Implementing
- Check crates.io for existing libraries.
- For non-trivial algorithms: check GitHub for implementations to fork/adapt.

## License

MIT License - see LICENSE file
