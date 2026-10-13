# CNA Vassal map geometry

## Source and coordinate conventions

Primary source: [CNAv2.1.0.vmod](https://obj.vassalengine.org/gls/9/4/CNAv2.1.0.vmod), ZIP member `buildFile.xml`, module version `2.1.0`, `VassalVersion="3.7.15"`. The board is `VASSAL.build.module.map.boardPicker.Board`, `name="CNA Original"`, image `CNA Map Vassal Mitch Guthrie 2021.png` (14310 × 4632 pixels). Its `VASSAL.build.module.map.boardPicker.board.ZonedGrid` contains the five zones below. Board placement is `(0,0)` and map `edgeWidth=edgeHeight=0`. Pixel X increases right; pixel Y increases down. All origins and polygon vertices are **board-image coordinates**, not crop-relative coordinates.

XML element abbreviations in the tables:

| Abbreviation | Exact XML element name |
|---|---|
| Zone | `VASSAL.build.module.map.boardPicker.board.mapgrid.Zone` |
| HexGrid | `VASSAL.build.module.map.boardPicker.board.HexGrid` |
| Numbering | `VASSAL.build.module.map.boardPicker.board.mapgrid.HexGridNumbering` |

Coordinate interpretation follows the module's engine version: [HexGridNumbering, Vassal 3.7.15](https://github.com/vassalengine/vassal/blob/3.7.15/vassal-app/src/main/java/VASSAL/build/module/map/boardPicker/board/mapgrid/HexGridNumbering.java), [RegularGridNumbering](https://github.com/vassalengine/vassal/blob/3.7.15/vassal-app/src/main/java/VASSAL/build/module/map/boardPicker/board/mapgrid/RegularGridNumbering.java), [HexGrid](https://github.com/vassalengine/vassal/blob/3.7.15/vassal-app/src/main/java/VASSAL/build/module/map/boardPicker/board/HexGrid.java), and [Zone](https://github.com/vassalengine/vassal/blob/3.7.15/vassal-app/src/main/java/VASSAL/build/module/map/boardPicker/board/mapgrid/Zone.java). The module does not declare an inter-sheet adjacency table; the integer lattice offsets below are derived from these settings.

## Hex spacing and origins

| XML element / attribute | Exact value, all five sheets | Meaning |
|---|---|---|
| HexGrid `dx` | `72.95` | Engine hex width / spacing between raw columns |
| HexGrid `dy` | `85.25` | Engine hex height / spacing between raw rows |
| HexGrid `sideways` | `true` | Swap internal X/Y when mapping to the board image |
| HexGrid `snapTo` | `true` | Snap to the grid |
| HexGrid `edgesLegal`, `cornersLegal` | `false`, `false` | Centers, not hexsides or vertices, are legal snap locations |
| HexGrid `visible`, `dotsVisible` | `false`, `false` | Engine grid and center dots hidden |
| Derived half-row shift | `42.625` pixels | `dy / 2` |
| Derived image horizontal center spacing | `85.25` pixels | Raw-row spacing after the sideways transform |
| Derived image vertical center spacing | `72.95` pixels | Raw-column spacing after the sideways transform |
| Derived image hex width / tip-to-tip height | `85.25` / `97.266666666666…` pixels | Hex width in the image; tip height is `4 × dx / 3`, not `dy` |

`dx` and `dy` are the two **independently configured** engine values; do not replace either with a regular-hex trigonometric approximation.

| Sheet | HexGrid `x0` | HexGrid `y0` | Board-image position of raw origin `(X,Y)` | Numbering `stagger` | Numbered-column stagger in image |
|---|---:|---:|---|---|---|
| A | 55 | 48 | `(48,55)` | `true` | Odd `cc` shifted left by `42.625` relative to even `cc` at the same `rr` |
| B | -16 | 7 | `(7,-16)` | `false` | Even `cc` shifted right by `42.625` relative to odd `cc` at the same `rr` |
| C | -16 | 10 | `(10,-16)` | `false` | Even `cc` shifted right by `42.625` |
| D | -16 | 16 | `(16,-16)` | `false` | Even `cc` shifted right by `42.625` |
| E | -17 | -66 | `(-66,-17)` | `false` | Even `cc` shifted right by `42.625` |

In unrotated raw engine coordinates, **odd raw columns** always have the positive `dy/2` row shift. After `sideways=true`, this means odd raw-column hex centers move right by `42.625` image pixels. The Numbering `stagger` attribute is a **number-label row adjustment**, not a different physical grid. A's extra label adjustment reconciles its opposite raw parity with the same numbered-column eastward stagger used on B–E.

## Numbering and coordinate mapping

A coordinate `ccrr` consists of the column first and row second. For example, `Map A0330` means column 03, row 30; `first="H"`, `hType=vType="N"`, `hLeading=vLeading="1"`, and `sep=""` produce the two numeric fields with at least two digits. All five zones have `locationFormat="$name$$gridLocation$"`; their Numbering child has `locationFormat="$gridLocation$"`.

| Sheet | Numbering `hOff` | Numbering `vOff` | `hDescend` | `vDescend` | Derived `N=getMaxRows()` | Raw-column-zero label `(cc,rr)` | Transcription envelope `(cc,rr)` |
|---|---:|---:|---|---|---:|---|---|
| A | 6 | 0 | `false` | `true` | 56 | `(62,00)` | `(01–56,01–34)` |
| B | 2 | -33 | `false` | `true` | 61 | `(63,-33)` | `(01–61,01–34)` |
| C | 12 | -66 | `false` | `true` | 51 | `(63,-66)` | `(01–51,01–34)` |
| D | 21 | -99 | `false` | `true` | 42 | `(63,-99)` | `(01–42,01–34)` |
| E | 19 | -133 | `false` | `true` | 44 | `(63,-133)` | `(01–44,01–34)` |

The raw-origin labels are mathematical labels, **not playable cells**. `hOff`/`vOff` are the engine's numbering offsets, not the first visible coordinate. Increasing printed `cc` moves **up** the image by `72.95` pixels; increasing printed `rr` moves **right** by `85.25` pixels. All Numbering elements also specify `rotateText="270"`, `hDrawOff="0"`, `vDrawOff="-30"`, `fontSize="12"`, `visible="true"`; these affect drawing, not cell topology.

For a sheet's printed `(cc,rr)`, define:

- `N = floor((zone_bottom - zone_top) / 72.95 + 0.5)`; `Zone.getSize()` uses the **polygon bounding size**, with no translation of the origin.
- `k = N + hOff - cc` (raw column).
- `p = k mod 2`, normalized to 0 or 1.
- `j = rr - vOff - p` on A, and `j = rr - vOff` on B–E.
- Ideal image center: `X = y0 + 85.25*j + 42.625*p`, `Y = x0 + 72.95*k`.
- Exact Vassal `getCenterPoint` integer position: `X = y0 + trunc(85.25*j + 42.625*p)`, `Y = x0 + trunc(72.95*k)`; Java casts truncate toward zero **before** adding the integer origin.

For this transcription's positive coordinate range, A has `k=62-cc`; B–E have `k=63-cc`.

## Zone bounds and exact polygons

These rectangles are **derived bounds**, not separate XML rectangle elements. Membership must use each Zone's `path`, not its enclosing rectangle. The last path vertex connects back to the first.

| Zone `name` | Bounds `(left,top,right,bottom)` | Bounding width | Bounding height | `path` vertices | `useParentGrid` |
|---|---|---:|---:|---:|---|
| Map A | `(44,447,2863,4557)` | 2819 | 4110 | 69 | `false` |
| Map B | `(2862,83,5681,4556)` | 2819 | 4473 | 32 | `false` |
| Map C | `(5680,811,8499,4557)` | 2819 | 3746 | 42 | `false` |
| Map D | `(8497,1468,11315,4556)` | 2818 | 3088 | 39 | `false` |
| Map E | `(11312,1320,14173,4558)` | 2861 | 3238 | 51 | `false` |

Exact `Zone.path` attribute values, preserving XML vertex order:

| Zone | `path` |
|---|---|
| Map A | `44,2586;88,2561;87,2515;129,2490;476,2489;516,2512;518,2562;559,2585;559,2635;601,2658;602,2707;644,2731;687,2709;729,2734;731,2779;774,2806;771,2854;815,2877;858,2852;899,2878;902,2925;944,2950;944,2999;983,3024;1029,2999;1072,3025;1073,3072;1114,3098;1157,3072;1197,3095;1198,3145;1240,3169;1283,3147;1326,3171;1369,3145;1409,3171;1455,3147;1496,3169;1642,3087;2054,2851;2054,2805;2221,2515;2308,2367;2353,2341;2349,2293;2308,2271;2306,2221;2350,2197;2220,1969;2136,1834;2137,1784;2178,1761;2181,1710;2137,1684;2094,1613;2095,1275;2350,837;2522,737;2522,690;2693,593;2692,546;2735,522;2738,469;2779,447;2820,474;2863,448;2863,4554;1207,4557;45,4555` |
| Map B | `2862,4556;5681,4555;5679,836;5639,815;5639,763;5596,737;5594,690;5553,668;5554,617;5510,594;5510,544;5555,521;5552,470;5510,446;5509,396;5465,373;5466,327;5427,302;5256,301;5253,254;5213,230;5040,229;5042,182;5001,156;4826,156;4827,106;4785,84;3759,83;3715,110;3716,155;3292,155;2863,397` |
| Map C | `5680,884;5936,883;5936,811;6063,812;6106,838;6109,885;6195,886;6235,910;6236,959;6320,958;6363,984;6363,1031;6451,1033;7301,1031;7346,1055;7346,1103;7428,1105;7473,1131;7471,1177;7517,1205;7517,1251;7473,1274;7472,1323;7514,1346;7515,1394;7558,1420;7559,1467;7599,1492;7600,1542;7645,1567;7643,1613;7690,1637;7730,1613;7773,1640;7813,1613;7813,1565;7856,1542;7898,1566;8071,1468;8497,1469;8499,4557;5680,4555` |
| Map D | `8497,4556;11312,4556;11315,2149;11273,2125;11272,2076;11230,2052;11059,2052;11056,2002;11015,1978;10970,2003;10931,1978;10759,1976;10501,1978;10461,2002;10419,1980;10420,1929;10377,1906;10377,1859;10336,1832;10251,1832;10076,1931;10033,1906;9949,1904;9949,1857;9906,1831;9905,1785;9863,1759;9864,1714;9821,1686;9480,1688;9479,1638;9439,1614;8840,1611;8840,1567;8800,1543;8628,1543;8627,1493;8588,1468;8498,1468` |
| Map E | `11312,2149;11354,2122;11396,2148;11399,2195;11443,2222;11444,2270;11485,2294;11525,2269;11569,2295;11613,2270;11653,2295;11697,2273;11695,2223;12383,1832;12381,1785;12422,1761;12424,1711;12466,1686;12722,1685;12722,1642;12764,1612;12763,1565;12808,1538;12806,1493;12850,1469;13278,1472;13279,1420;13318,1395;13362,1417;13405,1391;13446,1418;13491,1394;13489,1345;13533,1322;13660,1320;13789,1322;13832,1347;13830,1393;13873,1419;13914,1393;13959,1419;13960,1467;14003,1491;14045,1467;14088,1493;14086,1539;14132,1565;14173,1542;14173,2165;14173,4558;11315,4556` |

Malta and holding/operation/weather zones are not part of this five-sheet grid. Rectangle padding and ungridded sea are not playable hexes.

## Sheet-to-sheet adjacency

### Common numbered-column lattice

Keep `C = cc` and define common eastward row `R = rr + t`. Every sheet has the same printed-column axis; **no integer column offset is needed**. The common offset grid has **even columns shifted east/right by half a row spacing**. Equivalently, use integer axial coordinates `q=C`, `r=R-floor((C+1)/2)`; the six axial neighbors are `(q±1,r)`, `(q,r±1)`, `(q+1,r-1)`, `(q-1,r+1)`.

| Sheet | Common column `C` | Common row `R` | Sheet row shift `t` | Ideal `Y` intercept (`Y = intercept - 72.95*cc`) | Horizontal residual against A-aligned common lattice |
|---|---|---:|---:|---:|---:|
| A | `cc` | `rr` | 0 | 4577.90 | 0 |
| B | `cc` | `rr+33` | 33 | 4579.85 | +1.625 pixels |
| C | `cc` | `rr+66` | 66 | 4579.85 | +4.625 pixels |
| D | `cc` | `rr+99` | 99 | 4579.85 | +10.625 pixels |
| E | `cc` | `rr+132` | 132 | 4578.85 | +13.875 pixels |

The A-aligned common pixel lattice is `X = 5.375 + 85.25*R + (even C ? 42.625 : 0)`, `Y = 4577.90 - 72.95*C`. The residuals are exact derivations from the decimal XML settings: the artwork's local grids are **not perfectly coincident** in pixel space. Integer topology normalizes these small residuals; retaining exact Vassal rendering requires each sheet's original origin. E's `vOff=-133` is **not** a 133-row topology offset: `y0=-66` subtracts approximately one row pitch, so its derived common shift is 132.

| Seam | Destination origin relative to source in common `(C,R)` | Same common coordinate expressed on destination | Source eastern row 33 ↔ destination western row 01, same `cc` | Residual eastward pitch excess | Residual image vertical shift, same `cc` |
|---|---|---|---|---:|---:|
| A→B | `(0,+33)` | `(cc,rr-33)` | A`cc33` adjacent to B`cc01` | +1.625 pixels | +1.95 pixels |
| B→C | `(0,+33)` | `(cc,rr-33)` | B`cc33` adjacent to C`cc01` | +3 pixels | 0 pixels |
| C→D | `(0,+33)` | `(cc,rr-33)` | C`cc33` adjacent to D`cc01` | +6 pixels | 0 pixels |
| D→E | `(0,+33)` | `(cc,rr-33)` | D`cc33` adjacent to E`cc01` | +3.25 pixels | -1 pixel |

Rows 33 and 01 are **different neighboring cells**, not duplicates to merge. The “same common coordinate” formula is an algebraic conversion; it does not assert both zones contain that position. A–D row 34 is outside their center footprints and must not become an extra seam column. Zone edges are irregular; create a seam edge only when **both** endpoint centers belong to their respective zones. For an even `cc`, a source row-33 cell also has diagonal eastward neighbors in destination row 01 at `cc-1` and `cc+1`, if those cells exist. For an odd `cc`, there are no diagonal neighbors across this particular row-33/row-01 seam, although the same-column eastward neighbor remains. Use axial neighbors rather than a hand-coded all-rows seam rectangle.

### Vertical placement of sheet envelopes

The crops start at different northern positions, but printed column labels already compensate. Do not add the following crop displacement to the common `cc` mapping.

| Seam | Destination crop top minus source crop top | Exact decomposition into vertical column pitch and pixel residual | Change in maximum displayed `cc` |
|---|---:|---|---:|
| A→B | -364 pixels | `-5*72.95 + 0.75` | +5 (56→61) |
| B→C | +728 pixels | `+10*72.95 - 1.5` | -10 (61→51) |
| C→D | +657 pixels | `+9*72.95 + 0.45` | -9 (51→42) |
| D→E | -148 pixels | `-2*72.95 - 2.1` | +2 (42→44) |

## Terrain-grid footprint caveat

`map-terrain.md` records the bounded displays, not complete rules/overlay data. Strict point-in-polygon membership excludes the old sea entries A4224 and B5801; their centers are outside the XML paths even though rasterized crop masks previously included them. They are now padding `?`. Strict footprint totals are 7010 in-zone positions and 1626 outside-zone positions in the 8636-position envelope. Two missing terminal padding characters on E rows 06 and 07 are restored. Unknown **in-zone** base terrain remains `?`; it is not an instruction to omit a playable cell. Coast mixtures still require richer terrain encoding or further source interpretation before this one-character transcription can represent all mechanics.
