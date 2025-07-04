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
