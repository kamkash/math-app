
// Define a simple struct to be used in the trait and its implementations.
#[derive(Debug, PartialEq)]
pub struct Basic {
    value: f64,
}

impl Basic {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

// Define the trait that all explicit functions will implement.
pub trait ExplicitFunction {
    fn generate(&self, args: &[Basic]) -> Basic;
}

// Implement the trait for an 'Add' function.
pub struct Add;

impl ExplicitFunction for Add {
    fn generate(&self, args: &[Basic]) -> Basic {
        let sum = args.iter().map(|arg| arg.value).sum();
        Basic::new(sum)
    }
}

// Implement the trait for a 'Subtract' function.
pub struct Subtract;

impl ExplicitFunction for Subtract {
    fn generate(&self, args: &[Basic]) -> Basic {
        if args.is_empty() {
            return Basic::new(0.0);
        }
        let first = args[0].value;
        let result = args.iter().skip(1).map(|arg| arg.value).fold(first, |a, b| a - b);
        Basic::new(result)
    }
}

// Implement the trait for a 'Multiply' function.
pub struct Multiply;

impl ExplicitFunction for Multiply {
    fn generate(&self, args: &[Basic]) -> Basic {
        if args.is_empty() {
            return Basic::new(1.0);
        }
        let product = args.iter().map(|arg| arg.value).product();
        Basic::new(product)
    }
}

// The factory for creating function objects.
pub struct FunctionFactory;

impl FunctionFactory {
    pub fn create(name: &str) -> Option<Box<dyn ExplicitFunction>> {
        match name {
            "add" => Some(Box::new(Add)),
            "subtract" => Some(Box::new(Subtract)),
            "multiply" => Some(Box::new(Multiply)),
            _ => None,
        }
    }
}

// Example of how to use the factory.
fn main() {
    let factory = FunctionFactory;

    let functions_to_create = vec!["add", "subtract", "multiply", "divide"];

    for &name in &functions_to_create {
        println!("Creating function: {}", name);
        match FunctionFactory::create(name) {
            Some(func) => {
                let args = [Basic::new(10.0), Basic::new(5.0)];
                let result = func.generate(&args);
                println!("  Result: {:?}", result);
            }
            None => {
                println!("  Function '{}' not found.", name);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let add = Add;
        let args = [Basic::new(1.0), Basic::new(2.0), Basic::new(3.0)];
        assert_eq!(add.generate(&args), Basic::new(6.0));
    }

    #[test]
    fn test_subtract() {
        let subtract = Subtract;
        let args = [Basic::new(10.0), Basic::new(2.0), Basic::new(3.0)];
        assert_eq!(subtract.generate(&args), Basic::new(5.0));
    }

    #[test]
    fn test_multiply() {
        let multiply = Multiply;
        let args = [Basic::new(2.0), Basic::new(3.0), Basic::new(4.0)];
        assert_eq!(multiply.generate(&args), Basic::new(24.0));
    }

    #[test]
    fn test_factory() {
        let add = FunctionFactory::create("add").unwrap();
        let args = [Basic::new(1.0), Basic::new(2.0)];
        assert_eq!(add.generate(&args), Basic::new(3.0));

        let subtract = FunctionFactory::create("subtract").unwrap();
        let args = [Basic::new(10.0), Basic::new(5.0)];
        assert_eq!(subtract.generate(&args), Basic::new(5.0));

        let multiply = FunctionFactory::create("multiply").unwrap();
        let args = [Basic::new(2.0), Basic::new(3.0)];
        assert_eq!(multiply.generate(&args), Basic::new(6.0));

        assert!(FunctionFactory::create("divide").is_none());
    }
}
