# PRD - tokenledger

## Problem
Teams running multiple coding agents (Cursor, Droid, Codex, Claude) need one fast, real-time cost and token observability layer. Existing tooling is fragmented by provider and often too slow for live audits.

## E1: Usage Ingestion

### E1.1: Multi-Provider Ingestion
As an operator, I ingest token usage data from multiple providers (OpenAI, Anthropic, Google, etc.) into a unified ledger.

**Acceptance**: CLI-driven ingestion; CSV and API sources; streaming support for real-time audits.

## E2: Cost Analytics

### E2.1: Blended Monthly Cost View
As a team lead, I view blended monthly costs across providers and models in a single report.

**Acceptance**: Sub-second for <=100k events; per-model and per-provider $/MTok; session counts.

### E2.2: Optimization Tips
As a FinOps engineer, I receive prioritized optimization suggestions based on measured telemetry.

**Acceptance**: Tip engine emits at least one actionable suggestion from usage patterns.

## E3: Model Pricing

### E3.1: Pricing Database
As a system, I maintain a normalized model-provider pricing ledger with input/output token rates.

**Acceptance**: CSV-based pricing data; schema-validated; Pareto-optimal model views.

## E4: Routing

### E4.1: Intelligent Model Routing
As an operator, I route requests to cost-optimal models based on pricing and performance data.

**Acceptance**: Routing decisions based on $/MTok and quality metrics.

## Non-Goals (Phase 1)
1. Full GUI dashboard
2. Billing API write-back
3. Full provider scraping parity

## Success Metrics
1. Monthly report sub-second for <=100k events
2. Blended monthly cost, per-model $/MTok, session counts in report
3. Tip engine emits at least one prioritized suggestion
