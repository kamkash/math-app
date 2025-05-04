use log::info;

#[ctor::ctor]
fn init_logger() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();
    info!("Logger initialized");
}

pub mod symengine_evaluator_tests;
pub mod string_evaluator_tests;
pub mod general_tests;
