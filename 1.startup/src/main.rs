pub mod entities;
pub mod errors;

use error_stack::{Report, ResultExt};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::entities::user::{User, UserCommand};
use crate::errors::UnrecoverableError;

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

    let event = user
        .handle(input)
        .await
        .change_context_lazy(|| UnrecoverableError)?;

    // Eventはこんな感じで保存する
    // eventstore.persist(&event).await?;

    // Snapshotに関してもこんな感じになるだろう
    // snapshot.persist(&user).await?;

    user.apply(event).await;

    let input = UserCommand::ChangeName {
        name: "Test User 2".to_string(),
    };

    let event = user
        .handle(input)
        .await
        .change_context_lazy(|| UnrecoverableError)?;

    // ちなみにイベントストアに直に保存しているが、本来ならば楽観的ロックが必要になる
    // eventstore.persist(&event).await?;

    user.apply(event).await;

    Ok(())
}
