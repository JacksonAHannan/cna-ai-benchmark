# ADR 0005 — Fog-of-war as observation partition; one agent per side

## Status
Accepted (2026-10-12)

## Context
CNA is nominally two-player but recommends 10 players in two teams of five roles (C-in-C, Logistics,
Rear Area, Air, Front-line), with hidden information *between roles on the same team*. For a cross-model
benchmark we need a clean, fair unit of comparison.

## Decision
- The five roles are an **observation partition**: each role sees a scoped subset of state, but a single
  **agent controls one entire side** and receives role-scoped observations + issues role-scoped actions.
- Elo is therefore **1 agent vs 1 agent** per game.

## Consequences
- Fog-of-war is preserved (role-scoped visibility, detection range, air recon) without the ambiguity of
  scoring 5-vs-5 games.
- Role-level 5v5 Elo is a natural v2 extension; the observation layer already encodes role scoping.
