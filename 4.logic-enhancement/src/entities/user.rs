#![allow(dead_code)]

mod id;
mod name;

pub use self::id::*;
pub use self::name::*;

use async_trait::async_trait;
use error_stack::Report;

use crate::errors::CommandRejected;
use crate::handler::{CommandHandler, EventApplicator};
use crate::markers::{Aggregate, Command, Event};

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

impl Aggregate for User {}

#[derive(Debug, Clone)]
pub enum UserCommand {
    Create { name: String },
    ChangeName { name: String },
}

impl Command for UserCommand {}

#[derive(Debug, Clone)]
pub enum UserEvent {
    Created { id: UserId, name: UserName },
    ChangedName { id: UserId, name: UserName },
}

impl Event for UserEvent {}

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
    async fn handle(&self, command: UserCommand) -> Result<Self::Event, Self::Rejection> {
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
    async fn apply(&mut self, event: UserEvent) {
        // ここで保存を行う。
        // これにより、集約単位でのトランザクションを確保することが実現する
        // eventstore.persist(&event).await?;

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
