# The Campaign for North Africa: published-rules reference

Mechanics and numeric data, restated in original wording; not a transcription of rulebook prose. Scope: SPI's original game, conventionally dated 1978; the accessible rulebooks carry a **1979 copyright**. Use the full Land + Air + Logistics system unless a variant is identified.

## Sources and uncertainty

- **[R] Original rules and charts:** [SPI PDF](https://www.spigames.net/PDFv2/CampaignNorthAfrica.pdf), mirrored in the [Internet Archive item](https://archive.org/details/campaign-for-north-africa). The SPI PDF exceeds the reader's 50 MB limit. Archive [OCR text](https://archive.org/download/campaign-for-north-africa/The%20Campaign%20for%20North%20Africa_djvu.txt) was read locally after downloading the complete file; image links below resolve damaged OCR tables. Citations use the published rule/case numbers, which are more stable than scan page numbers.
- **[W] Secondary summary:** [Wikipedia](https://en.wikipedia.org/wiki/The_Campaign_for_North_Africa).
- **[B] Catalog:** [BoardGameGeek](https://boardgamegeek.com/boardgame/4815/the-campaign-for-north-africa-the-desert-war-1940), inaccessible to the reader (HTTP 403); no mechanics are attributed to it.
- **[NOT SPECIFIED]** means the consulted material does not establish the requested value, or a published ambiguity remains unresolved; the surrounding text distinguishes those situations. Tables expressly linked but not exhaustively reproduced remain published data, not missing rules.

**Two important corrections:** the original fuel rule is **6% / temporarily 9%**, not the popular **3% / 7%** account; close assault uses a **strength-differential CRT**, not a conventional odds CRT with AE/AR/EX/DR/DE. Do not build the latter assumptions into a purportedly faithful engine.

## 1. Turn sequence

**Time:** approximately **one week per Game-Turn**, split into **three Operations Stages**, each representing **2–3 days**. Initiative is determined once per Game-Turn; its holder chooses whether to act first or second separately in every Operations Stage. Thus neither Axis-first nor Allied-first is permanently fixed. [R §§5.1, 7.1, 48]

### Full weekly order

1. **Initiative determination.**
2. **Strategic air planning:** aircraft designation as strategic/land-support; Axis Malta availability; strategic mission assignment; Malta raid resolution.
3. **Naval convoy stage:** convoy scheduling; convoy resolution, in order: reconnaissance → lane/CAP/bomber assignments → air combat, flak suppression, AA and convoy bombing.
4. **Stores expenditure:** both sides distribute/consume stores; apply normal fuel/water evaporation and spillage.
5. **First Operations Stage** (below).
6. **Second Operations Stage:** repeat the complete Operations Stage.
7. **Third Operations Stage:** repeat again.
8. **Strategic air recovery:** return surviving strategic aircraft → aircraft maintenance.
9. **End of Game-Turn.** [R §48]

### Operations Stage: common phases, then alternating land activity

A. **Initiative declaration:** designate first player **A**, second player **B**.

B. **Weather determination:** determine weather and immediately apply its effects.

C. **Organization:** the following segments may be handled in **any order**, so their numbering is not a mandatory chronology:

1. Water distribution.
2. Reorganization: assignments, attachments, detachments, truck organization.
3. Attrition for deficient stores/water.
4. Construction: completion → initiation/continuation.
5. Training: completion → initiation/continuation.
6. Supply distribution and truck loading/unloading.
7. Tactical shipping between African ports.

D. **Convoy arrivals:** scheduled reinforcements, replacements and supplies arrive. Replacement planning also occurs here; full-logistics Commonwealth production is planned **two months ahead** (§48), unlike the shorter leads printed in abstract sequences.

E. **Commonwealth fleet:** ship assignments → ship repairs.

F. **Land-support air:** mission assignment → deployment → air-to-air, including scramble/interception → flak → mission completion → return to base → tactical maintenance. This is a common air phase, **not repeated separately for B**.

G. **A designates reserves.**

H. **A movement/combat loop**, with all four segments in every repetition:

1. Movement; B may react.
2. Vehicle breakdown determination for both sides.
3. Combat: position selection (Forward/Back) → simultaneous plotted barrages → B retreat before assault → secret force allocation to anti-armor/assault/withheld/probe → simultaneous anti-armor with immediate losses → close assaults in A's chosen order.
4. Reserve release.

J. **A truck-convoy movement:** second/third-line trucks, prisoners and guards.

K. **Commonwealth rail movement**, only if A is Commonwealth.

L. **A repair:** towing → maintenance of vehicles not towed in this phase.

M. **A patrol/reconnaissance**, subject to the no-combat condition in §48.

Then **B repeats G, H, J, K, L, M**, switching phasing/non-phasing roles. Both sides' expenditures and reactions draw on their **same Operations Stage CPA**. Repeat A–M for the next Operations Stage. [R §§33, 48; §33 explicitly supplies the B-repeat instruction omitted from the printed §48 outline. The outline intentionally skips letter I.]

**Variant distinction:** land-only §5.2 omits detailed air/logistics and uses simplified supply; §33 adds air with abstract logistics. These are not competing full-game turn sequences. **[NOT SPECIFIED]** an erratum resolving §48's stricter patrol wording versus §§5.2/33's allowance when the phasing player has not assaulted was not located.

## 2. Supply and logistics

### Resources, weights and access

Track **fuel, ammunition, stores and water** individually at units, dumps, trucks and air facilities. Stores include food and miscellaneous necessities; there is **no fifth, separate food stock** in the full system. [R §§49–54]

| Resource | Published weight per supply point |
|---|---:|
| Fuel | **1/8 ton**; approximately **250 lb / 35 gallons** |
| Ammunition | **4 tons** |
| Stores | **1 ton** |
| Water | **1/6 ton** |

[R §§49.11, 50.11, 51.15; [weight chart §54.5](https://archive.org/download/campaign-for-north-africa/page/n109.jpg)]

This is primarily **physical delivery**, not a generic radius-based supply trace: fuel, ammo and stores must be in the consumer's hex. Second/third-line convoy cargo must be **off-loaded before combat units consume it**; attached first-line trucks and dumps can supply the hex. Trucks, rail and appropriate aircraft move the resources. Victory clauses separately impose specified supply-route lengths. Prisoners/guards have a special nearest-dump exception for stores. Cairo provides Commonwealth **unlimited supplies of all four types**; Axis European supplies are unlimited but shipping/ports constrain delivery. [R §§49.15–18, 50.15–17, 51.12–17, 57]

### Consumption and deficiency

- **Fuel:** per vehicle TOE point, §49.13 charges its consumption factor for every **five movement CP or fraction thereof**; its example is factor **4**, movement **12 CP**, consuming **12 Fuel Points**. Combat CP without hex movement consume no fuel. Tank capacity is **CPA × consumption factor / 5**. Refueling is free in CP; emergency siphoning costs **3 CP to each participating unit**. Motorcycles and towing tanks/trucks back to repair facilities do not consume fuel. [R §§49.1]
- **Published fuel-table discrepancy:** [§49.19 chart](https://archive.org/download/campaign-for-north-africa/page/n105.jpg) instead gives fractional consumption for **1–4 CP** (factor 1: **0.2, 0.4, 0.6, 0.8**) and linear totals thereafter. **[NOT SPECIFIED]** which fractional calculation overrides §49.13's five-CP-block wording/example; preserve this as an implementation decision, not an invented exact rule.
- **Ammo per firing TOE point, full logistics:** barrage **4**; anti-armor **3**; close assault by armor/gun classes **2**; infantry/MG/heavy weapons **2**; AA firing at one target group **1**. A combat unit intrinsically carries enough ammo to fire **once**; additional ammunition needs trucks/dumps. No ammo means no attack, defense or ZOC, and surrender when enemy ZOC/fire triggers §50.12. Only **1/3 of captured ammo, rounded up**, is usable. [R §50; [§50.2 chart](https://archive.org/download/campaign-for-north-africa/page/n107.jpg)]
- **Stores:** ordinary TOE points require **4 per Game-Turn**; HQ/engineer units require **1 per Game-Turn**; guards **2 per guard point per Game-Turn**; prisoners **1 per five Prisoner Points per Operations Stage**, supplied first. Captured stores retain **50%**. Half rations use **2 instead of 4**, forbid voluntary CPA overrun and voluntary entry into enemy ZOC. Missing stores gives **1 disorganization point per week**; every second consecutive unsupplied week causes infantry losses escalating **2%, 4%, 6%, …**, rounded to nearest whole TOE point. [R §51]
- **Water:** each infantry battalion/company requires **1 Water Point per Operations Stage regardless of strength**; every active vehicle TOE point/truck point requires **1 per Operations Stage** if any CPA is used. Hot weather **doubles all unit water requirements**. Waterless units cannot offensively assault; vehicles cannot move; defense is **halved**. Infantry cannot voluntarily overrun CPA; after the first consecutive waterless Operations Stage, lose **1 TOE point per infantry unit per further stage**. [R §§29.35, 52.4–5]

### Evaporation: original rule versus popular account

**Original §49.3, visually checked:** each Stores Expenditure Stage, reduce **all on-map fuel stocks by 6%, rounded down**; fuel in sea convoys is excluded. Commonwealth uses **9% from September 1940 through the final August 1941 Game-Turn inclusive**, then **6%**. Each Hot Weather Operations Stage imposes an **additional 5% immediately**. Stored water follows these spillage/evaporation rules; wells/pipelines are exempt. §29.34 additionally excludes the water/fuel currently being consumed by units when applying heat losses. [R §§29.34, 49.3, 52.44; [original page image](https://archive.org/download/campaign-for-north-africa/page/n66.jpg)]

**Secondary-only:** Wikipedia reports **3% baseline / 7% British** and attributes the difference to British drums versus jerry cans. Those figures **are not the printed §49.3 values in the consulted original**. **[NOT SPECIFIED]** any original edition/erratum authorizing 3%/7% was not found. [W, Reception]

### Italian pasta water

Each **Italian battalion** requires **1 additional Water Point when weekly stores are distributed**—not per TOE point and not in each Operations Stage. Without it, the battalion cannot voluntarily exceed CPA that Game-Turn. If its cohesion is already **−10 or worse**, it immediately behaves as disorganized at **−26** until the extra water arrives; then restore the prior cohesion. [R §52.6]

### Water sources and transport

Drawing water costs **1 CP**. Major cities, oases, Tripoli/Tunisia and working pipelines are unlimited. Town die results 1–6 give **100/150/200/300/350/500 Water Points**; bir results give **50/100/150/200/300/400**. Town results 5–6 and bir results 4–6 require a second secret die: **1 depletes the source**. Rain replenishes depleted wells. Poisoning costs **1 CP**, succeeds on **1**; sweetening costs **5 CP**, succeeds on **1–3**. A pipeline hex costs **one construction phase and 10 stores**; at most **one hex per Operations Stage**. Commonwealth working rail acts as pipeline; oases also provide unlimited non-construction stores. [R §52; [water chart](https://archive.org/download/campaign-for-north-africa/page/n108.jpg)]

Per Truck Point, capacities are alternatives for the cargo carried, not simultaneously additive:

| Truck | Infantry TOE | Gun TOE | AA TOE | Ammo | Fuel | Stores | Water | Own fuel capacity |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Light | 1/2 | prohibited | 1 | 2 | 50 | 6 | 40 | 8 |
| Medium | 1 | 1 | 2 | 4 | 120 | 15 | 100 | 6 |
| Heavy | 2 | 1 | 4 | 8 | 250 | 30 | 200 | 6 |

All three have fuel factor **1**. Commonwealth rail carries **1,500 tons per Operations Stage**, one resource type at a time. Dump maxima, in Ammo/Fuel/Stores/Water points: village **2,500/8,000/3,000/1,000**; other terrain **1,500/5,000/1,000/1,000**; major city/Tripoli/Tunis **unlimited**. [R §§54.2–5; [truck/dump chart](https://archive.org/download/campaign-for-north-africa/page/n108.jpg)]

## 3. Movement

### Allowance is capability, not a fixed movement-only budget

**CPA** pays for movement, combat, retreats, loading and other actions. It renews each Operations Stage, is shared across both players' activity within that stage, and does not carry forward. Each CP above CPA gives **1 disorganization point**. Parent formations use the **lowest subordinate CPA**. Foot units with CPA ≤10 may voluntarily spend at most **150% of base CPA** in their own portion of the stage. Cohesion **−26 or worse** prevents movement/attack/defense. Resting for a whole stage earns **5 reorganization points**, capped at cohesion 0 by this method. [R §§6, 8.17]

Exact examples and chart allowances (not nationality-wide universal values):

| Unit / activity | CPA per Operations Stage |
|---|---:|
| Commonwealth infantry battalions, characteristic codes k–t | **10** |
| Commonwealth infantry codes h/j | **15** |
| Commonwealth MG battalion code u, e.g. 1st RNF | **8** |
| Commonwealth tank battalion-equivalent code g | **25** |
| Recce example in §8.53 | **45** |
| Commonwealth artillery battalion-equivalent x/y | **20** |
| Commonwealth anti-tank battalion-equivalent z/aa/bb | **15** |
| Light truck: carrying infantry / convoy supplies | **25 / 40** |
| Medium or heavy truck: infantry / guns / convoy supplies | **20 / 15 / 30** |
| Emplaced gun with printed 0 | **0 movement**, **10 nonmovement** |

[R §§6.11–17, 8.53; [Commonwealth characteristics](https://archive.org/download/campaign-for-north-africa/page/n166.jpg), [truck chart](https://archive.org/download/campaign-for-north-africa/page/n108.jpg)]

### Terrain CP costs

The first two numeric columns are non-motorized/motorized costs. **+** adds to the entered hex's cost; P means prohibited. These are CP, not a separate MP pool. [R [§8.37 terrain chart](https://archive.org/download/campaign-for-north-africa/page/n69.jpg)]

| Hex / hexside | Nonmotorized | Motorized |
|---|---:|---:|
| Clear; gravel | **2** | **2** |
| Salt marsh | **3** | **2**, restrictions below |
| Heavy vegetation | **3** | **3** |
| Rough | **3** | **4** |
| Mountain | **4** | **6** |
| Delta | **2** | **4** |
| Desert | **3** | **4**, light-truck/motorcycle restrictions |
| Major city | **1** | **1/2** |
| Road | **1** | **1/2** |
| Track | **1** | **1** |
| Ridge / uphill slope | **+2** | **+4** |
| Downhill slope | **+1** | **+2** |
| Uphill escarpment | **+6** | **P**, track exception |
| Downhill escarpment | **+4** | **+8** |
| Wadi | **+1** | **+4** |
| Major river | **+8** | **P**, road/rail exception |
| Minor river | **+3** | **+6** |
| Friendly minefield | **+1** | **+4** |
| Enemy minefield | **+4** | **+entire unit CPA** |

Village/bir/oasis/railroad/fortification otherwise use underlying terrain. Swamp entry requires road/rail; light trucks and motorcycles are prohibited even then. Salt-marsh vehicles other than light trucks/recce/motorcycles need a track; motorized combat into/out of salt marsh is prohibited. Desert excludes light trucks and motorcycles even on tracks. Roads negate hexside movement/breakdown penalties; tracks halve these penalties, **except downhill-escarpment penalties**. Track is necessary for motorized uphill-escarpment crossing; it does not permit motorized assault uphill. Major-river road/rail crossing allows motorized passage without additional CP/breakdown cost. Sandstorms **double movement costs**; rain converts road movement/breakdown to track rates and restricts wadi/river/delta crossing. [R §§8.3–4, 29.4–5, chart notes]

Enter enemy ZOC → stop for that Movement Segment. Leaving Contact costs **+2 CP**, Engaged **+4 CP**; generally no direct enemy-ZOC-to-enemy-ZOC movement. Repeated movement/combat is permitted, but a unit ending a Movement Segment **more than two hexes from enemy combat units** cannot move again in that phase unless reserve rules permit it. Reaction can be blocked when an adjacent attacker is **6 or more CPA faster** and declares assault; nonmotorized units cannot react. [R §§8.1–2, 8.5]

## 4. Combat

### Strength and CRT structure

Raw combat strength is **sum(TOE points × relevant weapon rating)**. Actual strength is **raw / 10, rounded to nearest integer**; AA does **not** divide by 10. Armor protection and gun vulnerability are loss-absorption ratings, not attack multipliers. Close-assault differential is **attacker's actual offensive strength minus defender's actual defensive strength**. [R §11.3]

The [actual §15.79 CRT](https://archive.org/download/campaign-for-north-africa/page/n98.jpg) has separate attacker/defender panels and these **18 columns**:

**≤−11; −10…−8; −7…−6; −5…−4; −3; −2; −1; 0; +1; +2; +3; +4; +5…+6; +7…+8; +9…+10; +11…+13; +14…+16; ≥+17.**

Loss rows: **0%, 5%, 10%, 15%, 20%, 25%, 30%, 40%, 50%**. Each side rolls **two distinguishable six-sided dice once**: read them as **11–66 sequentially** for its losses; use their **sum, 2–12**, for Engaged, retreat and capture checks. Capture triggers an additional single die on the prisoner table. Column modifiers affect **columns**, not necessarily one numerical differential point. **+11 and above is Overrun**, which exposes rear guns and rounds defender losses upward. [R §§15.7–8]

Concrete **+3** column:

| Loss | Attacker sequential roll | Defender sequential roll |
|---|---|---|
| 30% | — | **11** |
| 25% | — | **12–14** |
| 20% | **11–12** | **15–25** |
| 15% | **13–16** | **26–42** |
| 10% | **21–26** | **43–54** |
| 5% | **31–42** | **55–66** |
| 0% | **43–66** | — |

At +3, attacker dice sums **8, 10, 12** cause Engaged; defender sums **4–7** require one-hex retreat and **11** two-hex retreat; defender capture trigger sums **2–3**. The prose example §15.73 lists Engaged **8,9,10,12**, conflicting with the chart's **8,10,12**. **[NOT SPECIFIED]** an authoritative correction to that discrepancy was not found; the table above reports the chart itself.

**AE / AR / EX / DR / DE:** **[NOT SPECIFIED] as CNA result codes**. The consulted original CRT does not use them; assigning them conventional Attacker Eliminated/Retreat, Exchange, Defender Retreat/Eliminated meanings would import another combat system. CNA instead produces percentage casualties, **CAPT**, **ENG**, retreat distances, or **In Contact** when forces remain adjacent without Engaged. There is no universal exchange or automatic whole-counter elimination result. [R §§15.7–8]

### Terrain and other modifiers

Chart shifts **L** favor defender, **R** attacker:

| Terrain | Barrage | Anti-armor | Close assault |
|---|---|---|---|
| Heavy vegetation | — | **L1** | **L1** |
| Rough | **L1** | **L1** | **L2** |
| Mountain | **L2** | **L2** | **L3** |
| Salt marsh | — | — | **R1** |
| Ridge | — | **L2** | **L2** |
| Uphill slope | — | **L1** | **L2** |
| Downhill slope | — | **L1** | **R1** |
| Uphill escarpment | — | **P** | **L3** |
| Downhill escarpment | — | **L2** | **R1** |
| Wadi | — | — | **L1** |
| Major / minor river | — | — | **L3 / L2** |
| Fortification level 1 / 2 / 3 | **L1 / L2 / L2** | **L1 / L2 / L2** | **L2 / L3 / L4** |
| Friendly minefield | — | **L1** | **L1** |

Clear/gravel/desert/delta provide no combat shift. Alexandria/Cairo start as level **3** fortifications; other major cities level **2**. Fortification barrage protection for armor targets applies only in major cities. Anti-armor uses best in-hex terrain plus hexside effects; assault effects generally accumulate, with limits on combining separate hexside types. **Printed discrepancy:** §15.35's example gives a level-two fortification **L2**, whereas the chart gives **L3**. **[NOT SPECIFIED]** a correcting erratum was not located. [R §§12.3, 14.3, 15.3; §8.37 chart]

- At least **2:1 raw assault strength** gives **2 column shifts** to the stronger side; this is a modifier, **not an odds CRT**.
- Organizational advantage shifts: division versus 3-point brigade **1**; versus 2-point brigade **2**; versus battalion **4**; versus company **8**; brigade versus battalion/company **2/4**; battalion versus company **2**.
- Morale difference shifts that many columns toward the higher adjusted morale side.
- Combined arms requires **one infantry/MG/heavy-weapons TOE point per tank TOE point in the same hex**. Each **1–3 unsupported tank TOE points** reduces actual tank assault strength by **1**, maximum reduction **4**.
- Full assault commits at least **50% of available TOE strength**; less automatically becomes a probe. Phasing combat generally costs **5 CP**, defense **3 CP**; defense at final differential **−4 or worse costs 1 CP**. [R §§11.2, 15.4–6]

### Anti-tank, losses, retreat and advance

Anti-armor fire is simultaneous before assault and removes casualties immediately. A TOE point cannot perform both anti-armor and assault in the same segment. Fire yields **Damage Points**; remove enough armored TOE points that their armor protection absorbs those points. Example: **7 damage** removes one Italian M/13 point with armor **3** and two M/11 points with armor **2** each. SP guns absorb damage using **armor + vulnerability**; artillery firing anti-armor has vulnerability **2**. Withheld armor is normally immune to anti-armor fire. [R §14]

For assault losses, multiply the loss percentage by raw assault strength; **attacker rounds up**, **defender down**, except **Overrun rounds defender up**. Remove sufficient TOE points to absorb that raw-strength loss; divide losses proportionally among participating hexes. This is **point depletion**, not flipping every unit through a fixed two-step counter system. Losses **≥30% in one assault** give **3 disorganization points**. [R §§6.21, 15.8]

Retreats cost movement CP/fuel and must end at least the required hex distance away. Each mandated hex not retreated adds **10 percentage points of casualties**. Major-city defenders can ignore retreats. Retreat takes precedence over Engaged. Unpinned defenders may retreat before assault after barrage, subject to ZOC and movement restrictions. [R §§13, 15.81–83]

**No free advance after combat:** movement into the vacated adjacent hex occurs in the **next Movement Segment**, costs normal CP and is specifically allowed despite the usual restriction on moving between enemy ZOCs. [R §10.24]

## 5. Air

Aircraft operate individually through squadrons/SGSUs; aircraft-type counters can represent multiple actual aircraft on a mission. Fighter missions are limited to **one per Operations Stage, up to three land-support missions per Game-Turn**, or **one strategic mission per Game-Turn**. Fuel/refit/arming and losses can reduce actual sorties. A strategic aircraft is not simultaneously available as tactical support. [R §§33–40]

- **Readiness:** full type-specific fuel load is needed for a mission; non-transfer missions require another refit afterward. A refit attempt costs **1 Stores Point per squadron**. Squadron-refit die 1/2/3/4/5/6–7/8–9 yields **100/80/70/60/50/40/33%** refitted, rounding aircraft upward. Add **1 German**, **2 Italian**, and **1 for another squadron's SGSU**. Sandstorm exposure forces **20% of ready grounded aircraft, rounded up**, to refit again. [R §38; [capacity/refit chart](https://archive.org/download/campaign-for-north-africa/page/n104.jpg)]
- **Squadron ready/reserve/total capacities:** Italian **9/3/12**; German **12/4/16**; Commonwealth 1940–41 **12/4/16**, 1942–43 **18/6/24**. [R §35.23, same chart]
- **Arming:** **1 Ammo Point per squadron** supplies TacAir ammunition; **1 per bomber** supplies its entire bombload, or torpedoes/mines. Unused bombloads are nevertheless consumed on a sortie. [R §38.4]
- **CAP / air superiority:** local offensive/defensive CAP and escort resolution, not a single theater-wide numerical air-superiority bonus. **Three offensive-CAP fighters** exert air ZOC over their hex plus **six adjacent hexes**. Offensive CAP can intercept a crossing flight path, but only **one mission per Operations Stage**. Defensive CAP has no such ZOC. Africa-based planes cannot fly convoy CAP; the permitted off-map/Malta bases differ by side. [R §40.2]
- **Scramble:** one die per squadron; success is at most **6/5/4/3/2/1** for target distances **0–2/3–4/5–6/7–8/9–10/11–15 hexes**. Failed scramble still consumes a flight and needs refueling/refit. [R §§40.3–4; [scramble chart](https://archive.org/download/campaign-for-north-africa/page/n105.jpg)]
- **Air-to-air:** fighter TacAir adds pilot rating; nondive bombers gain formation bonus **+1 with 6–17 aircraft**, **+2 with ≥18**. Maneuver differences **0/1–3/4–7/8–14/15–20/≥21** yield **0/1/2/3/4/5** favorable TacAir adjustment. Individual matchups use a differential kill table and sequential 2d6. At adjusted differential **0**, kill is **≤14**; **+3 → ≤24**, **+6 → ≤34**, **≥+12 → ≤56**. The negative-differential fighter fires first; nonfighters fire before attacking fighters. [R §45; same chart]
- **Ground support / bombing:** resolve air combat → flak suppression → AA → surviving bombload against target → return. No flat assault bonus: personnel bombing **pins battalion-equivalents**; tank/gun units count as **two** for this purpose. Supply-dump bombs remove **0/10/20/30/40/50/75%** by the bombing table; each **10%** dump loss also destroys **1 unattached Truck Point** in the hex. Air-facility damage destroys **10% of grounded aircraft per capacity level lost**, rounded down. Fortifications lose at most **one level per Operations Stage**. Road/rail bombing can destroy the targeted hex's infrastructure. [R §§41.2–3; [bombing chart](https://archive.org/download/campaign-for-north-africa/page/n106.jpg)]
- **AA:** actual AA = TOE × AA rating, without /10; only attacks aircraft conducting missions in the AA hex. Tank/infantry/recce AA is limited to fighters/strafe/dive-bomber target groups. **Three suppressing fighters neutralize one light/ship AA point** for the stage; heavy AA cannot be suppressed this way. Nonfighter groups **1–23/24–47/48–71/≥72** shift AA effectiveness **0/1/2/3 columns** favorably. AA CRT separately produces aircraft destroyed/aborted; it has flak-strength bands from **1–4 to ≥37**. [R §§40.7, 46; [AA table](https://archive.org/download/campaign-for-north-africa/page/n107.jpg)]

**[NOT SPECIFIED]** a universal aircraft fuel factor/range/bombload is not defined: these are aircraft-model/loadout-specific in §4.44, not one common value. The full aircraft roster and complete air/AA lookup matrices are not reproduced here; cited original charts remain required for exhaustive implementation.

## 6. Naval

### Convoys and shipping loss

Axis convoys are planned **one Game-Turn ahead**. Monthly/yearly charts and a single die determine total tonnage. Example: **November 1941, capacity row E, die 4 → 21,000 tons**. Planned replacements consume tonnage; scheduled reinforcements do not. **Six lanes**: West Sicily–Bizerta; West Sicily–Tripoli; Italy–Benghazi; Greece–Benghazi; Greece–Tobruk; Italy–Tobruk. Greece lanes open **May II 1941**. Bizerta opens no earlier than **Game-Turn 35**, then requires a **summed 2d6 result 12**, checked weekly until successful. A destination captured before departure cancels that convoy. [R §§55.17, 56.1–2; [lane chart §56.18](https://archive.org/download/campaign-for-north-africa/page/n71.jpg)]

Each lane's convoy has **1 AA Point per 1,000 cargo tons**. Commonwealth strategic reconnaissance/bombing can attack it; bombing uses percentage losses, **0/5/10/20/25/30/40/50%**, not a universal weekly sinking rate. Cargo destroyed follows the result and convoy composition. Half or more attacking planes using torpedoes reduces effective bombing points by **25%**, but torpedoes permit **two attacks**. Unattacked convoy cargo arrives at its designated port. [R §§41.6–7, 56.16; [bombing table and key](https://archive.org/download/campaign-for-north-africa/page/n106.jpg), [continued key](https://archive.org/download/campaign-for-north-africa/page/n107.jpg)]

Axis coastal ships have CPA **50**, cost **1 per sea hex**, **5 CP loading** and **5 unloading**, carry **supplies only**, one type at a time, and consume no fuel. They cannot enter enemy/zero-capacity ports and **cannot be attacked from land/air** under §56.33. Commonwealth coastal transport is abstracted and bounded by port capacity. Commonwealth fleet combat is principally shore bombardment/AA, not ship-versus-ship naval combat. [R §§30, 56.3]

### Port limits

Maximum supply tonnage is **per Operations Stage**. Personnel capacities are Stacking Points, not TOE points; both directions share supply-tonnage handling. [R [§55.3 chart](https://archive.org/download/campaign-for-north-africa/page/n109.jpg)]

| Port | Maximum efficiency | Personnel in / out | Supply tons |
|---|---:|---:|---:|
| Tripoli | **10** | **10 / 15** | **15,000** |
| Bizerta | **10** | **10 / 15** | **3,333** |
| Alexandria | **10** | **5 / 10** | **15,000** |
| Tobruk | **5** | **1 / 3** | **1,700** |
| Benghazi | **3** | **2 / 5** | **2,500** |
| Mersa Matruh | **1** | **1 / 2** | **250** |
| Bardia | **1** | **0 / 1** | **400** |
| Sollum | **1** | **0 / 1** | **250** |
| Derna | **1** | **not allowed / 1** | **300** |
| Other ports | **1** | **not allowed / not allowed** | **100** |

Bardia/Sollum may receive **one zero-stacking unit**. Current capacity = listed capacity × **current/max efficiency**, rounding supply tons upward. Bomb damage heals **1 efficiency per undamaged Operations Stage**; scuttling/mines require engineering clearance. Tobruk's San Giorgio initially removes **3 efficiency levels**. Major ports receive personnel/supplies; minor ports receive supplies only. Scheduled reinforcements are exempt from usual capacity/efficiency limits. [R §55]

**[NOT SPECIFIED]** a universal probability of convoy loss is absent: monthly tonnage, reconnaissance, aircraft availability, convoy flak and bombing dice determine the outcome. The full month-by-month capacity table and coastal-ship counter tonnages are not reproduced here.

## 7. Victory conditions and official scenarios

There are **five scenario groups**, with separate short/full-campaign options. GT = Game-Turn; OS = Operations Stage. [R §§59–64]

| Official scenario | Start | End |
|---|---|---|
| Graziani's Offensive | **GT1 OS1** | **GT6 OS3** |
| The Italian Campaign, including Compass | **GT1 OS1** | **GT20 OS3** |
| The Race for Tobruk (Desert Fox group) | **GT26 OS3** | **GT38 OS3** |
| Operation Crusader | **GT57 OS3** | **GT65 OS1** |
| The Last Chance (El Alamein group) | **GT102 OS1** | intended **GT102 OS3**, but see ambiguity |
| The Long Retreat (El Alamein group) | **GT102 OS1** | **December IV 1942**, intended **GT111 OS3**, but see ambiguity |
| Full campaign | **GT1 OS1**, September III 1940 | **GT111 OS3** |
| Campaign beginning with Rommel | **GT26 OS3**, March IV 1941 | **GT111 OS3** |

**Published duration ambiguity:** §63.1 describes Last Chance as **one week**; §63.2 visibly combines “GT102 OS3” and “GT111 OS1” in the same endpoint sentence, with a December III/IV fragment for Long Retreat. The scan confirms this is not merely OCR corruption. **[NOT SPECIFIED]** an unambiguous corrected §63.2 endpoint was not located; the table labels the intended readings rather than pretending certainty. [R [§63.2 page](https://archive.org/download/campaign-for-north-africa/page/n83.jpg)]

### Scenario objectives

- **Graziani:** Italian strategic win: occupy any Alexandria/Cairo hex with a combat unit supplied from map D; decisive: supplied occupation of **Mersa Matruh and Siwa**; tactical: **Sidi Barrani, Sollum, Fort Maddalena, Giarabub**. Commonwealth tactical: **Sollum, Halfaya, Siwa**; decisive additionally **Bardia, Fort Maddalena, Sidi Omar**; strategic: **Tobruk**. Routes must link to the specified side's supply source. [R §60.81]
- **Italian Campaign:** most geographical points wins. Axis/Commonwealth values: Mersa Matruh **100/0**, Tobruk **50/25**, Derna **20/30**, Giarabub **10/10**, Siwa **10/10**, Benghazi **10/50**, Agedabia **10/50**, Mersa Brega **0/50**. Occupying combat units need a supply route of at most **one Game-Turn** to dump/city and onward to Cairo/Tripoli. No supplied Italian combat units on-map gives Commonwealth strategic victory. [R §60.82]
- **Race for Tobruk:** Tobruk's holder wins; Axis also holding Bardia is decisive; Axis holding Benghazi/Tobruk/Bardia plus **two Egyptian towns/cities** is strategic. Commonwealth holding all these qualifies for its highest stated victory. [R §61.8]
- **Crusader:** Axis/Commonwealth points: Tobruk **6/4**, Bardia **2/3**, Sollum **2/2**, both Halfaya hexes **1/1**, Fort Maddalena **1/2**, Sidi Barrani **3/0**, Giarabub **1/1**, Siwa **2/1**, Fort Capuzzo **1/1**. Occupiers need **one month of supplies and one week's ammo**. Point difference **10+ strategic**, **6–9 decisive**, **1–5 tactical**, **0 draw**. [R §62.8; Bardia resolved from page image n83]
- **Last Chance:** Axis strategic victory for one Alexandria hex held by at least a **brigade-equivalent**, regardless of supply. Otherwise compare Axis TOE points on map E with Commonwealth TOE points on map D (excluding Layforce/SAS); larger total wins tactically. [R §63.91]
- **Long Retreat:** Axis scores Mersa Matruh **1**, Bardia **3**, Tobruk **5**, Derna **2**, Benghazi **3**, El Agheila **1**, Marble Arch **1**. Occupiers need **one week's stores**, ammo to fire **twice**, fuel to move **20 CP**. **10+ strategic**, **4–9 substantive**, **1–3 marginal**; **0** gives Commonwealth victory. [R §63.92; [scoring image](https://archive.org/download/campaign-for-north-africa/page/n87.jpg)]

### Full-campaign victory

- **Automatic Axis victory:** all Alexandria **and** Cairo hexes occupied for **one full Game-Turn**, with truck supply route **≤90 CP** to a dump supplied onward from Tobruk/Tripoli.
- **Automatic Commonwealth victory:** from **GT35 OS1**, no on-map Axis combat unit can trace **≤60 truck CP** to a dump linked onward to Tobruk/Tripoli.
- Otherwise occupation needs at least **1 combat TOE point**, **one week of stores/water**, ammunition for **three firings**, fuel for **20 CP**.

| Location | Axis VP | Commonwealth VP |
|---|---:|---:|
| Mersa Matruh | **100** | **10** |
| Sidi Barrani | **50** | **10** |
| Siwa | **20** | **10** |
| Jalo | **10** | **20** |
| Giarabub | **15** | **10** |
| Bardia | **100** | **50** |
| Sollum | **25** | **10** |
| Tobruk | **200** | **100** |
| Derna | **25** | **50** |
| Benghazi | **75** | **100** |

Each unused eligible Replacement Point adds **1 VP**; aircraft/trucks excluded, and Commonwealth infantry excluded. Commonwealth voluntarily withdrawn qualifying battalions (at least **75% TOE**) earn **1/2 VP per week**, maximum **3 per unit**; return costs **2 VP**, and rewithdrawal must wait **six months**. Final higher/lower score ratio: **1:1 draw**; **>1:1 through 1.5:1 marginal**; **>1.5:1 through 2.5:1 decisive**; **>2.5:1 smashing**. [R §§64.71–76; [ratio/withdrawal image](https://archive.org/download/campaign-for-north-africa/page/n88.jpg)]

## 8. Unit organization and strength structure

The supplied **Organization-at-Arrival (OA) sheets**, reinforcement schedules and nationality characteristics charts are interlocking data: scenario setup/reinforcement date → unit and parent OA → characteristic code → ratings, maximum TOE, weapons and CPA. Create a TOE record for each represented unit; subordinate units can be carried inside parent counters or detached onto their own counters. **Assigned** denotes the organizational relationship; **attached** denotes current membership with the parent. A historical name such as regiment/brigade does not reliably indicate game size; use equivalence/stacking data. [R §§4.4–5, 9.2, 19]

- Hierarchy: **division → brigade/regiment-equivalent → battalion-equivalent → company/battery**. More than **25 land combat types** include foot/motorized/mechanized/motorcycle infantry, MG/heavy weapons, garrisons, airborne, tank, armored car/recce, field/SP/coastal artillery, anti-tank and light/heavy AA; support includes HQ, engineers, truck columns, SGSUs, raiders and naval units. [R §§3.2–5]
- **Counter inventory:** **nine sheets × 200 = 1,800 counters**, including markers/support, not 1,800 full-strength combat units. Map: **five sections**. **[NOT SPECIFIED]** a verified numerical category-by-category combat/support/marker counter count was not established; do not infer it from the total. [R §4.6]
- **One TOE point** roughly represents **100–200 infantry**, **5–8 tanks**, or **four guns**, depending on type. Ratings include offensive/defensive assault, barrage, anti-armor, AA, armor protection, gun vulnerability, fuel factor/capacity and CPA; normal basic morale spans **−3 through +3**. [R §3.5]
- **Example exact structures:** Commonwealth tank battalion-equivalent g permits **10 tank TOE points**; infantry k permits **7**, common l–q types **6**, h/r/s **5**, j **4**, t/u **3**; MG company w **2**. Artillery x/y permit **6/4 gun TOE points**; anti-tank z permits **8 AT + 3 artillery**, aa **8 AT**, bb **4 AT + 2 light AA**. These are characteristic maxima, not mandatory present strengths. [R [§4.46a chart](https://archive.org/download/campaign-for-north-africa/page/n166.jpg)]
- **Shell formation:** division with **≤50%** of allowable brigade-equivalents attached; brigade with **<2/3** allowable battalions attached (specific two-battalion exception in §9.26); battalion below **50% TOE**, artillery below **25% TOE**. Shell subordinates do not count toward maintaining a parent's full organizational status. [R §§9.26–28]
- Depletion is recorded by **TOE/weapon points**, not a universal front/full versus reverse/half step. Replacements cannot exceed stated maxima. Completely eliminated non-HQ units cannot normally be rebuilt; surviving HQ cadres can receive new subordinates within organizational limits. Destroyed tanks and broken vehicles have separate towing/repair/recovery handling. [R §§19.6, 20–22]

**Coverage limit:** the complete named OA roster, reinforcement timetable, aircraft/loadout roster and every weapon's characteristics are published in [R] but are not copied into this concise reference. **[NOT SPECIFIED]** no single universal infantry/tank/artillery strength or two-step counter template exists; each unit must obtain its values from the applicable OA/characteristics rows rather than reconstructed genre defaults.
