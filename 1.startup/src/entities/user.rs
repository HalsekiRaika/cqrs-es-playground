mod id;
mod name;

pub use self::id::*;
pub use self::name::*;

use error_stack::Report;
use serde::{Deserialize, Serialize};

use crate::errors::CommandRejected;

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone)]
pub enum UserCommand {
    Create { name: String },
    ChangeName { name: String },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum UserEvent {
    Created { id: UserId, name: UserName },
    ChangedName { id: UserId, name: UserName },
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

    // 内部状態の参照を使ってコマンドからイベントを生成する
    #[tracing::instrument(skip_all, fields(id = %self.id))]
    pub async fn handle(&self, command: UserCommand) -> Result<UserEvent, Report<CommandRejected>> {
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
        
        tracing::debug!("Accepted command. published event: {:?}", ev);
        
        Ok(ev)
    }

    // 内部状態の可変参照とイベントを使って内部状態を変更する。
    #[tracing::instrument(skip_all, fields(id = %self.id))]
    pub async fn apply(&mut self, event: UserEvent) {
        tracing::debug!("Accepted event: {:?}", event);
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
