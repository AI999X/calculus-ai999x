//! Calculus: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p calculus-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&calculus_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "extrema_on_interval", "inputs": {"a": "0", "b": "2", "f": "x^3 - 3x"}}));
    println!();
    show(json!({"key": "fourier_coefficients", "inputs": {"L": "3.141592654", "f": "x", "k": "2"}}));
    println!();
    show(json!({"key": "antiderivative", "inputs": {"f": "3x^2 + 2x"}}));
    println!();
    show(json!({"key": "limit_at_point", "inputs": {"a": "2", "f": "(x^2 - 4)/(x - 2)"}}));
    println!();
    show(json!({"key": "polar_double_integral", "inputs": {"al": "0", "be": "6.283185307", "f": "x^2 + y^2", "r1": "0", "r2": "1"}}));
    println!();
    show(json!({"key": "directional_derivative", "inputs": {"f": "x^2*y", "p": [1, 2], "u": [3, 4]}}));
    println!();
    show(json!({"key": "numerical_derivative", "inputs": {"a": "1", "f": "x^2", "h": "0.1", "method": "central"}}));
    println!();
    show(json!({"key": "gauss_legendre", "inputs": {"a": "0", "b": "2", "f": "x^3 + x^2", "n": "2"}}));
    println!();
    show(json!({"key": "newton_method", "inputs": {"f": "x^2 - 2", "n": "3", "x0": "1"}}));
    println!();
    show(json!({"key": "parametric_arc_length", "inputs": {"t0": "0", "t1": "3.141592654", "xt": "cos(t)", "yt": "sin(t)"}}));
    println!();
    show(json!({"key": "polar_area", "inputs": {"al": "0", "be": "6.283185307", "r": "1 + cos(theta)"}}));
    println!();
    show(json!({"key": "geometric_partial_sum", "inputs": {"a": "2", "n": "4", "r": "3"}}));
    println!();
    show(json!({"key": "bessel_j", "inputs": {"n": "0", "x": "1"}}));
    println!();
    show(json!({"key": "conservative_field", "inputs": {"P": "2x*y", "Q": "x^2"}}));
    println!();
    show(json!({"key": "space_curve_arc_length", "inputs": {"t0": "0", "t1": "6.283185307", "xt": "cos(t)", "yt": "sin(t)", "zt": "t"}}));
}
