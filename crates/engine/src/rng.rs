//! Deterministic, serializable PRNG (SplitMix64).
//!
//! Replay-safe: the entire engine state, including the RNG, can be serialized and
//! restored. `rand` is deliberately not used so the engine stays dependency-light
//! and every byte of state is reproducible.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        // Avoid a zero state, which is a fixed point for SplitMix64.
        Rng(seed.wrapping_add(0x9E37_79B9_7F4A_7C15))
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform value in `[0, n)`.
    #[inline]
    pub fn below(&mut self, n: u64) -> u64 {
        assert!(n > 0, "range must be non-empty");
        self.next_u64() % n
    }

    /// Uniform value in `[lo, hi]` inclusive.
    #[inline]
    pub fn between(&mut self, lo: u64, hi: u64) -> u64 {
        assert!(hi >= lo, "invalid range");
        lo + self.below(hi - lo + 1)
    }
}
