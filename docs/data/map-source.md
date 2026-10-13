# CNA map source

## Availability: FOUND in the primary archive

The [Internet Archive item](https://archive.org/details/campaign-for-north-africa) contains the five original map-sheet scans **inside the 192-page PDF**, not as five separately named map files. Inspecting only the [file metadata](https://archive.org/metadata/campaign-for-north-africa) would miss them: the listed original document is [`The Campaign for North Africa.pdf`](https://archive.org/download/campaign-for-north-africa/The%20Campaign%20for%20North%20Africa.pdf), with a 192-image JP2 derivative. The last five images, zero-based leaves **187–191**, are sheets **A–E**. All five JPGs were fetched and visually inspected; the original pages are sideways. Rotate their images 90 degrees clockwise to read the printed coordinates and sheet titles.

| Sheet | Zero-based leaf | PDF page (one-based) | Original scan size, pixels | Observed coordinate bounding envelope |
|---|---:|---:|---|---|
| A | 187 | 188 | 6669 × 10595 | columns 01–56; rows 01–34 |
| B | 188 | 189 | 6681 × 10596 | columns 01–57; rows 01–34 |
| C | 189 | 190 | 6687 × 10586 | columns 01–50; rows 01–34 |
| D | 190 | 191 | 6689 × 10585 | columns 01–42; rows 01–34 |
| E | 191 | 192 | 6693 × 10586 | columns 01–44; rows 01–34 |

Direct map scans:

- A: https://archive.org/download/campaign-for-north-africa/page/n187.jpg
- B: https://archive.org/download/campaign-for-north-africa/page/n188.jpg
- C: https://archive.org/download/campaign-for-north-africa/page/n189.jpg
- D: https://archive.org/download/campaign-for-north-africa/page/n190.jpg
- E: https://archive.org/download/campaign-for-north-africa/page/n191.jpg

**Dimensions above describe the bounding coordinate envelope, not a completely filled rectangular board.** The numbered footprint is irregular: numbered sea hexes follow the coast, while ungridded blue areas contain charts, holding boxes, tracks, and titles. Do not manufacture sea hexes in those areas. Map A also includes a separately drawn Malta inset; its coordinate layout was not resolved in this transcription. Sheet edges include partial hexes. These observations come from the linked sheet images, particularly A's southeast edge, B5701, C5001, D4201, and E4426–4429.

## Coordinate convention

The original main-map hex labels are four digits, `ccrr`, with a sheet prefix supplied when identifying a location: `A0130`, `B5701`, `C5001`, `D0201`, `E4428`. Each sheet restarts its numbering. With a scan rotated clockwise into readable landscape orientation, `cc` increases left to right and `rr` increases top to bottom; even columns are staggered downward by half a hex. This is a coordinate-oriented presentation, **not a north-up grid**. A's compass shows north toward the right in that orientation. Sources: [A](https://archive.org/download/campaign-for-north-africa/page/n187.jpg), [B](https://archive.org/download/campaign-for-north-africa/page/n188.jpg), [C](https://archive.org/download/campaign-for-north-africa/page/n189.jpg), [D](https://archive.org/download/campaign-for-north-africa/page/n190.jpg), [E](https://archive.org/download/campaign-for-north-africa/page/n191.jpg).

## Original terrain and feature legend

The **Terrain Key on sheet A** was read directly from [leaf 187](https://archive.org/download/campaign-for-north-africa/page/n187.jpg). Only its feature names and symbol distinctions are recorded here; no rulebook prose, movement-cost table, or artwork is reproduced.

| Kind | Names printed in the key |
|---|---|
| Area terrain | Clear; Salt Marsh; Desert; Rock/Gravel; Rough; Mountain; Heavy Vegetation; Delta; Swamp; Sea |
| Hex/location features | Coast; Port; Major City; Village/Bir.; Oasis |
| Transport | Road; Unfinished Road; Track; Railroad; Unfinished Railroad |
| Hexside/linear features | Ridge; Escarpment; Slope; Wadi; Border; Major River; Minor River |
| Facilities | Airfield; Flying Boat Basin; Flying Boat Alighting Area; Training Area; Off Map Airbases |

Important distinctions: **Rock/Gravel is not Desert**; **Rough is not Mountain**; **Salt Marsh is not Swamp**. Escarpments, ridges, slopes, roads, tracks, and rivers are not mutually exclusive base-hex terrain. A one-character base-terrain grid cannot encode their connections or affected hexsides. `map-terrain.md` therefore separates the base-terrain legend from feature codes and records only feature facts actually read.

## Alternate map source found

The [Vassal module library](https://vassalengine.org/library/projects/The_Campaign_for_North_Africa_The_Desert_War_1940-43) provides a downloadable [CNAv2.1.0.vmod](https://obj.vassalengine.org/gls/9/4/CNAv2.1.0.vmod). Its ZIP members were inspected in memory. It contains `images/CNA Map Vassal Mitch Guthrie 2021.png` (14310 × 4632 pixels) and `buildFile.xml` with named zones Map A–E and Malta. The library changelog says v0.93 replaced the map with a copy of the actual CNA map, v0.94 added an escarpment hexside at D2228/2328, and the 2021 release corrected the map aspect ratio. The original archive scans, rather than this modified combined map, are the authority for the initial terrain transcription. The Vassal image is a useful alternate overview, not evidence that every original feature or addendum was transcribed.

## Retrieval/search record

- Inspected https://archive.org/metadata/campaign-for-north-africa.
- Read the item's DjVu XML to locate the five map pages: https://archive.org/download/campaign-for-north-africa/The%20Campaign%20for%20North%20Africa_djvu.xml.
- Read its OCR text: https://archive.org/download/campaign-for-north-africa/The%20Campaign%20for%20North%20Africa_djvu.txt. The initial reader fetch failed with HTTP 500; a subsequent direct retrieval succeeded. OCR of the sideways map labels is unreliable and was not used as terrain truth.
- Retrieved and inspected all five `page/n187.jpg`–`page/n191.jpg` URLs above.
- Web search performed: `"Campaign for North Africa" map scan vassal`.
- Search found the Vassal library URL above, its [older module page](https://vassalengine.org/wiki_old/wiki/Module:The_Campaign_for_North_Africa:_The_Desert_War_1940-43), [BGG discussion of map scans](https://boardgamegeek.com/thread/2189632/resurrecting-the-vassal-mod), and [BGG module-completeness discussion](https://boardgamegeek.com/thread/3441784/is-vassal-module-complete). Those discussion pages were search leads, not independently inspected terrain sources.
- Inspected the current Vassal module archive in memory. A direct reader fetch of https://obj.vassalengine.org/images/a/af/CNA.vmod was refused because its 100043956-byte length exceeds the reader's binary-fetch limit; no local module or map artwork was saved.

## Transcription status

`map-terrain.md` is a **conservative, partial best-effort transcription**, not a complete playable terrain database. It gives a coordinate-addressable grid for each main sheet, retains unresolved positions as `?`, and identifies which regions supplied positive readings. Unknown entries include cells not individually resolved and positions whose membership in the irregular footprint is unresolved; they must not be interpreted as clear, sea, or confirmed playable hexes. The Malta inset and most transport/hexside connections remain unresolved rather than invented.
