use tracing::{Level, debug, info, instrument};
use tracing_subscriber::{self, FmtSubscriber};

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .without_time()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("GB Emulator starting");

    test_fn();
}

#[instrument]
fn test_fn() {
    let val = 0x12FE;

    debug!("test #{:4x}", val);
}
