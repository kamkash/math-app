use log::info;
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

#[test]
fn test_trig_functions() {

    let x = Basic::symbol("x");
    let y = Basic::symbol("y");
    let z = Basic::symbol("z");

    let im1 = Basic::integer(-1);    
    let i2 = Basic::integer(2);    
    let i3 = Basic::integer(3);    
    let i6 = Basic::integer(6);    
    let i12 = Basic::integer(12);    

    let r1 = Basic::sin(&x);
    let r2 = Basic::sin(&x);

    assert!(r1.equals(&r2));
    assert_eq!(r1.to_string(), "sin(x)");
    assert_eq!(r2.to_string(), "sin(x)");


}