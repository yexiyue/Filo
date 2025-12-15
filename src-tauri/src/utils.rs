use tracing::info;
use tracing_subscriber::EnvFilter;

pub fn init_logger() {
    let env_filter = EnvFilter::builder()
        .parse("off,tauri=info,filo_lib=trace")
        .expect("parse env-filter error");
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    info!("Starting filo-lib");
}
