use itertools::Itertools;
use log::*;
use symengine_rs::basic::Basic; // Import Itertools for the `sorted` method
use test_log::test;

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
    let [x, y, z] = Basic::symbols(["x", "y", "z"]);
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
    let [x, y, z] = Basic::symbols(["x", "y", "z"]);

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
        &[
            (&x, &Basic::real(1f64)),
            (&y, &Basic::real(2f64)),
            (&z, &Basic::real(3f64)),
        ],
    );
    info!("Eval result of min: {}", result);
    assert_eq!(result.to_string(), "1.0");
    let f: f64 = result.to_f64().unwrap();
    info!("Result of min f64: {:?}", f);
}

#[test]
fn test_basic_max() {
    let [x, y, z] = Basic::symbols(["x", "y", "z"]);

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
        &[(&x, &Basic::real(10f64)), (&y, &Basic::real(30f64))],
    );
    info!("Eval result of max: {}", result);
    assert_eq!(result.to_string(), "30.0");
    let f: f64 = result.to_f64().unwrap();
    info!("Result of max f64: {:?}", f);
    assert_eq!(f, 30.0f64);
}

#[test]
fn test_evaluation_addition() {
    let [x, y] = Basic::symbols(["x", "y"]);
    let expr = x.add(&y);
    let result = Basic::subs(
        &expr,
        &mut vec![(&x, &Basic::real(1f64)), (&y, &Basic::real(2f64))],
    );
    assert_eq!(result.to_string(), "3.0");
    info!("Result of substitution: {}", result);
    let f: f64 = result.to_f64().unwrap();
    info!("Result of f64: {:?}", f);
    assert_eq!(f, 3.0f64);
}

#[test_log::test]
fn test_evaluation_constants() {
    let [x, y] = Basic::symbols(["x", "y"]);
    let pi_expr = Basic::pi();
    let e_expr = Basic::e();
    let expr = x.mul(&pi_expr).add(&y);
    let expr1 = x.mul(&e_expr).add(&y);
    let result = Basic::subs(
        &expr,
        &[(&x, &Basic::real(2.0f64)), (&y, &Basic::real(0.5f64))],
    );
    let result1 = Basic::subs(
        &expr1,
        &[(&x, &Basic::real(2.0f64)), (&y, &Basic::real(0.5f64))],
    );
    info!("Pi expression: {:?}", pi_expr);
    info!("Pi value: {:?}", pi_expr.evalf(1, true));
    info!("e value: {:?}", e_expr.evalf(1, true));
    info!("result expression: {:?}", result);
    info!("result1 expression: {:?}", result1);
    info!("Result f64: {:?}", result.evalf(1, true));
    info!("Result1 f64: {:?}", result1.evalf(1, true));
}

#[test]
fn test_evaluation_compound_interest() {
    let [p, i, n] = Basic::symbols(["p", "i", "n"]);
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
        &[
            (&p, &Basic::real(num_p)),
            (&i, &Basic::real(num_i)),
            (&n, &Basic::real(num_n)),
        ],
    );
    info!("Eval result of compound interest: {:.2}", result);
    assert_eq!(result.to_f64().unwrap(), num_p * (1.0 + num_i).powf(num_n));
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
    let x = 10.0f64;
    let fx = x.powf(3.0) + x.powf(2.0) / 3.0 - 9.0 * x + 21.0;
    let input = "x^3 + x^2 / 3 - 9 * x + 21";

    let [symx] = Basic::symbols(["x"]);
    let exp = Basic::parse(input).expect("Failed to parse expression");
    let result = Basic::subs(&exp, &[(&symx, &Basic::real(x))]);
    info!("eval order exp = {}", result.to_string());
    assert_eq!(result.to_f64().unwrap(), fx);
}

