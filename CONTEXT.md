# CONTEXT.md — Domain Model

The shared glossary for the **Campaign for North Africa AI Benchmark** (`cna-ai-benchmark`).
Terms here are canonical; if you see a word used differently in code or docs, the code is wrong and
must be brought in line with this file.

## Core entities

- **Game**: one playthrough of a **Scenario**, from its start state until a **Victory** condition or the
  **Turn Horizon** is reached. A Game is fully determined by its seed and the actions submitted.
- **Scenario**: a bounded configuration — map layout, unit roster, supply layout, reinforcement schedule,
  victory objectives, and turn horizon. This project ships scenarios that are faithful-in-mechanics but
  reduced in scale; see `RULES-COVERAGE.md`.
- **Turn** (a.k.a. **Game Turn**, **Week**): one full cycle in which *each side* acts once. Represents one
  week of game time (faithful to CNA, where 111 turns = 111 weeks).
- **Phase**: a subdivision of a side's turn. Order: `Supply` → `Attrition` → `Movement` → `Combat` →
  `Air` → `RearArea`. The engine runs all phases deterministically; the model does not choose phase order.
- **Side** (a.k.a. **Faction**): `Allied` (British & Commonwealth) or `Axis` (German & Italian).
- **Role** (a.k.a. **Command**): CNA's five staff roles — `CinC`, `Logistics`, `RearArea`, `Air`,
  `FrontLine`. A Role is an **observation partition**: it determines which subset of the state a
  **Player** may see, not a separate competing agent.
- **Player**: one model controlling one entire Side (all five Roles). Elo is per-Player (1v1).
- **Agent**: the LLM-backed thing that receives **Observations** and emits **Actions**. A Player is
  implemented by one Agent. The term "Agent" and "Model" are interchangeable in this repo.
- **Model Adapter**: provider-agnostic interface (`ModelAdapter`) that turns an Observation + action
  schema into a requested Action. Implementations: OpenAI, Anthropic, any OpenAI-compatible endpoint,
  and a deterministic `Mock` agent used for offline smoke tests.

## World / units

- **Map**: a hex grid over a coastal-desert slice of the North African theatre. Hexes carry **Terrain**
  (`Road`, `Coast`, `Desert`, `Escarpment`, `Oasis`, `Port`, `Depot`, `Village`).
- **Depot** (a.k.a. **Port**): a supply source and objective. Supplies flow outward from Depots along
  `Road`/`Coast` hexes.
- **Unit** (a.k.a. **Counter**): a formation on the map. Fields: Side, Nationality (`British`,
  `Commonwealth`, `German`, `Italian`), Type (`Armor`, `Motorized`, `Infantry`, `Artillery`, `Air`),
  Strength (steps), movement allowance, fuel capacity (mobile units), and water demand.
- **Strength** / **Step**: a unit's combat power / remaining step losses.
- **Supply Status**: `InSupply` (traces a valid line to a friendly Depot) vs `OutOfSupply`. Supply is
  recomputed each Supply phase; it gates movement rate and attrition.
- **Fuel**: carried by mobile units and depots. Consumed by movement; **Evaporation** reduces it each turn.
- **Water**: supply abstracted as water; **Italian** infantry carry a higher water demand to reflect the
  *pasta rule*.

## Rules concepts

- **Evaporation**: per-turn percentage loss of fuel — **3%** baseline, **7% for British** units/depots
  (faithful to CNA's jerry-can-vs-drum detail).
- **Pasta Rule**: Italian units require extra water (an increased supply demand) before they may move or
  fight at full strength.
- **Supply Line**: a traced path from a unit to a friendly Depot over `Road`/`Coast` hexes; determines
  Supply Status.
- **CRT** (Combat Results Table): odds-ratio combat resolution. Deterministic given the game seed.
- **Fog-of-War** (a.k.a. **Intel**): what a Side may see. A Side always sees its own units fully; it sees
  enemy units only when within detection range of a friendly unit or revealed by **Air** reconnaissance.

## Actions & correctness

- **Action**: a structured, JSON-serializable instruction submitted by an Agent. Variants: `Move`,
  `Attack`, `AirSupport`, `AirRecon`, `Pass`, `EndTurn`.
- **Legality**: whether an Action passes engine validation. Illegal actions are rejected with an error
  that is fed back to the Agent (bounded retries). Legality rate is benchmark telemetry.
- **Validation**: the engine is the *sole authority* on game truth. Agents cannot mutate state directly;
  every Action is validated against ground-truth state (including fog-of-war: an Agent may not act on
  units it cannot see).

## Benchmark

- **Elo**: the headline rating. Updated after every completed Game via standard Elo (round-robin).
- **Event Log**: the append-only, ordered record of every Action, validation result, and resolution.
  It is the replay source and the substrate for the live spectator report.
- **Turn Horizon**: the scenario-defined maximum number of turns. Guarantees Games terminate so Elo is
  always computable (resolves the "full CNA never finishes" problem).
