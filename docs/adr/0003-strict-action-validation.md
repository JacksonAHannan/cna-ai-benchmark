# ADR 0003 — Strict action validation; engine as sole authority

## Status
Accepted (2026-10-12)

## Context
LLM agents, given a rules-heavy game, will emit illegal actions. For the benchmark to be trustworthy,
agents must be unable to mutate state except through validated actions.

## Decision
- Actions are structured (JSON-serializable). The engine validates every action against ground-truth
  state *and* the submitting side's fog-of-war view.
- Invalid actions are rejected with an error fed back to the agent; bounded retries, then the action
  is forfeited and logged as `Illegal` (feeds legality telemetry).

## Consequences
- Single source of truth in the engine; no hallucinated state mutations.
- Legality becomes a measurable gradient (who recovers vs. who spirals).
- The action schema and validation must be exhaustively covered by tests.
