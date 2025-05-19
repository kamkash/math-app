pub mod gen_parsers;
use log::info;

pub fn echo_parser(name: &str) -> String {
    info!("parser echo {}", name);
    format!("{}", name)
}
