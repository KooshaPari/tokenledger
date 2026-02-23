# ADR - tokenledger Core Decisions

## ADR-001: Rust Core for Aggregation

- Status: Accepted
- Context: Python/TypeScript implementations offer flexibility but can be slow for large live logs and repeated aggregation loops.
- Decision: Build the analytics core in Rust with streaming JSONL parsing.
- Consequence: Higher throughput and lower latency; adapter bridges can still be polyglot.

## ADR-002: Normalized Event Contract

- Status: Accepted
- Context: Each provider exposes different log/database shapes.
- Decision: All provider adapters emit a normalized JSONL event schema consumed by one cost engine.
- Consequence: Provider-specific complexity is isolated at ingestion edges.

## ADR-003: Blended Cost Model

- Status: Accepted
- Context: Token-only metrics understate real spend when subscriptions exist.
- Decision: Monthly total = variable token cost + allocated subscription cost; allocation proportional to monthly token share.
- Consequence: Fair cross-provider blended comparison with explicit assumptions.
