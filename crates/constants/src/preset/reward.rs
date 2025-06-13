//! Ethereum Beacon Chain Preset - Rewards and Penalties
//!
//! Constants related to validator rewards, penalties, and slashing mechanics.

pub const BASE_REWARD_FACTOR: u64 = 1 << 6; // 64
pub const WHISTLEBLOWER_REWARD_QUOTIENT: u64 = 1 << 9; // 512
pub const PROPOSER_REWARD_QUOTIENT: u64 = 1 << 3; // 8
pub const INACTIVITY_PENALTY_QUOTIENT: u64 = 1 << 26; // 67,108,864
pub const MIN_SLASHING_PENALTY_QUOTIENT: u64 = 1 << 7; // 128
pub const PROPORTIONAL_SLASHING_MULTIPLIER: u64 = 1; // 1
