// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{error::Error, sync::Arc};

use thiserror::Error;

use super::Actor;

#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum ActorError {
    #[error("Actor error: {0:?}")]
    Result(Arc<Box<dyn Error + Send + Sync>>),
    #[error("Actor panicked")]
    Panic,
    #[error("Actor aborted")]
    Aborted,
}

/// Reports if the [`Actor`] finished successfully or with an [`ActorError`].
pub type Report<A> = Result<A, ErrorReport<A>>;

/// Indicates that an [`Actor`] finished running with an error.
#[derive(Debug)]
pub struct ErrorReport<A: Actor> {
    /// The [`Actor`]'s state when it finished running.
    pub state: A,
    /// The error that occurred.
    pub error: ActorError,
}

impl<A: Actor> ErrorReport<A> {
    pub(crate) fn new(state: A, error: ActorError) -> Result<A, Self> {
        Err(Self { state, error })
    }
}
