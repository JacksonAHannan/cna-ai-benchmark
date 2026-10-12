# ADR 0001 — Faithful mechanics, original expression

## Status
Accepted (2026-10-12)

## Context
The reference game, SPI's *The Campaign for North Africa* (1978), is under copyright (Decision Games).
We want a "model-playable" version with high fidelity, but we must not reproduce the rulebook text,
counter art, or map artwork.

## Decision
Implement the game's **mechanics** in original code, text, and art. Game mechanics are not
copyrightable; the specific expression (rulebook prose, artwork) is. Where exact chart values are not
publicly documented, reconstruct them and log the reconstruction in `RULES-COVERAGE.md`.

## Consequences
- Legally clean: no copied text or art in the repository.
- Faithful *structure* (turn/phase/supply/evaporation/pasta/CRT), numerically original where required.
- Ongoing obligation: every reconstruction or approximation must be tracked in `RULES-COVERAGE.md`.
