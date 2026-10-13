//! Assemble the transcribed CNA map (docs/data/map-terrain.md) into an engine
//! `Map` and report its dimensions and terrain distribution.
//!
//! Coordinate mapping (docs/data/map-geometry.md): a printed `ccrr` on sheet
//! `s` maps to axial `q = cc`, `r = rr + t_s - (cc + 1) / 2`, with sheet row
//! shifts A=0, B=33, C=66, D=99, E=132.

use cna_engine::map::{Hex, Map, Terrain};

fn parse_sheet(grid: &str, t: i16) -> Vec<(Hex, Terrain)> {
    let mut out = Vec::new();
    for line in grid.lines() {
        let mut parts = line.split_whitespace();
        let rr: i16 = match parts.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => continue,
        };
        let cells = match parts.next() {
            Some(s) => s,
            None => continue,
        };
        for (i, ch) in cells.chars().enumerate() {
            let cc = i as i16 + 1;
            if let Some(terrain) = Terrain::from_transcription_code(ch) {
                let q = cc;
                let r = rr + t - (cc + 1) / 2;
                out.push((Hex::new(q, r), terrain));
            }
        }
    }
    out
}

fn main() {
    let text = include_str!("../../../docs/data/map-terrain.md");
    let sheets: [(&str, i16); 5] = [("A", 0), ("B", 33), ("C", 66), ("D", 99), ("E", 132)];

    let mut cells = Vec::new();
    for (name, t) in sheets {
        let marker = format!("## Sheet {name}");
        let Some(sec) = text.split(&marker).nth(1) else {
            eprintln!("missing sheet {name}");
            continue;
        };
        let grid = sec.split("\n## ").next().unwrap_or(sec);
        let n = cells.len();
        cells.extend(parse_sheet(grid, t));
        eprintln!("sheet {name}: {} cells", cells.len() - n);
    }

    let map = Map::from_axial_hexes(cells.iter().copied());

    // Terrain distribution.
    let mut counts = std::collections::HashMap::new();
    for m in &map.hexes {
        *counts.entry(m.terrain).or_insert(0u32) += 1;
    }

    println!("full map: {} rows x {} cols, {} cells", map.rows, map.cols, map.hexes.len());
    println!("resolved terrain entries: {}", cells.len());
    let mut sorted: Vec<_> = counts.into_iter().collect();
    sorted.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
    for (t, n) in sorted {
        println!("  {:?}: {}", t, n);
    }
}
