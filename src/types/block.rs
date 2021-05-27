use crate::types::{Bytes, H160, H2048, H256, H64, U256, U64};
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

use sp_std::{
    convert::TryFrom,
    fmt::Debug,
};

use sp_runtime::traits::{Hash as HashT};

/// Use type from Polkadot
// use sp_runtime::generic::{Block as SubstrateBlock};

/// The block header type returned from RPC calls.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct BlockHeader {
    /// Hash of the block
    pub hash: Option<H256>,
    /// Hash of the parent
    #[serde(rename = "parentHash")]
    pub parent_hash: H256,
    /// Hash of the uncles
    #[serde(rename = "sha3Uncles")]
    #[serde(default)] // Celo doesn't have this field.
    pub uncles_hash: H256,
    /// Miner/author's address.
    #[serde(rename = "miner")]
    pub author: H160,
    /// State root hash
    #[serde(rename = "stateRoot")]
    pub state_root: H256,
    /// Transactions root hash
    #[serde(rename = "transactionsRoot")]
    pub transactions_root: H256,
    /// Transactions receipts root hash
    #[serde(rename = "receiptsRoot")]
    pub receipts_root: H256,
    /// Block number. None if pending.
    pub number: Option<U64>,
    /// Gas Used
    #[serde(rename = "gasUsed")]
    pub gas_used: U256,
    /// Gas Limit
    #[serde(rename = "gasLimit")]
    #[serde(default)] // Celo doesn't have this field.
    pub gas_limit: U256,
    /// Extra data
    #[serde(rename = "extraData")]
    pub extra_data: Bytes,
    /// Logs bloom
    #[serde(rename = "logsBloom")]
    pub logs_bloom: H2048,
    /// Timestamp
    pub timestamp: U256,
    /// Difficulty
    #[serde(default)] // Celo doesn't have this field.
    pub difficulty: U256,
    /// Mix Hash
    #[serde(rename = "mixHash")]
    pub mix_hash: Option<H256>,
    /// Nonce
    pub nonce: Option<H64>,
}

// pub struct Header<Number: Copy + Into<U256> + TryFrom<U256>, Hash: HashT> {
//     /// The parent hash.
//     pub parent_hash: Hash::Output,
//     /// The block number.
//     pub number: Number,
//     /// The state trie merkle root
//     pub state_root: Hash::Output,
//     /// The merkle root of the extrinsics.
//     pub extrinsics_root: Hash::Output,
//     // /// A chain-specific digest of data useful for light clients or referencing auxiliary data.
//     //  pub digest: Digest<Hash::Output>,
// }


/// Abstraction over a substrate block.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Block<Header, Extrinsic> {
    /// The block header.
    pub header: Header,
    /// The accompanying extrinsics.
    pub extrinsics: Vec<Extrinsic>,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Header<Number: Copy + Into<U256> + TryFrom<U256>, Hash: HashT> {
    /// The parent hash.
    pub parent_hash: Hash::Output,
    /// The block number.
    #[cfg_attr(feature = "std", serde(
    serialize_with = "serialize_number",
    deserialize_with = "deserialize_number"))]
    pub number: Number,
    /// The state trie merkle root
    pub state_root: Hash::Output,
    /// The merkle root of the extrinsics.
    pub extrinsics_root: Hash::Output,
    //////A chain-specific digest of data useful for light clients or referencing auxiliary data.
    // pub digest: Digest<Hash::Output>,
}


