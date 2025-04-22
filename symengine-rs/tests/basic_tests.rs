use log::*;
use symengine_rs::basic::Basic;

#[test]
fn test_basic_symbol_add() {
    let x = Basic::symbol("x");
    let y = Basic::symbol("y");
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
        vec![
            (&x, &Basic::real(10f64)),
            (&y, &Basic::real(30f64)),
        ]
        .into_iter(),
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
