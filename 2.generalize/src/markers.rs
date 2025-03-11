// 集約を表すマーカートレイト
pub trait Aggregate: 'static + Sync + Send + Sized {}

// コマンドを表すマーカートレイト
pub trait Command: 'static + Sync + Send + Sized {}

// イベントを表すマーカートレイト
pub trait Event: 'static + Sync + Send + Sized {}
