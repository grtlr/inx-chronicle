// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use axum::response::IntoResponse;
use chronicle::db::model::{inclusion_state::LedgerInclusionState, sync::SyncData};
use derive_more::From;
use serde::{Deserialize, Serialize};

macro_rules! impl_success_response {
    ($($type:ty),*) => {
        $(
            impl IntoResponse for $type {
                fn into_response(self) -> axum::response::Response {
                    SuccessBody::from(self).into_response()
                }
            }
        )*
    };
}

pub(crate) use impl_success_response;
use serde_json::Value;

/// Response of `GET /api/<api_version>/info`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InfoResponse {
    pub name: String,
    pub version: String,
    #[serde(rename = "isHealthy")]
    pub is_healthy: bool,
}

impl IntoResponse for InfoResponse {
    fn into_response(self) -> axum::response::Response {
        SuccessBody::from(self).into_response()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncDataResponse(pub SyncData);

impl IntoResponse for SyncDataResponse {
    fn into_response(self) -> axum::response::Response {
        SuccessBody::from(self).into_response()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, From)]
#[serde(untagged)]
pub enum Expansion {
    Simple(String),
    Expanded(Record),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    #[serde(rename = "inclusionState")]
    pub inclusion_state: Option<LedgerInclusionState>,
    #[serde(rename = "milestoneIndex")]
    pub milestone_index: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transfer {
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "outputIndex")]
    pub output_index: u16,
    #[serde(rename = "isSpending")]
    pub is_spending: bool,
    #[serde(rename = "inclusionState")]
    pub inclusion_state: Option<LedgerInclusionState>,
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub amount: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaybeSpentOutput {
    pub output: Value,
    #[serde(rename = "spendingMessageId")]
    pub spending_message_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Unlock {
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub block: Value,
}

/// A success wrapper for API responses.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuccessBody<T> {
    data: T,
}

impl<T> Deref for SuccessBody<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> SuccessBody<T> {
    /// Create a new [`SuccessBody`] from any inner type.
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> From<T> for SuccessBody<T> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

impl<T: Serialize> IntoResponse for SuccessBody<T> {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}