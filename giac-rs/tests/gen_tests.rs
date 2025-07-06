// Unit tests for giac-rs/src/gen.rs
use giac_rs::context::Context;
use giac_rs::gen::Gen;
use log::info;

#[test_log::test]
fn test_gen_new_and_to_string() {
    let ctx = Context::new();
    let gen = Gen::new("x^2", &ctx).expect("Failed to create Gen");
    let s = gen.to_string();
    info!("Generated string: {}", s);
    assert!(s.contains("x") && s.contains("2"));
}

#[test_log::test]
fn test_gen_simplify() {
    let ctx = Context::new();
    let gen = Gen::new("2*x + 3*x", &ctx).expect("Failed to create Gen");
    let simplified = gen.simplify().expect("Failed to simplify");
    let s = simplified.to_string();
    info!("Simplified string: {}", s);
    assert!(s.contains("5*x") || s.contains("x*5") || s.contains("x5"));
}

#[test_log::test]
fn test_gen_diff() {
    let ctx = Context::new();
    let gen = Gen::new("x^3", &ctx).expect("Failed to create Gen");
    let diffed = gen.diff("x").expect("Failed to diff");
    let s = diffed.to_string();
    info!("Differentiated string: {}", s);
    assert!(s.contains("3*x^2") || s.contains("x^2*3") || s.contains("3*x*x"));
}

#[test_log::test]
fn test_gen_integrate() {
    let ctx = Context::new();
    let gen = Gen::new("x^4", &ctx).expect("Failed to create Gen");
    let integrated = gen.integrate().expect("Failed to integrate");
    let s = integrated.to_string();
    info!("Integrated string: {}", s);
    assert!(s.contains("x^5") || s.contains("1/5*x^5") || s.contains("x*x/2"));
}

#[test_log::test]
fn test_gen_arithmetic_operators() {
    let ctx = Context::new();
    let a = Gen::new("2", &ctx).unwrap();
    let b = Gen::new("3", &ctx).unwrap();
    let sum = a.add(&b).unwrap().to_string();
    let diff = a.sub(&b).unwrap().to_string();
    let prod = a.mul(&b).unwrap().to_string();
    let quot = a.div(&b).unwrap().to_string();
    let pow = a.pow(&b).unwrap().to_string();
    info!(
        "sum: {} diff: {} prod: {} quot: {} pow: {}",
        sum, diff, prod, quot, pow
    );
    assert!(sum.contains("5"));
    assert!(diff.contains("-1") || diff.contains("1-"));
    assert!(prod.contains("6"));
    assert!(quot.contains("2/3") || quot.contains("0.666") || quot.contains("0,666"));
    assert!(pow.contains("8"));
}

#[test_log::test]
fn test_gen_symbolic_constants() {
    let ctx = Context::new();
    let pi = Gen::pi(&ctx).unwrap().to_string();
    let e = Gen::e(&ctx).unwrap().to_string();
    info!("pi: {} e: {}", pi, e);
    assert!(pi.to_lowercase().contains("pi"));
    // assert!(e.to_lowercase().contains("e") || e.to_lowercase().contains("euler"));
}

#[test_log::test]
fn test_gen_symbolic_operators() {
    let ctx = Context::new();
    let x = Gen::new("x", &ctx).unwrap();
    let y = Gen::new("y", &ctx).unwrap();
    let plus = x.symb_plus(&y).unwrap().to_string();
    let mult = x.symb_mult(&y).unwrap().to_string();
    let pow = x.symb_pow(&y).unwrap().to_string();
    info!("plus: {} mult: {} pow: {}", plus, mult, pow);
    assert!(
        plus.contains("x") && plus.contains("y") && (plus.contains("+") || plus.contains("plus"))
    );
    assert!(
        mult.contains("x") && mult.contains("y") && (mult.contains("*") || mult.contains("mult"))
    );
    assert!(pow.contains("x") && pow.contains("y") && (pow.contains("^") || pow.contains("pow")));
}

#[test_log::test]
fn test_gen_subs() {
    let ctx = Context::new();
    let expr = Gen::new("x+y", &ctx).unwrap();
    let x_val = Gen::from_f64(2.0, &ctx).unwrap();
    let y_val = Gen::from_f64(3.0, &ctx).unwrap();
    let subs = expr
        .subs(&["x", "y"], &[&x_val, &y_val])
        .expect("subs failed");
    let s = subs.to_string();
    info!("Substituted: {}", s);
    let evaluated = subs.eval().expect("eval failed");
    info!("Evaluated: {}", evaluated.to_string()); // Should be "5"
    assert!(s.contains("5") || s.contains("2+3") || s.contains("3+2"));
}

#[test_log::test]
fn test_gen_eval() {
    let ctx = Context::new();
    let expr = Gen::new("2 + 3", &ctx).unwrap();
    let evaluated = expr.eval().expect("eval failed");
    let s = evaluated.to_string();
    info!("Evaluated: {}", s);
    assert!(s == "5" || s.contains("5"));

    let exp1 = Gen::new("x := 5", &ctx).unwrap();
    exp1.eval().expect("Failed to evaluate assignment");
    let expr2 = Gen::new("x^2 + 2*x + 1", &ctx).unwrap();
    let evaluated2 = expr2.eval().expect("eval failed");
    info!("Evaluated expression with assignment: {}", evaluated2.to_string());
    assert!(evaluated2.to_string().contains("36"));
}
