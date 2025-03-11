pub mod errors;
pub mod handler;
pub mod markers;
pub mod lifecycle;
pub mod entities;

use error_stack::{Report, ResultExt};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::errors::UnrecoverableError;
use crate::entities::user::{User, UserCommand};

#[tokio::main]
async fn main() -> Result<(), Report<UnrecoverableError>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::new("trace"))
        .init();
    
    let input = UserCommand::Create {
        name: "Test User 1".to_string(),
    };

    let user = User::create(input.clone())
        .change_context_lazy(|| UnrecoverableError)?;

    let receptor = lifecycle::run(user).await;
    
    receptor.send(input)
        .change_context_lazy(|| UnrecoverableError)
        .attach_printable("Failed to send command")?;

    let input = UserCommand::ChangeName {
        name: "Test User 2".to_string(),
    };
    
    receptor.send(input)
        .change_context_lazy(|| UnrecoverableError)
        .attach_printable("Failed to send command")?;
    
    Ok(())
}
