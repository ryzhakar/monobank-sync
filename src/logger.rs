use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn initialize_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting trace logger failed");
}
