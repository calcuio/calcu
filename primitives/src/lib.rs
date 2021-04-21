// Copyright (C) 2019-2021 Calcu Network Technologies Ltd.
// This file is part of Calcu.

#![cfg_attr(not(feature = "std"), no_std)]
use sp_runtime::{
    generic,
    traits::{IdentifyAccount, Verify},
    MultiSignature,
};
use sp_std::vec::Vec;

pub mod constants;
pub mod traits;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. 
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// An index to a block.
pub type BlockNumber = u32;

/// An instant or duration in time.
pub type Moment = u64;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;

/// The IAS signature type
pub type IASSig = Vec<u8>;

/// The ISV body type, contains the enclave code and public key
pub type ISVBody = Vec<u8>;

/// tars certification type, begin with `-----BEGIN CERTIFICATE-----`
/// and end with `-----END CERTIFICATE-----`
pub type TarsCert = Vec<u8>;

/// tars public key, little-endian-format, 64 bytes vec
pub type TarsPubKey = Vec<u8>;

/// tars anchor, just use TarsPubKey right now, 64 bytes vec
pub type TarsAnchor = TarsPubKey;

/// tars signature, little-endian-format, 64 bytes vec
pub type TarsSignature = Vec<u8>;

/// tars enclave code
pub type TarsCode = Vec<u8>;

/// Work report empty workload/storage merkle root
pub type MerkleRoot = Vec<u8>;

/// File Alias for a file
pub type FileAlias = Vec<u8>;

/// Report index, always be a multiple of era number
pub type ReportSlot = u64;

/// Murphy vendor's address info
pub type AddressInfo = Vec<u8>;