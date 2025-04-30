use itertools::Itertools;
use log::*;
use symengine_rs::basic::Basic; // Import Itertools for the `sorted` method

#[test]
fn test_basic_symbol() {
    let x = Basic::symbol("x");
    let y = Basic::symbol("y");
    assert!(x.is_symbol() && x.to_string() == "x");
    assert!(y.is_symbol() && y.to_string() == "y");

    let [z, t] = Basic::symbols(["z", "t"]);
    assert!(z.is_symbol() && z.to_string() == "z");
    assert!(t.is_symbol() && t.to_string() == "t");
}

#[test]
fn test_basic_symbol_add() {
    let [x, y] = Basic::symbols(["x", "y"]);
    let sum = x.add(&y);
    assert_eq!(sum.to_string(), "x + y");
}

#[test]
fn test_basic_integer_real() {
    let int = Basic::integer(42);
    let real = Basic::real(3.14);
    assert_eq!(int.to_string(), "42");
    assert_eq!(real.to_string(), "3.14");
}

#[test]
fn test_basic_mul_pow_eq() {
    let x = Basic::symbol("x");
    let y = Basic::symbol("y");
    let product = x.mul(&y);
    let power = x.pow(&Basic::integer(2));
    assert_eq!(product.to_string(), "x*y");
    assert_eq!(power.to_string(), "x**2");
    assert!(x.equals(&Basic::symbol("x")));
    assert!(!x.equals(&y));
}

#[allow(unused_variables)]
#[test]
fn test_basic_operations() {
    let x = Basic::symbol("x");
    let y = Basic::symbol("y");
    let z = Basic::symbol("z");

    let im1 = Basic::integer(-1);
    let i2 = Basic::integer(2);
    let i3 = Basic::integer(3);
    let i6 = Basic::integer(6);
    let i12 = Basic::integer(12);
    let r1 = Basic::sin(&x);
    let r2 = Basic::cos(&x);
    let r3 = z.neg();

    assert!(r1.equals(&r1));
    assert!(!r1.equals(&r2));

    assert_eq!(r1.to_string(), "sin(x)");
    assert_eq!(r2.to_string(), "cos(x)");
    assert_eq!(r3.to_string(), "-z");

    info!("r1: {}", r1);
    info!("r2: {}", r2);
    info!("r3 z.neg(): {}", r3);
    info!("z {}", z);
}

#[test]
fn test_pi_div_int() {
    let pi = Basic::pi();
    let angle = pi.div_int(6);
    assert_eq!(angle.to_string(), "(1/6)*pi");
}

#[test]
fn test_basic_min() {
    let x = Basic::symbol("x");
    let y = Basic::symbol("y");
    let z = Basic::symbol("z");

    // Test min with multiple arguments
    let minimum = Basic::min(vec![&x, &y, &z]);
    assert_eq!(minimum.to_string(), "min(x, y, z)");
    info!("Minimum of x, y, z: {}", minimum);

    // Test min with two arguments
    let minimum_two = Basic::min(vec![&x, &y]);
    assert_eq!(minimum_two.to_string(), "min(x, y)");
    info!("Minimum of x and y: {}", minimum_two);

    // evaluate
    let result = Basic::subs(
        &minimum,
        vec![
            (&x, &Basic::real(1f64)),
            (&y, &Basic::real(2f64)),
            (&z, &Basic::real(3f64)),
        ]
        .into_iter(),
    );
    info!("Eval result of min: {}", result);
    assert_eq!(result.to_string(), "1.0");
    let f: f64 = result.to_f64();
    info!("Result of min f64: {:?}", f);
}

#[test]
fn test_basic_max() {
    let x = Basic::symbol("x");
    let y = Basic::symbol("y");
    let z = Basic::symbol("z");

    // Test max with multiple arguments
    let maximum = Basic::max(vec![&x, &y, &z]);
    assert_eq!(maximum.to_string(), "max(x, y, z)");
    info!("Maximum of x, y, z: {}", maximum);

    // Test max with two arguments
    let maximum_two = Basic::max(vec![&x, &y]);
    assert_eq!(maximum_two.to_string(), "max(x, y)");
    info!("Maximum of x and y: {}", maximum_two);

    // Evaluate
    let result = Basic::subs(
        &maximum_two,
        vec![(&x, &Basic::real(10f64)), (&y, &Basic::real(30f64))].into_iter(),
    );
    info!("Eval result of max: {}", result);
    assert_eq!(result.to_string(), "30.0");
    let f: f64 = result.to_f64();
    info!("Result of max f64: {:?}", f);
    assert_eq!(f, 30.0f64);
}

#[test]
fn test_evaluation_addition() {
    let x = Basic::symbol("x");
    let y = Basic::symbol("y");
    let expr = x.add(&y);
    let result = Basic::subs(
        &expr,
        vec![(&x, &Basic::real(1f64)), (&y, &Basic::real(2f64))].into_iter(),
    );
    assert_eq!(result.to_string(), "3.0");
    info!("Result of substitution: {}", result);
    let f: f64 = result.to_f64();
    info!("Result of f64: {:?}", f);
    assert_eq!(f, 3.0f64);
}

