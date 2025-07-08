//! SIMD‑friendly bitboard representation for a Puyo board in Rust **with a column‑height cache**
//!
//! * **Board size**   6 columns × 13 rows = 78 cells ⇒ fits in a `u128`.
//! * **State slots**  0 = empty (unused), 1 = garbage, 2‥6 = 5 colors.
//! * **Column height cache**  `height: [u8; 6]` keeps the current column height so we
//!   can obtain the *first empty cell* in O(1).
//! * **NEW**  `set_xy` to place a puyo at an explicit (x,y) and `fmt::Display`
//!   implementation for **console debug‐printing**.
//!
//! Layout (little‑endian, column‑major):
//! ```text
//! bit 0   = column 0, row 0 (bottom‑left)
//! bit 12  = column 0, row 12 (top‑left)
//! bit 13  = column 1, row 0 … etc.
//! ```


#![allow(clippy::identity_op)]

use core::fmt;
use core::ops::{BitAnd, BitOr, Not};
use colored::*;

/// One 78‑bit board stored in a `u128`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BitBoard(pub u128);

impl BitBoard {
    pub const COLS: usize = 6;
    pub const ROWS: usize = 13;
    pub const COL_HEIGHT: u32 = Self::ROWS as u32; // 13

    // ---------------------------------------------------------------------
    // Pre‑computed column masks (6 variants)
    // ---------------------------------------------------------------------
    pub const COL_MASKS: [u128; Self::COLS] = {
        let base: u128 = (1u128 << Self::COL_HEIGHT) - 1; // 13 ones
        [
            base << (0 * Self::COL_HEIGHT),
            base << (1 * Self::COL_HEIGHT),
            base << (2 * Self::COL_HEIGHT),
            base << (3 * Self::COL_HEIGHT),
            base << (4 * Self::COL_HEIGHT),
            base << (5 * Self::COL_HEIGHT),
        ]
    };

    /// Bit mask of one full column (13 ones)
    #[inline(always)]
    pub const fn col_mask(col: usize) -> u128 {
        Self::COL_MASKS[col]
    }

    // ---------------------------------------------------------------------
    // Single‑step shifts (branch‑free, edge‑masked)
    // ---------------------------------------------------------------------

    #[inline(always)]
    pub fn shift_left(self) -> Self {
        BitBoard((self.0 << Self::COL_HEIGHT) & !LEFT_EDGE_MASK)
    }
    #[inline(always)]
    pub fn shift_right(self) -> Self {
        BitBoard((self.0 >> Self::COL_HEIGHT) & !RIGHT_EDGE_MASK)
    }
    #[inline(always)]
    pub fn shift_up(self) -> Self {
        BitBoard(self.0 << 1)
    }
    #[inline(always)]
    pub fn shift_down(self) -> Self {
        BitBoard(self.0 >> 1)
    }

    // ---------------------------------------------------------------------
    // Flood‑fill and component size check
    // ---------------------------------------------------------------------

    #[inline]
    pub fn flood_fill(seed: Self, occ: Self) -> Self {
        let mut grp = seed;
        loop {
            let expanded = (grp.shift_left().0
                | grp.shift_right().0
                | grp.shift_up().0
                | grp.shift_down().0)
                & occ.0;
            let nxt = BitBoard(grp.0 | expanded);
            if nxt == grp {
                return grp;
            }
            grp = nxt;
        }
    }

    #[inline]
    pub fn has_component_at_least(self, k: u32) -> bool {
        let mut rem = self;
        let occ = self;
        while rem.0 != 0 {
            let lsb = rem.0 & (!rem.0 + 1);
            let grp = Self::flood_fill(BitBoard(lsb), occ);
            if grp.0.count_ones() >= k {
                return true;
            }
            rem.0 &= !grp.0;
        }
        false
    }
}

// -----------------------  Board & helpers  --------------------------------

pub const LEFT_EDGE_MASK: u128 = BitBoard::COL_MASKS[0];
pub const RIGHT_EDGE_MASK: u128 = BitBoard::COL_MASKS[BitBoard::COLS - 1];

/// Board with 7 state bitboards **plus column‑height cache**.
#[derive(Clone, Debug)]
pub struct Board {
    pub(crate) bb: [BitBoard; 7],
    height: [u8; BitBoard::COLS], // 0‥=13, maintained incrementally
}

impl Default for Board {
    fn default() -> Self {
        Self { bb: [BitBoard::default(); 7], height: [0; BitBoard::COLS] }
    }
}

impl Board {
    // -------------------------  O(1) helpers  -----------------------------

    /// Height (0‥13) of the given column.
    #[inline(always)]
    pub fn col_height(&self, col: usize) -> u8 { self.height[col] }

    /// Bit mask of **the first empty cell** in `col`, or `None` if full.
    #[inline(always)]
    pub fn top_bit(&self, col: usize) -> Option<u128> {
        let h = self.height[col] as usize;
        if h >= BitBoard::ROWS { None }
        else {
            Some(1u128 << (col * BitBoard::ROWS + h))
        }
    }

    // -------------------------  Core queries  ----------------------------

    #[inline]
    pub fn occupancy(&self) -> BitBoard {
        let mut o = 0u128;
        for &b in &self.bb[1..] { o |= b.0; }
        BitBoard(o)
    }

    #[inline]
    pub fn has_n_chain(&self, n: u32) -> bool {
        for color in 2..=6 {
            if self.bb[color].has_component_at_least(n) { return true; }
        }
        false
    }

    // -------------------------  Mutations  -------------------------------

