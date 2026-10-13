//! Verify the full transcribed CNA map assembles into an engine `Map` and
//! report its dimensions and terrain distribution.

use cna_engine::scenario::full_map;

fn main() {
    let map = full_map();

    let mut counts = std::collections::HashMap::new();
    for m in &map.hexes {
        *counts.entry(m.terrain).or_insert(0u32) += 1;
    }

    println!("full map: {} rows x {} cols, {} cells", map.rows, map.cols, map.hexes.len());
    let mut sorted: Vec<_> = counts.into_iter().collect();
    sorted.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
    for (t, n) in sorted {
        println!("  {:?}: {}", t, n);
    }
}
