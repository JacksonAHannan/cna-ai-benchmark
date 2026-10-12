# ADR 0004 — Scenario horizon so games terminate for Elo

## Status
Accepted (2026-10-12)

## Context
The full CNA campaign is ~1,500 hours / 111 turns and has never been completed. Elo requires a winner,
so a full-fidelity campaign would never produce a rating.

## Decision
Keep full mechanics, but the benchmark plays **scenarios** with an explicit **turn horizon** and
victory conditions (objective control / elimination / final score). The full campaign is available as
a sandbox mode, not the benchmark default.

## Consequences
- Every benchmark game terminates → Elo is always computable.
- Fidelity to mechanics is preserved; only *scope* (map/roster/horizon) is bounded per scenario.
- Scenario authoring becomes a first-class artifact.
