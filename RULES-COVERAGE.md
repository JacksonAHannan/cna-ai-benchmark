# RULES-COVERAGE.md

Honest accounting of what the engine implements, relative to SPI's *The Campaign for North Africa*
(1978). The reference game is under copyright (Decision Games); this project implements **mechanics**
only, in original expression, with original map/counter art and no copied rulebook text.

The published-rules reference (`docs/cna-rules.md`) was compiled from the archived rulebook; it is the
authority for the numeric values below.

**Coverage status:** `faithful` = mechanic structurally matches CNA; `reconstructed` = implemented from
public descriptions where exact chart values were not available, marked and judgment-filled;
`approximated` = deliberately simplified; `pending` = not yet implemented.

## Implemented (Phase 1)

| Subsystem | Status | Notes |
|---|---|---|
| Turn = 1 week = 3 Operations Stages | faithful | Per-stage initiative roll; weekly supply/attrition, per-stage CP reset |
| Phase order: Supply → Attrition → Movement → Combat → Air → RearArea | faithful | Order fixed by engine |
| Fuel evaporation per turn | faithful | **6% baseline / 9% Commonwealth** through Aug 1941 (research-corrected §49.3); hot-weather +5% not yet modeled |
| Italian extra water demand ("pasta rule") | faithful | Higher supply demand for Italian infantry; gates movement/full-strength combat |
| Supply lines traced to Depots over Road/Coast | faithful | Out-of-supply penalties; depot capacities reflect published port tonnage (Tripoli/Alexandria major, Benghazi ~1/6) |
| Ammunition resource | faithful | Ammo points refilled in supply, consumed per attack; no ammo → cannot attack |
| Water resource | faithful | Water points refilled in supply, consumed per turn (Italian +1 pasta); waterless units can't move/attack and defend at half strength |
| Hex movement with terrain cost + fuel burn | reconstructed | Terrain costs are plausible values, not CNA's exact MP chart |
| Differential CRT combat (percentage losses, retreat) | faithful | Exact 18-column close-assault CRT transcribed from the archived chart (`docs/data/crt.md`) |
| Air game: recon, ground support, strategic bombing, interception | reconstructed | Bombing disrupts depot supply; enemy air within 4 hexes intercepts (air-to-air losses); refit/sortie detail not yet modeled |
| Rear-area reinforcements on schedule | faithful | Reinforcement schedule is scenario data |
| Fog-of-war: side intel + detection range + air recon | faithful | Per-role *observation partition*; one Agent per side acts for all roles |
| Victory: objective control at turn horizon / elimination | reconstructed | Scenario-defined; CNA's own scenarios vary |

## Not yet implemented (later phases)

| Subsystem | Status |
|---|---|
| Full ~1,800-counter roster and 9.5-ft map | pending (scenarios use reduced but real formations) |
| Full ~70 charts (all CRTs, MP, air, naval) | pending |
| Air sorties/refit/air-superiority/AA detail | pending (recon/bombing/interception done) |
| Per-role *agents* (5v5 Elo) | pending (role partition in observations done; one agent per side) |
| Client-side WASM engine for interactive replay | pending (Phase 1 = server-authoritative spectator) |

## Reconstruction log

- **Movement/terrain MP costs**: chosen as internally consistent values; not taken from CNA's movement
  point charts (exact values unpublished to us).
- **Differential CRT columns**: differential (attacker − defender + modifiers) → loss percentages.
  Structure follows CNA's 18-column differential CRT; the per-column values are reconstructed
  (only the +3 column was captured in research).
- **Scenario rosters**: real formation names (e.g. 7th Armoured, Afrika Korps) at reduced counter counts;
  strengths are illustrative, not sourced from CNA's OB.
