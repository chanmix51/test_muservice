use std::sync::Arc;

use tokio::sync::mpsc::{Receiver, Sender};

use crate::event::EventMessage;
use crate::services::ServiceContainer;
use crate::StdResult;

struct OpenGaliperRuntime {
    sender: Sender<EventMessage>,
    receiver: Receiver<EventMessage>,
    services: Arc<ServiceContainer>,
}

impl OpenGaliperRuntime {
    pub fn new(
        sender: Sender<EventMessage>,
        receiver: Receiver<EventMessage>,
        services: Arc<ServiceContainer>,
    ) -> Self {
        Self {
            sender,
            receiver,
            services,
        }
    }

    pub async fn run(&mut self) -> StdResult<()> {
        loop {
            let message = match self.receiver.recv().await {
                Some(m) => m,
                None => return Ok(()),
            };

            self.handle_message(&message).await?;
        }
    }

    async fn handle_message(&self, _message: &EventMessage) -> StdResult<()> {
        Ok(())
    }
}

struct GaliperRuntime {
    sender: Sender<EventMessage>,
    receiver: Receiver<EventMessage>,
    services: Arc<ServiceContainer>,
}

impl GaliperRuntime {
    pub fn new(
        sender: Sender<EventMessage>,
        receiver: Receiver<EventMessage>,
        services: Arc<ServiceContainer>,
    ) -> Self {
        Self {
            sender,
            receiver,
            services,
        }
    }

    pub async fn run(&mut self) -> StdResult<()> {
        loop {
            let message = match self.receiver.recv().await {
                Some(m) => m,
                None => return Ok(()),
            };

            self.handle_message(&message).await?;
        }
    }

    async fn handle_message(&self, _message: &EventMessage) -> StdResult<()> {
        Ok(())
    }
}