    /// Low‑level setter that updates bitboards **and height cache**.
    #[inline]
    fn set_state(&mut self, bit: u128, state: usize) {
        let col = (bit.trailing_zeros() as usize) / BitBoard::ROWS;
        // erase from every state
        for b in &mut self.bb { b.0 &= !bit; }
        if state != 0 { self.bb[state].0 |= bit; }
        // maintain height cache (count ones is cheap, only 13 bits)
        let m = BitBoard::col_mask(col);
        let col_occ = self.occupancy().0 & m;
        self.height[col] = col_occ.count_ones() as u8;
    }

    /// Place a new puyo of `state` on top of column `col`. Returns `false` if full.
    #[inline]
    pub fn push_puyo(&mut self, col: usize, state: usize) -> bool {
        if let Some(bit) = self.top_bit(col) {
            self.set_state(bit, state);
            true
        } else { false }
    }

    /// **NEW**: place a puyo directly at `(x, y)` (0‑based, bottom‑left origin).
    /// Returns `false` if coordinates are out of bounds or cell is already occupied.
    #[inline]
    pub fn set_xy(&mut self, x: usize, y: usize, state: usize) -> bool {
        // if x >= BitBoard::COLS || y >= BitBoard::ROWS { return false; }
        let bit = 1u128 << (x * BitBoard::ROWS + y);
        if (self.occupancy().0 & bit) != 0 { return false; } // already filled
        self.set_state(bit, state);
        true
    }

    /// Apply gravity to **every column** and rebuild `height` incrementally.
    pub fn apply_gravity(&mut self) {
        #[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))]
        unsafe {
            use core::arch::x86_64::{_pext_u64, _pdep_u64};
            for col in 0..BitBoard::COLS {
                let m = BitBoard::COL_MASKS[col];
                let mut stack: u64 = 0;
                for state in (1..=6).rev() {
                    let slice = (self.bb[state].0 & m) >> (col as u32 * BitBoard::COL_HEIGHT);
                    let cnt = slice.count_ones() as u64;
                    let packed = _pext_u64(slice as u64, (1u64 << BitBoard::COL_HEIGHT) - 1);
                    stack |= _pdep_u64(packed, ((1u64 << cnt) - 1) << stack.trailing_zeros());
                }
                // clear column
                for b in &mut self.bb { b.0 &= !m; }
                // write back stacked bits (bottom‑packed)
                let mut cursor = 0u32;
                for state in (1..=6).rev() {
                    let bits = (stack & (((1u64 << BitBoard::COL_HEIGHT) - 1) << cursor)) >> cursor;
                    let cnt = bits.count_ones();
                    if cnt > 0 {
                        let mask = (((1u128 << cnt) - 1) << cursor) << (col as u32 * BitBoard::COL_HEIGHT);
                        self.bb[state].0 |= mask;
                        cursor += cnt;
                    }
                }
                self.height[col] = cursor as u8;
            }
            return;
        }
        // ---- portable fallback ----
        for col in 0..BitBoard::COLS {
            let m = BitBoard::COL_MASKS[col];
            let mut cursor = 0u32;
            for row in 0..BitBoard::ROWS {
                let bit = 1u128 << (col * BitBoard::ROWS + row);
                let st = self.state_at(bit);
                if st != 0 {
                    self.set_state(bit, 0);
                    self.set_state(1u128 << (col * BitBoard::ROWS + cursor as usize), st);
                    cursor += 1;
                }
            }
            self.height[col] = cursor as u8;
        }
    }

    // helper to look up state at specific bit (slow path, used rarely)
    fn state_at(&self, bit: u128) -> usize {
        for (i, b) in self.bb.iter().enumerate() {
            if b.0 & bit != 0 { return i; }
        }
        0
    }
}

// -------------------------------------------------------------------------
// Colored ASCII rendering
// -------------------------------------------------------------------------
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (0..BitBoard::ROWS).rev() {
            for x in 0..BitBoard::COLS {
                let bit = 1u128 << (x*BitBoard::ROWS + y);
                let cell = match self.state_at(bit) {
                    0 => "·".normal(),           // empty
                    1 => "■".bright_white(),      // garbage
                    2 => "●".red(),
                    3 => "●".yellow(),
                    4 => "●".green(),
                    5 => "●".blue(),
                    6 => "●".magenta(),
                    _ => "?".white(),
                };
                write!(f, "{} ", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// ----------------  Optional AVX2 flood‑fill (unchanged) ------------------

#[cfg(all(target_feature = "avx2", target_arch = "x86_64"))]
mod avx2 {
    use super::*;
    use core::arch::x86_64::*;

    #[inline]
    pub unsafe fn flood_fill_avx2(seed: BitBoard, occ: BitBoard) -> BitBoard {
        let v_occ = _mm256_set1_epi64x(occ.0 as i64);
        let mut grp = _mm256_set1_epi64x(seed.0 as i64);
        let mask_left = _mm256_set1_epi64x(!LEFT_EDGE_MASK as i64);
        let mask_right = _mm256_set1_epi64x(!RIGHT_EDGE_MASK as i64);
        loop {
            let shl1 = _mm256_and_si256(_mm256_slli_epi64(grp, 1), mask_right);
            let shr1 = _mm256_and_si256(_mm256_srli_epi64(grp, 1), mask_left);
            let up   = _mm256_slli_epi64(grp, BitBoard::COL_HEIGHT);
            let down = _mm256_srli_epi64(grp, BitBoard::COL_HEIGHT);
            let exp = _mm256_and_si256(_mm256_or_si256(_mm256_or_si256(shl1, shr1), _mm256_or_si256(up, down)), v_occ);
            let nxt = _mm256_or_si256(grp, exp);
            if _mm256_testc_si256(nxt, grp) != 0 {
                return BitBoard(_mm256_extract_epi64::<0>(nxt) as u128);
            }
            grp = nxt;
        }
    }
}
