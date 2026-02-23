# TokenLedger

Enterprise-grade token management and pricing governance system.

## Quick Start

```bash
# Development
cargo run

# Testing
cargo test

# Linting
cargo clippy
```

---

# Development Philosophy

### Extend, Never Duplicate

- NEVER create a v2 file. Refactor the original.
- NEVER create a new class if an existing one can be made generic.
- NEVER create custom implementations when an OSS library exists.
- Before writing ANY new code: search the codebase for existing patterns.

### Primitives First

- Build generic building blocks before application logic.
- A provider interface + registry is better than N isolated classes.
- Template strings > hardcoded messages. Config-driven > code-driven.

### Research Before Implementing

- Check project deps for existing libraries.
- Search crates.io before writing custom code.
- For non-trivial algorithms: check GitHub for 80%+ implementations to fork/adapt.

---

# CLIProxy Comparison Matrix

## Version Overview

| Feature | CLIProxy (Original) | CLIProxy+ | CLIProxy++ |
|---------|-------------------|----------|------------|
| **Core LLM Routing** | ✅ | ✅ | ✅ |
| **OpenAI Compatible API** | ✅ | ✅ | ✅ |
| **Third-Party Providers** | ❌ | ✅ | ✅ |
| **OAuth Authentication** | ❌ | ✅ | ✅ |
| **Rate Limiting** | Basic | Advanced | ✅ |
| **Metrics & Monitoring** | ❌ | ✅ | ✅ |
| **Model Conversion** | ❌ | ✅ | ✅ |
| **SDK Support** | ❌ | ❌ | ✅ |
| **Multi-Provider Fallback** | ❌ | ❌ | ✅ |

## Detailed Feature Comparison

### Authentication & Security

| Feature | CLIProxy | CLIProxy+ | CLIProxy++ |
|---------|---------|----------|------------|
| API Key Auth | ✅ | ✅ | ✅ |
| OAuth Web Login | ❌ | ✅ (Kiro, GitHub Copilot) | ✅ |
| Device Fingerprint | ❌ | ✅ | ✅ |
| Token Refresh (Auto) | ❌ | ✅ | ✅ |
| AWS Builder ID | ❌ | ✅ | ✅ |
| AWS Identity Center | ❌ | ✅ | ✅ |

### Provider Support

| Provider | CLIProxy | CLIProxy+ | CLIProxy++ |
|----------|---------|----------|------------|
| OpenAI | ✅ | ✅ | ✅ |
| Anthropic | ✅ | ✅ | ✅ |
| Azure OpenAI | ✅ | ✅ | ✅ |
| Google Gemini | ✅ | ✅ | ✅ |
| AWS Bedrock | ✅ | ✅ | ✅ |
| GitHub Copilot | ❌ | ✅ | ✅ |
| Kiro (CodeWhisperer) | ❌ | ✅ | ✅ |
| Ollama (Local) | ❌ | ✅ | ✅ |
| LM Studio | ❌ | ✅ | ✅ |
| Custom Providers | ❌ | Community | ✅ |

### Rate Limiting & Performance

| Feature | CLIProxy | CLIProxy+ | CLIProxy++ |
|---------|---------|----------|------------|
| Request Throttling | Basic | ✅ | ✅ |
| Token Bucket | ❌ | ✅ | ✅ |
| Cooldown Management | ❌ | ✅ | ✅ |
| Usage Quotas | ❌ | ✅ | ✅ |
| Real-time Monitoring | ❌ | ✅ | ✅ |
| Adaptive Rate Limiting | ❌ | ❌ | ✅ |

### Observability

| Feature | CLIProxy | CLIProxy+ | CLIProxy++ |
|---------|---------|----------|------------|
| Request Logging | Basic | ✅ | ✅ |
| Metrics Collection | ❌ | ✅ | ✅ |
| Cost Tracking | ❌ | ✅ | ✅ |
| Latency Monitoring | ❌ | ✅ | ✅ |
| Error Rate Tracking | ❌ | ✅ | ✅ |
| Usage Dashboards | ❌ | ❌ | ✅ |

### Developer Experience

| Feature | CLIProxy | CLIProxy+ | CLIProxy++ |
|---------|---------|----------|------------|
| Docker Support | ✅ | ✅ | ✅ |
| Config File (YAML) | ✅ | ✅ | ✅ |
| Environment Variables | ✅ | ✅ | ✅ |
| Python SDK | ❌ | ❌ | ✅ |
| Go SDK | ❌ | ❌ | ✅ |
| OpenAPI Spec | ❌ | ✅ | ✅ |
| SDK Auto-Generation | ❌ | ❌ | ✅ |

### Advanced Features

| Feature | CLIProxy | CLIProxy+ | CLIProxy++ |
|---------|---------|----------|------------|
| Model Routing | Basic | ✅ | ✅ |
| Load Balancing | ❌ | ✅ | ✅ |
| Automatic Retries | ✅ | ✅ | ✅ |
| Request Caching | ❌ | ✅ | ✅ |
| Response Streaming | ✅ | ✅ | ✅ |
| UTF-8 Stream Processing | ❌ | ✅ | ✅ |
| Cost Optimization | ❌ | Basic | Advanced |

---

## Provider Matrix

### Supported Providers by Version

| Provider | Auth Type | CLIProxy | CLIProxy+ | CLIProxy++ |
|----------|-----------|----------|-----------|------------|
| OpenAI | API Key | ✅ | ✅ | ✅ |
| Anthropic | API Key | ✅ | ✅ | ✅ |
| Azure OpenAI | API Key/OAuth | ✅ | ✅ | ✅ |
| Google Gemini | API Key | ✅ | ✅ | ✅ |
| AWS Bedrock | IAM | ✅ | ✅ | ✅ |
| AWS CodeWhisperer (Kiro) | OAuth | ❌ | ✅ | ✅ |
| GitHub Copilot | OAuth | ❌ | ✅ | ✅ |
| Ollama | Local | ❌ | ✅ | ✅ |
| LM Studio | Local | ❌ | ✅ | ✅ |
| Vertex AI | OAuth | ❌ | ✅ | ✅ |

---

## Architecture Comparison

### CLIProxy (Original)
```
Client → Router → Provider → Response
```

### CLIProxy+
```
Client → Router → [Rate Limiter] → Provider → [Metrics] → Response
         ↓
    [OAuth Handler]
```

### CLIProxy++
```
Client → Router → [Cache] → [Rate Limiter] → [Load Balancer] → Provider
    ↓           ↓              ↓
 [Auth N]  [Metrics]    [Circuit Breaker]
                    ↓
              [Fallback Provider]
```

---

## When to Use Each Version

| Use Case | Recommended Version |
|----------|-------------------|
| Simple LLM proxy | CLIProxy |
| Third-party providers (Kiro, Copilot) | CLIProxy+ |
| Production with monitoring | CLIProxy+ |
| Multi-provider fallback | CLIProxy++ |
| SDK integration needed | CLIProxy++ |
| Advanced cost optimization | CLIProxy++ |

---

## Migration Path

```
CLIProxy → CLIProxy+ → CLIProxy++
   ↓           ↓           ↓
Basic     Third-party  Full-featured
Proxy     + OAuth      Production
```

---

## License

MIT License - see LICENSE file
