mod utils;

use utils::{alpha_m, DEFAULT_B};
use std::hash::Hash;

pub struct HyperLogLog {
    /// Number of registers (m = 2^b)
    m: u32,
    /// Register array, each storing position of first 1-bit
    registers: Vec<u8>,
    /// Precision bits (b value)
    b: u8,
}

impl HyperLogLog {
    pub fn new() -> Self {
        Self::with_precision(DEFAULT_B)
    }

    pub fn with_precision(b: u8) -> Self {
        assert!(
            (4..=16).contains(&b),
            "Precision b must be between 4 and 16"
        );
        let m = 1u32 << b; // m = 2^b
        HyperLogLog {
            m,
            registers: vec![0u8; m as usize],
            b,
        }
    }

    pub fn add<T: Hash>(&mut self, item: &T) {
        let hash_value = self.hash(item);
        self.add_hash(hash_value);
    }

    fn add_hash(&mut self, hash_value: u64) {
        // Extract first b bits for register index
        let j = (hash_value >> (64 - self.b)) as usize;

        // Get remaining bits
        let w = hash_value & ((1u64 << (64 - self.b)) - 1);

        // Calculate ρ(w): position of first 1-bit in the remaining bits
        let register_value = self.rho(w);
        if register_value > self.registers[j] {
            self.registers[j] = register_value;
        }
    }

    /// Estimate the cardinality
    pub fn cardinality(&self) -> f64 {
        // Step 1: Compute raw estimate using harmonic mean
        let z = self.compute_z();
        let alpha = alpha_m(self.m);
        let raw_estimate = alpha * (self.m as f64) * (self.m as f64) / z;

        // Step 2 & 3: Apply small and large range corrections
        self.correct_estimate(raw_estimate)
    }

    pub fn merge(&mut self, other: &HyperLogLog) {
        assert_eq!(
            self.m, other.m,
            "Cannot merge HyperLogLogs with different precision"
        );
        for i in 0..self.registers.len() {
            self.registers[i] = self.registers[i].max(other.registers[i]);
        }
    }

    fn rho(&self, w: u64) -> u8 {
        if w == 0 {
            (64 - self.b + 1) as u8
        } else {
            (w.leading_zeros() + 1) as u8
        }
    }

    fn compute_z(&self) -> f64 {
        let mut sum: f64 = 0.0;
        for &register in &self.registers {
            sum += 2f64.powi(-(register as i32));
        }
        sum
    }

    fn correct_estimate(&self, raw_estimate: f64) -> f64 {
        // Small range correction - use empty register count
        let empty_registers = self
            .registers
            .iter()
            .filter(|&&r| r == 0)
            .count() as f64;

        if raw_estimate <= 2.5 * self.m as f64 && empty_registers > 0.0 {
            return (self.m as f64) * ((self.m as f64) / empty_registers).ln();
        }

        // Large range correction - handle hash collisions
        let max_hash_bits = (1u64 << 32) as f64;
        if raw_estimate > (1.0 / 30.0) * max_hash_bits {
            return -max_hash_bits * (1.0 - raw_estimate / max_hash_bits).ln();
        }

        raw_estimate
    }

    fn hash<T: Hash>(&self, item: &T) -> u64 {
        use std::hash::Hasher;
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        hasher.finish()
    }
}