#[test]
fn test_basic_funcs() {
    let [angle, x] = Basic::symbols(["angle", "x"]);
    let pi_over_4 = Basic::real(std::f64::consts::FRAC_PI_4);
    let x_val = Basic::real(100.0);
    let value_table = vec![(&angle, &pi_over_4), (&x, &x_val)];
    let bsin = Basic::sin(&angle);
    let ln = Basic::ln(&x);
    let basic_log = Basic::ln(&x);
    let log10 = Basic::log10(&x);
    let log2 = Basic::logb(&x, 2);

    let tolerance = 1e-9; // Adjusted tolerance for f64 comparisons
    let mut result = Basic::subs(&bsin, &value_table);
    assert!((result.to_f64().unwrap() - std::f64::consts::FRAC_PI_4.sin()).abs() < tolerance);
    result = Basic::subs(&ln, &value_table);
    assert!((result.to_f64().unwrap() - x_val.to_f64().unwrap().ln()).abs() < tolerance);
    result = Basic::subs(&basic_log, &value_table);
    assert!((result.to_f64().unwrap() - x_val.to_f64().unwrap().ln()).abs() < tolerance);
    result = Basic::subs(&log10, &value_table);
    assert!((result.to_f64().unwrap() - x_val.to_f64().unwrap().log10()).abs() < tolerance);
    result = Basic::subs(&log2, &value_table);
    assert!((result.to_f64().unwrap() - x_val.to_f64().unwrap().log2()).abs() < tolerance);
}

#[test]
fn test_basic_differentiation() {
    let [x] = Basic::symbols(["x"]);
    let expr = x.pow(&Basic::integer(3)).add(&x.mul(&Basic::integer(2)));
    let derivative = Basic::diff(&expr, &x);
    info!("Derivative of {}: {}", expr, derivative);
    // assert_eq!(derivative.to_string(), "3*x**2 + 2");
    assert_eq!(derivative, Basic::parse("3*x**2 + 2").unwrap());

    let trig_expr = Basic::sin(&x).add(&Basic::cos(&x));
    dbg!(trig_expr.to_string());
    let trig_derivative = Basic::diff(&trig_expr, &x);
    info!("Derivative of {}: {}", trig_expr, trig_derivative);
    assert_eq!(trig_derivative, Basic::parse("cos(x) - sin(x)").unwrap());
}

#[test]
fn test_basic_mul_add_ops() {
    let mul_expr = Basic::mul_op();
    let div_expr = Basic::div_op();
    let add_expr = Basic::add_op();
    let sub_expr = Basic::sub_op();
    let pow_exp = Basic::pow_op();

    info!(
        "Multiplication: {} {} {}",
        mul_expr,
        mul_expr.get_type(),
        mul_expr.is_mul_op()
    );
    info!(
        "Division: {} {} {}",
        div_expr,
        div_expr.get_type(),
        div_expr.is_div_op()
    );
    info!(
        "Addition: {} {} {}",
        add_expr,
        add_expr.get_type(),
        add_expr.is_add_op()
    );
    info!(
        "Subtraction: {} {} {}",
        sub_expr,
        sub_expr.get_type(),
        sub_expr.is_sub_op()
    );

    assert!(mul_expr.is_mul_op());
    assert!(div_expr.is_div_op());
    assert!(add_expr.is_add_op());
    assert!(sub_expr.is_sub_op());
    assert!(pow_exp.is_pow_op());
    assert!(!sub_expr.is_add_op());
    assert!(!add_expr.is_sub_op());
    assert!(!mul_expr.is_div_op());
    assert!(!div_expr.is_mul_op());
}

#[test]
fn test_basic_functions() {
    let [x] = Basic::symbols(["x"]);
    assert_eq!(x.to_string(), "x");

    let sin_sym = Basic::sin_func_sym();
    dbg!(&sin_sym);
    dbg!(sin_sym.to_string());
    dbg!(sin_sym.get_type());
    dbg!(Basic::is_function(&sin_sym));

    let logb = Basic::logb_func_sym(2);
    dbg!(&logb);
    dbg!(logb.to_string());
    dbg!(logb.get_type());
    dbg!(Basic::is_function(&logb));
    
    assert!(Basic::is_function(&logb));
}



