// Default precision: m = 2^14 = 16384 registers
pub const DEFAULT_B: u8 = 14;

// Alpha constant for harmonic mean computation
pub fn alpha_m(m: u32) -> f64 {
    match m {
        16 => 0.673,
        32 => 0.697,
        64 => 0.709,
        _ => 0.7213 / (1.0 + 1.079 / m as f64),
    }
}