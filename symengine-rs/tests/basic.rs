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