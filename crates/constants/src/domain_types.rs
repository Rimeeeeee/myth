//! Domain constants

use alloy_primitives::FixedBytes;
use myth_types::DomainType;

pub const DOMAIN_BEACON_PROPOSER: DomainType = FixedBytes([0x00, 0x00, 0x00, 0x00]);
pub const DOMAIN_BEACON_ATTESTER: DomainType = FixedBytes([0x01, 0x00, 0x00, 0x00]);
pub const DOMAIN_RANDAO: DomainType = FixedBytes([0x02, 0x00, 0x00, 0x00]);
pub const DOMAIN_DEPOSIT: DomainType = FixedBytes([0x03, 0x00, 0x00, 0x00]);
pub const DOMAIN_VOLUNTARY_EXIT: DomainType = FixedBytes([0x04, 0x00, 0x00, 0x00]);
pub const DOMAIN_SELECTION_PROOF: DomainType = FixedBytes([0x05, 0x00, 0x00, 0x00]);
pub const DOMAIN_AGGREGATE_AND_PROOF: DomainType = FixedBytes([0x06, 0x00, 0x00, 0x00]);
pub const DOMAIN_APPLICATION_MASK: DomainType = FixedBytes([0x00, 0x00, 0x00, 0x01]);
