//! Common type aliases for Ethereum Consensus.

use alloy_primitives::{B256, FixedBytes};

/// Custom Ethereum consensus types.
///
/// See: <https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#custom-types>
///
/// |------------------------------------------------------------------------|
/// | Type            | SSZ equivalent    | Description                         |
/// |-----------------|----------------|-------------------------------------|
/// | `Slot`          | `u64`          | A slot number                       |
/// | `Epoch`         | `u64`          | An epoch number                     |
/// | `CommitteeIndex`| `u64`          | A committee index at a slot         |
/// | `ValidatorIndex`| `u64`          | A validator registry index          |
/// | `Gwei`          | `u64`          | An amount in Gwei                   |
/// | `Root`          | `Bytes32`      | A Merkle root                       |
/// | `Hash32`        | `Bytes32`      | A 256-bit hash                      |
/// | `Version`       | `Bytes4`       | A fork version number               |
/// | `DomainType`    | `Bytes4`       | A domain type                       |
/// | `ForkDigest`    | `Bytes4`       | A digest of the current fork data   |
/// | `Domain`        | `Bytes32`      | A signature domain                  |
/// | `BLSPubkey`     | `Bytes48`      | A BLS12-381 public key              |
/// | `BLSSignature`  | `Bytes96`      | A BLS12-381 signature               |
/// |------------------------------------------------------------------------|

/// A slot number.
pub type Slot = u64;

/// An epoch number.
pub type Epoch = u64;

/// A committee index at a slot.
pub type CommitteeIndex = u64;

/// A validator registry index.
pub type ValidatorIndex = u64;

/// An amount in Gwei.
pub type Gwei = u64;

/// A Merkle root.
pub type Root = B256;

/// A 256-bit hash.
pub type Hash32 = B256;

/// A fork version number.
pub type Version = FixedBytes<4>;

/// A domain type.
pub type DomainType = FixedBytes<4>;

/// A digest of the current fork data.
pub type ForkDigest = FixedBytes<4>;

/// A signature domain.
pub type Domain = FixedBytes<32>;

/// A BLS12-381 public key.
pub type BLSPubkey = FixedBytes<48>;

/// A BLS12-381 signature.
pub type BLSSignature = FixedBytes<96>;
