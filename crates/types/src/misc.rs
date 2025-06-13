//! Misc Dependencies for Ethereum Consensus.

use alloy_primitives::FixedBytes;

use crate::alias::{
    BLSPubkey, BLSSignature, CommitteeIndex, Domain, Epoch, Gwei, Hash32, Root, Slot,
    ValidatorIndex, Version,
};
use r_ssz::composite::BitList;
///
/// See: <https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#misc-dependencies>

#[derive(Debug)]
pub struct Fork {
    pub previous_version: Version,
    pub current_version: Version,
    pub epoch: Epoch,
}

#[derive(Debug)]
pub struct ForkData {
    pub current_version: Version,
    pub genesis_validators_root: Root,
}

#[derive(Debug)]
pub struct Checkpoint {
    pub epoch: Epoch,
    pub root: Root,
}

#[derive(Debug)]
pub struct Validator {
    pub pubkey: BLSPubkey,
    pub withdrawal_credentials: FixedBytes<32>,
    pub effective_balance: Gwei,
    pub slashed: bool,
    pub activation_eligibility_epoch: Epoch,
    pub activation_epoch: Epoch,
    pub exit_epoch: Epoch,
    pub withdrawable_epoch: Epoch,
}

#[derive(Debug)]
pub struct AttestationData {
    pub slot: Slot,
    pub index: CommitteeIndex,
    pub beacon_block_root: Root,
    pub source: Checkpoint,
    pub target: Checkpoint,
}

#[derive(Debug)]
pub struct IndexedAttestation {
    pub attesting_indices: [ValidatorIndex; 2048],
    pub data: AttestationData,
    pub signature: BLSSignature,
}

#[derive(Debug)]
pub struct PendingAttestation {
    pub aggregation_bits: BitList<2048>,
    pub data: AttestationData,
    pub inclusion_delay: Slot,
    pub proposer_index: ValidatorIndex,
}

#[derive(Debug)]
pub struct Eth1Data {
    pub deposit_root: Root,
    pub deposit_count: u64,
    pub block_hash: Hash32,
}

#[derive(Debug)]
pub struct HistoricalBatch {
    pub block_roots: Vec<Root>, //TODO : convert to ->Vector[Root, SLOTS_PER_HISTORICAL_ROOT]
    pub state_roots: Vec<Root>, //TODO : convert to ->Vector[Root, SLOTS_PER_HISTORICAL_ROOT]
}

#[derive(Debug)]
pub struct DepositMessage {
    pub pubkey: BLSPubkey,
    pub withdrawal_credentials: FixedBytes<32>,
    pub amount: Gwei,
}

#[derive(Debug)]
pub struct DepositData {
    pub pubkey: BLSPubkey,
    pub withdrawal_credentials: FixedBytes<32>,
    pub amount: Gwei,
    pub signature: BLSSignature,
}

#[derive(Debug)]
pub struct BeaconBlockHeader {
    pub slot: Slot,
    pub proposer_index: ValidatorIndex,
    pub parent_root: Root,
    pub state_root: Root,
    pub body_root: Root,
}

#[derive(Debug)]
pub struct SigningData {
    pub object_root: Root,
    pub domain: Domain,
}
