#![allow(dead_code)]

mod id;
mod name;

pub use self::id::*;
pub use self::name::*;

use async_trait::async_trait;
use error_stack::Report;
use serde::{Deserialize, Serialize};
use crate::errors::{CommandRejected, DeserializeError, SerializeError};
use crate::eventstore::process::WithPersistence;
use crate::identifier::{EntityId, ToEntityId};
use crate::process::handler::{CommandHandler, EventApplicator};
use crate::markers::{Aggregate, Command, Event};
use crate::process::Context;

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    name: UserName,
}

impl User {
    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }
}

impl Aggregate for User {
    fn id(&self) -> EntityId {
        self.id.to_entity_id()
    }
}

impl WithPersistence for User {}

#[derive(Debug, Clone)]
pub enum UserCommand {
    Create { name: String },
    ChangeName { name: String },
}

impl Command for UserCommand {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum UserEvent {
    Created { id: UserId, name: UserName },
    ChangedName { id: UserId, name: UserName },
}

impl Event for UserEvent {
    const REGISTRY_KEY: &'static str = "user-event";
    
    fn to_bytes(&self) -> Result<Vec<u8>, SerializeError> {
        Ok(serde_json::to_vec(self)?)
    }
    
    fn from_bytes(bytes: &[u8]) -> Result<Self, DeserializeError> {
        Ok(serde_json::from_slice(bytes)?)
    }
}

impl User {
    pub fn create(event: UserCommand) -> Result<Self, Report<CommandRejected>> {
        let UserCommand::Create { name } = event else {
            return Err(Report::new(CommandRejected).attach_printable("Invalid command"));
        };
        Ok(User {
            id: UserId::default(),
            name: UserName::new(name),
        })
    }
}

#[async_trait]
impl CommandHandler<UserCommand> for User {
    type Event = UserEvent;
    type Rejection = Report<CommandRejected>;

    #[tracing::instrument(skip_all, fields(id = %self.id))]
    async fn handle(&self, command: UserCommand, _: &mut Context) -> Result<Self::Event, Self::Rejection> {
        let ev = match command {
            UserCommand::Create { .. } => UserEvent::Created {
                id: self.id,
                name: self.name.clone(),
            },
            UserCommand::ChangeName { name } => {
                let name = UserName::new(name);
                UserEvent::ChangedName { id: self.id, name }
            }
        };
        tracing::debug!("Accepted command, published event: {:?}", ev);
        Ok(ev)
    }
}

#[async_trait]
impl EventApplicator<UserEvent> for User {
    #[tracing::instrument(skip_all, fields(id = %self.id))]
    async fn apply(&mut self, event: UserEvent, ctx: &mut Context) {
        self.persist(&event, ctx).await;

        tracing::debug!("Accept event: {:?}", event);

        match event {
            UserEvent::Created { .. } => {
                // Do nothing
            }
            UserEvent::ChangedName { name, .. } => {
                self.name = name;
            }
        }

        tracing::debug!("current state: {:?}", self);
    }
}
