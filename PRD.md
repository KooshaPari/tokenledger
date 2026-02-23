# PRD - tokenledger

## Problem
Teams running multiple coding agents (`cursor`, `droid`, `codex`, `claude`) need one fast, real-time cost and token observability layer. Existing tooling is fragmented by provider and often too slow for live audits.

## Goals

1. Provide a single monthly economic view across providers and models.
2. Include both subscription and token/session costs in one blended model.
3. Surface `$ / MTok` per model and provider with actionable optimization tips.
4. Support real-time usage audits via fast streaming ingestion.

## Non-Goals (Phase 1)

1. Full GUI dashboard.
2. Full provider scraping parity with every edge-case log format.
3. Billing API write-back.

## Users

1. Operator running multiple AI coding agents.
2. Team lead controlling model spend.
3. Infra/FinOps engineer auditing usage policy.

## Success Metrics

1. Monthly report executes sub-second for <= 100k events and low-single-digit seconds for ~1M events (target).
2. Report includes blended monthly cost, per-model and per-provider `$ / MTok`, and session counts.
3. Tip engine emits at least one prioritized suggestion from measured telemetry.
