# Code Entity Map - tokenledger

## Forward Map (Code -> Requirements)

| Entity | File | FR |
|--------|------|----|
| Ingestion module | `src/ingest/` | FR-ING-001, FR-ING-002, FR-ING-003 |
| Analytics engine | `src/analytics.rs` | FR-ANA-001, FR-ANA-002, FR-TIP-001 |
| Benchmark suite | `src/bench.rs` | FR-ANA-003 |
| Pricing module | `src/pricing.rs` | FR-PRC-001 |
| Pricing ledger data | `ledger/` | FR-PRC-001, FR-PRC-002 |
| Routing module | `src/routing/` | FR-RTE-001 |
| CLI handler | `src/cli.rs` | FR-CLI-001 |
| Cache module | `src/cache.rs` | FR-CACHE-001 |
| Cost formatter | `src/cost.rs` | FR-ANA-001 |
| Models | `src/models.rs` | FR-ING-001 |

## Reverse Map (Requirements -> Code)

| FR | Primary Entities |
|----|-----------------|
| FR-ING-001 | `src/ingest/`, `src/models.rs` |
| FR-ING-002 | `src/ingest/` (CSV parser) |
| FR-ING-003 | `src/ingest/` (streaming) |
| FR-ANA-001 | `src/analytics.rs`, `src/cost.rs` |
| FR-ANA-002 | `src/analytics.rs` |
| FR-ANA-003 | `src/bench.rs` |
| FR-TIP-001 | `src/analytics.rs` (tip engine) |
| FR-PRC-001 | `src/pricing.rs`, `ledger/` |
| FR-PRC-002 | `ledger/unified_model_provider_pareto.csv` |
| FR-RTE-001 | `src/routing/` |
| FR-CLI-001 | `src/cli.rs` |
| FR-CACHE-001 | `src/cache.rs` |
