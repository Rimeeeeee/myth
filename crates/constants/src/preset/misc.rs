//! Ethereum Beacon Chain Preset - Misc Configuration
//!
//! Constants defining committee configuration and validator hysteresis parameters.

pub const MAX_COMMITTEES_PER_SLOT: u64 = 1 << 6; // 64
pub const TARGET_COMMITTEE_SIZE: u64 = 1 << 7; // 128
pub const MAX_VALIDATORS_PER_COMMITTEE: u64 = 1 << 11; // 2048
pub const SHUFFLE_ROUND_COUNT: u64 = 90;

pub const HYSTERESIS_QUOTIENT: u64 = 4;
pub const HYSTERESIS_DOWNWARD_MULTIPLIER: u64 = 1;
pub const HYSTERESIS_UPWARD_MULTIPLIER: u64 = 5;
