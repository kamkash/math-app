#[ctor::ctor]
fn init_logger() {
    let _ = env_logger::builder()
    .filter_level(log::LevelFilter::Debug)
    .is_test(true).try_init();
    // log::info!("Logger initialized");
    // log::debug!("Debug logging enabled");
    // log::trace!("Trace logging enabled");
    // log::warn!("Warning logging enabled");
    // log::error!("Error logging enabled");
}