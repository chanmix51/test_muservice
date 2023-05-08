use std::collections::HashMap;

use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::StdResult;

const CHANNEL_BUFFER_SIZE: usize = 100;

#[derive(Debug, Clone)]
pub struct EventMessage {
    creator: String,
    subject: String,
    message: String,
}

impl EventMessage {
    /// constructor
    pub fn new(creator: &str, subject: &str, message: &str) -> Self {
        Self {
            creator: creator.to_owned(),
            subject: subject.to_owned(),
            message: message.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct SubjectPattern {
    _pattern: String,
}

impl SubjectPattern {
    pub fn new(pattern: &str) -> Self {
        Self {
            _pattern: pattern.to_owned(),
        }
    }

    pub fn matches(&self, _subject: &str) -> bool {
        true
    }
}

type SenderPatternTuple = (SubjectPattern, Sender<EventMessage>);

struct Synapps {
    receiver: Receiver<EventMessage>,
    senders: HashMap<String, SenderPatternTuple>,
}

impl Synapps {
    pub fn new(receiver: Receiver<EventMessage>) -> Self {
        Self {
            receiver,
            senders: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, pattern: &str) -> Receiver<EventMessage> {
        let pattern = SubjectPattern::new(pattern);
        let (send, recv) = channel::<EventMessage>(CHANNEL_BUFFER_SIZE);
        self.senders.insert(name.to_string(), (pattern, send));

        recv
    }

    pub async fn run(&mut self) -> StdResult<()> {
        loop {
            let message = match self.receiver.recv().await {
                Some(m) => m,
                None => return Ok(()),
            };

            for (_name, (pattern, sender)) in self
                .senders
                .iter()
                .filter(|(name, _)| name != &&message.creator)
            {
                if pattern.matches(&message.subject) {
                    sender.send(message.clone()).await?;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
