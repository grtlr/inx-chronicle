// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use tokio_stream::wrappers::UnboundedReceiverStream;

use super::{
    envelope::{Envelope, HandleEvent},
    error::Report,
    handle::Addr,
    Actor,
};
use crate::runtime::{error::RuntimeError, scope::RuntimeScope, shutdown::ShutdownStream};

/// The context of an [`Actor`], consiting of the [`RuntimeScope`] and the [`Actor`]'s communication channels.
pub struct ActorContext<A: Actor> {
    pub(crate) scope: RuntimeScope,
    pub(crate) handle: Addr<A>,
    pub(crate) receiver: ShutdownStream<UnboundedReceiverStream<Envelope<A>>>,
}

impl<A: Actor> ActorContext<A> {
    pub(crate) fn new(
        scope: RuntimeScope,
        handle: Addr<A>,
        receiver: ShutdownStream<UnboundedReceiverStream<Envelope<A>>>,
    ) -> Self {
        Self {
            handle,
            scope,
            receiver,
        }
    }

    /// Spawns a new supervised child actor.
    pub async fn spawn_actor_supervised<OtherA>(&mut self, actor: OtherA) -> Result<Addr<OtherA>, RuntimeError>
    where
        OtherA: 'static + Actor + Debug + Send + Sync,
        A: 'static + Send + HandleEvent<Report<OtherA>>,
    {
        let handle = self.handle().clone();
        self.scope.spawn_actor_supervised(actor, handle).await
    }

    /// Get this actors's handle
    pub fn handle(&self) -> &Addr<A> {
        &self.handle
    }

    /// Gets the inbox
    pub fn inbox(&mut self) -> &mut ShutdownStream<UnboundedReceiverStream<Envelope<A>>> {
        &mut self.receiver
    }

    /// Shutdown the actor
    pub async fn shutdown(&self) {
        self.handle().shutdown().await;
    }
}

impl<A: Actor> Deref for ActorContext<A> {
    type Target = RuntimeScope;

    fn deref(&self) -> &Self::Target {
        &self.scope
    }
}

impl<A: Actor> DerefMut for ActorContext<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.scope
    }
}
