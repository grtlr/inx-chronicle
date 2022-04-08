// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::error::Error;

use async_trait::async_trait;
use futures::StreamExt;

pub mod envelope;

mod context;
mod error;
mod handle;

pub use self::{
    context::ActorContext,
    error::{ActorError, ErrorReport, Report},
    handle::{Addr, SendError},
};

#[async_trait]
/// Actors represent a concrete task together with the state it requires.
pub trait Actor: Send + Sync + Sized + 'static {
    /// The internal state that is required by the [`Actor`] to perform it's task.
    type Data: Send + Sync;
    /// Signals potential errors that can occur during execution.
    type Error: Error + Send + Sync;

    /// Set this [`Actor`]'s name, primarily for debugging purposes.
    fn name(&self) -> String {
        std::any::type_name::<Self>().into()
    }

    /// Initialize the [`Actor`].
    async fn init(&mut self, cx: &mut ActorContext<Self>) -> Result<Self::Data, Self::Error>;

    /// Start the [`Actor`]. This should call [`Actor::run`] if the actor should process events.
    async fn start(&mut self, cx: &mut ActorContext<Self>, data: &mut Self::Data) -> Result<(), Self::Error> {
        self.run(cx, data).await
    }

    /// Run the [`Actor`]'s event loop.
    async fn run(&mut self, cx: &mut ActorContext<Self>, data: &mut Self::Data) -> Result<(), Self::Error> {
        while let Some(evt) = cx.inbox().next().await {
            // Handle the event
            evt.handle(cx, self, data).await?;
        }
        Ok(())
    }

    /// Handle any processing that needs to happen on shutdown.
    async fn shutdown(&mut self, _cx: &mut ActorContext<Self>, _data: &mut Self::Data) -> Result<(), Self::Error> {
        log::debug!("{} shutting down.", self.name());
        Ok(())
    }
}
