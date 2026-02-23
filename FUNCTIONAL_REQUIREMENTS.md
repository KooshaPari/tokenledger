# Functional Requirements - tokenledger

- FR-COST-001: System SHALL compute monthly variable token cost from per-model rate cards.
- FR-COST-002: System SHALL include provider subscription allocation in monthly blended totals.
- FR-COST-003: System SHALL compute blended `$ / MTok` globally, per provider, and per model.
- FR-TOK-001: System SHALL report token type breakdown: input, output, cache write/read, tool input/output.
- FR-SES-001: System SHALL report unique monthly session counts globally and by dimension.
- FR-ING-001: System SHALL ingest normalized events from JSONL files and directories recursively.
- FR-OUT-001: System SHALL support both human-readable table output and JSON output.
- FR-TIP-001: System SHALL generate optimization tips based on measured token/cost signals.
