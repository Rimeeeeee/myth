//! Statelist Length Constants

use crate::Epoch;

pub const EPOCHS_PER_HISTORICAL_VECTOR: Epoch = 1 << 16; // 65,536 epochs
pub const EPOCHS_PER_SLASHINGS_VECTOR: Epoch = 1 << 13; // 8,192 epochs
pub const HISTORICAL_ROOTS_LIMIT: u64 = 1 << 24; // 16,777,216 historical roots
pub const VALIDATOR_REGISTRY_LIMIT: u64 = 1 << 40; // 1,099,511,627,776 validators
