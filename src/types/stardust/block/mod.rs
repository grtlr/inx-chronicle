// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod address;
mod input;
mod output;
mod payload;
mod signature;
mod unlock;

use std::str::FromStr;

use bee_block_stardust as bee;
use serde::{Deserialize, Serialize};

pub use self::{address::*, input::*, output::*, payload::*, signature::*, unlock::*};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Hash, Ord, PartialOrd, Eq)]
#[serde(transparent)]
pub struct BlockId(#[serde(with = "serde_bytes")] pub Box<[u8]>);

impl BlockId {
    pub fn to_hex(&self) -> String {
        prefix_hex::encode(self.0.as_ref())
    }
}

impl From<bee::BlockId> for BlockId {
    fn from(value: bee::BlockId) -> Self {
        Self(value.to_vec().into_boxed_slice())
    }
}

impl TryFrom<BlockId> for bee::BlockId {
    type Error = crate::types::error::Error;

    fn try_from(value: BlockId) -> Result<Self, Self::Error> {
        Ok(bee::BlockId::new(value.0.as_ref().try_into()?))
    }
}

impl FromStr for BlockId {
    type Err = crate::types::error::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(bee::BlockId::from_str(s)?.into())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub id: BlockId,
    pub protocol_version: u8,
    pub parents: Box<[BlockId]>,
    pub payload: Option<Payload>,
    #[serde(with = "crate::types::stringify")]
    pub nonce: u64,
}

impl From<bee::Block> for Block {
    fn from(value: bee::Block) -> Self {
        Self {
            id: value.id().into(),
            protocol_version: value.protocol_version(),
            parents: value.parents().iter().map(|id| BlockId::from(*id)).collect(),
            payload: value.payload().map(Into::into),
            nonce: value.nonce(),
        }
    }
}

impl TryFrom<Block> for bee::Block {
    type Error = crate::types::error::Error;

    fn try_from(value: Block) -> Result<Self, Self::Error> {
        let mut builder = bee::BlockBuilder::<u64>::new(bee::parent::Parents::new(
            Vec::from(value.parents)
                .into_iter()
                .map(|p| p.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        )?)
        .with_nonce_provider(value.nonce, 0.0);
        if let Some(payload) = value.payload {
            builder = builder.with_payload(payload.try_into()?)
        }
        Ok(builder.finish()?)
    }
}

#[cfg(test)]
mod tests {
    use mongodb::bson::{from_bson, to_bson};

    use super::{
        payload::test::{get_test_milestone_payload, get_test_tagged_data_payload, get_test_transaction_payload},
        *,
    };

    #[test]
    fn test_block_id_bson() {
        let block_id = BlockId::from(bee_test::rand::block::rand_block_id());
        let bson = to_bson(&block_id).unwrap();
        from_bson::<BlockId>(bson).unwrap();
    }

    #[test]
    fn test_block_bson() {
        let block = get_test_transaction_block();
        let bson = to_bson(&block).unwrap();
        assert_eq!(block, from_bson::<Block>(bson).unwrap());

        let block = get_test_milestone_block();
        let bson = to_bson(&block).unwrap();
        assert_eq!(block, from_bson::<Block>(bson).unwrap());

        let block = get_test_tagged_data_block();
        let bson = to_bson(&block).unwrap();
        assert_eq!(block, from_bson::<Block>(bson).unwrap());
    }

    fn get_test_transaction_block() -> Block {
        Block::from(
            bee::BlockBuilder::<u64>::new(bee_test::rand::parents::rand_parents())
                .with_nonce_provider(u64::MAX, 0.0)
                .with_payload(get_test_transaction_payload().try_into().unwrap())
                .finish()
                .unwrap(),
        )
    }

    fn get_test_milestone_block() -> Block {
        Block::from(
            bee::BlockBuilder::<u64>::new(bee_test::rand::parents::rand_parents())
                .with_nonce_provider(u64::MAX, 0.0)
                .with_payload(get_test_milestone_payload().try_into().unwrap())
                .finish()
                .unwrap(),
        )
    }

    fn get_test_tagged_data_block() -> Block {
        Block::from(
            bee::BlockBuilder::<u64>::new(bee_test::rand::parents::rand_parents())
                .with_nonce_provider(u64::MAX, 0.0)
                .with_payload(get_test_tagged_data_payload().try_into().unwrap())
                .finish()
                .unwrap(),
        )
    }
}
