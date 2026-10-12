# Campaign for North Africa — AI Benchmark

A benchmark for pitting different AI models against each other in a **model-playable,
faithful-in-spirit digital adaptation of SPI's *The Campaign for North Africa*** (1978) —
the famously longest, most logistics-obsessed wargame ever published. It ships a headless
benchmark runner (round-robin + Elo) and a live **spectator UI** with a running report of
everything that happens.

> **Legal note:** the reference game is © Decision Games. This project implements the game's
> *mechanics* only, in original code/text/art — no copied rulebook text, counters, or map art.
> See [`RULES-COVERAGE.md`](RULES-COVERAGE.md) for exactly what is faithful, reconstructed, and pending.

## What's inside

- **A pure, deterministic rules engine** (`crates/engine`) — supply lines, weekly turns, fuel
  **evaporation** (3%, 7% for the British), the Italian **pasta-water rule**, odds-based combat,
  and **fog of war**.
- **Provider-agnostic model adapter** (`crates/models`) — OpenAI, Anthropic, any OpenAI-compatible
  endpoint (open weights via Ollama/vLLM), plus a deterministic offline `Mock` baseline.
- **Headless benchmark** (`crates/bench`) — concurrent round-robin games, Elo ratings, and
  per-game event logs persisted to `games/`.
- **Live-spectate server + UI** (`crates/server`, `web/`) — a war-room-themed React UI streaming
  the game state and an event ticker over WebSocket; replays completed games.

The shared domain glossary lives in [`CONTEXT.md`](CONTEXT.md); architecture decisions are in
[`docs/adr/`](docs/adr/).

## Quick start

```bash
# 1) Headless benchmark (two mock agents, side-swapped round-robin)
cargo run -p cna-bench -- --games 2 --seed 42

# 2) Spectator: build the UI once, then run the server
cd web && npm install && npm run build && cd ..
cargo run -p cna-server
# open http://127.0.0.1:8080  →  "▶ Start new game" and watch it unfold
```

The server serves `web/dist` and streams games over `ws://…/ws/spectate/{id}`. For UI hot-reload
during development, run `cd web && npm run dev` instead (Vite proxies `/api` and `/ws` to `:8080`).

## Running real models against each other

Write an agent config (a JSON array of [`AgentConfig`] objects):

```json
[
  { "provider": "OpenAi",   "name": "gpt-4o",   "model": "gpt-4o", "temperature": 0.0 },
  { "provider": "Anthropic","name": "claude",    "model": "claude-3-5-sonnet-latest", "temperature": 0.0 }
]
```

```bash
export OPENAI_API_KEY=...      # used by OpenAI agents
export ANTHROPIC_API_KEY=...   # used by Anthropic agents
cargo run -p cna-bench -- --config agents.json --games 4 --seed 7
```

`OpenAiCompatible` agents read `base_url` from config (default `https://api.openai.com/v1`) and
`api_key` from config or `OPENAI_API_KEY` — point it at an Ollama/vLLM endpoint to benchmark
local models. See [`agents.example.json`](agents.example.json).

The spectator server reads the **same** `agents.json` (or `$AGENTS_CONFIG`) from the repo root to
pick the two live-game agents — place one there before `cargo run -p cna-server` to watch real
models play live in the browser, or omit it to use the offline `mock-a`/`mock-b` baseline.

## How a game plays

Each side is controlled by one model (all five of CNA's staff roles are its observation
partitions). A **turn = one week**; phases run `Supply → Attrition → Movement → Combat → Air →
Rear Area`. The model sees only what its side can observe (fog of war) and submits **strictly
validated actions** — `move`, `attack`, `airRecon`, `pass`, `endTurn`. Illegal actions are rejected
with feedback and logged (legality is a tracked metric). Games terminate at a scenario horizon or
when a side is eliminated, so Elo is always computable.

The shipped scenario, **"Operation Compass (reduced)"**, is a coastal-desert slice of the Via
Balbia with a reduced but real order of battle — faithful in *mechanics*, reduced in *scale*.

## What the benchmark measures

- **Elo** (primary) — round-robin, sides swapped, K=32.
- **Legality rate** (secondary) — how often a model emits an action the rules reject.
- Per-game telemetry — turns, winner, illegal-action counts, and the full event log.

## Repository layout

```
crates/
  engine/    pure deterministic rules (no I/O)
  models/    Agent trait + providers + the shared game-driving loop
  bench/     headless round-robin + Elo + event-log persistence
  server/    Axum + WebSocket live stream, replay, static UI
web/         React + Vite spectator UI
docs/adr/    architecture decision records
```
