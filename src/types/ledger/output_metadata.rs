// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::types::{
    stardust::{
        block::{
            output::{Output, OutputId},
            payload::transaction::TransactionId,
            BlockId,
        },
        milestone::MilestoneTimestamp,
    },
    tangle::MilestoneIndex,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MilestoneIndexTimestamp {
    pub milestone_index: MilestoneIndex,
    pub milestone_timestamp: MilestoneTimestamp,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SpentMetadata {
    pub transaction_id: TransactionId,
    pub spent: MilestoneIndexTimestamp,
}

/// Block metadata.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OutputMetadata {
    pub block_id: BlockId,
    pub booked: MilestoneIndexTimestamp,
    pub spent_metadata: Option<SpentMetadata>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LedgerOutput {
    pub output_id: OutputId,
    pub block_id: BlockId,
    pub booked: MilestoneIndexTimestamp,
    pub output: Output,
    pub rent_structure: RentStructureBytes,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LedgerSpent {
    pub output: LedgerOutput,
    pub spent_metadata: SpentMetadata,
}

#[cfg(feature = "inx")]
// TODO: Write unit test cases for this function for each of the output types.
fn compute_rent_structure(output: &bee_block_stardust::output::Output) -> RentStructureBytes {
    use bee_block_stardust::output::{Rent, RentStructureBuilder};

    let rent_cost = |byte_cost, data_factor, key_factor| {
        output.rent_cost(
            &RentStructureBuilder::new()
                .byte_cost(byte_cost)
                .data_factor(data_factor)
                .key_factor(key_factor)
                .finish(),
        )
    };

    RentStructureBytes {
        num_data_bytes: rent_cost(1, 1, 0),
        num_key_bytes: rent_cost(1, 0, 1),
    }
}

#[cfg(feature = "inx")]
impl TryFrom<bee_inx::LedgerOutput> for LedgerOutput {
    type Error = bee_inx::Error;

    fn try_from(value: bee_inx::LedgerOutput) -> Result<Self, Self::Error> {
        let bee_output = value.output.inner_unverified()?;

        Ok(Self {
            rent_structure: compute_rent_structure(&bee_output),
            output: Into::into(&bee_output),
            output_id: value.output_id.into(),
            block_id: value.block_id.into(),
            booked: MilestoneIndexTimestamp {
                milestone_index: value.milestone_index_booked.into(),
                milestone_timestamp: value.milestone_timestamp_booked.into(),
            },
        })
    }
}

#[cfg(feature = "inx")]
impl crate::types::context::TryFromWithContext<bee_inx::LedgerOutput> for LedgerOutput {
    type Error = bee_inx::Error;

    fn try_from_with_context(
        ctx: &bee_block_stardust::protocol::ProtocolParameters,
        value: bee_inx::LedgerOutput,
    ) -> Result<Self, Self::Error> {
        let bee_output = value.output.inner(ctx)?;

        Ok(Self {
            rent_structure: compute_rent_structure(&bee_output),
            output: Into::into(&bee_output),
            output_id: value.output_id.into(),
            block_id: value.block_id.into(),
            booked: MilestoneIndexTimestamp {
                milestone_index: value.milestone_index_booked.into(),
                milestone_timestamp: value.milestone_timestamp_booked.into(),
            },
        })
    }
}

#[cfg(feature = "inx")]
impl TryFrom<bee_inx::LedgerSpent> for LedgerSpent {
    type Error = bee_inx::Error;

    fn try_from(value: bee_inx::LedgerSpent) -> Result<Self, Self::Error> {
        let output = LedgerOutput::try_from(value.output)?;

        Ok(Self {
            output,
            spent_metadata: SpentMetadata {
                transaction_id: value.transaction_id_spent.into(),
                spent: MilestoneIndexTimestamp {
                    milestone_index: value.milestone_index_spent.into(),
                    milestone_timestamp: value.milestone_timestamp_spent.into(),
                },
            },
        })
    }
}

#[cfg(feature = "inx")]
impl crate::types::context::TryFromWithContext<bee_inx::LedgerSpent> for LedgerSpent {
    type Error = bee_inx::Error;

    fn try_from_with_context(
        ctx: &bee_block_stardust::protocol::ProtocolParameters,
        value: bee_inx::LedgerSpent,
    ) -> Result<Self, Self::Error> {
        let output = LedgerOutput::try_from_with_context(ctx, value.output)?;

        Ok(Self {
            output,
            spent_metadata: SpentMetadata {
                transaction_id: value.transaction_id_spent.into(),
                spent: MilestoneIndexTimestamp {
                    milestone_index: value.milestone_index_spent.into(),
                    milestone_timestamp: value.milestone_timestamp_spent.into(),
                },
            },
        })
    }
}

/// The different number of bytes that are used for computing the rent cost.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RentStructureBytes {
    /// The number of key bytes in an output.
    pub num_key_bytes: u64,
    /// The number of data bytes in an output.
    pub num_data_bytes: u64,
}
