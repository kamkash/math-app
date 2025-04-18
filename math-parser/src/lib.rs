use log::info;

pub mod calc_evaluator;
pub mod gen_calc_parser;

pub fn echo_parser(name: &str) -> String {
    info!("parser echo {}", name);
    format!("{}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_parser() {
        let input = "test_name";
        let result = echo_parser(input);
        assert_eq!(result, input);
    }
}
