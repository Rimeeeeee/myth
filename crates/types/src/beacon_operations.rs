//! Beacon Operations Dependencies for Ethereum Consensus.

use crate::{
    misc::{AttestationData, DepositData, IndexedAttestation},
    signed_envelopes::SignedBeaconBlockHeader,
};
use alloy_primitives::FixedBytes;
use myth_constants::{
    BLSSignature, Epoch, ValidatorIndex, misc::DEPOSIT_CONTRACT_TREE_DEPTH,
    preset::MAX_VALIDATORS_PER_COMMITTEE,
};
use r_ssz::{BitList, fixed_vectors::FixedVector};
///
/// See: <https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#beacon-operations>
const TOTAL_DEPTH: usize = DEPOSIT_CONTRACT_TREE_DEPTH as usize + 1;

#[derive(Debug)]
pub struct ProposerSlashing {
    pub signed_header_1: SignedBeaconBlockHeader,
    pub signed_header_2: SignedBeaconBlockHeader,
}

#[derive(Debug)]
pub struct AttesterSlashing {
    pub attestation_1: IndexedAttestation,
    pub attestation_2: IndexedAttestation,
}

#[derive(Debug)]
pub struct Attestation {
    pub aggregation_bits: BitList<{ MAX_VALIDATORS_PER_COMMITTEE as usize }>,
    pub data: AttestationData,
    pub signature: BLSSignature,
}

#[derive(Debug)]
pub struct Deposit {
    pub proof: FixedVector<FixedBytes<32>, TOTAL_DEPTH>,
    pub data: DepositData,
}

#[derive(Debug)]
pub struct VoluntaryExit {
    pub epoch: Epoch,
    pub validator_index: ValidatorIndex,
}
