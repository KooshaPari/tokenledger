# tokenledger

Enterprise-grade token management and pricing governance system for AI coding agents.

This repository works with Claude and other AI agents as autonomous software engineers.

## Quick Start

```bash
# Development
cargo run

# Testing
cargo test

# Linting
cargo clippy
```

## Environment

```bash
# Required environment variables
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-..."
```

---

## Development Philosophy

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

- Check crates.io for existing libraries.
- Search GitHub for 80%+ implementations to fork/adapt.

---

## Library Preferences (DO NOT REINVENT)

| Need | Use | NOT |
|------|-----|-----|
| Async runtime | tokio | custom async |
| HTTP client | reqwest | custom wrappers |
| Logging | tracing | print() or log::logger |
| CLI | clap | manual arg parsing |
| Validation | validator | manual if/else |
| Database | sqlx | raw SQL strings |
| Rate limiting | governor | custom rate limiter |

---

## Code Quality Non-Negotiables

- Zero new lint suppressions without inline justification
- All new code must pass: cargo clippy, cargo fmt, tests
- Max function: 40 lines
- No placeholder TODOs in committed code
- All Rust code must be clippy-clean

---

## Verifiable Constraints

| Metric | Threshold | Enforcement |
|--------|-----------|-------------|
| Test coverage | >= 80% | cargo-tarpaulin |
| Security findings | 0 high/critical | cargo-audit |
| Clippy warnings | 0 | CI gate |

---

## Domain-Specific Patterns

### What tokenledger Is

tokenledger is a **token and cost tracking system** for AI coding agents. The core domain is: provide unified token and cost tracking across multiple providers with optimization recommendations.

### Key Interfaces

| Interface | Responsibility |
|-----------|---------------|
| CLI commands | report, costs, optimize |
| Provider traits | Multi-provider abstraction |
| Storage | SQLite, PostgreSQL support |

---

## Integration

### With thegent

```python
# thegent config
llm:
  provider: cliproxy
  base_url: http://localhost:8317/v1
```

### With agentapi

```bash
agentapi --cliproxy http://localhost:8317
```

---

## License

MIT License - see LICENSE file
