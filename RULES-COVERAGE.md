# RULES-COVERAGE.md

Honest accounting of what the engine implements, relative to SPI's *The Campaign for North Africa*
(1978). The reference game is under copyright (Decision Games); this project implements **mechanics**
only, in original expression, with original map/counter art and no copied rulebook text.

The published-rules reference (`docs/cna-rules.md`) and the extracted chart data (`docs/data/*.md`)
are the authority for the numeric values below.

**Coverage status:** `faithful` = mechanic structurally matches CNA and uses the published chart
values; `reconstructed` = implemented from public descriptions where exact chart values were not
available, marked and judgment-filled; `approximated` = deliberately simplified; `pending` = not yet
implemented.

## Implemented

| Subsystem | Status | Notes |
|---|---|---|
| Turn = 1 week = 3 Operations Stages | faithful | Per-stage initiative roll; weekly supply/attrition, per-stage CP reset |
| Phase order: Supply → Attrition → Movement → Combat → Air → RearArea | faithful | Order fixed by engine |
| Close-assault CRT | faithful | Exact 18-column table + dice-sum effects: capture, Engaged, 1/2/3-hex retreat (`charts.rs`, `docs/data/crt.md`) |
| Combat column modifiers | faithful | Terrain shifts, organization (stacking points §9.4/§15.53), morale difference (§15.61), 2:1 raw strength (§15.51), air support |
| Prisoners / capture | faithful | Shared §15.89 table (10/25/33/50/50/75%) resolves captured fraction of losses |
| Engaged contact | faithful | Attacker's Engaged result locks both sides in contact; engaged units can only attack |
| Hex movement + terrain costs | faithful | Exact §8.37 chart: full terrain set (clear/gravel, desert, rough, mountain, salt marsh, heavy vegetation, delta, swamp, escarpment) + motorized/non-motorized costs + assault-defense shifts |
| Fuel evaporation per turn | faithful | 6% baseline / 9% Commonwealth through Aug 1941 (research-corrected §49.3); hot-weather +5% not modeled |
| Italian extra water demand ("pasta rule") | faithful | +1 water for Italian battalions; waterless units can't move/attack, defend at half strength |
| Supply: fuel/ammo/stores/water | faithful | 4 resources, per-depot stock depleted by draw, resource weights (§54.5), truck/dump/rail chart data |
| Water sources | faithful | Oases and major cities provide unlimited water (§52); wells/pipelines pending |
| Truck delivery | faithful | Truck points extend supply delivery beyond trace range (§54), replenished weekly |
| Air: squadrons, refit, sorties | faithful | Squadron ready/reserve capacities (§35.23), refit table (§38.37), per-turn sorties |
| Air: air-to-air, scramble, bombing, AA | faithful | TacAir kill table (§45.5), scramble by distance (§40.4), supply-dump bombing table (§41.5), flak bands + destroyed/aborted (§46) |
| Naval: convoy tonnage + lanes | faithful | Monthly tonnage chart (§56.4–5), 6 lanes, port capacity table (§55.3) |
| Rear-area reinforcements on schedule | faithful | Reinforcement schedule is scenario data |
| Fog-of-war: side intel + detection + air recon | faithful | Per-role *observation partition*; one Agent per side acts for all roles |
| Team play (5v5) | faithful | Five roles per side (`Role::controls`), role-scoped observations, `run_streaming_team` loop |
| Victory: objective control at horizon / elimination | reconstructed | Scenario-defined; CNA's own scenarios vary |

## Not yet implemented (later phases)

| Subsystem | Status |
|---|---|
| Full ~1,800-counter roster and 9.5-ft map | partial (map 85% = 5,990 hexes from Vassal source; OB ~900 counters; scenario assembly + remaining transcription ongoing) |
| Physical truck/dump/well/pipeline delivery | partial (per-depot stock, water sources, truck points done; pipeline construction pending) |
| Dynamic morale check (§17.4 disorganization table) | pending (static per-unit morale −3..+3; shift = difference is wired) |
| Per-role *agents* (5v5 Elo) | partial (role partition + team loop + `--teams` done; per-role Elo round-robin config pending) |
| Client-side WASM engine for interactive replay | pending (Phase 1 = server-authoritative spectator) |

## Placeholders (not yet sourced from the OB)

- **Morale and AA ratings** are placeholder values (German/Commonwealth high, Italian low; artillery
  carries flak) until the full ~1,800-counter OB is transcribed.
- **Scenario port capacities** use the published *relative* tonnage scaled to the reduced scenario
  (Tripoli/Alexandria major, Benghazi/Tobruk/Mersa Matruh minor).

## Reconstruction log

- **Scenario rosters**: real formation names (e.g. 7th Armoured, Afrika Korps) at reduced counter counts;
  strengths are illustrative, not sourced from CNA's OB.
- **Reduced-scenario tonnage scale**: port/convey tonnage is scaled ÷500 (Tripoli 15,000 t ≈ 30 supply
  points) to match the reduced counter count; the full OB will use published tons directly.
