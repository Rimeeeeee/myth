//! Ethereum Beacon Chain Preset - Validator Lifecycle Parameters
//!
//! Thresholds and limits for validator activation, exit, and churn.

use myth_types::Gwei;

pub const EJECTION_BALANCE: Gwei = 16_000_000_000; // 2^4 * 10^9 Gwei
pub const MIN_PER_EPOCH_CHURN_LIMIT: u64 = 1 << 2; // 4
pub const CHURN_LIMIT_QUOTIENT: u64 = 1 << 16; // 65,536
