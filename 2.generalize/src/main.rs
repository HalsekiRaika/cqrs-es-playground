pub mod errors;

pub mod entities;
pub mod handler;
pub mod markers;

use error_stack::{Report, ResultExt};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::entities::user::{User, UserCommand};
use crate::errors::UnrecoverableError;

// Eventの保存処理は省略
#[tokio::main]
async fn main() -> Result<(), Report<UnrecoverableError>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::new("trace"))
        .init();

    let input = UserCommand::Create {
        name: "Test User 1".to_string(),
    };

    let mut user = User::create(input.clone()).change_context_lazy(|| UnrecoverableError)?;

    let event = handler::handle(&user, input)
        .await
        .change_context_lazy(|| UnrecoverableError)?;

    handler::apply(&mut user, event).await;

    let input = UserCommand::ChangeName {
        name: "Test User 2".to_string(),
    };

    let event = handler::handle(&user, input)
        .await
        .change_context_lazy(|| UnrecoverableError)?;

    handler::apply(&mut user, event).await;

    Ok(())
}
