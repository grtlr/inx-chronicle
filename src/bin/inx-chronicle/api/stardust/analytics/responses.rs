// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Range;

use bee_api_types_stardust::responses::RentStructureResponse;
use chronicle::db::collections::DistributionStat;
use serde::{Deserialize, Serialize};

use crate::api::responses::impl_success_response;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressAnalyticsResponse {
    pub total_active_addresses: String,
    pub receiving_addresses: String,
    pub sending_addresses: String,
}

impl_success_response!(AddressAnalyticsResponse);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputAnalyticsResponse {
    pub count: String,
    pub total_value: String,
}

impl_success_response!(OutputAnalyticsResponse);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockAnalyticsResponse {
    pub count: String,
}

impl_success_response!(BlockAnalyticsResponse);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageDepositAnalyticsResponse {
    pub output_count: String,
    pub storage_deposit_return_count: String,
    pub storage_deposit_return_total_value: String,
    pub total_key_bytes: String,
    pub total_data_bytes: String,
    pub total_byte_cost: String,
    pub ledger_index: u32,
    pub rent_structure: RentStructureResponse,
}

impl_success_response!(StorageDepositAnalyticsResponse);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputDiffAnalyticsResponse {
    pub created_count: String,
    pub transferred_count: String,
    pub burned_count: String,
}

impl_success_response!(OutputDiffAnalyticsResponse);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichestAddressesResponse {
    pub top: Vec<AddressStatDto>,
    pub ledger_index: u32,
}

impl_success_response!(RichestAddressesResponse);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddressStatDto {
    pub address: String,
    pub balance: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenDistributionResponse {
    pub distribution: Vec<DistributionStatDto>,
    pub ledger_index: u32,
}

impl_success_response!(TokenDistributionResponse);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DistributionStatDto {
    pub range: Range<u64>,
    pub address_count: String,
    pub total_balance: String,
}

impl From<DistributionStat> for DistributionStatDto {
    fn from(s: DistributionStat) -> Self {
        Self {
            range: 10_u64.pow(s.index)..10_u64.pow(s.index + 1),
            address_count: s.address_count.to_string(),
            total_balance: s.total_balance,
        }
    }
}