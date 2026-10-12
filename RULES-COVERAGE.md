# RULES-COVERAGE.md

Honest accounting of what the engine implements, relative to SPI's *The Campaign for North Africa*
(1978). The reference game is under copyright (Decision Games); this project implements **mechanics**
only, in original expression, with original map/counter art and no copied rulebook text.

**Coverage status:** `faithful` = mechanic structurally matches CNA; `reconstructed` = implemented from
public descriptions where exact chart values were not available, marked and judgment-filled;
`approximated` = deliberately simplified; `pending` = not yet implemented.

## Implemented (Phase 1)

| Subsystem | Status | Notes |
|---|---|---|
| Turn = 1 week; both sides act per turn | faithful | I-go-you-go |
| Phase order: Supply → Attrition → Movement → Combat → Air → RearArea | faithful | Order fixed by engine |
| Fuel evaporation per turn | faithful | 3% baseline; **7% British** (drum storage) |
| Italian extra water demand ("pasta rule") | faithful | Higher supply demand for Italian infantry; gates movement/full-strength combat |
| Supply lines traced to Depots over Road/Coast | faithful | Out-of-supply penalties |
| Hex movement with terrain cost + fuel burn | reconstructed | Terrain costs are plausible values, not CNA's exact MP chart |
| Odds-based CRT combat with step losses/retreats | reconstructed | CRT table is our own odds ladder, structurally CRT-like |
| Air support (column shift) + recon (fog reveal) | approximated | CNA's air game is far more granular; v1 is a single abstracted mission |
| Rear-area reinforcements on schedule | faithful | Reinforcement schedule is scenario data |
| Fog-of-war: side intel + detection range + air recon | faithful | Per-role *observation partition*; one Agent per side acts for all roles |
| Victory: objective control at turn horizon / elimination | reconstructed | Scenario-defined; CNA's own scenarios vary |

## Not yet implemented (later phases)

| Subsystem | Status |
|---|---|
| Full ~1,800-counter roster and 9.5-ft map | pending (scenarios use reduced but real formations) |
| Full ~70 charts (all CRTs, MP, air, naval) | pending |
| Naval / convoy game | pending |
| Granular air game (sorties, air superiority, interception) | pending |
| 10-player team structure with per-role *agents* (5v5 Elo) | pending (v2; Phase 1 = one agent per side) |
| Client-side WASM engine for interactive replay | pending (Phase 1 = server-authoritative spectator) |

## Reconstruction log

- **Movement/terrain MP costs**: chosen as internally consistent values; not taken from CNA's movement
  point charts (exact values unpublished to us).
- **CRT odds ladder**: our own arithmetic ladder (attacker strength ÷ defender strength → result class);
  structurally faithful (odds-based), numerically original.
- **Scenario rosters**: real formation names (e.g. 7th Armoured, Afrika Korps) at reduced counter counts;
  strengths are illustrative, not sourced from CNA's OB.
