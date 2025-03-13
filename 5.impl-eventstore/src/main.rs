mod entities;
mod errors;
mod markers;
mod process;
mod eventstore;
mod identifier;

use std::time::Duration;

use error_stack::{Report, ResultExt};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::process::lifecycle;
use crate::entities::user::{User, UserCommand};
use crate::errors::UnrecoverableError;
use crate::eventstore::adaptor::sqlite::SqliteEventStore;

#[tokio::main]
async fn main() -> Result<(), Report<UnrecoverableError>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::new("trace"))
        .init();
    
    // イベントストアの初期設定
    eventstore::setup(SqliteEventStore::setup("sqlite://:memory:").await);
    

    let input = UserCommand::Create {
        name: "Test User 1".to_string(),
    };

    let user = User::create(input.clone()).change_context_lazy(|| UnrecoverableError)?;

    let receptor = lifecycle::run(user).await;

    // handleとapplyはoneshotを使ったコールバックを利用しているため実行を待つことができる。
    // 所謂akkaで言えばask(?)相当の動きになるだろう
    let ev = receptor
        .handle(input)
        .await
        .change_context_lazy(|| UnrecoverableError)
        .attach_printable("Cannot accept command")?;

    receptor.apply(ev).await;

    let input = UserCommand::ChangeName {
        name: "Test User 2".to_string(),
    };

    // entrustは文字通り「コマンドを投げた後の処理は知らない」ので、この後のsleepが無ければ結果を見ることができない。
    // これもakkaで例えるならtell(!)に相当するだろう
    receptor.entrust(input).await;

    tokio::time::sleep(Duration::from_millis(100)).await;

    Ok(())
}