// // The block type returned from RPC calls.
// // This is generic over a `TX` type.
// #[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
// pub struct Block<TX> {
//     /// Hash of the block
//     pub hash: Option<H256>,
//     /// Hash of the parent
//     #[serde(rename = "parentHash")]
//     pub parent_hash: H256,
//     /// Hash of the uncles
//     #[serde(rename = "sha3Uncles")]
//     #[serde(default)] // Celo doesn't have this field.
//     pub uncles_hash: H256,
//     /// Miner/author's address.
//     #[serde(rename = "miner")]
//     pub author: H160,
//     /// State root hash
//     #[serde(rename = "stateRoot")]
//     pub state_root: H256,
//     /// Transactions root hash
//     #[serde(rename = "transactionsRoot")]
//     pub transactions_root: H256,
//     /// Transactions receipts root hash
//     #[serde(rename = "receiptsRoot")]
//     pub receipts_root: H256,
//     /// Block number. None if pending.
//     pub number: Option<U64>,
//     /// Gas Used
//     #[serde(rename = "gasUsed")]
//     pub gas_used: U256,
//     /// Gas Limit
//     #[serde(rename = "gasLimit")]
//     #[serde(default)] // Celo doesn't have this field.
//     pub gas_limit: U256,
//     /// Extra data
//     #[serde(rename = "extraData")]
//     pub extra_data: Bytes,
//     /// Logs bloom
//     #[serde(rename = "logsBloom")]
//     pub logs_bloom: Option<H2048>,
//     /// Timestamp
//     pub timestamp: U256,
//     /// Difficulty
//     #[serde(default)] // Celo doesn't have this field.
//     pub difficulty: U256,
//     /// Total difficulty
//     #[serde(rename = "totalDifficulty")]
//     pub total_difficulty: Option<U256>,
//     /// Seal fields
//     #[serde(default, rename = "sealFields")]
//     pub seal_fields: Vec<Bytes>,
//     /// Uncles' hashes
//     #[serde(default)] // Celo doesn't have this field.
//     pub uncles: Vec<H256>,
//     /// Transactions
//     pub transactions: Vec<TX>,
//     /// Size in bytes
//     pub size: Option<U256>,
//     /// Mix Hash
//     #[serde(rename = "mixHash")]
//     pub mix_hash: Option<H256>,
//     /// Nonce
//     pub nonce: Option<H64>,
// }

/// Block Number
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlockNumber {
    /// Latest block
    Latest,
    /// Earliest block (genesis)
    Earliest,
    /// Pending block (not yet part of the blockchain)
    Pending,
    /// Block by number from canon chain
    Number(U64),
}

impl<T: Into<U64>> From<T> for BlockNumber {
    fn from(num: T) -> Self {
        BlockNumber::Number(num.into())
    }
}

impl Serialize for BlockNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            BlockNumber::Number(ref x) => serializer.serialize_str(&format!("0x{:x}", x)),
            BlockNumber::Latest => serializer.serialize_str("latest"),
            BlockNumber::Earliest => serializer.serialize_str("earliest"),
            BlockNumber::Pending => serializer.serialize_str("pending"),
        }
    }
}

/// Block Identifier
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlockId {
    /// By Hash
    Hash(H256),
    /// By Number
    Number(BlockNumber),
}

impl Serialize for BlockId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            BlockId::Hash(ref x) => {
                let mut s = serializer.serialize_struct("BlockIdEip1898", 1)?;
                s.serialize_field("blockHash", &format!("{:?}", x))?;
                s.end()
            }
            BlockId::Number(ref num) => num.serialize(serializer),
        }
    }
}

impl From<U64> for BlockId {
    fn from(num: U64) -> Self {
        BlockNumber::Number(num).into()
    }
}

impl From<BlockNumber> for BlockId {
    fn from(num: BlockNumber) -> Self {
        BlockId::Number(num)
    }
}

impl From<H256> for BlockId {
    fn from(hash: H256) -> Self {
        BlockId::Hash(hash)
    }
}


// #[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
// pub struct InternalBlockMassbit {
//     pub extrinsics: Vec<String>,
// }
//
//
// /// MASSBIT SUBSTRATE The block type returned from RPC calls
// /// This is generic over a `TX` type.
// #[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
// pub struct SubstrateBlock<> {
//     pub block: blockHeader,
//     pub justification: Option<String>,
// }
//
//
// #[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
// pub struct SubstrateVersion<> {
//     // Parse spec Version, not sure if this is the same as chain ID
//     #[serde(rename = "specVersion")]
//     pub spec_version: u32,
// }
