# Functional Requirements - tokenledger

## FR-ING-001: Multi-Provider Ingestion
The system SHALL ingest token usage events from multiple LLM providers.

## FR-ING-002: CSV Source Support
The system SHALL support CSV files as ingestion sources.

## FR-ING-003: Streaming Ingestion
The system SHALL support streaming ingestion for real-time usage audits.

## FR-ANA-001: Monthly Cost Report
The system SHALL generate blended monthly cost reports across providers and models.

## FR-ANA-002: Per-Model Metrics
Reports SHALL include per-model and per-provider $/MTok and session counts.

## FR-ANA-003: Sub-Second Performance
Monthly reports SHALL execute sub-second for <=100k events.

## FR-TIP-001: Optimization Tips
The tip engine SHALL emit at least one prioritized optimization suggestion from telemetry.

## FR-PRC-001: Pricing Database
The system SHALL maintain a normalized model-provider pricing ledger.

## FR-PRC-002: Pareto Views
The system SHALL provide Pareto-optimal model views based on cost/quality tradeoffs.

## FR-RTE-001: Model Routing
The system SHALL support routing decisions based on pricing and performance data.

## FR-CLI-001: CLI Interface
The system SHALL provide a CLI for ingestion, analytics, and report generation.

## FR-CACHE-001: Analytics Caching
The system SHALL cache computed analytics for repeated queries.