#[test]
fn test_evaluation_compound_interest() {
    let p = Basic::symbol("p"); // principal
    let i = Basic::symbol("i"); // interest
    let n = Basic::symbol("n"); // years
    let const_one = Basic::integer(1);

    let one_plus_i = Basic::add(&const_one, &i);
    let compound_interest = p.mul(&one_plus_i.pow(&n));
    info!("Result of compound interest: {}", compound_interest);
    assert_eq!(compound_interest.to_string(), "p*(1 + i)**n");

    // evaluate
    let num_p = 100_000.00f64;
    let num_i = 0.10f64;
    let num_n = 5.00f64;

    let result = Basic::subs(
        &compound_interest,
        vec![
            (&p, &Basic::real(num_p)),
            (&i, &Basic::real(num_i)),
            (&n, &Basic::real(num_n)),
        ]
        .into_iter(),
    );
    info!("Eval result of compound interest: {:.2}", result);
    assert_eq!(result.to_f64(), (num_p * (1.0 + num_i).powf(num_n)));
}

#[test]
fn test_basic_type() {
    let x = Basic::symbol("x");
    let basic_int = Basic::integer(42);
    let basic_real = Basic::real(3.14);
    let x_plus_int = Basic::add(&basic_int, &x);
    info!(
        "Type of x, basic_int, basic_real, x_plus_int: {} {} {} {}",
        x.get_type(),
        basic_int.get_type(),
        basic_real.get_type(),
        x_plus_int.get_type()
    );
    assert_eq!(x.get_type_str(), "Symbol");
    assert_eq!(basic_int.get_type_str(), "Integer");
    assert_eq!(basic_real.get_type_str(), "Real");
    assert_eq!(x_plus_int.get_type_str(), "Add");
    assert!(x.is_symbol());
    assert!(basic_int.is_integer());
}

#[test]
fn test_univariate_polynomial_solver() {
    let x = Basic::symbol("x");

    // Create a univariate polynomial: x^2 + 2*x + 1
    let mut poly = x.sqr().add(&x.mul(&Basic::integer(2))).add(&Basic::one());
    info!("Polynomial: {}", poly);

    // Solve the polynomial for x
    let mut solutions = Basic::solve_poly(&poly, &x);
    info!("Solutions: {:?}", solutions);
    assert!(solutions.len() == 1);
    assert!(solutions.contains(&Basic::integer(-1)));

    poly = x.sqr().sub(&Basic::one());
    info!("Polynomial: {}", poly);
    solutions = Basic::solve_poly(&poly, &x);
    info!("Solutions: {:?}", solutions);
    assert!(solutions.len() == 2);
    assert!(solutions.contains(&Basic::integer(-1)));
    assert!(solutions.contains(&Basic::one()));

    poly = x.sqr().sub(&Basic::integer(9));
    info!("Polynomial: {}", poly);
    solutions = Basic::solve_poly(&poly, &x);
    info!("Solutions: {:?}", solutions);
    assert!(solutions.len() == 2);
    assert!(solutions.contains(&Basic::integer(-3)));
    assert!(solutions.contains(&Basic::integer(3)));

    poly = x
        .sqr()
        .sub(&x.mul(&Basic::integer(2)))
        .add(&Basic::integer(6));
    info!("Polynomial: {}", poly);
    solutions = Basic::solve_poly(&poly, &x);
    info!("Solutions: {:?}", solutions);
    assert!(solutions.len() == 2);
}

#[test]
fn test_basic_parse() {
    let mut b_expr: Basic = Basic::parse("x + y").expect("Failed to parse expression");
    assert_eq!(b_expr.to_string(), "x + y");
    info!("Parsed expression: {}", b_expr);

    b_expr = Basic::parse("sin(x) + cos(y)").expect("Failed to parse expression");
    assert_eq!(b_expr.to_string(), "sin(x) + cos(y)");
    info!("Parsed expression: {}", b_expr);

    let exp_str_inp = "x**2 + 2*x + 1";
    b_expr = Basic::parse(exp_str_inp).expect("Failed to parse expression");

    let sorted_input: String = exp_str_inp.chars().sorted().collect();
    let sorted_expr: String = b_expr.to_string().chars().sorted().collect();
    assert_eq!(sorted_expr, sorted_input);
    info!("Parsed expression: {}", b_expr.to_string());
}

#[test]
fn test_basic_eval_order() {
    let [x, y] = Basic::symbols(["x", "y"]);
    info!("x + y: {}", x.add(&y));
    info!("x - y: {}", x.sub(&y));
    info!("x * y: {}", x.mul(&y));
    info!("x / y: {}", x.div(&y));
}
