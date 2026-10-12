# ADR 0002 — Rust core, server-authoritative, React presentation

## Status
Accepted (2026-10-12)

## Context
The benchmark needs (a) a correctness-critical deterministic engine, (b) a headless benchmark runner,
(c) a live spectator UI. User chose Rust for the core.

## Decision
- **Rust** owns correctness: pure engine crate (no I/O), benchmark runner, and an Axum server.
- The server is the **authoritative** game runner; the browser is a pure spectator that renders
  JSON state and an event stream over WebSocket.
- Presentation is **React/TypeScript** for the richest thematic UI with least friction.

## Consequences
- One language (Rust) for engine + harness; deterministic engine reusable headless and in-server.
- Client-side WASM engine is deferred (not needed for a spectator); documented as a later option.
- A TS toolchain is required alongside Rust.
