// Unit tests for giac-rs/src/gen.rs
use giac_rs::context::Context;
use giac_rs::gen::{Gen, GEN_ADD, GEN_DIV, GEN_MUL, GEN_POW};
use giac_rs::r#gen::{GEN_AND, GEN_EQ, GEN_GE, GEN_GT, GEN_LE, GEN_LT, GEN_NOT, GEN_OR, GEN_SUB};
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
    let x = Gen::new("x", &ctx).unwrap();
    let y = Gen::new("y", &ctx).unwrap();
    let sum = a.add(&b).unwrap().to_string();
    let diff = a.sub(&b).unwrap().to_string();
    let prod = a.mul(&b).unwrap().to_string();
    let quot = a.div(&b).unwrap().to_string();
    let pow = a.pow(&b).unwrap().to_string();
    let sumxy = x.add(&y).unwrap().to_string();
    info!(
        "sum: {} diff: {} prod: {} quot: {} pow: {} sumxy: {}",
        sum, diff, prod, quot, pow, sumxy
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
    let a = Gen::new("2", &ctx).unwrap();
    let b = Gen::new("3", &ctx).unwrap();
    let x = Gen::new("x", &ctx).unwrap();
    let y = Gen::new("y", &ctx).unwrap();
    let plus = x.symb_plus(&y).unwrap();
    let plusab = a.symb_plus(&b).unwrap();
    let mult = x.symb_mult(&y).unwrap();
    let pow = x.symb_pow(&y).unwrap();
    info!(
        "plus: {} mult: {} pow: {}, plusab: {}",
        plus, mult, pow, plusab
    );
    assert!(
        plus.to_string().contains("x")
            && plus.to_string().contains("y")
            && (plus.to_string().contains("+") || plus.to_string().contains("plus"))
    );
    assert!(
        mult.to_string().contains("x")
            && mult.to_string().contains("y")
            && (mult.to_string().contains("*") || mult.to_string().contains("mult"))
    );
    assert!(
        pow.to_string().contains("x")
            && pow.to_string().contains("y")
            && (pow.to_string().contains("^") || pow.to_string().contains("pow"))
    );

    let x_val = Gen::from_f64(2.0, &ctx).unwrap();
    let y_val = Gen::from_f64(3.0, &ctx).unwrap();
    let subs = plus
        .subs(&["x", "y"], &[&x_val, &y_val])
        .expect("subs failed");
    let s = subs.to_string();
    info!("Substituted: {}", s);
    assert!(s.contains("5") || s.contains("2+3") || s.contains("3+2"));
    let subs = mult
        .subs(&["x", "y"], &[&x_val, &y_val])
        .expect("subs failed");
    let s = subs.to_string();
    info!("Substituted: {}", s);
    assert!(s.contains("6") || s.contains("2*3") || s.contains("3*2"));
    let subs = pow
        .subs(&["x", "y"], &[&x_val, &y_val])
        .expect("subs failed");
    let s = subs.to_string();
    info!("Substituted: {}", s);
    assert!(s.contains("8") || s.contains("2^3") || s.contains("3^2"));
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
    info!("Expression {} Evaluated: {}", expr, s);
    assert!(s == "5" || s.contains("5"));

    let exp1 = Gen::new("x := 5", &ctx).unwrap();
    exp1.eval().expect("Failed to evaluate assignment");
    let expr2 = Gen::new("x^2 + 2*x + 1", &ctx).unwrap();
    let evaluated2 = expr2.eval().expect("eval failed");
    info!(
        "Evaluated expression with assignment: {}",
        evaluated2.to_string()
    );
    assert!(evaluated2.to_string().contains("36"));
}

#[test_log::test]
fn test_gen_is_some_op() {
    let mult_op = GEN_MUL.clone();
    let div_op = GEN_DIV.clone();
    let add_op = GEN_ADD.clone();
    let sub_op = GEN_SUB.clone();
    let pow_op = GEN_POW.clone();

    let and_op = GEN_AND.clone();
    let or_op = GEN_OR.clone();
    let not_op = GEN_NOT.clone();

    let eq_op = GEN_EQ.clone();
    let lt_op = GEN_LT.clone();
    let le_op = GEN_LE.clone();
    let gt_op = GEN_GT.clone();
    let ge_op = GEN_GE.clone();

    assert!(eq_op.is_eq());
    assert!(lt_op.is_lt());
    assert!(le_op.is_le());
    assert!(gt_op.is_gt());
    assert!(ge_op.is_ge());

    assert!(and_op.is_and());
    assert!(or_op.is_or());
    assert!(not_op.is_not());

    assert!(pow_op.is_pow());
    assert!(add_op.is_add());
    assert!(sub_op.is_sub());
    assert!(mult_op.is_mul());
    assert!(div_op.is_div());
    assert!(add_op.is_op());
    assert!(sub_op.is_op());
    assert!(mult_op.is_op());
    assert!(div_op.is_op());
    assert!(pow_op.is_op());
}
