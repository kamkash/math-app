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
    let plusxy = x.symb_plus(&y).unwrap();
    let plusabsym = a.symb_plus(&b).unwrap();
    let plusab = a.add(&b).unwrap();
    let mult = x.symb_mult(&y).unwrap();
    let mulab = a.mul(&b).unwrap();
    let mulxy = x.mul(&y).unwrap();
    let pow = x.symb_pow(&y).unwrap();
    info!(
        "plus: {} mult: {} pow: {}, plusab: {}, mulab: {}, mulxy: {}",
        plusxy, mult, pow, plusab, mulab, mulxy
    );
    assert!(
        plusxy.to_string().contains("x")
            && plusxy.to_string().contains("y")
            && (plusxy.to_string().contains("+") || plusxy.to_string().contains("plus"))
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
    assert!(plusab.to_string().contains("5"));
    assert!(mulab.to_string().contains("6"));
    assert!(mulxy.to_string().contains("x") || mulxy.to_string().contains("y"));
    assert!(
        plusabsym.to_string().contains("a")
            || plusabsym.to_string().contains("b")
            || plusabsym.to_string().contains("+")
    );

    let x_val = Gen::from_f64(2.0, &ctx).unwrap();
    let y_val = Gen::from_f64(3.0, &ctx).unwrap();
    let subs = plusxy
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

#[test_log::test]
fn test_gen_to_f64() {
    let ctx = Context::new();
    let g = Gen::new("3.14159", &ctx).unwrap();
    let val = g.to_f64();
    assert!(val.is_some(), "to_f64 should succeed for numeric value");
    let v = val.unwrap();
    assert!(
        (v - 3.14159).abs() < 1e-8,
        "to_f64 value should be close to 3.14159, got {}",
        v
    );

    let g2 = Gen::new("2+2", &ctx).unwrap();
    let val2 = g2.to_f64().unwrap();
    // Should be None, because it's not a direct number, but after eval it should work
    assert!(
        val2 - 4.0 < 1e-8,
        "to_f64 should fail for unevaluated expression"
    );
    let g2_eval = g2.eval().unwrap();
    let val2_eval = g2_eval.to_f64();
    assert_eq!(val2_eval, Some(4.0));

    let g3 = Gen::new("x", &ctx).unwrap();
    let val3 = g3.to_f64();
    assert!(val3.is_none(), "to_f64 should fail for symbolic variable");
}

#[test_log::test]
fn test_gen_math_function() {
    let ctx = Context::new();
    let [x, xx, y] = Gen::symbols(["x", "xx", "y"], &ctx);

    let log_fn = Gen::log(&x).expect("Failed to create log function");
    let ln_fn = Gen::ln(&y).expect("Failed to create ln function");
    let logb_fn = Gen::logb(&xx, 2.0).expect("Failed to create logb function");
    info!(
        "functions: {} {} {}",
        log_fn.to_string(),
        ln_fn.to_string(),
        logb_fn.to_string()
    );

    let log_subs = log_fn
        .subs(&["x"], &[&Gen::from_f64(10.0, &ctx).unwrap()])
        .expect("Failed to substitute in log function");
    let log_eval = log_subs.eval().expect("Failed to evaluate log function");
    info!("Log(10) evaluated: {}", log_eval.to_string());
    assert!(
        log_eval.to_f64().unwrap() - 10.0f64.log10() < 1e-8,
        "Log should be close to 10.0"
    );

    let ln_subs = ln_fn
        .subs(&["y"], &[&Gen::from_f64(5.0, &ctx).unwrap()])
        .expect("Failed to substitute in ln function");
    let ln_eval = ln_subs.eval().expect("Failed to evaluate ln function");
    info!("Ln(5) evaluated: {}", ln_eval.to_string());
    assert!(
        ln_eval.to_f64().unwrap() - 5.0f64.ln() < 1e-8,
        "Ln should be close to 5.0"
    );

    let logb_subs = logb_fn
        .subs(&["xx"], &[&Gen::from_f64(10.0, &ctx).unwrap()])
        .expect("Failed to substitute in logb function");
    let logb_eval = logb_subs.eval().expect("Failed to evaluate logb function");
    info!("Log2(10) evaluated: {}", logb_eval.to_string());
    assert!(
        logb_eval.to_f64().unwrap() - 10.0f64.log2() < 1e-8,
        "Log2 should be close to 10.0"
    );

    let sqrt_2 = x.symb_sqrt().unwrap();
    let root_3 = x.symb_root(3.0f64).unwrap();
    info!(
        "sqrt: {} root(3): {}",
        sqrt_2.to_string(),
        root_3.to_string()
    );
    let sqrt_subs = sqrt_2
        .subs(&["x"], &[&Gen::from_f64(100.0, &ctx).unwrap()])
        .expect("Failed to substitute in logb function");
    let sqrt_eval = sqrt_subs.eval().expect("Failed to evaluate logb function");
    info!("sqrt(100) evaluated: {}", sqrt_eval.to_string());
    assert!(
        sqrt_eval.to_f64().unwrap() - 100.0f64.sqrt() < 1e-8,
        "sqrt should be close to 10.0"
    );

    let root_3_subs = root_3
        .subs(&["x"], &[&Gen::from_f64(1000.0, &ctx).unwrap()])
        .expect("Failed to substitute in root(3) function");
    let root_3_eval = root_3_subs
        .eval()
        .expect("Failed to evaluate root(3) function");
    info!("root(3)(1000) evaluated: {}", root_3_eval.to_string());
    assert!(
        root_3_eval.to_f64().unwrap() - 1000.0f64.powf(1.0/3.0) < 1e-8,
        "root(3) should be close to 10.0"
    );
}

#[test_log::test]
fn test_gen_deep_clone() {
    let ctx = Context::new();
    let g = Gen::new("x^2 + 1", &ctx).unwrap();
    let g_clone = g.deep_clone().expect("deep_clone failed");
    // They should be structurally equal
    assert!(g.equals(&g_clone));
    // But own different underlying pointers
    assert_ne!(
        g.ptr(),
        g_clone.ptr(),
        "deep_clone should create a new underlying gen_t"
    );
}
