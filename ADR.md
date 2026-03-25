# Architecture Decision Records - tokenledger

## ADR-001: Rust for Performance-Critical Analytics
**Status**: Accepted
**Context**: Token analytics must process large event volumes at sub-second latency.
**Decision**: Rust for core analytics engine with CLI interface.
**Consequences**: Fast execution; memory safety; steep learning curve offset by Phenotype Rust ecosystem.

## ADR-002: CSV-Based Pricing Ledger
**Status**: Accepted
**Context**: Model pricing data needs to be version-controlled and human-reviewable.
**Decision**: Normalized CSV files with SQL schema for validation and seeding.
**Consequences**: Easy to update via PRs; queryable via SQL views; no external DB dependency.

## ADR-003: Modular Crate Architecture
**Status**: Accepted
**Context**: Analytics, ingestion, pricing, and routing are distinct concerns.
**Decision**: Single binary with modular source files; future extraction to crates as needed.
**Consequences**: Simple build; clear module boundaries; ready for decomposition.
