// src/lib.rs

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub mod utils {
    pub fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn greeting_test() {
        assert_eq!(utils::greet("World"), "Hello, World!");
    }
}
