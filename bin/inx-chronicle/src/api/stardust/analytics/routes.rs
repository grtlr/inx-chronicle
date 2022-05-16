// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use axum::{routing::get, Extension, Router};
use chronicle::db::MongoDb;

use super::responses::AddressAnalyticsDto;
use crate::api::{extractors::TimeRange, ApiError, ApiResult};

pub fn routes() -> Router {
    Router::new().route("/addresses", get(address_analytics))
}

async fn address_analytics(
    database: Extension<MongoDb>,
    TimeRange {
        start_timestamp,
        end_timestamp,
    }: TimeRange,
) -> ApiResult<AddressAnalyticsDto> {
    let start_milestone = database
        .find_first_milestone(start_timestamp)
        .await?
        .ok_or(ApiError::NoResults)?;
    let end_milestone = database
        .find_last_milestone(end_timestamp)
        .await?
        .ok_or(ApiError::NoResults)?;

    let res = database
        .aggregate_addresses(start_milestone, end_milestone)
        .await?
        .ok_or(ApiError::NoResults)?;

    Ok(AddressAnalyticsDto {
        total_addresses: res.total_addresses as u64,
        recv_addresses: res.recv_addresses as u64,
        send_addresses: res.send_addresses as u64,
    })
}
