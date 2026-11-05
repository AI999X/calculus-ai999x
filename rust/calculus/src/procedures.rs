//! calculus: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/calculus/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "arc_length",
    "area_between_curves",
    "area_enclosed_curves",
    "arithmetic_series_sum",
    "average_rate_of_change",
    "average_value",
    "bessel_j",
    "beta_function",
    "centroid_region",
    "chebyshev_polynomial",
    "critical_points_classify",
    "curl",
    "curvature",
    "definite_integral",
    "derivative_at_point",
    "directional_derivative",
    "disk_volume",
    "divergence",
    "divergence_theorem_box",
    "double_integral_rectangle",
    "double_integral_region",
    "error_function",
    "extrema_on_interval",
    "fourier_coefficients",
    "ftc_derivative",
    "gamma_function",
    "gauss_legendre",
    "geometric_partial_sum",
    "geometric_series_sum",
    "gradient",
    "greens_theorem",
    "hessian_classification",
    "implicit_derivative",
    "improper_integral",
    "inflection_points",
    "jacobian_determinant",
    "laplacian",
    "left_riemann_sum",
    "legendre_polynomial",
    "limit_at_infinity",
    "limit_at_point",
    "line_integral_scalar",
    "line_integral_vector",
    "linear_approximation",
    "mean_value_theorem",
    "midpoint_rule",
    "newton_method",
    "normal_line",
    "nth_derivative_at_point",
    "numerical_derivative",
    "parametric_arc_length",
    "parametric_slope",
    "partial_derivative",
    "polar_area",
    "ratio_test",
    "regularized_gamma",
    "riemann_zeta",
    "right_riemann_sum",
    "second_derivative_at_point",
    "series_sum",
    "shell_volume",
    "simpson_rule",
    "surface_area_graph",
    "surface_area_revolution",
    "tangent_line",
    "tangent_plane",
    "taylor_polynomial",
    "trapezoid_rule",
    "triple_integral_box",
    "velocity_acceleration",
    "washer_volume",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "arc_length" => {
            let r = f_arc_length(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "area_between_curves" => {
            let r = f_area_between_curves(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_16(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "area_enclosed_curves" => {
            let r = f_area_enclosed_curves(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_42(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "arithmetic_series_sum" => {
            let r = f_arithmetic_series_sum(match v.get("a1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_57(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "average_rate_of_change" => {
            let r = f_average_rate_of_change(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_68(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "average_value" => {
            let r = f_average_value(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_80(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bessel_j" => {
            let r = f_bessel_j(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_94(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "beta_function" => {
            let r = f_beta_function(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_107(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "centroid_region" => {
            let r = f_centroid_region(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_127(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chebyshev_polynomial" => {
            let r = f_chebyshev_polynomial(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_143(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "critical_points_classify" => {
            let r = f_critical_points_classify(match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_168(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "curl" => {
            let r = f_curl(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_218(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "curvature" => {
            let r = f_curvature(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_238(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "definite_integral" => {
            let r = f_definite_integral(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_248(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "derivative_at_point" => {
            let r = f_derivative_at_point(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_256(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "directional_derivative" => {
            let r = f_directional_derivative(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_277(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "disk_volume" => {
            let r = f_disk_volume(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_295(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "divergence" => {
            let r = f_divergence(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_314(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "divergence_theorem_box" => {
            let r = f_divergence_theorem_box(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_374(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "double_integral_rectangle" => {
            let r = f_double_integral_rectangle(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_413(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "double_integral_region" => {
            let r = f_double_integral_region(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_453(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "error_function" => {
            let r = f_error_function(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_458(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "extrema_on_interval" => {
            let r = f_extrema_on_interval(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_472(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fourier_coefficients" => {
            let r = f_fourier_coefficients(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_512(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ftc_derivative" => {
            let r = f_ftc_derivative(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_521(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gamma_function" => {
            let r = f_gamma_function(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_530(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gauss_legendre" => {
            let r = f_gauss_legendre(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_563(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geometric_partial_sum" => {
            let r = f_geometric_partial_sum(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_583(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geometric_series_sum" => {
            let r = f_geometric_series_sum(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_594(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gradient" => {
            let r = f_gradient(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_608(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "greens_theorem" => {
            let r = f_greens_theorem(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_666(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hessian_classification" => {
            let r = f_hessian_classification(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_685(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "implicit_derivative" => {
            let r = f_implicit_derivative(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_711(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "improper_integral" => {
            let r = f_improper_integral(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_721(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inflection_points" => {
            let r = f_inflection_points(match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_743(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "jacobian_determinant" => {
            let r = f_jacobian_determinant(match v.get("xu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_776(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "laplacian" => {
            let r = f_laplacian(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_790(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "left_riemann_sum" => {
            let r = f_left_riemann_sum(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_814(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "legendre_polynomial" => {
            let r = f_legendre_polynomial(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_833(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "limit_at_infinity" => {
            let r = f_limit_at_infinity(match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_852(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "limit_at_point" => {
            let r = f_limit_at_point(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_870(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "line_integral_scalar" => {
            let r = f_line_integral_scalar(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_883(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "line_integral_vector" => {
            let r = f_line_integral_vector(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_899(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_approximation" => {
            let r = f_linear_approximation(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_913(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mean_value_theorem" => {
            let r = f_mean_value_theorem(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_925(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "midpoint_rule" => {
            let r = s_957(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_986(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "newton_method" => {
            let r = f_newton_method(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1005(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "normal_line" => {
            let r = f_normal_line(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1017(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nth_derivative_at_point" => {
            let r = f_nth_derivative_at_point(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1028(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "numerical_derivative" => {
            let r = f_numerical_derivative(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("method") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1043(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parametric_arc_length" => {
            let r = f_parametric_arc_length(match v.get("xt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1056(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parametric_slope" => {
            let r = f_parametric_slope(match v.get("xt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1071(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "partial_derivative" => {
            let r = f_partial_derivative(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("var") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1086(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polar_area" => {
            let r = f_polar_area(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("al") { Some(x) => py::from_val(x), None => { return None } }, match v.get("be") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1102(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ratio_test" => {
            let r = f_ratio_test(match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1130(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "regularized_gamma" => {
            let r = f_regularized_gamma(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1161(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "riemann_zeta" => {
            let r = f_riemann_zeta(match v.get("s") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1173(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "right_riemann_sum" => {
            let r = f_right_riemann_sum(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1195(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "second_derivative_at_point" => {
            let r = f_second_derivative_at_point(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1206(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "series_sum" => {
            let r = f_series_sum(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1226(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shell_volume" => {
            let r = f_shell_volume(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1244(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "simpson_rule" => {
            let r = s_1252(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1281(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "surface_area_graph" => {
            let r = f_surface_area_graph(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1325(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "surface_area_revolution" => {
            let r = f_surface_area_revolution(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1343(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "tangent_line" => {
            let r = f_tangent_line(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1373(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "tangent_plane" => {
            let r = f_tangent_plane(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1389(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "taylor_polynomial" => {
            let r = f_taylor_polynomial(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1405(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "trapezoid_rule" => {
            let r = s_1422(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1451(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "triple_integral_box" => {
            let r = f_triple_integral_box(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1502(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "velocity_acceleration" => {
            let r = f_velocity_acceleration(match v.get("s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1507(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "washer_volume" => {
            let r = f_washer_volume(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1526(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_NN() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pysym::symbol("n")) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_T() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pysym::symbol("t")) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_TH() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pysym::symbol("theta")) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_X() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pysym::symbol("x")) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_Y() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pysym::symbol("y")) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_Z() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pysym::symbol("z")) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__B2K() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![py::fraction(&[Py::Int(1i64), Py::Int(6i64)])?, py::fraction(&[py::neg(&Py::Int(1i64))?, Py::Int(30i64)])?, py::fraction(&[Py::Int(1i64), Py::Int(42i64)])?, py::fraction(&[py::neg(&Py::Int(1i64))?, Py::Int(30i64)])?, py::fraction(&[Py::Int(5i64), Py::Int(66i64)])?, py::fraction(&[py::neg(&Py::Int(691i64))?, Py::Int(2730i64)])?, py::fraction(&[Py::Int(7i64), Py::Int(6i64)])?, py::fraction(&[py::neg(&Py::Int(3617i64))?, Py::Int(510i64)])?])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__GLF() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__KS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pysym::symbol("k")) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__SYMS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("x"), ax_engine::pysym::symbol("x")), (Py::str("y"), ax_engine::pysym::symbol("y")), (Py::str("z"), ax_engine::pysym::symbol("z")), (Py::str("t"), ax_engine::pysym::symbol("t")), (Py::str("u"), ax_engine::pysym::symbol("u")), (Py::str("v"), ax_engine::pysym::symbol("v")), (Py::str("r"), ax_engine::pysym::symbol("r")), (Py::str("theta"), ax_engine::pysym::symbol("theta")), (Py::str("n"), ax_engine::pysym::symbol("n"))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__XY() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::str("x"), Py::str("y")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__XYZ() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::str("x"), Py::str("y"), Py::str("z")])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__curl_from(mut v_part: Py) -> R {
    return Ok(Py::list(vec![py::sub(&py::call(&v_part.clone(), &[Py::Int(2i64), Py::Int(1i64)])?, &py::call(&v_part.clone(), &[Py::Int(1i64), Py::Int(2i64)])?)?, py::sub(&py::call(&v_part.clone(), &[Py::Int(0i64), Py::Int(2i64)])?, &py::call(&v_part.clone(), &[Py::Int(2i64), Py::Int(0i64)])?)?, py::sub(&py::call(&v_part.clone(), &[Py::Int(1i64), Py::Int(0i64)])?, &py::call(&v_part.clone(), &[Py::Int(0i64), Py::Int(1i64)])?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__distinct_real_roots(mut v_q: Py) -> R {
    return Ok(py::sorted_key(&py::set(&ax_engine::pysym::call("real_roots", &[v_q.clone()], &[])?)?, &{ let v_q = v_q.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::float(&v_r.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__eqn(mut v_text: Py, mut v_names: Py) -> R {
    let mut v_left: Py = Py::None;
    let mut v_right: Py = Py::None;
    if ({ let t702 = py::not(&Py::Bool(matches!(v_text.clone(), Py::Str(_)))); if t702.truthy() { t702 } else { { let t700 = py::method(&v_text.clone(), "count", &[Py::str("=")])?; Py::Bool({ let t701 = Py::Int(1i64); if py::ne(&t700, &t701) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("write the curve as one equation, e.g. x^2 + y^2 = 25"))));
    }
    let t703 = (py::method(&v_text.clone(), "split", &[Py::str("=")])?).items()?;
    if t703.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_left = t703[0].clone();
    v_right = t703[1].clone();
    return Ok(py::sub(&ax_engine::pysym::helper("calculus:parse_expr", &[py::method(&v_left.clone(), "strip", &[])?, v_names.clone()], &[])?, &ax_engine::pysym::helper("calculus:parse_expr", &[py::method(&v_right.clone(), "strip", &[])?, v_names.clone()], &[])?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__eqn_text(mut v_text: Py) -> R {
    let mut v_left: Py = Py::None;
    let mut v_right: Py = Py::None;
    let t712 = (py::method(&v_text.clone(), "split", &[Py::str("=")])?).items()?;
    if t712.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_left = t712[0].clone();
    v_right = t712[1].clone();
    return Ok(Py::str(&["(".to_string(), py::format(&py::method(&v_left.clone(), "strip", &[])?, "")?, ") - (".to_string(), py::format(&py::method(&v_right.clone(), "strip", &[])?, "")?, ")".to_string()].concat()));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__exact(mut v_e: Py, mut v_a: Py, mut v_b: Py) -> R {
    let _ = ax_engine::pysym::helper("calculus:continuous_on", &[v_e.clone(), c_X()?, v_a.clone(), v_b.clone()], &[])?;
    return Ok(ax_engine::pysym::helper("calculus:quad", &[ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[])?, v_a.clone(), v_b.clone()], &[])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__field(mut v_P: Py, mut v_Q: Py, mut v_R: Py) -> R {
    let mut v_names: Py = Py::None;
    v_names = (if ({ let t336 = v_R.clone(); Py::Bool({ let t337 = Py::None; if !py::is(&t336, &t337) { true } else { false } }) }).truthy() { c__XYZ()? } else { c__XY()? });
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t340 in py::iter(&((if ({ let t338 = v_R.clone(); Py::Bool({ let t339 = Py::None; if !py::is(&t338, &t339) { true } else { false } }) }).truthy() { Py::list(vec![v_P.clone(), v_Q.clone(), v_R.clone()]) } else { Py::list(vec![v_P.clone(), v_Q.clone()]) })))? { let mut v_c: Py; 
v_c = t340;
out.push(ax_engine::pysym::helper("calculus:parse_expr", &[v_c.clone(), v_names.clone()], &[])?); } Py::list(out) }, v_names.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_v: Py, mut v_what: Py) -> R {
    v_v = py::float(&v_v.clone())?;
    if (py::not(&py::math1("isfinite", &v_v.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_what.clone(), "")?, " is not finite".to_string()].concat()))));
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fl(mut v_e: Py, mut v_names: Py) -> R {
    return Ok(ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), v_names.clone()], &[(String::from("use_mp"), Py::Bool(false))])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__float_sum(mut v_f: Py, mut v_pts: Py, mut v_w: Py) -> R {
    let mut v_g: Py = Py::None;
    v_g = ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?, Py::tuple(vec![c_X()?])], &[(String::from("use_mp"), Py::Bool(false))])?;
    return Ok(py::fsum(&{ let mut out = vec![]; for t1457 in py::iter(&(py::zip(&[v_w.clone(), v_pts.clone()])?))? { let mut v_p: Py; let mut v_wi: Py; 
let t1458 = (t1457).items()?;
if t1458.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_wi = t1458[0].clone();
v_p = t1458[1].clone();
out.push(py::mul(&v_wi.clone(), &py::call(&v_g.clone(), &[v_p.clone()])?)?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fourier_gl(mut v_f: Py, mut v_L: Py, mut v_k: Py) -> R {
    let mut v_G: Py = Py::None;
    let mut v_g: Py = Py::None;
    v_G = { let v_L = v_L.clone(); let v_f = v_f.clone(); let v_k = v_k.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_tr: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(ax_engine::pysym::helper("calculus:_gl", &[{ let v_L = v_L.clone(); let v_f = v_f.clone(); let v_k = v_k.clone(); let v_tr = v_tr.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::call(&v_tr.clone(), &[v_x.clone()])?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::neg(&v_L.clone())?, v_L.clone()], &[(String::from("panels"), py::minmax(&[Py::Int(8i64), py::mul(&Py::Int(4i64), &v_k.clone())?], true)?)])?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        v_g = ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?, Py::tuple(vec![c_X()?])], &[])?;
        return Ok(Py::tuple(vec![py::truediv(&py::float(&py::call(&v_G.clone(), &[v_g.clone()])?)?, &v_L.clone())?, py::truediv(&py::float(&py::call(&v_G.clone(), &[{ let v_G = v_G.clone(); let v_L = v_L.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_k = v_k.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&py::call(&v_g.clone(), &[v_x.clone()])?, &ax_engine::pymp::call("cos", &[py::truediv(&py::mul(&py::mul(&v_k.clone(), &ax_engine::pymp::constant("pi")?)?, &v_x.clone())?, &v_L.clone())?], &[])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }])?)?, &v_L.clone())?, py::truediv(&py::float(&py::call(&v_G.clone(), &[{ let v_G = v_G.clone(); let v_L = v_L.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_k = v_k.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&py::call(&v_g.clone(), &[v_x.clone()])?, &ax_engine::pymp::call("sin", &[py::truediv(&py::mul(&py::mul(&v_k.clone(), &ax_engine::pymp::constant("pi")?)?, &v_x.clone())?, &v_L.clone())?], &[])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }])?)?, &v_L.clone())?]));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__frac(mut v_v: Py) -> R {
    return Ok(py::fraction(&[Py::str(&py::repr(&py::float(&v_v.clone())?))])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gauss_nodes(mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_dp: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p0: Py = Py::None;
    let mut v_p1: Py = Py::None;
    let mut v_x: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(40i64))?;
        v_out = Py::list(vec![]);
        for t1480 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_i = t1480;
            v_x = ax_engine::pymp::call("cos", &[py::truediv(&py::mul(&ax_engine::pymp::constant("pi")?, &py::sub(&v_i.clone(), &py::truediv(&ax_engine::pymp::call("mpf", &[Py::Int(1i64)], &[])?, &Py::Int(4i64))?)?)?, &py::add(&v_n.clone(), &py::truediv(&ax_engine::pymp::call("mpf", &[Py::Int(1i64)], &[])?, &Py::Int(2i64))?)?)?], &[])?;
            for t1481 in py::range_iter(&[(Py::Int(100i64)).i()?]) {
                v__ = t1481;
                let t1482 = (Py::tuple(vec![ax_engine::pymp::call("mpf", &[Py::Int(1i64)], &[])?, v_x.clone()])).items()?;
                if t1482.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_p0 = t1482[0].clone();
                v_p1 = t1482[1].clone();
                for t1483 in py::range_iter(&[(Py::Int(2i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                    v_k = t1483;
                    let t1484 = (Py::tuple(vec![v_p1.clone(), py::truediv(&py::sub(&py::mul(&py::mul(&py::sub(&py::mul(&Py::Int(2i64), &v_k.clone())?, &Py::Int(1i64))?, &v_x.clone())?, &v_p1.clone())?, &py::mul(&py::sub(&v_k.clone(), &Py::Int(1i64))?, &v_p0.clone())?)?, &v_k.clone())?])).items()?;
                    if t1484.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_p0 = t1484[0].clone();
                    v_p1 = t1484[1].clone();
                }
                v_dp = (if ({ let t1485 = v_n.clone(); Py::Bool({ let t1486 = Py::Int(1i64); if py::gt(&t1485, &t1486)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&v_n.clone(), &py::sub(&py::mul(&v_x.clone(), &v_p1.clone())?, &v_p0.clone())?)?, &py::sub(&py::mul(&v_x.clone(), &v_x.clone())?, &Py::Int(1i64))?)? } else { ax_engine::pymp::call("mpf", &[Py::Int(1i64)], &[])? });
                if ({ let t1487 = v_n.clone(); Py::Bool({ let t1488 = Py::Int(1i64); if py::eq(&t1487, &t1488) { true } else { false } }) }).truthy() {
                    let t1489 = (Py::tuple(vec![v_x.clone(), ax_engine::pymp::call("mpf", &[Py::Int(1i64)], &[])?])).items()?;
                    if t1489.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_p1 = t1489[0].clone();
                    v_dp = t1489[1].clone();
                }
                v_dx = py::truediv(&v_p1.clone(), &v_dp.clone())?;
                v_x = py::sub(&v_x, &v_dx.clone())?;
                if ({ let t1490 = py::abs(&v_dx.clone())?; Py::Bool({ let t1491 = py::pow(&ax_engine::pymp::call("mpf", &[Py::Int(10i64)], &[])?, &py::neg(&Py::Int(35i64))?)?; if py::lt(&t1490, &t1491)? { true } else { false } }) }).truthy() {
                    break;
                }
            }
            let t1492 = (Py::tuple(vec![ax_engine::pymp::call("mpf", &[Py::Int(1i64)], &[])?, v_x.clone()])).items()?;
            if t1492.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_p0 = t1492[0].clone();
            v_p1 = t1492[1].clone();
            for t1493 in py::range_iter(&[(Py::Int(2i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                v_k = t1493;
                let t1494 = (Py::tuple(vec![v_p1.clone(), py::truediv(&py::sub(&py::mul(&py::mul(&py::sub(&py::mul(&Py::Int(2i64), &v_k.clone())?, &Py::Int(1i64))?, &v_x.clone())?, &v_p1.clone())?, &py::mul(&py::sub(&v_k.clone(), &Py::Int(1i64))?, &v_p0.clone())?)?, &v_k.clone())?])).items()?;
                if t1494.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_p0 = t1494[0].clone();
                v_p1 = t1494[1].clone();
            }
            v_dp = (if ({ let t1495 = v_n.clone(); Py::Bool({ let t1496 = Py::Int(1i64); if py::gt(&t1495, &t1496)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&v_n.clone(), &py::sub(&py::mul(&v_x.clone(), &v_p1.clone())?, &v_p0.clone())?)?, &py::sub(&py::mul(&v_x.clone(), &v_x.clone())?, &Py::Int(1i64))?)? } else { ax_engine::pymp::call("mpf", &[Py::Int(1i64)], &[])? });
            let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![v_x.clone(), py::truediv(&Py::Int(2i64), &py::mul(&py::mul(&py::sub(&Py::Int(1i64), &py::mul(&v_x.clone(), &v_x.clone())?)?, &v_dp.clone())?, &v_dp.clone())?)?])])?;
        }
        return Ok(Py::tuple((py::sorted(&v_out.clone(), (Py::Bool(false)).truthy())?).items()?));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gl1(mut v_g: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t1477 = (Py::tuple(vec![py::truediv(&py::add(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?, py::truediv(&py::sub(&v_b.clone(), &v_a.clone())?, &Py::Int(2i64))?])).items()?;
    if t1477.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t1477[0].clone();
    v_r = t1477[1].clone();
    return Ok(py::mul(&v_r.clone(), &py::fsum(&{ let mut out = vec![]; for t1499 in py::iter(&(f__glf(v_n.clone())?))? { let mut v_w: Py; let mut v_x: Py; 
let t1500 = (t1499).items()?;
if t1500.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1500[0].clone();
v_w = t1500[1].clone();
out.push(py::mul(&v_w.clone(), &py::call(&v_g.clone(), &[py::add(&v_c.clone(), &py::mul(&v_r.clone(), &v_x.clone())?)?])?)?); } Py::list(out) })?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gl2(mut v_g: Py, mut v_a: Py, mut v_b: Py, mut v_lo: Py, mut v_hi: Py, mut v_n: Py) -> R {
    return Ok(f__gl1({ let v_a = v_a.clone(); let v_b = v_b.clone(); let v_g = v_g.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__gl1({ let v_a = v_a.clone(); let v_b = v_b.clone(); let v_g = v_g.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_n = v_n.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_y: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::call(&v_g.clone(), &[v_x.clone(), v_y.clone()])?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::call(&v_lo.clone(), &[v_x.clone()])?, py::call(&v_hi.clone(), &[v_x.clone()])?, v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_a.clone(), v_b.clone(), v_n.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gl3(mut v_g: Py, mut v_bx: Py, mut v_n: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_f: Py = Py::None;
    let t1473 = (v_bx.clone()).items()?;
    if t1473.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    let t1474 = (t1473[0].clone()).items()?;
    if t1474.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1474[0].clone();
    v_b = t1474[1].clone();
    let t1475 = (t1473[1].clone()).items()?;
    if t1475.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t1475[0].clone();
    v_d = t1475[1].clone();
    let t1476 = (t1473[2].clone()).items()?;
    if t1476.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_e = t1476[0].clone();
    v_f = t1476[1].clone();
    return Ok(f__gl1({ let v_a = v_a.clone(); let v_b = v_b.clone(); let v_bx = v_bx.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_e = v_e.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__gl1({ let v_a = v_a.clone(); let v_b = v_b.clone(); let v_bx = v_bx.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_e = v_e.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_n = v_n.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_y: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__gl1({ let v_a = v_a.clone(); let v_b = v_b.clone(); let v_bx = v_bx.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_e = v_e.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_n = v_n.clone(); let v_x = v_x.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::call(&v_g.clone(), &[v_x.clone(), v_y.clone(), v_z.clone()])?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_e.clone(), v_f.clone(), v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_c.clone(), v_d.clone(), v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_a.clone(), v_b.clone(), v_n.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gl_check(mut v_text: Py, mut v_a: Py, mut v_b: Py, mut v_names: Py, mut v_transform: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_g: Py = Py::None;
    v_f = ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::helper("calculus:parse_expr", &[v_text.clone(), v_names.clone()], &[])?, v_names.clone()], &[])?;
    v_g = (if (v_transform.clone()).truthy() { { let v_a = v_a.clone(); let v_b = v_b.clone(); let v_f = v_f.clone(); let v_names = v_names.clone(); let v_text = v_text.clone(); let v_transform = v_transform.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::call(&v_transform.clone(), &[v_f.clone(), v_x.clone()])?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) } } else { v_f.clone() });
    return Ok(ax_engine::pysym::helper("calculus:_gl", &[v_g.clone(), v_a.clone(), v_b.clone()], &[])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__glf(mut v_n: Py) -> R {
    if ({ let t1478 = v_n.clone(); Py::Bool({ let t1479 = c__GLF()?; if !py::contains(&t1479, &t1478)? { true } else { false } }) }).truthy() {
        py::setitem(&c__GLF()?, &v_n.clone(), { let mut out = vec![]; for t1497 in py::iter(&(f__gauss_nodes(v_n.clone())?))? { let mut v_w: Py; let mut v_x: Py; 
let t1498 = (t1497).items()?;
if t1498.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1498[0].clone();
v_w = t1498[1].clone();
out.push(Py::tuple(vec![py::float(&v_x.clone())?, py::float(&v_w.clone())?])); } Py::list(out) })?;
    }
    return Ok(py::getitem(&c__GLF()?, &v_n.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__grid(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    if ({ let t1428 = { let t1423 = py::int(&v_n.clone())?; Py::Bool({ let t1424 = v_n.clone(); if py::ne(&t1423, &t1424) { true } else { false } }) }; if t1428.truthy() { t1428 } else { py::not(&{ let t1425 = Py::Int(1i64); Py::Bool({ let t1426 = v_n.clone(); if py::le(&t1425, &t1426)? { { let t1427 = Py::Int(100000i64); if py::le(&t1426, &t1427)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be a whole number of subintervals between 1 and 100000"))));
    }
    if (py::not(&{ let t1429 = v_a.clone(); Py::Bool({ let t1430 = v_b.clone(); if py::lt(&t1429, &t1430)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    return Ok(Py::tuple(vec![ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?, py::truediv(&py::sub(&v_b.clone(), &v_a.clone())?, &py::int(&v_n.clone())?)?, py::int(&v_n.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__limval(mut v_L: Py) -> R {
    let mut v_v: Py = Py::None;
    if ({ let t1222 = py::not(&Py::Bool(matches!(v_L.clone(), Py::Sym(_)))); if t1222.truthy() { t1222 } else { { let t1221 = py::getattr(&v_L.clone(), "free_symbols")?; if t1221.truthy() { t1221 } else { { let t1220 = py::method(&v_L.clone(), "has", &[ax_engine::pysym::constant("Limit")?])?; if t1220.truthy() { t1220 } else { { let t1219 = py::method(&v_L.clone(), "has", &[ax_engine::pysym::constant("Sum")?])?; if t1219.truthy() { t1219 } else { py::method(&v_L.clone(), "has", &[ax_engine::pysym::constant("AccumBounds")?])? } } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the limit does not exist or could not be determined"))));
    }
    if (py::method(&v_L.clone(), "has", &[ax_engine::pysym::constant("oo")?, py::neg(&ax_engine::pysym::constant("oo")?)?, ax_engine::pysym::constant("zoo")?, ax_engine::pysym::constant("nan")?])?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the limit is infinite (it does not exist as a finite number)"))));
    }
    v_v = py::complex(&[ax_engine::pysym::call("N", &[v_L.clone(), Py::Int(30i64)], &[])?])?;
    if ({ let t1225 = { let t1223 = py::abs(&py::imag(&v_v.clone())?)?; Py::Bool({ let t1224 = py::mul(&Py::Float(1e-20), &py::minmax(&[Py::Float(1.0), py::abs(&py::real(&v_v.clone())?)?], true)?)?; if py::gt(&t1223, &t1224)? { true } else { false } }) }; if t1225.truthy() { t1225 } else { py::not(&py::math1("isfinite", &py::real(&v_v.clone())?)?) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the limit is not a finite real number"))));
    }
    return Ok(py::real(&v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mp_partial(mut v_text: Py, mut v_names: Py, mut v_point: Py, mut v_orders: Py) -> R {
    let mut v_g: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        v_g = ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::helper("calculus:parse_expr", &[v_text.clone(), v_names.clone()], &[])?, v_names.clone()], &[])?;
        return Ok(ax_engine::pymp::call("diff", &[v_g.clone(), Py::tuple(({ let mut out = vec![]; for t1390 in py::iter(&(v_point.clone()))? { let mut v_v: Py; 
v_v = t1390;
out.push(ax_engine::pymp::call("mpf", &[v_v.clone()], &[])?); } Py::list(out) }).items()?), Py::tuple((v_orders.clone()).items()?)], &[])?);
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mpq(mut v_q: Py) -> R {
    return Ok(py::truediv(&ax_engine::pymp::call("mpf", &[py::getattr(&v_q.clone(), "p")?], &[])?, &py::getattr(&v_q.clone(), "q")?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__n_roots_open(mut v_q: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_s: Py = Py::None;
    if (py::getattr(&v_q.clone(), "is_zero")?).truthy() {
        return Ok(py::neg(&Py::Int(1i64))?);
    }
    v_s = ax_engine::pysym::call("Poly", &[ax_engine::pysym::call("sqf_part", &[py::method(&v_q.clone(), "as_expr", &[])?], &[])?, c_X()?], &[])?;
    if ({ let t930 = py::method(&v_s.clone(), "degree", &[])?; Py::Bool({ let t931 = Py::Int(1i64); if py::lt(&t930, &t931)? { true } else { false } }) }).truthy() {
        return Ok(Py::Int(0i64));
    }
    if ({ let t932 = v_a.clone(); Py::Bool({ let t933 = Py::None; if py::is(&t932, &t933) { true } else { false } }) }).truthy() {
        return Ok(py::method(&v_s.clone(), "count_roots", &[])?);
    }
    let t934 = (Py::tuple(vec![f__rat(v_a.clone())?, f__rat(v_b.clone())?])).items()?;
    if t934.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t934[0].clone();
    v_B = t934[1].clone();
    return Ok(py::sub(&py::sub(&py::method(&v_s.clone(), "count_roots", &[v_A.clone(), v_B.clone()])?, &py::int(&{ let t935 = py::method(&v_s.clone(), "eval", &[v_A.clone()])?; Py::Bool({ let t936 = Py::Int(0i64); if py::eq(&t935, &t936) { true } else { false } }) })?)?, &py::int(&{ let t937 = py::method(&v_s.clone(), "eval", &[v_B.clone()])?; Py::Bool({ let t938 = Py::Int(0i64); if py::eq(&t937, &t938) { true } else { false } }) })?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg_on(mut v_p: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_pts: Py = Py::None;
    let mut v_tests: Py = Py::None;
    v_pts = py::add(&py::add(&Py::list(vec![ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_a.clone())?))], &[])?]), &ax_engine::pysym::helper("calculus:real_roots_in", &[v_p.clone(), v_a.clone(), v_b.clone()], &[])?)?, &Py::list(vec![ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_b.clone())?))], &[])?]))?;
    v_tests = py::add(&Py::list(vec![py::getitem(&v_pts.clone(), &Py::Int(0i64))?, py::getitem(&v_pts.clone(), &py::neg(&Py::Int(1i64))?)?]), &{ let mut out = vec![]; for t1518 in py::range_iter(&[(py::sub(&py::len(&v_pts.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1518;
out.push(py::truediv(&py::add(&py::getitem(&v_pts.clone(), &v_k.clone())?, &py::getitem(&v_pts.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?, &Py::Int(2i64))?); } Py::list(out) })?;
    return Ok(Py::Bool(({ let mut out = vec![]; for t1519 in py::iter(&(v_tests.clone()))? { let mut v_t: Py; 
v_t = t1519;
out.push((if (py::getattr(&v_t.clone(), "is_Rational")?).truthy() { { let t1520 = py::method(&v_p.clone(), "eval", &[ax_engine::pysym::call("nsimplify", &[v_t.clone()], &[])?])?; Py::Bool({ let t1521 = Py::Int(0i64); if py::ge(&t1520, &t1521)? { true } else { false } }) } } else { { let t1522 = py::float(&ax_engine::pysym::call("N", &[py::method(&py::method(&v_p.clone(), "as_expr", &[])?, "subs", &[py::getattr(&v_p.clone(), "gen")?, v_t.clone()])?, Py::Int(30i64)], &[])?)?; Py::Bool({ let t1523 = Py::Int(0i64); if py::ge(&t1522, &t1523)? { true } else { false } }) } })); } Py::list(out) }).items()?.iter().all(|x| x.truthy())));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__path(mut v_xt: Py, mut v_yt: Py) -> R {
    return Ok(Py::tuple(vec![ax_engine::pysym::helper("calculus:parse_expr", &[v_xt.clone(), Py::tuple(vec![Py::str("t")])], &[])?, ax_engine::pysym::helper("calculus:parse_expr", &[v_yt.clone(), Py::tuple(vec![Py::str("t")])], &[])?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__peval(mut v_p: Py, mut v_v: Py) -> R {
    return Ok(py::float(&py::method(&v_p.clone(), "eval", &[f__rat(v_v.clone())?])?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pfloat(mut v_v: Py) -> R {
    return Ok(f__fin(ax_engine::pysym::call("N", &[v_v.clone(), Py::Int(30i64)], &[])?, Py::str("the result"))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__poly_abs_integral(mut v_h: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_H: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_tot: Py = Py::None;
    v_H = py::method(&v_h.clone(), "integrate", &[])?;
    v_pts = py::add(&py::add(&Py::list(vec![ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_a.clone())?))], &[])?]), &ax_engine::pysym::helper("calculus:real_roots_in", &[v_h.clone(), v_a.clone(), v_b.clone()], &[])?)?, &Py::list(vec![ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_b.clone())?))], &[])?]))?;
    v_tot = Py::Int(0i64);
    for t13 in py::range_iter(&[(py::sub(&py::len(&v_pts.clone())?, &Py::Int(1i64))?).i()?]) {
        v_k = t13;
        v_tot = py::iadd(&v_tot, &ax_engine::pysym::call("Abs", &[py::sub(&py::method(&py::method(&v_H.clone(), "as_expr", &[])?, "subs", &[py::getattr(&v_h.clone(), "gen")?, py::getitem(&v_pts.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?])?, &py::method(&py::method(&v_H.clone(), "as_expr", &[])?, "subs", &[py::getattr(&v_h.clone(), "gen")?, py::getitem(&v_pts.clone(), &v_k.clone())?])?)?], &[])?)?;
    }
    return Ok(Py::tuple(vec![py::float(&ax_engine::pysym::call("N", &[v_tot.clone(), Py::Int(30i64)], &[])?)?, { let mut out = vec![]; for t14 in py::iter(&(py::slice(&v_pts.clone(), Some((Py::Int(1i64)).i()?), Some((py::neg(&Py::Int(1i64))?).i()?), None)?))? { let mut v_r: Py; 
v_r = t14;
out.push(py::float(&ax_engine::pysym::call("N", &[v_r.clone(), Py::Int(30i64)], &[])?)?); } Py::list(out) }]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__q_t(mut v_e: Py, mut v_t0: Py, mut v_t1: Py) -> R {
    if ({ let t1054 = v_t0.clone(); Py::Bool({ let t1055 = v_t1.clone(); if py::eq(&t1054, &t1055) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    let _ = ax_engine::pysym::helper("calculus:continuous_on", &[v_e.clone(), c_T()?, v_t0.clone(), v_t1.clone()], &[])?;
    return Ok(ax_engine::pysym::helper("calculus:quad", &[ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_T()?])], &[])?, v_t0.clone(), v_t1.clone()], &[])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rat(mut v_v: Py) -> R {
    return Ok(ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_v.clone())?))], &[])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__reg_gamma(mut v_a: Py, mut v_x: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_an: Py = Py::None;
    let mut v_ap: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_de: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lead: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_tiny: Py = Py::None;
    if ({ let t1144 = v_x.clone(); Py::Bool({ let t1145 = Py::Int(0i64); if py::eq(&t1144, &t1145) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)]));
    }
    v_lead = py::math1("exp", &py::sub(&py::add(&py::neg(&v_x.clone())?, &py::mul(&v_a.clone(), &py::math1("log", &v_x.clone())?)?)?, &py::math1("lgamma", &v_a.clone())?)?)?;
    if ({ let t1146 = v_x.clone(); Py::Bool({ let t1147 = py::add(&v_a.clone(), &Py::Int(1i64))?; if py::lt(&t1146, &t1147)? { true } else { false } }) }).truthy() {
        let t1148 = (Py::tuple(vec![v_a.clone(), py::truediv(&Py::Float(1.0), &v_a.clone())?])).items()?;
        if t1148.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ap = t1148[0].clone();
        v_d = t1148[1].clone();
        v_s = v_d.clone();
        for t1149 in py::range_iter(&[(Py::Int(100000i64)).i()?]) {
            v__ = t1149;
            v_ap = py::iadd(&v_ap, &Py::Int(1i64))?;
            v_d = py::mul(&v_d, &py::truediv(&v_x.clone(), &v_ap.clone())?)?;
            v_s = py::iadd(&v_s, &v_d.clone())?;
            if ({ let t1150 = py::abs(&v_d.clone())?; Py::Bool({ let t1151 = py::mul(&py::abs(&v_s.clone())?, &Py::Float(1e-17))?; if py::lt(&t1150, &t1151)? { true } else { false } }) }).truthy() {
                break;
            }
        }
        v_P = py::mul(&v_s.clone(), &v_lead.clone())?;
        return Ok(Py::tuple(vec![v_P.clone(), py::sub(&Py::Float(1.0), &v_P.clone())?]));
    }
    v_tiny = Py::Float(1e-300);
    v_b = py::sub(&py::add(&v_x.clone(), &Py::Int(1i64))?, &v_a.clone())?;
    let t1152 = (Py::tuple(vec![py::truediv(&Py::Int(1i64), &v_tiny.clone())?, py::truediv(&Py::Int(1i64), &v_b.clone())?])).items()?;
    if t1152.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t1152[0].clone();
    v_d = t1152[1].clone();
    v_h = v_d.clone();
    for t1153 in py::range_iter(&[(Py::Int(1i64)).i()?, (Py::Int(100000i64)).i()?]) {
        v_k = t1153;
        v_an = py::mul(&py::neg(&v_k.clone())?, &py::sub(&v_k.clone(), &v_a.clone())?)?;
        v_b = py::iadd(&v_b, &Py::Int(2i64))?;
        v_d = py::add(&py::mul(&v_an.clone(), &v_d.clone())?, &v_b.clone())?;
        v_d = (if ({ let t1154 = py::abs(&v_d.clone())?; Py::Bool({ let t1155 = v_tiny.clone(); if py::lt(&t1154, &t1155)? { true } else { false } }) }).truthy() { v_tiny.clone() } else { v_d.clone() });
        v_c = py::add(&v_b.clone(), &py::truediv(&v_an.clone(), &v_c.clone())?)?;
        v_c = (if ({ let t1156 = py::abs(&v_c.clone())?; Py::Bool({ let t1157 = v_tiny.clone(); if py::lt(&t1156, &t1157)? { true } else { false } }) }).truthy() { v_tiny.clone() } else { v_c.clone() });
        v_d = py::truediv(&Py::Int(1i64), &v_d.clone())?;
        v_de = py::mul(&v_d.clone(), &v_c.clone())?;
        v_h = py::mul(&v_h, &v_de.clone())?;
        if ({ let t1158 = py::abs(&py::sub(&v_de.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t1159 = Py::Float(3e-16); if py::lt(&t1158, &t1159)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    v_Q = py::mul(&v_lead.clone(), &v_h.clone())?;
    return Ok(Py::tuple(vec![py::sub(&Py::Float(1.0), &v_Q.clone())?, v_Q.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__root_res(mut v_q: Py, mut v_c: Py) -> R {
    let mut v_co: Py = Py::None;
    let mut v_deg: Py = Py::None;
    let mut v_scale: Py = Py::None;
    v_co = py::method(&v_q.clone(), "all_coeffs", &[])?;
    v_deg = py::sub(&py::len(&v_co.clone())?, &Py::Int(1i64))?;
    v_scale = { let t950 = py::sum(&{ let mut out = vec![]; for t948 in py::iter(&(py::enumerate(&v_co.clone(), 0)?))? { let mut v_j: Py; let mut v_k: Py; 
let t949 = (t948).items()?;
if t949.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t949[0].clone();
v_k = t949[1].clone();
out.push(py::mul(&py::abs(&py::float(&v_k.clone())?)?, &py::pow(&py::minmax(&[Py::Float(1.0), py::abs(&v_c.clone())?], true)?, &py::sub(&v_deg.clone(), &v_j.clone())?)?)?); } Py::list(out) }, None)?; if t950.truthy() { t950 } else { Py::Float(1.0) } };
    return Ok(py::minmax(&[Py::Float(0.0), py::sub(&py::truediv(&py::abs(&py::float(&py::method(&v_q.clone(), "eval", &[f__rat(v_c.clone())?])?)?)?, &v_scale.clone())?, &Py::Float(1e-12))?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rule_n(mut v_e: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py, mut v_kind: Py) -> R {
    let mut v_h: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_h = py::truediv(&py::sub(&v_b.clone(), &v_a.clone())?, &v_n.clone())?;
    if ({ let t1435 = v_kind.clone(); Py::Bool({ let t1436 = Py::str("trapezoid"); if py::eq(&t1435, &t1436) { true } else { false } }) }).truthy() {
        let t1438 = (Py::tuple(vec![{ let mut out = vec![]; for t1437 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1437;
out.push(py::add(&v_a.clone(), &py::mul(&v_k.clone(), &v_h.clone())?)?); } Py::list(out) }, py::add(&py::add(&Py::list(vec![py::truediv(&v_h.clone(), &Py::Int(2i64))?]), &py::mul(&Py::list(vec![v_h.clone()]), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?, &Py::list(vec![py::truediv(&v_h.clone(), &Py::Int(2i64))?]))?])).items()?;
        if t1438.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_pts = t1438[0].clone();
        v_w = t1438[1].clone();
    } else {
        if ({ let t1439 = v_kind.clone(); Py::Bool({ let t1440 = Py::str("simpson"); if py::eq(&t1439, &t1440) { true } else { false } }) }).truthy() {
            v_pts = { let mut out = vec![]; for t1441 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1441;
out.push(py::add(&v_a.clone(), &py::mul(&v_k.clone(), &v_h.clone())?)?); } Py::list(out) };
            v_w = { let mut out = vec![]; for t1442 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1442;
out.push(py::mul(&py::truediv(&v_h.clone(), &Py::Int(3i64))?, &(if ({ let t1443 = v_k.clone(); Py::Bool({ let t1444 = Py::tuple(vec![Py::Int(0i64), v_n.clone()]); if py::contains(&t1444, &t1443)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { (if (py::modulo(&v_k.clone(), &Py::Int(2i64))?).truthy() { Py::Int(4i64) } else { Py::Int(2i64) }) }))?); } Py::list(out) };
        } else {
            let t1446 = (Py::tuple(vec![{ let mut out = vec![]; for t1445 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t1445;
out.push(py::add(&v_a.clone(), &py::mul(&py::add(&v_k.clone(), &Py::Float(0.5))?, &v_h.clone())?)?); } Py::list(out) }, py::mul(&Py::list(vec![v_h.clone()]), &v_n.clone())?])).items()?;
            if t1446.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_pts = t1446[0].clone();
            v_w = t1446[1].clone();
        }
    }
    return Ok(Py::tuple(vec![v_pts.clone(), v_w.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__same(mut v_a: Py, mut v_b: Py) -> R {
    return Ok((if ({ let t695 = v_a.clone(); Py::Bool({ let t696 = v_b.clone(); if py::eq(&t695, &t696) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__scal_integrand(mut v_i: Py) -> R {
    let mut v_ef: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let mut v_ey: Py = Py::None;
    let t885 = (f__path(py::getitem(&v_i.clone(), &Py::str("xt"))?, py::getitem(&v_i.clone(), &Py::str("yt"))?)?).items()?;
    if t885.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ex = t885[0].clone();
    v_ey = t885[1].clone();
    v_ef = py::method(&ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?, c__XY()?], &[])?, "subs", &[Py::dict(vec![(c_X()?, v_ex.clone()), (c_Y()?, v_ey.clone())])])?;
    return Ok(py::mul(&v_ef.clone(), &ax_engine::pysym::call("sqrt", &[py::add(&py::pow(&ax_engine::pysym::call("diff", &[v_ex.clone(), c_T()?], &[])?, &Py::Int(2i64))?, &py::pow(&ax_engine::pysym::call("diff", &[v_ey.clone(), c_T()?], &[])?, &Py::Int(2i64))?)?], &[])?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__scaled_err(mut v_got: Py, mut v_want: Py, mut v_S: Py, mut v_slack: Py) -> R {
    let mut v_d: Py = Py::None;
    v_d = py::truediv(&py::abs(&py::sub(&py::float(&v_got.clone())?, &py::float(&v_want.clone())?)?)?, &py::minmax(&[py::abs(&py::float(&v_want.clone())?)?, py::mul(&Py::Float(0.001), &v_S.clone())?, Py::Float(1e-300)], true)?)?;
    return Ok(py::minmax(&[Py::Float(0.0), py::sub(&v_d.clone(), &v_slack.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__series_terms(mut v_a: Py) -> R {
    return Ok(py::method(&ax_engine::pysym::helper("calculus:parse_expr", &[v_a.clone(), Py::tuple(vec![Py::str("n")])], &[])?, "subs", &[c_NN()?, c__KS()?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__settled(mut v_fn: Py, mut v_absfn: Py, mut v_ns: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_prev: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_S = py::minmax(&[py::call(&v_absfn.clone(), &[py::getitem(&v_ns.clone(), &Py::Int(0i64))?])?, Py::Float(1e-300)], true)?;
    v_prev = py::call(&v_fn.clone(), &[py::getitem(&v_ns.clone(), &Py::Int(0i64))?])?;
    for t1470 in py::iter(&(py::slice(&v_ns.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? {
        v_n = t1470;
        v_v = py::call(&v_fn.clone(), &[v_n.clone()])?;
        if ({ let t1471 = py::abs(&py::sub(&v_v.clone(), &v_prev.clone())?)?; Py::Bool({ let t1472 = py::mul(&Py::Float(1e-12), &v_S.clone())?; if py::le(&t1471, &t1472)? { true } else { false } }) }).truthy() {
            return Ok(Py::tuple(vec![v_v.clone(), v_S.clone()]));
        }
        v_prev = v_v.clone();
    }
    return Err(py::value_error(py::to_str(&Py::str("the integral did not settle numerically (singular or strongly oscillating integrand)"))));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__side_signs(mut v_q: Py, mut v_xs: Py) -> R {
    let mut v_cuts: Py = Py::None;
    let mut v_fl: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_fl = { let mut out = vec![]; for t734 in py::iter(&(v_xs.clone()))? { let mut v_v: Py; 
v_v = t734;
out.push(py::float(&v_v.clone())?); } Py::list(out) };
    v_cuts = py::add(&py::add(&Py::list(vec![py::sub(&py::getitem(&v_fl.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?]), &{ let mut out = vec![]; for t735 in py::range_iter(&[(py::sub(&py::len(&v_fl.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t735;
out.push(py::truediv(&py::add(&py::getitem(&v_fl.clone(), &v_k.clone())?, &py::getitem(&v_fl.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?, &Py::Int(2i64))?); } Py::list(out) })?, &Py::list(vec![py::add(&py::getitem(&v_fl.clone(), &py::neg(&Py::Int(1i64))?)?, &Py::Int(1i64))?]))?;
    v_s = { let mut out = vec![]; for t736 in py::iter(&(v_cuts.clone()))? { let mut v_c: Py; 
v_c = t736;
out.push(py::int(&ax_engine::pysym::call("sign", &[py::method(&v_q.clone(), "eval", &[f__rat(v_c.clone())?])?], &[])?)?); } Py::list(out) };
    return Ok({ let mut out = vec![]; for t737 in py::range_iter(&[(py::len(&v_fl.clone())?).i()?]) { let mut v_k: Py; 
v_k = t737;
out.push(Py::tuple(vec![py::getitem(&v_s.clone(), &v_k.clone())?, py::getitem(&v_s.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?])); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sign_changes(mut v_g: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_flo: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_ys: Py = Py::None;
    v_xs = { let mut out = vec![]; for t1349 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1349;
out.push(py::add(&v_a.clone(), &py::truediv(&py::mul(&py::sub(&v_b.clone(), &v_a.clone())?, &v_k.clone())?, &v_n.clone())?)?); } Py::list(out) };
    v_ys = { let mut out = vec![]; for t1350 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t1350;
out.push(py::call(&v_g.clone(), &[v_x.clone()])?); } Py::list(out) };
    v_out = Py::list(vec![]);
    for t1351 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t1351;
        let t1352 = (Py::tuple(vec![py::getitem(&v_xs.clone(), &v_k.clone())?, py::getitem(&v_xs.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, py::getitem(&v_ys.clone(), &v_k.clone())?])).items()?;
        if t1352.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_lo = t1352[0].clone();
        v_hi = t1352[1].clone();
        v_flo = t1352[2].clone();
        if ({ let t1355 = { let t1353 = v_flo.clone(); Py::Bool({ let t1354 = Py::Int(0i64); if py::eq(&t1353, &t1354) { true } else { false } }) }; if !t1355.truthy() { t1355 } else { v_k.clone() } }).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[v_lo.clone()])?;
        } else {
            if ({ let t1356 = py::mul(&v_flo.clone(), &py::getitem(&v_ys.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t1357 = Py::Int(0i64); if py::lt(&t1356, &t1357)? { true } else { false } }) }).truthy() {
                for t1358 in py::range_iter(&[(Py::Int(80i64)).i()?]) {
                    v__ = t1358;
                    v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
                    v_fm = py::call(&v_g.clone(), &[v_mid.clone()])?;
                    if ({ let t1359 = v_fm.clone(); Py::Bool({ let t1360 = Py::Int(0i64); if py::eq(&t1359, &t1360) { true } else { false } }) }).truthy() {
                        let t1361 = v_mid.clone();
                        v_lo = t1361.clone();
                        v_hi = t1361.clone();
                        break;
                    }
                    if ({ let t1362 = { let t1364 = v_fm.clone(); Py::Bool({ let t1365 = Py::Int(0i64); if py::lt(&t1364, &t1365)? { true } else { false } }) }; Py::Bool({ let t1363 = { let t1366 = v_flo.clone(); Py::Bool({ let t1367 = Py::Int(0i64); if py::lt(&t1366, &t1367)? { true } else { false } }) }; if py::eq(&t1362, &t1363) { true } else { false } }) }).truthy() {
                        let t1368 = (Py::tuple(vec![v_mid.clone(), v_fm.clone()])).items()?;
                        if t1368.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                        v_lo = t1368[0].clone();
                        v_flo = t1368[1].clone();
                    } else {
                        v_hi = v_mid.clone();
                    }
                }
                let _ = py::method(&v_out.clone(), "append", &[py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?])?;
            }
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sympy_limit(mut v_e: Py, mut v_var: Py, mut v_point: Py, mut v_d: Py) -> R {
    let mut v_exc: Py = Py::None;
    let t1124: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            return Ok(Some((if ({ let t1122 = v_d.clone(); Py::Bool({ let t1123 = Py::None; if py::is(&t1122, &t1123) { true } else { false } }) }).truthy() { ax_engine::pysym::call("limit", &[v_e.clone(), v_var.clone(), v_point.clone()], &[])? } else { ax_engine::pysym::call("limit", &[v_e.clone(), v_var.clone(), v_point.clone(), v_d.clone()], &[])? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1124 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            v_exc = Py::str(&match &e { PyErr::Value(m) => m.clone(), other => format!("{:?}", other) });
            return Err(py::value_error(py::to_str(&{ let t1125 = py::getitem(&py::method(&Py::str(&py::to_str(&v_exc.clone())), "split", &[Py::str(".")])?, &Py::Int(0i64))?; if t1125.truthy() { t1125 } else { Py::str("the limit does not exist") } })));
        }
        Err(e) if (matches!(&e, PyErr::Other(c) if c.as_str() == "NotImplementedError")) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(&e, PyErr::Other(c) if c.as_str() == "RecursionError")) || (matches!(e, PyErr::ZeroDivision)) || (matches!(&e, PyErr::Other(c) if c.as_str() == "AttributeError")) => {
            return Err(py::value_error(py::to_str(&Py::str("the limit could not be determined"))));
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__values(mut v_e: Py, mut v_pts: Py) -> R {
    let mut v_g: Py = Py::None;
    v_g = ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[])?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        return Ok({ let mut out = vec![]; for t1448 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t1448;
out.push(py::call(&v_g.clone(), &[ax_engine::pymp::call("mpf", &[v_p.clone()], &[])?])?); } Py::list(out) });
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vars(mut v_p: Py) -> R {
    if ({ let t1381 = py::len(&v_p.clone())?; Py::Bool({ let t1382 = Py::Int(2i64); if py::eq(&t1381, &t1382) { true } else { false } }) }).truthy() {
        return Ok(c__XY()?);
    }
    if ({ let t1383 = py::len(&v_p.clone())?; Py::Bool({ let t1384 = Py::Int(3i64); if py::eq(&t1383, &t1384) { true } else { false } }) }).truthy() {
        return Ok(c__XYZ()?);
    }
    return Err(py::value_error(py::to_str(&Py::str("the point needs 2 or 3 coordinates"))));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vec_err(mut v_got: Py, mut v_want: Py, mut v_slack: Py) -> R {
    let t1410 = (Py::tuple(vec![Py::list((v_got.clone()).items()?), Py::list((v_want.clone()).items()?)])).items()?;
    if t1410.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_got = t1410[0].clone();
    v_want = t1410[1].clone();
    if ({ let t1411 = py::len(&v_got.clone())?; Py::Bool({ let t1412 = py::len(&v_want.clone())?; if py::ne(&t1411, &t1412) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(f_worst({ let mut v = vec![]; v.extend(({ let mut out = vec![]; for t1413 in py::iter(&(py::zip(&[v_got.clone(), v_want.clone()])?))? { let mut v_g: Py; let mut v_w: Py; 
let t1414 = (t1413).items()?;
if t1414.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_g = t1414[0].clone();
v_w = t1414[1].clone();
out.push(f_rel_err(v_g.clone(), v_w.clone(), v_slack.clone())?); } Py::list(out) }).items()?); Py::tuple(v) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vec_integrand(mut v_i: Py) -> R {
    let mut v_eP: Py = Py::None;
    let mut v_eQ: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let mut v_ey: Py = Py::None;
    let t901 = (f__path(py::getitem(&v_i.clone(), &Py::str("xt"))?, py::getitem(&v_i.clone(), &Py::str("yt"))?)?).items()?;
    if t901.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ex = t901[0].clone();
    v_ey = t901[1].clone();
    let t903 = ({ let mut out = vec![]; for t902 in py::iter(&(Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("P"))?, py::getitem(&v_i.clone(), &Py::str("Q"))?])))? { let mut v_c: Py; 
v_c = t902;
out.push(py::method(&ax_engine::pysym::helper("calculus:parse_expr", &[v_c.clone(), c__XY()?], &[])?, "subs", &[Py::dict(vec![(c_X()?, v_ex.clone()), (c_Y()?, v_ey.clone())])])?); } Py::list(out) }).items()?;
    if t903.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_eP = t903[0].clone();
    v_eQ = t903[1].clone();
    return Ok(py::add(&py::mul(&v_eP.clone(), &ax_engine::pysym::call("diff", &[v_ex.clone(), c_T()?], &[])?)?, &py::mul(&v_eQ.clone(), &ax_engine::pysym::call("diff", &[v_ey.clone(), c_T()?], &[])?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_abs_area(mut v_f: Py, mut v_g: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_co: Py = Py::None;
    let mut v_cuts: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_knots: Py = Py::None;
    let mut v_rs: Py = Py::None;
    v_co = { let mut out = vec![]; for t21 in py::iter(&(py::method(&py::sub(&ax_engine::pysym::helper("calculus:poly_of", &[v_f.clone()], &[])?, &ax_engine::pysym::helper("calculus:poly_of", &[v_g.clone()], &[])?)?, "all_coeffs", &[])?))? { let mut v_c: Py; 
v_c = t21;
out.push(py::truediv(&ax_engine::pymp::call("mpf", &[py::getattr(&v_c.clone(), "p")?], &[])?, &py::getattr(&v_c.clone(), "q")?)?); } Py::list(out) };
    let t31: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            {
                let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
                v_rs = (if ({ let t22 = py::len(&v_co.clone())?; Py::Bool({ let t23 = Py::Int(1i64); if py::gt(&t22, &t23)? { true } else { false } }) }).truthy() { ax_engine::pymp::call("polyroots", &[v_co.clone()], &[(String::from("maxsteps"), Py::Int(400i64)), (String::from("extraprec"), Py::Int(120i64))])? } else { Py::list(vec![]) });
            }
            v_cuts = py::sorted(&{ let mut out = vec![]; for t24 in py::iter(&(v_rs.clone()))? { let mut v_r: Py; 
v_r = t24;
if ({ let t30 = { let t25 = py::abs(&ax_engine::pymp::call("im", &[v_r.clone()], &[])?)?; Py::Bool({ let t26 = Py::Float(1e-20); if py::lt(&t25, &t26)? { true } else { false } }) }; if !t30.truthy() { t30 } else { { let t27 = v_a.clone(); Py::Bool({ let t28 = py::float(&ax_engine::pymp::call("re", &[v_r.clone()], &[])?)?; if py::lt(&t27, &t28)? { { let t29 = v_b.clone(); if py::lt(&t28, &t29)? { true } else { false } } } else { false } }) } } }).truthy() { out.push(py::float(&ax_engine::pymp::call("re", &[v_r.clone()], &[])?)?); }} Py::list(out) }, (Py::Bool(false)).truthy())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t31 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            v_cuts = { let mut out = vec![]; for t32 in py::iter(&(ax_engine::pysym::helper("calculus:real_roots_in", &[py::sub(&ax_engine::pysym::helper("calculus:poly_of", &[v_f.clone()], &[])?, &ax_engine::pysym::helper("calculus:poly_of", &[v_g.clone()], &[])?)?, v_a.clone(), v_b.clone()], &[])?))? { let mut v_r: Py; 
v_r = t32;
out.push(py::float(&v_r.clone())?); } Py::list(out) };
        }
        Err(e) => return Err(e),
    }
    v_knots = py::add(&py::add(&Py::list(vec![v_a.clone()]), &v_cuts.clone())?, &Py::list(vec![v_b.clone()]))?;
    v_h = Py::str(&["(".to_string(), py::format(&v_f.clone(), "")?, ") - (".to_string(), py::format(&v_g.clone(), "")?, ")".to_string()].concat());
    return Ok(py::sum(&{ let mut out = vec![]; for t33 in py::iter(&(py::zip(&[py::slice(&v_knots.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, py::slice(&v_knots.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_hi: Py; let mut v_lo: Py; 
let t34 = (t33).items()?;
if t34.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_lo = t34[0].clone();
v_hi = t34[1].clone();
out.push(py::abs(&f__gl_check(v_h.clone(), v_lo.clone(), v_hi.clone(), Py::tuple(vec![Py::str("x")]), Py::None)?)?); } Py::list(out) }, None)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_classify(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_cs: Py = Py::None;
    let mut v_dp: Py = Py::None;
    let mut v_left: Py = Py::None;
    let mut v_mn: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_right: Py = Py::None;
    v_p = ax_engine::pysym::helper("calculus:poly_of", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?;
    v_dp = py::method(&v_p.clone(), "diff", &[c_X()?])?;
    v_cs = Py::list((py::getitem(&v_o.clone(), &Py::str("critical_points"))?).items()?);
    if ({ let t182 = { let t169 = py::len(&v_cs.clone())?; Py::Bool({ let t170 = f__n_roots_open(v_dp.clone(), Py::None, Py::None)?; if py::ne(&t169, &t170) { true } else { false } }) }; if t182.truthy() { t182 } else { { let t180 = v_cs.clone(); Py::Bool({ let t181 = py::sorted(&v_cs.clone(), (Py::Bool(false)).truthy())?; if py::ne(&t180, &t181) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    let t183 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t183.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t183[0].clone();
    v_mn = t183[1].clone();
    for t184 in py::iter(&(py::zip(&[v_cs.clone(), (if (v_cs.clone()).truthy() { f__side_signs(v_dp.clone(), v_cs.clone())? } else { Py::list(vec![]) })])?))? {
        let t191 = (t184).items()?;
        if t191.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t191[0].clone();
        let t192 = (t191[1].clone()).items()?;
        if t192.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_left = t192[0].clone();
        v_right = t192[1].clone();
        let _ = py::method(&(if ({ let t185 = v_left.clone(); Py::Bool({ let t186 = Py::Int(0i64); if py::gt(&t185, &t186)? { { let t187 = v_right.clone(); if py::gt(&t186, &t187)? { true } else { false } } } else { false } }) }).truthy() { v_mx.clone() } else { (if ({ let t188 = v_left.clone(); Py::Bool({ let t189 = Py::Int(0i64); if py::lt(&t188, &t189)? { { let t190 = v_right.clone(); if py::lt(&t189, &t190)? { true } else { false } } } else { false } }) }).truthy() { v_mn.clone() } else { Py::list(vec![]) }) }), "append", &[v_c.clone()])?;
    }
    return Ok({ let mut v: Vec<Py> = vec![]; v.extend(({ let mut out = vec![]; for t193 in py::iter(&(v_cs.clone()))? { let mut v_c: Py; 
v_c = t193;
out.push(f__root_res(v_dp.clone(), v_c.clone())?); } Py::list(out) }).items()?); v.push(f__vec_err(py::getitem(&v_o.clone(), &Py::str("local_maxima"))?, v_mx.clone(), Py::Int(0i64))?); v.push(f__vec_err(py::getitem(&v_o.clone(), &Py::str("local_minima"))?, v_mn.clone(), Py::Int(0i64))?); let mut d: Vec<(Py, Py)> = vec![];   if !d.is_empty() { return Err(PyErr::Type("worst() got an unexpected keyword argument".into())) } f_worst(Py::tuple(v.get(0..).map(|s| s.to_vec()).unwrap_or_default()))? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_dbl(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_g: Py = Py::None;
    v_g = f__fl(ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?, c__XY()?], &[])?, c__XY()?)?;
    let t414 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("c"))?, py::getitem(&v_i.clone(), &Py::str("d"))?])).items()?;
    if t414.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t414[0].clone();
    v_d = t414[1].clone();
    v_S = f__gl2({ let v_c = v_c.clone(); let v_d = v_d.clone(); let v_g = v_g.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_y: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::call(&v_g.clone(), &[v_x.clone(), v_y.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, { let v_c = v_c.clone(); let v_d = v_d.clone(); let v_g = v_g.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_c.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_c = v_c.clone(); let v_d = v_d.clone(); let v_g = v_g.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_d.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Int(32i64))?;
    return Ok(f__scaled_err(py::getitem(&v_o.clone(), &Py::str("integral"))?, f__gl2(v_g.clone(), py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, { let v_S = v_S.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_g = v_g.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_c.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_S = v_S.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_g = v_g.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_d.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Int(32i64))?, v_S.clone(), Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_dblr(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    v_g = f__fl(ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?, c__XY()?], &[])?, c__XY()?)?;
    let t454 = (Py::tuple(vec![f__fl(ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("g1"))?], &[])?, Py::tuple(vec![Py::str("x")]))?, f__fl(ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("g2"))?], &[])?, Py::tuple(vec![Py::str("x")]))?])).items()?;
    if t454.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t454[0].clone();
    v_hi = t454[1].clone();
    v_S = f__gl2({ let v_g = v_g.clone(); let v_hi = v_hi.clone(); let v_i = v_i.clone(); let v_lo = v_lo.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_y: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::call(&v_g.clone(), &[v_x.clone(), v_y.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, v_lo.clone(), v_hi.clone(), Py::Int(32i64))?;
    return Ok(f__scaled_err(py::getitem(&v_o.clone(), &Py::str("integral"))?, f__gl2(v_g.clone(), py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, v_lo.clone(), v_hi.clone(), Py::Int(32i64))?, v_S.clone(), Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_dirder(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_nu: Py = Py::None;
    let mut v_u: Py = Py::None;
    let t278 = (Py::tuple(vec![f__vars(py::getitem(&v_i.clone(), &Py::str("p"))?)?, py::getitem(&v_i.clone(), &Py::str("u"))?])).items()?;
    if t278.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_names = t278[0].clone();
    v_u = t278[1].clone();
    v_nu = py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t279 in py::iter(&(v_u.clone()))? { let mut v_c: Py; 
v_c = t279;
out.push(py::mul(&v_c.clone(), &v_c.clone())?); } Py::list(out) })?)?;
    v_g = { let mut out = vec![]; for t280 in py::range_iter(&[(py::len(&v_names.clone())?).i()?]) { let mut v_j: Py; 
v_j = t280;
out.push(py::float(&f__mp_partial(py::getitem(&v_i.clone(), &Py::str("f"))?, v_names.clone(), py::getitem(&v_i.clone(), &Py::str("p"))?, { let mut out = vec![]; for t282 in py::range_iter(&[(py::len(&v_names.clone())?).i()?]) { let mut v_k: Py; 
v_k = t282;
out.push(py::int(&{ let t283 = v_k.clone(); Py::Bool({ let t284 = v_j.clone(); if py::eq(&t283, &t284) { true } else { false } }) })?); } Py::list(out) })?)?); } Py::list(out) };
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("directional_derivative"))?, py::truediv(&py::fsum(&{ let mut out = vec![]; for t288 in py::iter(&(py::zip(&[v_g.clone(), v_u.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t289 = (t288).items()?;
if t289.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t289[0].clone();
v_b = t289[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?, &v_nu.clone())?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_divthm(mut v_i: Py, mut v_o: Py) -> R {
    let mut v__n: Py = Py::None;
    let mut v_bx: Py = Py::None;
    let mut v_eP: Py = Py::None;
    let mut v_eQ: Py = Py::None;
    let mut v_eR: Py = Py::None;
    let mut v_g: Py = Py::None;
    let t375 = (f__field(py::getitem(&v_i.clone(), &Py::str("P"))?, py::getitem(&v_i.clone(), &Py::str("Q"))?, py::getitem(&v_i.clone(), &Py::str("R"))?)?).items()?;
    if t375.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t376 = (t375[0].clone()).items()?;
    if t376.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_eP = t376[0].clone();
    v_eQ = t376[1].clone();
    v_eR = t376[2].clone();
    v__n = t375[1].clone();
    v_g = f__fl(py::add(&py::add(&ax_engine::pysym::call("diff", &[v_eP.clone(), c_X()?], &[])?, &ax_engine::pysym::call("diff", &[v_eQ.clone(), c_Y()?], &[])?)?, &ax_engine::pysym::call("diff", &[v_eR.clone(), c_Z()?], &[])?)?, c__XYZ()?)?;
    v_bx = Py::tuple(vec![Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("x0"))?, py::getitem(&v_i.clone(), &Py::str("x1"))?]), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("y0"))?, py::getitem(&v_i.clone(), &Py::str("y1"))?]), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("z0"))?, py::getitem(&v_i.clone(), &Py::str("z1"))?])]);
    return Ok(f__scaled_err(py::getitem(&v_o.clone(), &Py::str("flux"))?, f__gl3(v_g.clone(), v_bx.clone(), Py::Int(16i64))?, f__gl3({ let v__n = v__n.clone(); let v_bx = v_bx.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_eR = v_eR.clone(); let v_g = v_g.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_y: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_z: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::call(&v_g.clone(), &[v_x.clone(), v_y.clone(), v_z.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_bx.clone(), Py::Int(16i64))?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_extrema(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_cs: Py = Py::None;
    let mut v_dp: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t473 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:poly_of", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::list((py::getitem(&v_o.clone(), &Py::str("critical_points"))?).items()?)])).items()?;
    if t473.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t473[0].clone();
    v_a = t473[1].clone();
    v_b = t473[2].clone();
    v_cs = t473[3].clone();
    v_dp = py::method(&v_p.clone(), "diff", &[c_X()?])?;
    v_n = (if ({ let t474 = py::method(&v_p.clone(), "degree", &[])?; Py::Bool({ let t475 = Py::Int(2i64); if py::ge(&t474, &t475)? { true } else { false } }) }).truthy() { f__n_roots_open(v_dp.clone(), v_a.clone(), v_b.clone())? } else { Py::Int(0i64) });
    if ({ let t494 = { let t485 = py::len(&v_cs.clone())?; Py::Bool({ let t486 = v_n.clone(); if py::ne(&t485, &t486) { true } else { false } }) }; if t494.truthy() { t494 } else { { let t493 = { let t487 = v_cs.clone(); Py::Bool({ let t488 = py::sorted(&v_cs.clone(), (Py::Bool(false)).truthy())?; if py::ne(&t487, &t488) { true } else { false } }) }; if t493.truthy() { t493 } else { Py::Bool(({ let mut out = vec![]; for t489 in py::iter(&(v_cs.clone()))? { let mut v_c: Py; 
v_c = t489;
out.push(py::not(&{ let t490 = v_a.clone(); Py::Bool({ let t491 = v_c.clone(); if py::lt(&t490, &t491)? { { let t492 = v_b.clone(); if py::lt(&t491, &t492)? { true } else { false } } } else { false } }) })); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_vals = { let mut out = vec![]; for t495 in py::iter(&(py::add(&Py::list(vec![v_a.clone(), v_b.clone()]), &v_cs.clone())?))? { let mut v_v: Py; 
v_v = t495;
out.push(f__peval(v_p.clone(), v_v.clone())?); } Py::list(out) };
    return Ok({ let mut v: Vec<Py> = vec![]; v.extend(({ let mut out = vec![]; for t496 in py::iter(&(v_cs.clone()))? { let mut v_c: Py; 
v_c = t496;
out.push(f__root_res(v_dp.clone(), v_c.clone())?); } Py::list(out) }).items()?); v.push(f_rel_err(py::getitem(&v_o.clone(), &Py::str("absolute_max"))?, py::minmax(&[v_vals.clone()], true)?, Py::Float(1e-12))?); v.push(f_rel_err(py::getitem(&v_o.clone(), &Py::str("absolute_min"))?, py::minmax(&[v_vals.clone()], false)?, Py::Float(1e-12))?); let mut d: Vec<(Py, Py)> = vec![];   if !d.is_empty() { return Err(PyErr::Type("worst() got an unexpected keyword argument".into())) } f_worst(Py::tuple(v.get(0..).map(|s| s.to_vec()).unwrap_or_default()))? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_fourier(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a0: Py = Py::None;
    let mut v_ak: Py = Py::None;
    let mut v_bk: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t513 = (f__fourier_gl(py::getitem(&v_i.clone(), &Py::str("f"))?, py::float(&py::getitem(&v_i.clone(), &Py::str("L"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?).items()?;
    if t513.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a0 = t513[0].clone();
    v_ak = t513[1].clone();
    v_bk = t513[2].clone();
    v_s = py::minmax(&[Py::Float(1.0), py::abs(&v_a0.clone())?], true)?;
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("a0"))?, v_a0.clone(), Py::Float(1e-10))?, f_rel_err(py::truediv(&py::getitem(&v_o.clone(), &Py::str("a_k"))?, &v_s.clone())?, py::truediv(&v_ak.clone(), &v_s.clone())?, Py::Float(1e-10))?, f_rel_err(py::truediv(&py::getitem(&v_o.clone(), &Py::str("b_k"))?, &v_s.clone())?, py::truediv(&v_bk.clone(), &v_s.clone())?, Py::Float(1e-10))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ftc(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_G: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x0: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        let t522 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?, Py::tuple(vec![Py::str("t")])], &[])?, Py::tuple(vec![c_T()?])], &[])?, ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("g"))?], &[])?, Py::tuple(vec![c_X()?])], &[])?])).items()?;
        if t522.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_F = t522[0].clone();
        v_G = t522[1].clone();
        v_x0 = ax_engine::pymp::call("mpf", &[py::getitem(&v_i.clone(), &Py::str("x0"))?], &[])?;
        v_v = py::mul(&py::call(&v_F.clone(), &[py::call(&v_G.clone(), &[v_x0.clone()])?])?, &ax_engine::pymp::call("diff", &[v_G.clone(), v_x0.clone()], &[])?)?;
    }
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("derivative"))?, py::float(&v_v.clone())?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_gradient(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_names: Py = Py::None;
    v_names = f__vars(py::getitem(&v_i.clone(), &Py::str("p"))?)?;
    v_g = { let mut out = vec![]; for t609 in py::range_iter(&[(py::len(&v_names.clone())?).i()?]) { let mut v_j: Py; 
v_j = t609;
out.push(py::float(&f__mp_partial(py::getitem(&v_i.clone(), &Py::str("f"))?, v_names.clone(), py::getitem(&v_i.clone(), &Py::str("p"))?, { let mut out = vec![]; for t611 in py::range_iter(&[(py::len(&v_names.clone())?).i()?]) { let mut v_k: Py; 
v_k = t611;
out.push(py::int(&{ let t612 = v_k.clone(); Py::Bool({ let t613 = v_j.clone(); if py::eq(&t612, &t613) { true } else { false } }) })?); } Py::list(out) })?)?); } Py::list(out) };
    return Ok(f_worst(Py::tuple(vec![f__vec_err(py::getitem(&v_o.clone(), &Py::str("gradient"))?, v_g.clone(), Py::Float(1e-11))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("magnitude"))?, py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t622 in py::iter(&(v_g.clone()))? { let mut v_c: Py; 
v_c = t622;
out.push(py::mul(&v_c.clone(), &v_c.clone())?); } Py::list(out) })?)?, Py::Float(1e-11))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_green(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_eP: Py = Py::None;
    let mut v_eQ: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t667 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("P"))?, c__XY()?], &[])?, ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("Q"))?, c__XY()?], &[])?])).items()?;
    if t667.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_eP = t667[0].clone();
    v_eQ = t667[1].clone();
    v_g = f__fl(py::sub(&ax_engine::pysym::call("diff", &[v_eQ.clone(), c_X()?], &[])?, &ax_engine::pysym::call("diff", &[v_eP.clone(), c_Y()?], &[])?)?, c__XY()?)?;
    let t668 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("c"))?, py::getitem(&v_i.clone(), &Py::str("d"))?])).items()?;
    if t668.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t668[0].clone();
    v_d = t668[1].clone();
    v_S = f__gl2({ let v_c = v_c.clone(); let v_d = v_d.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_g = v_g.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_y: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::call(&v_g.clone(), &[v_x.clone(), v_y.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, { let v_c = v_c.clone(); let v_d = v_d.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_g = v_g.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_c.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_c = v_c.clone(); let v_d = v_d.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_g = v_g.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_d.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Int(32i64))?;
    v_v = f__gl2(v_g.clone(), py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, { let v_S = v_S.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_g = v_g.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_c.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_S = v_S.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_g = v_g.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_d.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Int(32i64))?;
    return Ok(f_worst(Py::tuple(vec![f__scaled_err(py::getitem(&v_o.clone(), &Py::str("double_integral"))?, v_v.clone(), v_S.clone(), Py::Float(1e-10))?, f__scaled_err(py::getitem(&v_o.clone(), &Py::str("circulation"))?, v_v.clone(), v_S.clone(), Py::Float(1e-10))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_hessian(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_kind: Py = Py::None;
    v_h = { let mut out = vec![]; for t686 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::Int(2i64), Py::Int(0i64)]), Py::tuple(vec![Py::Int(1i64), Py::Int(1i64)]), Py::tuple(vec![Py::Int(0i64), Py::Int(2i64)])])))? { let mut v_v: Py; 
v_v = t686;
out.push(py::float(&f__mp_partial(py::getitem(&v_i.clone(), &Py::str("f"))?, c__XY()?, py::getitem(&v_i.clone(), &Py::str("p"))?, v_v.clone())?)?); } Py::list(out) };
    v_D = py::sub(&py::mul(&py::getitem(&v_h.clone(), &Py::Int(0i64))?, &py::getitem(&v_h.clone(), &Py::Int(2i64))?)?, &py::pow(&py::getitem(&v_h.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?;
    v_kind = (if ({ let t688 = py::getitem(&v_o.clone(), &Py::str("D"))?; Py::Bool({ let t689 = Py::Int(0i64); if py::lt(&t688, &t689)? { true } else { false } }) }).truthy() { Py::str("saddle point") } else { (if ({ let t690 = py::getitem(&v_o.clone(), &Py::str("fxx"))?; Py::Bool({ let t691 = Py::Int(0i64); if py::gt(&t690, &t691)? { true } else { false } }) }).truthy() { Py::str("local minimum") } else { Py::str("local maximum") }) });
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("fxx"))?, py::getitem(&v_h.clone(), &Py::Int(0i64))?, Py::Float(1e-10))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("fxy"))?, py::getitem(&v_h.clone(), &Py::Int(1i64))?, Py::Float(1e-10))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("fyy"))?, py::getitem(&v_h.clone(), &Py::Int(2i64))?, Py::Float(1e-10))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("D"))?, v_D.clone(), Py::Float(1e-09))?, f__same(py::getitem(&v_o.clone(), &Py::str("classification"))?, v_kind.clone())?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_implicit(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_fx: Py = Py::None;
    let mut v_fy: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_t = f__eqn_text(py::getitem(&v_i.clone(), &Py::str("F"))?)?;
    let t714 = (Py::tuple(vec![f__mp_partial(v_t.clone(), c__XY()?, py::getitem(&v_i.clone(), &Py::str("p"))?, Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)]))?, f__mp_partial(v_t.clone(), c__XY()?, py::getitem(&v_i.clone(), &Py::str("p"))?, Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)]))?])).items()?;
    if t714.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fx = t714[0].clone();
    v_fy = t714[1].clone();
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("dy_dx"))?, py::float(&py::truediv(&py::neg(&v_fx.clone())?, &v_fy.clone())?)?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_inflection(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_d2: Py = Py::None;
    let mut v_odd: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_q: Py = Py::None;
    v_p = ax_engine::pysym::helper("calculus:poly_of", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?;
    v_d2 = py::method(&py::method(&v_p.clone(), "diff", &[c_X()?])?, "diff", &[c_X()?])?;
    v_pts = Py::list((py::getitem(&v_o.clone(), &Py::str("inflection_points"))?).items()?);
    if ({ let t746 = py::getattr(&v_d2.clone(), "is_zero")?; if t746.truthy() { t746 } else { { let t744 = py::method(&v_d2.clone(), "degree", &[])?; Py::Bool({ let t745 = Py::Int(1i64); if py::lt(&t744, &t745)? { true } else { false } }) } } }).truthy() {
        return Ok((if (py::not(&v_pts.clone())).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    v_odd = ax_engine::pysym::call("prod", &[{ let mut out = vec![]; for t747 in py::iter(&(py::getitem(&ax_engine::pysym::call("sqf_list", &[py::method(&v_d2.clone(), "as_expr", &[])?], &[])?, &Py::Int(1i64))?))? { let mut v_fac: Py; let mut v_m: Py; 
let t748 = (t747).items()?;
if t748.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_fac = t748[0].clone();
v_m = t748[1].clone();
if ({ let t749 = py::modulo(&v_m.clone(), &Py::Int(2i64))?; Py::Bool({ let t750 = Py::Int(1i64); if py::eq(&t749, &t750) { true } else { false } }) }).truthy() { out.push(v_fac.clone()); }} Py::list(out) }], &[])?;
    v_q = ax_engine::pysym::call("Poly", &[v_odd.clone(), c_X()?], &[])?;
    if ({ let t766 = { let t751 = py::len(&v_pts.clone())?; Py::Bool({ let t752 = (if ({ let t753 = py::method(&v_q.clone(), "degree", &[])?; Py::Bool({ let t754 = Py::Int(1i64); if py::ge(&t753, &t754)? { true } else { false } }) }).truthy() { f__n_roots_open(v_q.clone(), Py::None, Py::None)? } else { Py::Int(0i64) }); if py::ne(&t751, &t752) { true } else { false } }) }; if t766.truthy() { t766 } else { { let t764 = v_pts.clone(); Py::Bool({ let t765 = py::sorted(&v_pts.clone(), (Py::Bool(false)).truthy())?; if py::ne(&t764, &t765) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(f_worst({ let mut v = vec![]; v.extend(({ let mut out = vec![]; for t767 in py::iter(&(v_pts.clone()))? { let mut v_c: Py; 
v_c = t767;
out.push(f__root_res(v_q.clone(), v_c.clone())?); } Py::list(out) }).items()?); Py::tuple(v) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_limit_inf(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_v: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(60i64))?;
        v_v = py::call(&ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?, Py::tuple(vec![c_X()?])], &[])?, &[py::pow(&ax_engine::pymp::call("mpf", &[Py::Int(10i64)], &[])?, &Py::Int(20i64))?])?;
    }
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("limit"))?, py::float(&v_v.clone())?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_limit_pt(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_v: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(50i64))?;
        v_g = ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?, Py::tuple(vec![c_X()?])], &[])?;
        let t871 = (Py::tuple(vec![f__mpq(f__rat(py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, py::pow(&ax_engine::pymp::call("mpf", &[Py::Int(10i64)], &[])?, &py::neg(&Py::Int(12i64))?)?])).items()?;
        if t871.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_A = t871[0].clone();
        v_h = t871[1].clone();
        v_v = py::truediv(&py::add(&py::call(&v_g.clone(), &[py::add(&v_A.clone(), &v_h.clone())?])?, &py::call(&v_g.clone(), &[py::sub(&v_A.clone(), &v_h.clone())?])?)?, &Py::Int(2i64))?;
    }
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("limit"))?, py::float(&v_v.clone())?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_mvt(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_cs: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let t926 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:poly_of", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?])).items()?;
    if t926.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p = t926[0].clone();
    v_a = t926[1].clone();
    v_b = t926[2].clone();
    v_m = py::truediv(&py::sub(&f__peval(v_p.clone(), v_b.clone())?, &f__peval(v_p.clone(), v_a.clone())?)?, &py::sub(&v_b.clone(), &v_a.clone())?)?;
    let t927 = (Py::tuple(vec![f__rat(v_a.clone())?, f__rat(v_b.clone())?])).items()?;
    if t927.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t927[0].clone();
    v_B = t927[1].clone();
    v_q = py::sub(&py::method(&v_p.clone(), "diff", &[c_X()?])?, &py::truediv(&py::sub(&py::method(&v_p.clone(), "eval", &[v_B.clone()])?, &py::method(&v_p.clone(), "eval", &[v_A.clone()])?)?, &py::sub(&v_B.clone(), &v_A.clone())?)?)?;
    v_cs = Py::list((py::getitem(&v_o.clone(), &Py::str("c_values"))?).items()?);
    if ({ let t946 = { let t928 = py::len(&v_cs.clone())?; Py::Bool({ let t929 = f__n_roots_open(v_q.clone(), v_a.clone(), v_b.clone())?; if py::ne(&t928, &t929) { true } else { false } }) }; if t946.truthy() { t946 } else { { let t945 = { let t939 = v_cs.clone(); Py::Bool({ let t940 = py::sorted(&v_cs.clone(), (Py::Bool(false)).truthy())?; if py::ne(&t939, &t940) { true } else { false } }) }; if t945.truthy() { t945 } else { Py::Bool(({ let mut out = vec![]; for t941 in py::iter(&(v_cs.clone()))? { let mut v_c: Py; 
v_c = t941;
out.push(py::not(&{ let t942 = v_a.clone(); Py::Bool({ let t943 = v_c.clone(); if py::lt(&t942, &t943)? { { let t944 = v_b.clone(); if py::lt(&t943, &t944)? { true } else { false } } } else { false } }) })); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(f_worst({ let mut v = vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("secant_slope"))?, v_m.clone(), Py::Float(1e-12))?]; v.extend(({ let mut out = vec![]; for t947 in py::iter(&(v_cs.clone()))? { let mut v_c: Py; 
v_c = t947;
out.push(f__root_res(v_q.clone(), v_c.clone())?); } Py::list(out) }).items()?); Py::tuple(v) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_newton(mut v_f: Py, mut v_x0: Py, mut v_o: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_its: Py = Py::None;
    let mut v_steps: Py = Py::None;
    v_g = ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?, Py::tuple(vec![c_X()?])], &[])?;
    v_its = py::getitem(&v_o.clone(), &Py::str("iterates"))?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        v_steps = { let mut out = vec![]; for t1007 in py::iter(&(py::zip(&[py::add(&Py::list(vec![v_x0.clone()]), &py::slice(&v_its.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?)?, v_its.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1008 = (t1007).items()?;
if t1008.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1008[0].clone();
v_b = t1008[1].clone();
out.push(f_rel_err(v_b.clone(), py::float(&py::sub(&ax_engine::pymp::call("mpf", &[v_a.clone()], &[])?, &py::truediv(&py::call(&v_g.clone(), &[ax_engine::pymp::call("mpf", &[v_a.clone()], &[])?])?, &ax_engine::pymp::call("diff", &[v_g.clone(), ax_engine::pymp::call("mpf", &[v_a.clone()], &[])?], &[])?)?)?)?, Py::Float(1e-10))?); } Py::list(out) };
    }
    return Ok(py::minmax(&[py::add(&v_steps.clone(), &Py::list(vec![f_rel_err(py::getitem(&v_its.clone(), &py::neg(&Py::Int(1i64))?)?, py::getitem(&v_o.clone(), &Py::str("x_n"))?, Py::Float(0.0))?]))?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_pslope(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_T0: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_gx: Py = Py::None;
    let mut v_gy: Py = Py::None;
    let mut v_m: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        let t1073 = ({ let mut out = vec![]; for t1072 in py::iter(&(f__path(py::getitem(&v_i.clone(), &Py::str("xt"))?, py::getitem(&v_i.clone(), &Py::str("yt"))?)?))? { let mut v_e: Py; 
v_e = t1072;
out.push(ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_T()?])], &[])?); } Py::list(out) }).items()?;
        if t1073.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_gx = t1073[0].clone();
        v_gy = t1073[1].clone();
        v_T0 = ax_engine::pymp::call("mpf", &[py::getitem(&v_i.clone(), &Py::str("t0"))?], &[])?;
        v_m = py::truediv(&ax_engine::pymp::call("diff", &[v_gy.clone(), v_T0.clone()], &[])?, &ax_engine::pymp::call("diff", &[v_gx.clone(), v_T0.clone()], &[])?)?;
        v_d2 = py::truediv(&ax_engine::pymp::call("diff", &[{ let v_T0 = v_T0.clone(); let v_gx = v_gx.clone(); let v_gy = v_gy.clone(); let v_i = v_i.clone(); let v_m = v_m.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&ax_engine::pymp::call("diff", &[v_gy.clone(), v_t.clone()], &[])?, &ax_engine::pymp::call("diff", &[v_gx.clone(), v_t.clone()], &[])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_T0.clone()], &[])?, &ax_engine::pymp::call("diff", &[v_gx.clone(), v_T0.clone()], &[])?)?;
    }
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("dy_dx"))?, py::float(&v_m.clone())?, Py::Float(1e-11))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("d2y_dx2"))?, py::float(&v_d2.clone())?, Py::Float(1e-09))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(non_snake_case)]
fn f__verify_ratio(v_i: Py, v_o: Py) -> R {
    ax_engine::pysym::helper("calculus:_verify_ratio", &[v_i, v_o], &[])
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_surface(mut v_f: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_cuts: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_knots: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    let t1348 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[])?, ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?, Py::tuple(vec![c_X()?])], &[])?])).items()?;
    if t1348.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t1348[0].clone();
    v_D = t1348[1].clone();
    v_cuts = f__sign_changes(ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[(String::from("use_mp"), Py::Bool(false))])?, v_a.clone(), v_b.clone(), Py::Int(1000i64))?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        v_knots = py::add(&py::add(&Py::list(vec![ax_engine::pymp::call("mpf", &[v_a.clone()], &[])?]), &{ let mut out = vec![]; for t1369 in py::iter(&(v_cuts.clone()))? { let mut v_c: Py; 
v_c = t1369;
out.push(ax_engine::pymp::call("mpf", &[v_c.clone()], &[])?); } Py::list(out) })?, &Py::list(vec![ax_engine::pymp::call("mpf", &[v_b.clone()], &[])?]))?;
        return Ok(py::mul(&py::mul(&Py::Int(2i64), &ax_engine::pymp::constant("pi")?)?, &ax_engine::pymp::call("quad", &[{ let v_D = v_D.clone(); let v_F = v_F.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_cuts = v_cuts.clone(); let v_e = v_e.clone(); let v_f = v_f.clone(); let v_knots = v_knots.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&py::abs(&py::call(&v_F.clone(), &[v_x.clone()])?)?, &ax_engine::pymp::call("sqrt", &[py::add(&Py::Int(1i64), &py::pow(&py::call(&v_D.clone(), &[v_x.clone()])?, &Py::Int(2i64))?)?], &[])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_knots.clone()], &[(String::from("method"), Py::str("gauss-legendre"))])?)?);
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_taylor(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_co: Py = Py::None;
    let mut v_fx1: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_ref: Py = Py::None;
    let mut v_val: Py = Py::None;
    let t1406 = (Py::tuple(vec![Py::list((py::getitem(&v_o.clone(), &Py::str("coefficients"))?).items()?), py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::sub(&py::getitem(&v_i.clone(), &Py::str("x1"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?])).items()?;
    if t1406.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_co = t1406[0].clone();
    v_n = t1406[1].clone();
    v_h = t1406[2].clone();
    if ({ let t1407 = py::len(&v_co.clone())?; Py::Bool({ let t1408 = py::add(&v_n.clone(), &Py::Int(1i64))?; if py::ne(&t1407, &t1408) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        v_g = ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?, Py::tuple(vec![c_X()?])], &[])?;
        v_ref = ax_engine::pymp::call("taylor", &[v_g.clone(), ax_engine::pymp::call("mpf", &[py::getitem(&v_i.clone(), &Py::str("a"))?], &[])?, v_n.clone()], &[])?;
        v_fx1 = py::call(&v_g.clone(), &[ax_engine::pymp::call("mpf", &[py::getitem(&v_i.clone(), &Py::str("x1"))?], &[])?])?;
    }
    v_val = Py::Float(0.0);
    for t1409 in py::iter(&(py::reversed(&v_co.clone())?))? {
        v_c = t1409;
        v_val = py::add(&py::mul(&v_val.clone(), &v_h.clone())?, &v_c.clone())?;
    }
    return Ok(f_worst(Py::tuple(vec![f__vec_err(v_co.clone(), { let mut out = vec![]; for t1418 in py::iter(&(v_ref.clone()))? { let mut v_c: Py; 
v_c = t1418;
out.push(py::float(&v_c.clone())?); } Py::list(out) }, Py::Float(1e-11))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("value"))?, v_val.clone(), Py::Float(1e-12))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("function_value"))?, py::float(&v_fx1.clone())?, Py::Float(1e-12))?, py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("error"))?, &py::sub(&py::getitem(&v_o.clone(), &Py::str("function_value"))?, &py::getitem(&v_o.clone(), &Py::str("value"))?)?)?)?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_tpl(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_bx: Py = Py::None;
    let mut v_g: Py = Py::None;
    v_g = f__fl(ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?, c__XYZ()?], &[])?, c__XYZ()?)?;
    v_bx = Py::tuple(vec![Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("x0"))?, py::getitem(&v_i.clone(), &Py::str("x1"))?]), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("y0"))?, py::getitem(&v_i.clone(), &Py::str("y1"))?]), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("z0"))?, py::getitem(&v_i.clone(), &Py::str("z1"))?])]);
    return Ok(f__scaled_err(py::getitem(&v_o.clone(), &Py::str("integral"))?, f__gl3(v_g.clone(), v_bx.clone(), Py::Int(16i64))?, f__gl3({ let v_bx = v_bx.clone(); let v_g = v_g.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_y: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_z: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::call(&v_g.clone(), &[v_x.clone(), v_y.clone(), v_z.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_bx.clone(), Py::Int(16i64))?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_tplane(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_fx: Py = Py::None;
    let mut v_fy: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_z0: Py = Py::None;
    v_p = py::getitem(&v_i.clone(), &Py::str("p"))?;
    let t1391 = (Py::tuple(vec![py::float(&f__mp_partial(py::getitem(&v_i.clone(), &Py::str("f"))?, c__XY()?, v_p.clone(), Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)]))?)?, py::float(&f__mp_partial(py::getitem(&v_i.clone(), &Py::str("f"))?, c__XY()?, v_p.clone(), Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)]))?)?])).items()?;
    if t1391.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fx = t1391[0].clone();
    v_fy = t1391[1].clone();
    v_z0 = { let mut v: Vec<Py> = vec![]; v.push(ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?, c__XY()?], &[])?); v.push(c__XY()?); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? };
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("fx"))?, v_fx.clone(), Py::Float(1e-11))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("fy"))?, v_fy.clone(), Py::Float(1e-11))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("z0"))?, v_z0.clone(), Py::Float(1e-12))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("c"))?, py::sub(&py::sub(&v_z0.clone(), &py::mul(&v_fx.clone(), &py::getitem(&v_p.clone(), &Py::Int(0i64))?)?)?, &py::mul(&v_fy.clone(), &py::getitem(&v_p.clone(), &Py::Int(1i64))?)?)?, Py::Float(1e-11))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__zeta_em(mut v_s: Py, mut v_N: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_rise: Py = Py::None;
    let mut v_v: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        v_s = ax_engine::pymp::call("mpf", &[v_s.clone()], &[])?;
        v_v = py::add(&py::add(&ax_engine::pymp::call("fsum", &[{ let mut out = vec![]; for t1178 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_N.clone()).i()?]) { let mut v_n: Py; 
v_n = t1178;
out.push(py::pow(&ax_engine::pymp::call("mpf", &[v_n.clone()], &[])?, &py::neg(&v_s.clone())?)?); } Py::list(out) }], &[])?, &py::truediv(&py::pow(&ax_engine::pymp::call("mpf", &[v_N.clone()], &[])?, &py::sub(&Py::Int(1i64), &v_s.clone())?)?, &py::sub(&v_s.clone(), &Py::Int(1i64))?)?)?, &py::truediv(&py::pow(&ax_engine::pymp::call("mpf", &[v_N.clone()], &[])?, &py::neg(&v_s.clone())?)?, &Py::Int(2i64))?)?;
        v_rise = v_s.clone();
        for t1179 in py::iter(&(py::enumerate(&c__B2K()?, (Py::Int(1i64)).i()?)?))? {
            let t1180 = (t1179).items()?;
            if t1180.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_k = t1180[0].clone();
            v_B = t1180[1].clone();
            v_v = py::iadd(&v_v, &py::mul(&py::mul(&py::truediv(&py::truediv(&ax_engine::pymp::call("mpf", &[py::numerator(&v_B.clone())?], &[])?, &py::denominator(&v_B.clone())?)?, &ax_engine::pymp::call("factorial", &[py::mul(&Py::Int(2i64), &v_k.clone())?], &[])?)?, &v_rise.clone())?, &py::pow(&ax_engine::pymp::call("mpf", &[v_N.clone()], &[])?, &py::add(&py::sub(&py::neg(&v_s.clone())?, &py::mul(&Py::Int(2i64), &v_k.clone())?)?, &Py::Int(1i64))?)?)?)?;
            v_rise = py::mul(&v_rise, &py::mul(&py::sub(&py::add(&v_s.clone(), &py::mul(&Py::Int(2i64), &v_k.clone())?)?, &Py::Int(1i64))?, &py::add(&v_s.clone(), &py::mul(&Py::Int(2i64), &v_k.clone())?)?)?)?;
        }
        return Ok(py::float(&v_v.clone())?);
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_arc_length(mut v_f: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    if (py::not(&{ let t1 = v_a.clone(); Py::Bool({ let t2 = v_b.clone(); if py::lt(&t1, &t2)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    v_d = ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?;
    let _ = ax_engine::pysym::helper("calculus:continuous_on", &[v_d.clone(), c_X()?, v_a.clone(), v_b.clone()], &[])?;
    return Ok(Py::dict(vec![(Py::str("arc_length"), ax_engine::pysym::helper("calculus:quad", &[ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::call("sqrt", &[py::add(&Py::Int(1i64), &py::pow(&v_d.clone(), &Py::Int(2i64))?)?], &[])?, Py::tuple(vec![c_X()?])], &[])?, v_a.clone(), v_b.clone()], &[])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_area_between_curves(mut v_f: Py, mut v_g: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_area: Py = Py::None;
    let mut v_cross: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_net: Py = Py::None;
    if (py::not(&{ let t11 = v_a.clone(); Py::Bool({ let t12 = v_b.clone(); if py::lt(&t11, &t12)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    v_h = py::sub(&ax_engine::pysym::helper("calculus:poly_of", &[v_f.clone()], &[])?, &ax_engine::pysym::helper("calculus:poly_of", &[v_g.clone()], &[])?)?;
    let t15 = (f__poly_abs_integral(v_h.clone(), v_a.clone(), v_b.clone())?).items()?;
    if t15.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_area = t15[0].clone();
    v_cross = t15[1].clone();
    v_net = py::float(&ax_engine::pysym::call("N", &[py::sub(&py::method(&py::method(&py::method(&v_h.clone(), "integrate", &[])?, "as_expr", &[])?, "subs", &[c_X()?, ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_b.clone())?))], &[])?])?, &py::method(&py::method(&py::method(&v_h.clone(), "integrate", &[])?, "as_expr", &[])?, "subs", &[c_X()?, ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_a.clone())?))], &[])?])?)?, Py::Int(30i64)], &[])?)?;
    return Ok(Py::dict(vec![(Py::str("area"), v_area.clone()), (Py::str("net_integral"), v_net.clone()), (Py::str("crossings"), v_cross.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_area_enclosed_curves(mut v_f: Py, mut v_g: Py) -> R {
    let mut v_H: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_roots: Py = Py::None;
    let mut v_tot: Py = Py::None;
    v_h = py::sub(&ax_engine::pysym::helper("calculus:poly_of", &[v_f.clone()], &[])?, &ax_engine::pysym::helper("calculus:poly_of", &[v_g.clone()], &[])?)?;
    if (py::getattr(&v_h.clone(), "is_zero")?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the curves coincide"))));
    }
    v_roots = py::sorted_key(&py::set(&ax_engine::pysym::call("real_roots", &[v_h.clone()], &[])?)?, &{ let v_f = v_f.clone(); let v_g = v_g.clone(); let v_h = v_h.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::float(&v_r.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    if ({ let t38 = py::len(&v_roots.clone())?; Py::Bool({ let t39 = Py::Int(2i64); if py::lt(&t38, &t39)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the curves meet in fewer than two points, so they enclose no bounded region"))));
    }
    v_H = py::method(&py::method(&v_h.clone(), "integrate", &[])?, "as_expr", &[])?;
    v_tot = py::sum(&{ let mut out = vec![]; for t40 in py::range_iter(&[(py::sub(&py::len(&v_roots.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t40;
out.push(ax_engine::pysym::call("Abs", &[py::sub(&py::method(&v_H.clone(), "subs", &[c_X()?, py::getitem(&v_roots.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?])?, &py::method(&v_H.clone(), "subs", &[c_X()?, py::getitem(&v_roots.clone(), &v_k.clone())?])?)?], &[])?); } Py::list(out) }, None)?;
    return Ok(Py::dict(vec![(Py::str("area"), py::float(&ax_engine::pysym::call("N", &[v_tot.clone(), Py::Int(30i64)], &[])?)?), (Py::str("x_left"), py::float(&ax_engine::pysym::call("N", &[py::getitem(&v_roots.clone(), &Py::Int(0i64))?, Py::Int(30i64)], &[])?)?), (Py::str("x_right"), py::float(&ax_engine::pysym::call("N", &[py::getitem(&v_roots.clone(), &py::neg(&Py::Int(1i64))?)?, Py::Int(30i64)], &[])?)?), (Py::str("intersections"), { let mut out = vec![]; for t41 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t41;
out.push(py::float(&ax_engine::pysym::call("N", &[v_r.clone(), Py::Int(30i64)], &[])?)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_arithmetic_series_sum(mut v_a1: Py, mut v_d: Py, mut v_n: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_D: Py = Py::None;
    v_n = py::int(&v_n.clone())?;
    if (py::not(&{ let t53 = Py::Int(1i64); Py::Bool({ let t54 = v_n.clone(); if py::le(&t53, &t54)? { { let t55 = py::pow(&Py::Int(10i64), &Py::Int(6i64))?; if py::le(&t54, &t55)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be between 1 and 1000000"))));
    }
    let t56 = (Py::tuple(vec![f__frac(v_a1.clone())?, f__frac(v_d.clone())?])).items()?;
    if t56.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t56[0].clone();
    v_D = t56[1].clone();
    return Ok(Py::dict(vec![(Py::str("sum"), py::float(&py::mul(&py::fraction(&[v_n.clone(), Py::Int(2i64)])?, &py::add(&py::mul(&Py::Int(2i64), &v_A.clone())?, &py::mul(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_D.clone())?)?)?)?), (Py::str("last_term"), py::float(&py::add(&v_A.clone(), &py::mul(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_D.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_average_rate_of_change(mut v_f: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_fa: Py = Py::None;
    let mut v_fb: Py = Py::None;
    if ({ let t65 = v_a.clone(); Py::Bool({ let t66 = v_b.clone(); if py::eq(&t65, &t66) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two points must differ"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    let t67 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:at", &[v_e.clone(), Py::tuple(vec![c_X()?]), v_a.clone()], &[])?, ax_engine::pysym::helper("calculus:at", &[v_e.clone(), Py::tuple(vec![c_X()?]), v_b.clone()], &[])?])).items()?;
    if t67.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t67[0].clone();
    v_fb = t67[1].clone();
    return Ok(Py::dict(vec![(Py::str("average_rate"), py::truediv(&py::sub(&v_fb.clone(), &v_fa.clone())?, &py::sub(&v_b.clone(), &v_a.clone())?)?), (Py::str("fa"), v_fa.clone()), (Py::str("fb"), v_fb.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_average_value(mut v_f: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_i: Py = Py::None;
    if (py::not(&{ let t76 = v_a.clone(); Py::Bool({ let t77 = v_b.clone(); if py::lt(&t76, &t77)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    v_i = py::getitem(&f_definite_integral(v_f.clone(), v_a.clone(), v_b.clone())?, &Py::str("integral"))?;
    return Ok(Py::dict(vec![(Py::str("average_value"), py::truediv(&v_i.clone(), &py::sub(&v_b.clone(), &v_a.clone())?)?), (Py::str("integral"), v_i.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bessel_j(mut v_n: Py, mut v_x: Py) -> R {
    v_n = py::int(&v_n.clone())?;
    if ({ let t93 = py::not(&{ let t88 = Py::Int(0i64); Py::Bool({ let t89 = v_n.clone(); if py::le(&t88, &t89)? { { let t90 = Py::Int(50i64); if py::le(&t89, &t90)? { true } else { false } } } else { false } }) }); if t93.truthy() { t93 } else { { let t91 = py::abs(&v_x.clone())?; Py::Bool({ let t92 = Py::Int(1000i64); if py::gt(&t91, &t92)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("this capability covers orders 0..50 and |x| <= 1000"))));
    }
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        return Ok(Py::dict(vec![(Py::str("J"), f__fin(ax_engine::pymp::call("besselj", &[v_n.clone(), v_x.clone()], &[])?, Py::str("the result"))?)]));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_beta_function(mut v_a: Py, mut v_b: Py) -> R {
    if (py::not(&{ let t106 = { let t102 = v_a.clone(); Py::Bool({ let t103 = Py::Int(0i64); if py::gt(&t102, &t103)? { true } else { false } }) }; if !t106.truthy() { t106 } else { { let t104 = v_b.clone(); Py::Bool({ let t105 = Py::Int(0i64); if py::gt(&t104, &t105)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("this capability needs a > 0 and b > 0"))));
    }
    return Ok(Py::dict(vec![(Py::str("beta"), py::math1("exp", &py::sub(&py::add(&py::math1("lgamma", &v_a.clone())?, &py::math1("lgamma", &v_b.clone())?)?, &py::math1("lgamma", &py::add(&v_a.clone(), &v_b.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_centroid_region(mut v_f: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_A_: Py = Py::None;
    let mut v_B_: Py = Py::None;
    let mut v_area: Py = Py::None;
    let mut v_integ: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_xb: Py = Py::None;
    let mut v_yb: Py = Py::None;
    if (py::not(&{ let t115 = v_a.clone(); Py::Bool({ let t116 = v_b.clone(); if py::lt(&t115, &t116)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    v_p = ax_engine::pysym::helper("calculus:poly_of", &[v_f.clone()], &[])?;
    if (py::not(&f__nonneg_on(v_p.clone(), v_a.clone(), v_b.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the region under the curve needs f(x) >= 0 on [a, b]"))));
    }
    let t123 = (Py::tuple(vec![ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_a.clone())?))], &[])?, ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_b.clone())?))], &[])?])).items()?;
    if t123.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A_ = t123[0].clone();
    v_B_ = t123[1].clone();
    v_integ = { let v_A_ = v_A_.clone(); let v_B_ = v_B_.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_f = v_f.clone(); let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_q: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::call(&{ let v_A_ = v_A_.clone(); let v_B_ = v_B_.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_f = v_f.clone(); let v_p = v_p.clone(); let v_q = v_q.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_Q: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::method(&v_Q.clone(), "subs", &[c_X()?, v_B_.clone()])?, &py::method(&v_Q.clone(), "subs", &[c_X()?, v_A_.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::method(&py::method(&v_q.clone(), "integrate", &[])?, "as_expr", &[])?])?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_area = py::call(&v_integ.clone(), &[v_p.clone()])?;
    if ({ let t124 = v_area.clone(); Py::Bool({ let t125 = Py::Int(0i64); if py::eq(&t124, &t125) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the region has zero area"))));
    }
    let t126 = (Py::tuple(vec![py::truediv(&py::call(&v_integ.clone(), &[py::mul(&ax_engine::pysym::call("Poly", &[c_X()?, c_X()?], &[])?, &v_p.clone())?])?, &v_area.clone())?, py::truediv(&py::call(&v_integ.clone(), &[py::pow(&v_p.clone(), &Py::Int(2i64))?])?, &py::mul(&Py::Int(2i64), &v_area.clone())?)?])).items()?;
    if t126.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xb = t126[0].clone();
    v_yb = t126[1].clone();
    return Ok(Py::dict(vec![(Py::str("area"), py::float(&v_area.clone())?), (Py::str("x_bar"), py::float(&v_xb.clone())?), (Py::str("y_bar"), py::float(&v_yb.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chebyshev_polynomial(mut v_n: Py, mut v_x: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_t0: Py = Py::None;
    let mut v_t1: Py = Py::None;
    v_n = py::int(&v_n.clone())?;
    if (py::not(&{ let t135 = Py::Int(0i64); Py::Bool({ let t136 = v_n.clone(); if py::le(&t135, &t136)? { { let t137 = Py::Int(500i64); if py::le(&t136, &t137)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the degree must be between 0 and 500"))));
    }
    let t138 = (Py::tuple(vec![Py::Float(1.0), v_x.clone()])).items()?;
    if t138.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t0 = t138[0].clone();
    v_t1 = t138[1].clone();
    if ({ let t139 = v_n.clone(); Py::Bool({ let t140 = Py::Int(0i64); if py::eq(&t139, &t140) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("T"), Py::Float(1.0))]));
    }
    for t141 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_n.clone()).i()?]) {
        v__ = t141;
        let t142 = (Py::tuple(vec![v_t1.clone(), py::sub(&py::mul(&py::mul(&Py::Int(2i64), &v_x.clone())?, &v_t1.clone())?, &v_t0.clone())?])).items()?;
        if t142.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_t0 = t142[0].clone();
        v_t1 = t142[1].clone();
    }
    return Ok(Py::dict(vec![(Py::str("T"), v_t1.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_critical_points_classify(mut v_f: Py) -> R {
    let mut v_dp: Py = Py::None;
    let mut v_left: Py = Py::None;
    let mut v_mn: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_right: Py = Py::None;
    let mut v_rs: Py = Py::None;
    v_p = ax_engine::pysym::helper("calculus:poly_of", &[v_f.clone()], &[])?;
    if ({ let t151 = py::method(&v_p.clone(), "degree", &[])?; Py::Bool({ let t152 = Py::Int(2i64); if py::lt(&t151, &t152)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs a polynomial of degree 2 or more (a line has no isolated critical points)"))));
    }
    v_dp = py::method(&v_p.clone(), "diff", &[c_X()?])?;
    v_rs = f__distinct_real_roots(v_dp.clone())?;
    let t153 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t153.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t153[0].clone();
    v_mn = t153[1].clone();
    for t158 in py::iter(&(py::zip(&[v_rs.clone(), (if (v_rs.clone()).truthy() { f__side_signs(v_dp.clone(), v_rs.clone())? } else { Py::list(vec![]) })])?))? {
        let t165 = (t158).items()?;
        if t165.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_r = t165[0].clone();
        let t166 = (t165[1].clone()).items()?;
        if t166.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_left = t166[0].clone();
        v_right = t166[1].clone();
        if ({ let t159 = v_left.clone(); Py::Bool({ let t160 = Py::Int(0i64); if py::gt(&t159, &t160)? { { let t161 = v_right.clone(); if py::gt(&t160, &t161)? { true } else { false } } } else { false } }) }).truthy() {
            let _ = py::method(&v_mx.clone(), "append", &[f__pfloat(v_r.clone())?])?;
        } else {
            if ({ let t162 = v_left.clone(); Py::Bool({ let t163 = Py::Int(0i64); if py::lt(&t162, &t163)? { { let t164 = v_right.clone(); if py::lt(&t163, &t164)? { true } else { false } } } else { false } }) }).truthy() {
                let _ = py::method(&v_mn.clone(), "append", &[f__pfloat(v_r.clone())?])?;
            }
        }
    }
    return Ok(Py::dict(vec![(Py::str("critical_points"), { let mut out = vec![]; for t167 in py::iter(&(v_rs.clone()))? { let mut v_r: Py; 
v_r = t167;
out.push(f__pfloat(v_r.clone())?); } Py::list(out) }), (Py::str("local_maxima"), v_mx.clone()), (Py::str("local_minima"), v_mn.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_curl(mut v_P: Py, mut v_Q: Py, mut v_R: Py, mut v_p: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_eP: Py = Py::None;
    let mut v_eQ: Py = Py::None;
    let mut v_eR: Py = Py::None;
    let mut v_names: Py = Py::None;
    if ({ let t208 = py::len(&v_p.clone())?; Py::Bool({ let t209 = Py::Int(3i64); if py::ne(&t208, &t209) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the point needs 3 coordinates (x, y, z)"))));
    }
    let t215 = (f__field(v_P.clone(), v_Q.clone(), v_R.clone())?).items()?;
    if t215.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t216 = (t215[0].clone()).items()?;
    if t216.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_eP = t216[0].clone();
    v_eQ = t216[1].clone();
    v_eR = t216[2].clone();
    v_names = t215[1].clone();
    for t217 in py::iter(&(Py::tuple(vec![v_eP.clone(), v_eQ.clone(), v_eR.clone()])))? {
        v_c = t217;
        let _ = { let mut v: Vec<Py> = vec![]; v.push(v_c.clone()); v.push(v_names.clone()); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? };
    }
    v_d = { let v_P = v_P.clone(); let v_Q = v_Q.clone(); let v_R = v_R.clone(); let v_c = v_c.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_eR = v_eR.clone(); let v_names = v_names.clone(); let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_e: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_s: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok({ let mut v: Vec<Py> = vec![]; v.push(ax_engine::pysym::call("diff", &[v_e.clone(), v_s.clone()], &[])?); v.push(v_names.clone()); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::dict(vec![(Py::str("curl"), Py::list(vec![py::sub(&py::call(&v_d.clone(), &[v_eR.clone(), c_Y()?])?, &py::call(&v_d.clone(), &[v_eQ.clone(), c_Z()?])?)?, py::sub(&py::call(&v_d.clone(), &[v_eP.clone(), c_Z()?])?, &py::call(&v_d.clone(), &[v_eR.clone(), c_X()?])?)?, py::sub(&py::call(&v_d.clone(), &[v_eQ.clone(), c_X()?])?, &py::call(&v_d.clone(), &[v_eP.clone(), c_Y()?])?)?]))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_curvature(mut v_f: Py, mut v_a: Py) -> R {
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    let _ = ax_engine::pysym::helper("calculus:at", &[v_e.clone(), Py::tuple(vec![c_X()?]), v_a.clone()], &[])?;
    let t235 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?, Py::tuple(vec![c_X()?]), v_a.clone()], &[])?, ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?, Py::Int(2i64)], &[])?, Py::tuple(vec![c_X()?]), v_a.clone()], &[])?])).items()?;
    if t235.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d1 = t235[0].clone();
    v_d2 = t235[1].clone();
    v_k = py::truediv(&py::abs(&v_d2.clone())?, &py::pow(&py::add(&Py::Int(1i64), &py::mul(&v_d1.clone(), &v_d1.clone())?)?, &Py::Float(1.5))?)?;
    v_out = Py::dict(vec![(Py::str("curvature"), v_k.clone())]);
    if ({ let t236 = v_k.clone(); Py::Bool({ let t237 = Py::Int(0i64); if py::gt(&t236, &t237)? { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("radius_of_curvature"), py::truediv(&Py::Int(1i64), &v_k.clone())?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_definite_integral(mut v_f: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_e: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    if ({ let t246 = v_a.clone(); Py::Bool({ let t247 = v_b.clone(); if py::eq(&t246, &t247) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("integral"), Py::Float(0.0))]));
    }
    let _ = ax_engine::pysym::helper("calculus:continuous_on", &[v_e.clone(), c_X()?, v_a.clone(), v_b.clone()], &[])?;
    return Ok(Py::dict(vec![(Py::str("integral"), ax_engine::pysym::helper("calculus:quad", &[ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[])?, v_a.clone(), v_b.clone()], &[])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_derivative_at_point(mut v_f: Py, mut v_a: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    let _ = ax_engine::pysym::helper("calculus:at", &[v_e.clone(), Py::tuple(vec![c_X()?]), v_a.clone()], &[])?;
    v_d = ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?;
    return Ok(Py::dict(vec![(Py::str("derivative"), ax_engine::pysym::helper("calculus:at", &[v_d.clone(), Py::tuple(vec![c_X()?]), v_a.clone()], &[])?), (Py::str("derivative_expression"), ax_engine::pysym::helper("calculus:show", &[v_d.clone()], &[])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_directional_derivative(mut v_f: Py, mut v_p: Py, mut v_u: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_nu: Py = Py::None;
    if ({ let t264 = py::len(&v_u.clone())?; Py::Bool({ let t265 = py::len(&v_p.clone())?; if py::ne(&t264, &t265) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the direction needs as many components as the point"))));
    }
    v_nu = py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t266 in py::iter(&(v_u.clone()))? { let mut v_c: Py; 
v_c = t266;
out.push(py::mul(&v_c.clone(), &v_c.clone())?); } Py::list(out) })?)?;
    if ({ let t267 = v_nu.clone(); Py::Bool({ let t268 = Py::Int(0i64); if py::eq(&t267, &t268) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the direction vector must not be zero"))));
    }
    v_g = py::getitem(&f_gradient(v_f.clone(), v_p.clone())?, &Py::str("gradient"))?;
    return Ok(Py::dict(vec![(Py::str("directional_derivative"), py::truediv(&py::fsum(&{ let mut out = vec![]; for t275 in py::iter(&(py::zip(&[v_g.clone(), v_u.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t276 = (t275).items()?;
if t276.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t276[0].clone();
v_b = t276[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?, &v_nu.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_disk_volume(mut v_f: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_e: Py = Py::None;
    if (py::not(&{ let t293 = v_a.clone(); Py::Bool({ let t294 = v_b.clone(); if py::lt(&t293, &t294)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    let _ = ax_engine::pysym::helper("calculus:continuous_on", &[v_e.clone(), c_X()?, v_a.clone(), v_b.clone()], &[])?;
    return Ok(Py::dict(vec![(Py::str("volume"), py::mul(&py::float(&ax_engine::pymp::constant("pi")?)?, &ax_engine::pysym::helper("calculus:quad", &[ax_engine::pysym::helper("calculus:_compile", &[py::pow(&v_e.clone(), &Py::Int(2i64))?, Py::tuple(vec![c_X()?])], &[])?, v_a.clone(), v_b.clone()], &[])?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_divergence(mut v_P: Py, mut v_Q: Py, mut v_R: Py, mut v_p: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_names: Py = Py::None;
    if ({ let t303 = py::len(&v_p.clone())?; Py::Bool({ let t304 = Py::Int(3i64); if py::ne(&t303, &t304) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the point needs 3 coordinates (x, y, z)"))));
    }
    let t310 = (f__field(v_P.clone(), v_Q.clone(), v_R.clone())?).items()?;
    if t310.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t310[0].clone();
    v_names = t310[1].clone();
    for t311 in py::iter(&(v_F.clone()))? {
        v_c = t311;
        let _ = { let mut v: Vec<Py> = vec![]; v.push(v_c.clone()); v.push(v_names.clone()); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? };
    }
    return Ok(Py::dict(vec![(Py::str("divergence"), py::fsum(&{ let mut out = vec![]; for t312 in py::iter(&(py::zip(&[v_F.clone(), Py::tuple(vec![c_X()?, c_Y()?, c_Z()?])])?))? { let mut v_c: Py; let mut v_s: Py; 
let t313 = (t312).items()?;
if t313.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t313[0].clone();
v_s = t313[1].clone();
out.push({ let mut v: Vec<Py> = vec![]; v.push(ax_engine::pysym::call("diff", &[v_c.clone(), v_s.clone()], &[])?); v.push(v_names.clone()); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? }); } Py::list(out) })?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_divergence_theorem_box(mut v_P: Py, mut v_Q: Py, mut v_R: Py, mut v_x0: Py, mut v_x1: Py, mut v_y0: Py, mut v_y1: Py, mut v_z0: Py, mut v_z1: Py) -> R {
    let mut v__n: Py = Py::None;
    let mut v_bx: Py = Py::None;
    let mut v_div: Py = Py::None;
    let mut v_eP: Py = Py::None;
    let mut v_eQ: Py = Py::None;
    let mut v_eR: Py = Py::None;
    let mut v_g: Py = Py::None;
    if (py::not(&{ let t335 = { let t328 = v_x0.clone(); Py::Bool({ let t329 = v_x1.clone(); if py::lt(&t328, &t329)? { true } else { false } }) }; if !t335.truthy() { t335 } else { { let t334 = { let t330 = v_y0.clone(); Py::Bool({ let t331 = v_y1.clone(); if py::lt(&t330, &t331)? { true } else { false } }) }; if !t334.truthy() { t334 } else { { let t332 = v_z0.clone(); Py::Bool({ let t333 = v_z1.clone(); if py::lt(&t332, &t333)? { true } else { false } }) } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the box needs x0 < x1, y0 < y1, z0 < z1"))));
    }
    let t341 = (f__field(v_P.clone(), v_Q.clone(), v_R.clone())?).items()?;
    if t341.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t342 = (t341[0].clone()).items()?;
    if t342.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_eP = t342[0].clone();
    v_eQ = t342[1].clone();
    v_eR = t342[2].clone();
    v__n = t341[1].clone();
    v_div = py::add(&py::add(&ax_engine::pysym::call("diff", &[v_eP.clone(), c_X()?], &[])?, &ax_engine::pysym::call("diff", &[v_eQ.clone(), c_Y()?], &[])?)?, &ax_engine::pysym::call("diff", &[v_eR.clone(), c_Z()?], &[])?)?;
    if (py::method(&v_div.clone(), "is_polynomial", &[c_X()?, c_Y()?, c_Z()?])?).truthy() {
        return Ok(Py::dict(vec![(Py::str("flux"), f__pfloat(ax_engine::pysym::call("integrate", &[v_div.clone(), Py::tuple(vec![c_X()?, f__rat(v_x0.clone())?, f__rat(v_x1.clone())?]), Py::tuple(vec![c_Y()?, f__rat(v_y0.clone())?, f__rat(v_y1.clone())?]), Py::tuple(vec![c_Z()?, f__rat(v_z0.clone())?, f__rat(v_z1.clone())?])], &[])?)?)]));
    }
    v_g = f__fl(v_div.clone(), c__XYZ()?)?;
    v_bx = Py::tuple(vec![Py::tuple(vec![v_x0.clone(), v_x1.clone()]), Py::tuple(vec![v_y0.clone(), v_y1.clone()]), Py::tuple(vec![v_z0.clone(), v_z1.clone()])]);
    return Ok(Py::dict(vec![(Py::str("flux"), py::getitem(&f__settled({ let v_P = v_P.clone(); let v_Q = v_Q.clone(); let v_R = v_R.clone(); let v__n = v__n.clone(); let v_bx = v_bx.clone(); let v_div = v_div.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_eR = v_eR.clone(); let v_g = v_g.clone(); let v_x0 = v_x0.clone(); let v_x1 = v_x1.clone(); let v_y0 = v_y0.clone(); let v_y1 = v_y1.clone(); let v_z0 = v_z0.clone(); let v_z1 = v_z1.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__gl3(v_g.clone(), v_bx.clone(), v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_P = v_P.clone(); let v_Q = v_Q.clone(); let v_R = v_R.clone(); let v__n = v__n.clone(); let v_bx = v_bx.clone(); let v_div = v_div.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_eR = v_eR.clone(); let v_g = v_g.clone(); let v_x0 = v_x0.clone(); let v_x1 = v_x1.clone(); let v_y0 = v_y0.clone(); let v_y1 = v_y1.clone(); let v_z0 = v_z0.clone(); let v_z1 = v_z1.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__gl3({ let v_P = v_P.clone(); let v_Q = v_Q.clone(); let v_R = v_R.clone(); let v__n = v__n.clone(); let v_bx = v_bx.clone(); let v_div = v_div.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_eR = v_eR.clone(); let v_g = v_g.clone(); let v_n = v_n.clone(); let v_x0 = v_x0.clone(); let v_x1 = v_x1.clone(); let v_y0 = v_y0.clone(); let v_y1 = v_y1.clone(); let v_z0 = v_z0.clone(); let v_z1 = v_z1.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_y: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_z: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::call(&v_g.clone(), &[v_x.clone(), v_y.clone(), v_z.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_bx.clone(), v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::tuple(vec![Py::Int(12i64), Py::Int(20i64), Py::Int(28i64)]))?, &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_double_integral_rectangle(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py) -> R {
    let mut v__S: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_v: Py = Py::None;
    if (py::not(&{ let t384 = { let t380 = v_a.clone(); Py::Bool({ let t381 = v_b.clone(); if py::lt(&t380, &t381)? { true } else { false } }) }; if !t384.truthy() { t384 } else { { let t382 = v_c.clone(); Py::Bool({ let t383 = v_d.clone(); if py::lt(&t382, &t383)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the rectangle needs a < b and c < d"))));
    }
    v_g = f__fl(ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), c__XY()?], &[])?, c__XY()?)?;
    let t412 = (f__settled({ let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__gl2(v_g.clone(), v_a.clone(), v_b.clone(), { let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_c.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_d.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__gl2({ let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_y: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::call(&v_g.clone(), &[v_x.clone(), v_y.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_a.clone(), v_b.clone(), { let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_c.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_d.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::tuple(vec![Py::Int(24i64), Py::Int(40i64), Py::Int(64i64)]))?).items()?;
    if t412.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t412[0].clone();
    v__S = t412[1].clone();
    return Ok(Py::dict(vec![(Py::str("integral"), v_v.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_double_integral_region(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_g1: Py, mut v_g2: Py) -> R {
    let mut v__S: Py = Py::None;
    let mut v__w: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_xx: Py = Py::None;
    if (py::not(&{ let t418 = v_a.clone(); Py::Bool({ let t419 = v_b.clone(); if py::lt(&t418, &t419)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the x-interval needs a < b"))));
    }
    v_g = f__fl(ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), c__XY()?], &[])?, c__XY()?)?;
    let t420 = (Py::tuple(vec![f__fl(ax_engine::pysym::helper("calculus:parse_expr", &[v_g1.clone()], &[])?, Py::tuple(vec![Py::str("x")]))?, f__fl(ax_engine::pysym::helper("calculus:parse_expr", &[v_g2.clone()], &[])?, Py::tuple(vec![Py::str("x")]))?])).items()?;
    if t420.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t420[0].clone();
    v_hi = t420[1].clone();
    for t442 in py::iter(&(f__glf(Py::Int(64i64))?))? {
        let t445 = (t442).items()?;
        if t445.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t445[0].clone();
        v__w = t445[1].clone();
        v_xx = py::add(&py::truediv(&py::add(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?, &py::mul(&py::truediv(&py::sub(&v_b.clone(), &v_a.clone())?, &Py::Int(2i64))?, &v_x.clone())?)?;
        if ({ let t443 = py::call(&v_lo.clone(), &[v_xx.clone()])?; Py::Bool({ let t444 = py::add(&py::call(&v_hi.clone(), &[v_xx.clone()])?, &py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&py::call(&v_hi.clone(), &[v_xx.clone()])?)?], true)?)?)?; if py::gt(&t443, &t444)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the lower boundary must lie below the upper boundary on [a, b]"))));
        }
    }
    let t452 = (f__settled({ let v__w = v__w.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_g1 = v_g1.clone(); let v_g2 = v_g2.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_x = v_x.clone(); let v_xx = v_xx.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__gl2(v_g.clone(), v_a.clone(), v_b.clone(), v_lo.clone(), v_hi.clone(), v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v__w = v__w.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_g1 = v_g1.clone(); let v_g2 = v_g2.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_x = v_x.clone(); let v_xx = v_xx.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__gl2({ let v__w = v__w.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_g1 = v_g1.clone(); let v_g2 = v_g2.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_n = v_n.clone(); let v_xx = v_xx.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_y: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::call(&v_g.clone(), &[v_x.clone(), v_y.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_a.clone(), v_b.clone(), v_lo.clone(), v_hi.clone(), v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::tuple(vec![Py::Int(24i64), Py::Int(40i64), Py::Int(64i64)]))?).items()?;
    if t452.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t452[0].clone();
    v__S = t452[1].clone();
    return Ok(Py::dict(vec![(Py::str("integral"), v_v.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_error_function(mut v_x: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("erf"), py::math1("erf", &v_x.clone())?), (Py::str("erfc"), py::math1("erfc", &v_x.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_extrema_on_interval(mut v_f: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_crit: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_vals: Py = Py::None;
    if (py::not(&{ let t466 = v_a.clone(); Py::Bool({ let t467 = v_b.clone(); if py::lt(&t466, &t467)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    v_p = ax_engine::pysym::helper("calculus:poly_of", &[v_f.clone()], &[])?;
    v_crit = (if ({ let t468 = py::method(&v_p.clone(), "degree", &[])?; Py::Bool({ let t469 = Py::Int(2i64); if py::ge(&t468, &t469)? { true } else { false } }) }).truthy() { { let mut out = vec![]; for t470 in py::iter(&(ax_engine::pysym::helper("calculus:real_roots_in", &[py::method(&v_p.clone(), "diff", &[c_X()?])?, v_a.clone(), v_b.clone()], &[])?))? { let mut v_r: Py; 
v_r = t470;
out.push(f__pfloat(v_r.clone())?); } Py::list(out) } } else { Py::list(vec![]) });
    v_vals = { let mut out = vec![]; for t471 in py::iter(&(py::add(&Py::list(vec![v_a.clone(), v_b.clone()]), &v_crit.clone())?))? { let mut v_v: Py; 
v_v = t471;
out.push(f__peval(v_p.clone(), v_v.clone())?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("absolute_max"), py::minmax(&[v_vals.clone()], true)?), (Py::str("absolute_min"), py::minmax(&[v_vals.clone()], false)?), (Py::str("critical_points"), v_crit.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fourier_coefficients(mut v_f: Py, mut v_L: Py, mut v_k: Py) -> R {
    let mut v_a0: Py = Py::None;
    let mut v_ak: Py = Py::None;
    let mut v_bk: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_pts: Py = Py::None;
    v_k = py::int(&v_k.clone())?;
    if (py::not(&{ let t506 = v_L.clone(); Py::Bool({ let t507 = Py::Int(0i64); if py::gt(&t506, &t507)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the half-period L must be positive"))));
    }
    if (py::not(&{ let t508 = Py::Int(1i64); Py::Bool({ let t509 = v_k.clone(); if py::le(&t508, &t509)? { { let t510 = Py::Int(50i64); if py::le(&t509, &t510)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the order must be between 1 and 50"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    let _ = ax_engine::pysym::helper("calculus:continuous_on", &[v_e.clone(), c_X()?, py::neg(&v_L.clone())?, v_L.clone()], &[])?;
    v_g = ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[])?;
    v_pts = { let mut out = vec![]; for t511 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::mul(&Py::Int(2i64), &v_k.clone())?).i()?]) { let mut v_j: Py; 
v_j = t511;
out.push(py::add(&py::neg(&v_L.clone())?, &py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_L.clone())?, &v_j.clone())?, &py::mul(&Py::Int(2i64), &v_k.clone())?)?)?); } Py::list(out) };
    v_a0 = py::truediv(&ax_engine::pysym::helper("calculus:quad", &[v_g.clone(), py::neg(&v_L.clone())?, v_L.clone()], &[])?, &v_L.clone())?;
    v_ak = py::truediv(&ax_engine::pysym::helper("calculus:quad", &[{ let v_L = v_L.clone(); let v_a0 = v_a0.clone(); let v_e = v_e.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_k = v_k.clone(); let v_pts = v_pts.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&py::call(&v_g.clone(), &[v_x.clone()])?, &ax_engine::pymp::call("cos", &[py::truediv(&py::mul(&py::mul(&v_k.clone(), &ax_engine::pymp::constant("pi")?)?, &v_x.clone())?, &v_L.clone())?], &[])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::neg(&v_L.clone())?, v_L.clone()], &[(String::from("points"), v_pts.clone())])?, &v_L.clone())?;
    v_bk = py::truediv(&ax_engine::pysym::helper("calculus:quad", &[{ let v_L = v_L.clone(); let v_a0 = v_a0.clone(); let v_ak = v_ak.clone(); let v_e = v_e.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_k = v_k.clone(); let v_pts = v_pts.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&py::call(&v_g.clone(), &[v_x.clone()])?, &ax_engine::pymp::call("sin", &[py::truediv(&py::mul(&py::mul(&v_k.clone(), &ax_engine::pymp::constant("pi")?)?, &v_x.clone())?, &v_L.clone())?], &[])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::neg(&v_L.clone())?, v_L.clone()], &[(String::from("points"), v_pts.clone())])?, &v_L.clone())?;
    return Ok(Py::dict(vec![(Py::str("a0"), v_a0.clone()), (Py::str("a_k"), v_ak.clone()), (Py::str("b_k"), v_bk.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ftc_derivative(mut v_f: Py, mut v_a: Py, mut v_g: Py, mut v_x0: Py) -> R {
    let mut v_ef: Py = Py::None;
    let mut v_eg: Py = Py::None;
    let mut v_u: Py = Py::None;
    let t520 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), Py::tuple(vec![Py::str("t")])], &[])?, ax_engine::pysym::helper("calculus:parse_expr", &[v_g.clone()], &[])?])).items()?;
    if t520.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ef = t520[0].clone();
    v_eg = t520[1].clone();
    v_u = ax_engine::pysym::helper("calculus:at", &[v_eg.clone(), Py::tuple(vec![c_X()?]), v_x0.clone()], &[])?;
    let _ = ax_engine::pysym::helper("calculus:continuous_on", &[v_ef.clone(), c_T()?, v_a.clone(), v_u.clone()], &[])?;
    return Ok(Py::dict(vec![(Py::str("derivative"), py::mul(&ax_engine::pysym::helper("calculus:at", &[v_ef.clone(), Py::tuple(vec![c_T()?]), v_u.clone()], &[])?, &ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_eg.clone(), c_X()?], &[])?, Py::tuple(vec![c_X()?]), v_x0.clone()], &[])?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gamma_function(mut v_x: Py) -> R {
    let mut v_v: Py = Py::None;
    let t529: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_v = py::math1("gamma", &v_x.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t529 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::Overflow)) => {
            return Err(py::value_error(py::to_str(&Py::str("the gamma function is undefined at 0 and negative whole numbers, and overflows beyond x = 171"))));
        }
        Err(e) => return Err(e),
    }
    return Ok(Py::dict(vec![(Py::str("gamma"), v_v.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gauss_legendre(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v_approx: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_exact: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_n = py::int(&v_n.clone())?;
    if (py::not(&{ let t538 = Py::Int(1i64); Py::Bool({ let t539 = v_n.clone(); if py::le(&t538, &t539)? { { let t540 = Py::Int(64i64); if py::le(&t539, &t540)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of nodes must be between 1 and 64"))));
    }
    if (py::not(&{ let t541 = v_a.clone(); Py::Bool({ let t542 = v_b.clone(); if py::lt(&t541, &t542)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    v_g = ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[])?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        let t543 = (Py::tuple(vec![py::truediv(&py::add(&ax_engine::pymp::call("mpf", &[v_a.clone()], &[])?, &v_b.clone())?, &Py::Int(2i64))?, py::truediv(&py::sub(&ax_engine::pymp::call("mpf", &[v_b.clone()], &[])?, &v_a.clone())?, &Py::Int(2i64))?])).items()?;
        if t543.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t543[0].clone();
        v_r = t543[1].clone();
        v_approx = py::float(&py::mul(&v_r.clone(), &ax_engine::pymp::call("fsum", &[{ let mut out = vec![]; for t561 in py::iter(&(f__gauss_nodes(v_n.clone())?))? { let mut v_w: Py; let mut v_x: Py; 
let t562 = (t561).items()?;
if t562.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t562[0].clone();
v_w = t562[1].clone();
out.push(py::mul(&v_w.clone(), &py::call(&v_g.clone(), &[py::add(&v_c.clone(), &py::mul(&v_r.clone(), &v_x.clone())?)?])?)?); } Py::list(out) }], &[])?)?)?;
    }
    v_exact = f__exact(v_e.clone(), v_a.clone(), v_b.clone())?;
    return Ok(Py::dict(vec![(Py::str("approximation"), v_approx.clone()), (Py::str("exact"), v_exact.clone()), (Py::str("error"), py::sub(&v_approx.clone(), &v_exact.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_geometric_partial_sum(mut v_a: Py, mut v_r: Py, mut v_n: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_R: Py = Py::None;
    v_n = py::int(&v_n.clone())?;
    if (py::not(&{ let t577 = Py::Int(1i64); Py::Bool({ let t578 = v_n.clone(); if py::le(&t577, &t578)? { { let t579 = Py::Int(500i64); if py::le(&t578, &t579)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be between 1 and 500"))));
    }
    let t580 = (Py::tuple(vec![f__frac(v_a.clone())?, f__frac(v_r.clone())?])).items()?;
    if t580.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t580[0].clone();
    v_R = t580[1].clone();
    return Ok(Py::dict(vec![(Py::str("sum"), py::float(&(if ({ let t581 = v_R.clone(); Py::Bool({ let t582 = Py::Int(1i64); if py::eq(&t581, &t582) { true } else { false } }) }).truthy() { py::mul(&v_n.clone(), &v_A.clone())? } else { py::truediv(&py::mul(&v_A.clone(), &py::sub(&Py::Int(1i64), &py::pow(&v_R.clone(), &v_n.clone())?)?)?, &py::sub(&Py::Int(1i64), &v_R.clone())?)? }))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_geometric_series_sum(mut v_a: Py, mut v_r: Py) -> R {
    if (py::not(&{ let t592 = py::abs(&v_r.clone())?; Py::Bool({ let t593 = Py::Int(1i64); if py::lt(&t592, &t593)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("an infinite geometric series converges only when |r| < 1"))));
    }
    return Ok(Py::dict(vec![(Py::str("sum"), py::float(&py::truediv(&f__frac(v_a.clone())?, &py::sub(&Py::Int(1i64), &f__frac(v_r.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gradient(mut v_f: Py, mut v_p: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_names: Py = Py::None;
    v_names = f__vars(v_p.clone())?;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), v_names.clone()], &[])?;
    let _ = { let mut v: Vec<Py> = vec![]; v.push(v_e.clone()); v.push(v_names.clone()); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? };
    v_g = { let mut out = vec![]; for t1385 in py::iter(&(v_names.clone()))? { let mut v_v: Py; 
v_v = t1385;
out.push({ let mut v: Vec<Py> = vec![]; v.push(ax_engine::pysym::call("diff", &[v_e.clone(), py::getitem(&c__SYMS()?, &v_v.clone())?], &[])?); v.push(v_names.clone()); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? }); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("gradient"), v_g.clone()), (Py::str("magnitude"), py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t1386 in py::iter(&(v_g.clone()))? { let mut v_c: Py; 
v_c = t1386;
out.push(py::mul(&v_c.clone(), &v_c.clone())?); } Py::list(out) })?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_greens_theorem(mut v_P: Py, mut v_Q: Py, mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_circ: Py = Py::None;
    let mut v_const: Py = Py::None;
    let mut v_dbl: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_eP: Py = Py::None;
    let mut v_eQ: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_parts: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_side: Py = Py::None;
    let mut v_v: Py = Py::None;
    if (py::not(&{ let t630 = { let t626 = v_a.clone(); Py::Bool({ let t627 = v_b.clone(); if py::lt(&t626, &t627)? { true } else { false } }) }; if !t630.truthy() { t630 } else { { let t628 = v_c.clone(); Py::Bool({ let t629 = v_d.clone(); if py::lt(&t628, &t629)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the rectangle needs a < b and c < d"))));
    }
    let t631 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:parse_expr", &[v_P.clone(), c__XY()?], &[])?, ax_engine::pysym::helper("calculus:parse_expr", &[v_Q.clone(), c__XY()?], &[])?])).items()?;
    if t631.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_eP = t631[0].clone();
    v_eQ = t631[1].clone();
    v_g = f__fl(py::sub(&ax_engine::pysym::call("diff", &[v_eQ.clone(), c_X()?], &[])?, &ax_engine::pysym::call("diff", &[v_eP.clone(), c_Y()?], &[])?)?, c__XY()?)?;
    let t659 = (f__settled({ let v_P = v_P.clone(); let v_Q = v_Q.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_g = v_g.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__gl2(v_g.clone(), v_a.clone(), v_b.clone(), { let v_P = v_P.clone(); let v_Q = v_Q.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_g = v_g.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_c.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_P = v_P.clone(); let v_Q = v_Q.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_g = v_g.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_d.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_P = v_P.clone(); let v_Q = v_Q.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_g = v_g.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__gl2({ let v_P = v_P.clone(); let v_Q = v_Q.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_g = v_g.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_y: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::call(&v_g.clone(), &[v_x.clone(), v_y.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_a.clone(), v_b.clone(), { let v_P = v_P.clone(); let v_Q = v_Q.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_g = v_g.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_c.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_P = v_P.clone(); let v_Q = v_Q.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_g = v_g.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_d.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::tuple(vec![Py::Int(24i64), Py::Int(40i64), Py::Int(64i64)]))?).items()?;
    if t659.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dbl = t659[0].clone();
    v_S = t659[1].clone();
    v_side = { let v_P = v_P.clone(); let v_Q = v_Q.clone(); let v_S = v_S.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_dbl = v_dbl.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_g = v_g.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_e: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_v: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_lo: Py = a.get(2).cloned().unwrap_or(Py::None);
            let mut v_hi: Py = a.get(3).cloned().unwrap_or(Py::None);
            return Ok((if (py::not(&py::getattr(&v_e.clone(), "free_symbols")?)).truthy() { Py::Float(0.0) } else { ax_engine::pysym::helper("calculus:quad", &[ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![v_v.clone()])], &[])?, v_lo.clone(), v_hi.clone()], &[])? }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_const = { let v_P = v_P.clone(); let v_Q = v_Q.clone(); let v_S = v_S.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_dbl = v_dbl.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_g = v_g.clone(); let v_side = v_side.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_e: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_lo: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_hi: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok((if (py::not(&py::getattr(&v_e.clone(), "free_symbols")?)).truthy() { py::mul(&py::float(&v_e.clone())?, &py::sub(&v_hi.clone(), &v_lo.clone())?)? } else { Py::None }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_parts = Py::list(vec![]);
    for t660 in py::iter(&(Py::tuple(vec![Py::tuple(vec![py::method(&v_eP.clone(), "subs", &[c_Y()?, f__rat(v_c.clone())?])?, c_X()?, v_a.clone(), v_b.clone(), Py::Int(1i64)]), Py::tuple(vec![py::method(&v_eQ.clone(), "subs", &[c_X()?, f__rat(v_b.clone())?])?, c_Y()?, v_c.clone(), v_d.clone(), Py::Int(1i64)]), Py::tuple(vec![py::method(&v_eP.clone(), "subs", &[c_Y()?, f__rat(v_d.clone())?])?, c_X()?, v_a.clone(), v_b.clone(), py::neg(&Py::Int(1i64))?]), Py::tuple(vec![py::method(&v_eQ.clone(), "subs", &[c_X()?, f__rat(v_a.clone())?])?, c_Y()?, v_c.clone(), v_d.clone(), py::neg(&Py::Int(1i64))?])])))? {
        let t663 = (t660).items()?;
        if t663.len() != 5 { return Err(PyErr::Value("unpack".into())); }
        v_e = t663[0].clone();
        v_v = t663[1].clone();
        v_lo = t663[2].clone();
        v_hi = t663[3].clone();
        v_s = t663[4].clone();
        v_k = py::call(&v_const.clone(), &[v_e.clone(), v_lo.clone(), v_hi.clone()])?;
        let _ = py::method(&v_parts.clone(), "append", &[py::mul(&v_s.clone(), &(if ({ let t661 = v_k.clone(); Py::Bool({ let t662 = Py::None; if !py::is(&t661, &t662) { true } else { false } }) }).truthy() { v_k.clone() } else { py::call(&v_side.clone(), &[v_e.clone(), v_v.clone(), v_lo.clone(), v_hi.clone()])? }))?])?;
    }
    v_circ = py::fsum(&v_parts.clone())?;
    if ({ let t664 = py::abs(&py::sub(&v_circ.clone(), &v_dbl.clone())?)?; Py::Bool({ let t665 = py::mul(&Py::Float(1e-08), &py::minmax(&[py::abs(&v_dbl.clone())?, py::mul(&Py::Float(0.001), &v_S.clone())?, Py::Float(1e-12)], true)?)?; if py::gt(&t664, &t665)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two sides of Green's theorem disagree numerically (is the field smooth on the rectangle?)"))));
    }
    return Ok(Py::dict(vec![(Py::str("circulation"), v_circ.clone()), (Py::str("double_integral"), v_dbl.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hessian_classification(mut v_f: Py, mut v_p: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_fx: Py = Py::None;
    let mut v_fxx: Py = Py::None;
    let mut v_fxy: Py = Py::None;
    let mut v_fy: Py = Py::None;
    let mut v_fyy: Py = Py::None;
    let mut v_kind: Py = Py::None;
    let mut v_scale: Py = Py::None;
    if ({ let t672 = py::len(&v_p.clone())?; Py::Bool({ let t673 = Py::Int(2i64); if py::ne(&t672, &t673) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("this test is for f(x, y) at a point (x, y)"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), c__XY()?], &[])?;
    let _ = { let mut v: Vec<Py> = vec![]; v.push(v_e.clone()); v.push(c__XY()?); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? };
    let t674 = (Py::tuple(vec![{ let mut v: Vec<Py> = vec![]; v.push(ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?); v.push(c__XY()?); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? }, { let mut v: Vec<Py> = vec![]; v.push(ax_engine::pysym::call("diff", &[v_e.clone(), c_Y()?], &[])?); v.push(c__XY()?); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? }])).items()?;
    if t674.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fx = t674[0].clone();
    v_fy = t674[1].clone();
    let t676 = ({ let mut out = vec![]; for t675 in py::iter(&(Py::tuple(vec![Py::tuple(vec![c_X()?, c_X()?]), Py::tuple(vec![c_Y()?, c_Y()?]), Py::tuple(vec![c_X()?, c_Y()?])])))? { let mut v_v: Py; 
v_v = t675;
out.push({ let mut v: Vec<Py> = vec![]; v.push({ let mut v: Vec<Py> = vec![]; v.push(v_e.clone()); v.extend((v_v.clone()).items()?); ax_engine::pysym::call("diff", &v, &[])? }); v.push(c__XY()?); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? }); } Py::list(out) }).items()?;
    if t676.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_fxx = t676[0].clone();
    v_fyy = t676[1].clone();
    v_fxy = t676[2].clone();
    v_scale = py::minmax(&[Py::Float(1.0), py::abs(&v_fxx.clone())?, py::abs(&v_fyy.clone())?, py::abs(&v_fxy.clone())?], true)?;
    if ({ let t677 = py::minmax(&[py::abs(&v_fx.clone())?, py::abs(&v_fy.clone())?], true)?; Py::Bool({ let t678 = py::mul(&Py::Float(1e-10), &v_scale.clone())?; if py::gt(&t677, &t678)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["(".to_string(), py::format(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, "g")?, ", ".to_string(), py::format(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, "g")?, ") is not a critical point (the gradient is (".to_string(), py::format(&v_fx.clone(), ".6g")?, ", ".to_string(), py::format(&v_fy.clone(), ".6g")?, "))".to_string()].concat()))));
    }
    v_D = py::sub(&py::mul(&v_fxx.clone(), &v_fyy.clone())?, &py::mul(&v_fxy.clone(), &v_fxy.clone())?)?;
    if ({ let t679 = py::abs(&v_D.clone())?; Py::Bool({ let t680 = py::mul(&py::mul(&Py::Float(1e-12), &v_scale.clone())?, &v_scale.clone())?; if py::le(&t679, &t680)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("D = 0: the second derivative test is inconclusive"))));
    }
    v_kind = (if ({ let t681 = v_D.clone(); Py::Bool({ let t682 = Py::Int(0i64); if py::lt(&t681, &t682)? { true } else { false } }) }).truthy() { Py::str("saddle point") } else { (if ({ let t683 = v_fxx.clone(); Py::Bool({ let t684 = Py::Int(0i64); if py::gt(&t683, &t684)? { true } else { false } }) }).truthy() { Py::str("local minimum") } else { Py::str("local maximum") }) });
    return Ok(Py::dict(vec![(Py::str("fxx"), v_fxx.clone()), (Py::str("fxy"), v_fxy.clone()), (Py::str("fyy"), v_fyy.clone()), (Py::str("D"), v_D.clone()), (Py::str("classification"), v_kind.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_implicit_derivative(mut v_F: Py, mut v_p: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_fx: Py = Py::None;
    let mut v_fy: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_x0: Py = Py::None;
    let mut v_y0: Py = Py::None;
    v_e = f__eqn(v_F.clone(), c__XY()?)?;
    let t704 = (v_p.clone()).items()?;
    if t704.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x0 = t704[0].clone();
    v_y0 = t704[1].clone();
    v_scale = py::minmax(&[py::add(&Py::list(vec![Py::Float(1.0)]), &{ let mut out = vec![]; for t705 in py::iter(&(ax_engine::pysym::call("Add.make_args", &[ax_engine::pysym::call("expand", &[v_e.clone()], &[])?], &[])?))? { let mut v_t: Py; 
v_t = t705;
out.push(py::abs(&ax_engine::pysym::helper("calculus:at", &[v_t.clone(), c__XY()?, v_x0.clone(), v_y0.clone()], &[])?)?); } Py::list(out) })?], true)?;
    if ({ let t706 = py::abs(&ax_engine::pysym::helper("calculus:at", &[v_e.clone(), c__XY()?, v_x0.clone(), v_y0.clone()], &[])?)?; Py::Bool({ let t707 = py::mul(&Py::Float(1e-09), &v_scale.clone())?; if py::gt(&t706, &t707)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the point (".to_string(), py::format(&v_x0.clone(), "g")?, ", ".to_string(), py::format(&v_y0.clone(), "g")?, ") is not on the curve".to_string()].concat()))));
    }
    let t708 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?, c__XY()?, v_x0.clone(), v_y0.clone()], &[])?, ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_Y()?], &[])?, c__XY()?, v_x0.clone(), v_y0.clone()], &[])?])).items()?;
    if t708.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fx = t708[0].clone();
    v_fy = t708[1].clone();
    if ({ let t709 = py::abs(&v_fy.clone())?; Py::Bool({ let t710 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_fx.clone())?], true)?)?; if py::le(&t709, &t710)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("F_y = 0 at this point: the tangent is vertical (dy/dx is undefined)"))));
    }
    return Ok(Py::dict(vec![(Py::str("dy_dx"), py::truediv(&py::neg(&v_fx.clone())?, &v_fy.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(non_snake_case)]
fn f_improper_integral(v_f: Py, v_a: Py) -> R {
    ax_engine::pysym::helper("calculus:improper_integral", &[v_f, v_a], &[])
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_inflection_points(mut v_f: Py) -> R {
    let mut v_d2: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_rs: Py = Py::None;
    v_p = ax_engine::pysym::helper("calculus:poly_of", &[v_f.clone()], &[])?;
    if ({ let t729 = py::method(&v_p.clone(), "degree", &[])?; Py::Bool({ let t730 = Py::Int(2i64); if py::lt(&t729, &t730)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a polynomial of degree < 2 has no concavity changes"))));
    }
    v_d2 = py::method(&py::method(&v_p.clone(), "diff", &[c_X()?])?, "diff", &[c_X()?])?;
    if ({ let t733 = py::getattr(&v_d2.clone(), "is_zero")?; if t733.truthy() { t733 } else { { let t731 = py::method(&v_d2.clone(), "degree", &[])?; Py::Bool({ let t732 = Py::Int(1i64); if py::lt(&t731, &t732)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::dict(vec![(Py::str("inflection_points"), Py::list(vec![]))]));
    }
    v_rs = f__distinct_real_roots(v_d2.clone())?;
    return Ok(Py::dict(vec![(Py::str("inflection_points"), { let mut out = vec![]; for t738 in py::iter(&(py::zip(&[v_rs.clone(), (if (v_rs.clone()).truthy() { f__side_signs(v_d2.clone(), v_rs.clone())? } else { Py::list(vec![]) })])?))? { let mut v_left: Py; let mut v_r: Py; let mut v_right: Py; 
let t739 = (t738).items()?;
if t739.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t739[0].clone();
let t740 = (t739[1].clone()).items()?;
if t740.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_left = t740[0].clone();
v_right = t740[1].clone();
if ({ let t741 = py::mul(&v_left.clone(), &v_right.clone())?; Py::Bool({ let t742 = Py::Int(0i64); if py::lt(&t741, &t742)? { true } else { false } }) }).truthy() { out.push(f__pfloat(v_r.clone())?); }} Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_jacobian_determinant(mut v_xu: Py, mut v_yv: Py, mut v_p: Py) -> R {
    let mut v_U_: Py = Py::None;
    let mut v_V_s: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let mut v_ey: Py = Py::None;
    let mut v_names: Py = Py::None;
    v_names = Py::tuple(vec![Py::str("u"), Py::str("v")]);
    let t774 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:parse_expr", &[v_xu.clone(), v_names.clone()], &[])?, ax_engine::pysym::helper("calculus:parse_expr", &[v_yv.clone(), v_names.clone()], &[])?])).items()?;
    if t774.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ex = t774[0].clone();
    v_ey = t774[1].clone();
    let t775 = (Py::tuple(vec![py::getitem(&c__SYMS()?, &Py::str("u"))?, py::getitem(&c__SYMS()?, &Py::str("v"))?])).items()?;
    if t775.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_U_ = t775[0].clone();
    v_V_s = t775[1].clone();
    v_d = { let v_U_ = v_U_.clone(); let v_V_s = v_V_s.clone(); let v_ex = v_ex.clone(); let v_ey = v_ey.clone(); let v_names = v_names.clone(); let v_p = v_p.clone(); let v_xu = v_xu.clone(); let v_yv = v_yv.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_e: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_s: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok({ let mut v: Vec<Py> = vec![]; v.push(ax_engine::pysym::call("diff", &[v_e.clone(), v_s.clone()], &[])?); v.push(v_names.clone()); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::dict(vec![(Py::str("jacobian"), py::sub(&py::mul(&py::call(&v_d.clone(), &[v_ex.clone(), v_U_.clone()])?, &py::call(&v_d.clone(), &[v_ey.clone(), v_V_s.clone()])?)?, &py::mul(&py::call(&v_d.clone(), &[v_ex.clone(), v_V_s.clone()])?, &py::call(&v_d.clone(), &[v_ey.clone(), v_U_.clone()])?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_laplacian(mut v_f: Py, mut v_p: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_names: Py = Py::None;
    v_names = f__vars(v_p.clone())?;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), v_names.clone()], &[])?;
    let _ = { let mut v: Vec<Py> = vec![]; v.push(v_e.clone()); v.push(v_names.clone()); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? };
    return Ok(Py::dict(vec![(Py::str("laplacian"), py::fsum(&{ let mut out = vec![]; for t789 in py::iter(&(v_names.clone()))? { let mut v_v: Py; 
v_v = t789;
out.push({ let mut v: Vec<Py> = vec![]; v.push(ax_engine::pysym::call("diff", &[v_e.clone(), py::getitem(&c__SYMS()?, &v_v.clone())?, Py::Int(2i64)], &[])?); v.push(v_names.clone()); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? }); } Py::list(out) })?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_left_riemann_sum(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_h: Py = Py::None;
    let t811 = (f__grid(v_f.clone(), v_a.clone(), v_b.clone(), v_n.clone())?).items()?;
    if t811.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_e = t811[0].clone();
    v_h = t811[1].clone();
    v_n = t811[2].clone();
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        return Ok(Py::dict(vec![(Py::str("approximation"), py::float(&py::mul(&v_h.clone(), &ax_engine::pymp::call("fsum", &[f__values(v_e.clone(), { let mut out = vec![]; for t813 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t813;
out.push(py::add(&v_a.clone(), &py::mul(&v_k.clone(), &v_h.clone())?)?); } Py::list(out) })?], &[])?)?)?)]));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_legendre_polynomial(mut v_n: Py, mut v_x: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_p0: Py = Py::None;
    let mut v_p1: Py = Py::None;
    v_n = py::int(&v_n.clone())?;
    if (py::not(&{ let t825 = Py::Int(0i64); Py::Bool({ let t826 = v_n.clone(); if py::le(&t825, &t826)? { { let t827 = Py::Int(200i64); if py::le(&t826, &t827)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the degree must be between 0 and 200"))));
    }
    let t828 = (Py::tuple(vec![Py::Float(1.0), v_x.clone()])).items()?;
    if t828.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p0 = t828[0].clone();
    v_p1 = t828[1].clone();
    if ({ let t829 = v_n.clone(); Py::Bool({ let t830 = Py::Int(0i64); if py::eq(&t829, &t830) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("P"), Py::Float(1.0))]));
    }
    for t831 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_n.clone()).i()?]) {
        v_k = t831;
        let t832 = (Py::tuple(vec![v_p1.clone(), py::truediv(&py::sub(&py::mul(&py::mul(&py::add(&py::mul(&Py::Int(2i64), &v_k.clone())?, &Py::Int(1i64))?, &v_x.clone())?, &v_p1.clone())?, &py::mul(&v_k.clone(), &v_p0.clone())?)?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?])).items()?;
        if t832.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p0 = t832[0].clone();
        v_p1 = t832[1].clone();
    }
    return Ok(Py::dict(vec![(Py::str("P"), v_p1.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_limit_at_infinity(mut v_f: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("limit"), f__limval(f__sympy_limit(ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?, c_X()?, ax_engine::pysym::constant("oo")?, Py::None)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_limit_at_point(mut v_f: Py, mut v_a: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("limit"), f__limval(f__sympy_limit(ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?, c_X()?, f__rat(v_a.clone())?, Py::str("+-"))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_line_integral_scalar(mut v_f: Py, mut v_xt: Py, mut v_yt: Py, mut v_t0: Py, mut v_t1: Py) -> R {
    let mut v_ef: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let mut v_ey: Py = Py::None;
    if (py::not(&{ let t878 = v_t0.clone(); Py::Bool({ let t879 = v_t1.clone(); if py::lt(&t878, &t879)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the parameter must run forward (t0 < t1) for an integral with respect to arc length"))));
    }
    let t880 = (f__path(v_xt.clone(), v_yt.clone())?).items()?;
    if t880.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ex = t880[0].clone();
    v_ey = t880[1].clone();
    v_ef = py::method(&ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), c__XY()?], &[])?, "subs", &[Py::dict(vec![(c_X()?, v_ex.clone()), (c_Y()?, v_ey.clone())])])?;
    return Ok(Py::dict(vec![(Py::str("line_integral"), f__q_t(py::mul(&v_ef.clone(), &ax_engine::pysym::call("sqrt", &[py::add(&py::pow(&ax_engine::pysym::call("diff", &[v_ex.clone(), c_T()?], &[])?, &Py::Int(2i64))?, &py::pow(&ax_engine::pysym::call("diff", &[v_ey.clone(), c_T()?], &[])?, &Py::Int(2i64))?)?], &[])?)?, v_t0.clone(), v_t1.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_line_integral_vector(mut v_P: Py, mut v_Q: Py, mut v_xt: Py, mut v_yt: Py, mut v_t0: Py, mut v_t1: Py) -> R {
    let mut v_eP: Py = Py::None;
    let mut v_eQ: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let mut v_ey: Py = Py::None;
    let t894 = (f__path(v_xt.clone(), v_yt.clone())?).items()?;
    if t894.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ex = t894[0].clone();
    v_ey = t894[1].clone();
    let t896 = ({ let mut out = vec![]; for t895 in py::iter(&(Py::tuple(vec![v_P.clone(), v_Q.clone()])))? { let mut v_c: Py; 
v_c = t895;
out.push(py::method(&ax_engine::pysym::helper("calculus:parse_expr", &[v_c.clone(), c__XY()?], &[])?, "subs", &[Py::dict(vec![(c_X()?, v_ex.clone()), (c_Y()?, v_ey.clone())])])?); } Py::list(out) }).items()?;
    if t896.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_eP = t896[0].clone();
    v_eQ = t896[1].clone();
    return Ok(Py::dict(vec![(Py::str("line_integral"), f__q_t(py::add(&py::mul(&v_eP.clone(), &ax_engine::pysym::call("diff", &[v_ex.clone(), c_T()?], &[])?)?, &py::mul(&v_eQ.clone(), &ax_engine::pysym::call("diff", &[v_ey.clone(), c_T()?], &[])?)?)?, v_t0.clone(), v_t1.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_linear_approximation(mut v_f: Py, mut v_a: Py, mut v_x1: Py) -> R {
    let mut v_actual: Py = Py::None;
    let mut v_approx: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_fa: Py = Py::None;
    let mut v_m: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    let t912 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:at", &[v_e.clone(), Py::tuple(vec![c_X()?]), v_a.clone()], &[])?, ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?, Py::tuple(vec![c_X()?]), v_a.clone()], &[])?])).items()?;
    if t912.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t912[0].clone();
    v_m = t912[1].clone();
    v_approx = py::add(&v_fa.clone(), &py::mul(&v_m.clone(), &py::sub(&v_x1.clone(), &v_a.clone())?)?)?;
    v_actual = ax_engine::pysym::helper("calculus:at", &[v_e.clone(), Py::tuple(vec![c_X()?]), v_x1.clone()], &[])?;
    return Ok(Py::dict(vec![(Py::str("approximation"), v_approx.clone()), (Py::str("actual"), v_actual.clone()), (Py::str("error"), py::sub(&v_actual.clone(), &v_approx.clone())?), (Py::str("slope"), v_m.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mean_value_theorem(mut v_f: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    if (py::not(&{ let t921 = v_a.clone(); Py::Bool({ let t922 = v_b.clone(); if py::lt(&t921, &t922)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    v_p = ax_engine::pysym::helper("calculus:poly_of", &[v_f.clone()], &[])?;
    let t923 = (Py::tuple(vec![f__rat(v_a.clone())?, f__rat(v_b.clone())?])).items()?;
    if t923.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t923[0].clone();
    v_B = t923[1].clone();
    v_m = py::truediv(&py::sub(&py::method(&v_p.clone(), "eval", &[v_B.clone()])?, &py::method(&v_p.clone(), "eval", &[v_A.clone()])?)?, &py::sub(&v_B.clone(), &v_A.clone())?)?;
    v_q = py::sub(&py::method(&v_p.clone(), "diff", &[c_X()?])?, &v_m.clone())?;
    if (py::getattr(&v_q.clone(), "is_zero")?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("f is linear: every c in (a, b) satisfies the mean value theorem"))));
    }
    return Ok(Py::dict(vec![(Py::str("secant_slope"), py::float(&v_m.clone())?), (Py::str("c_values"), { let mut out = vec![]; for t924 in py::iter(&(ax_engine::pysym::helper("calculus:real_roots_in", &[v_q.clone(), v_a.clone(), v_b.clone()], &[])?))? { let mut v_c: Py; 
v_c = t924;
out.push(f__pfloat(v_c.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_newton_method(mut v_f: Py, mut v_x0: Py, mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dg: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_fx_: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_its: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_n = py::int(&v_n.clone())?;
    if (py::not(&{ let t997 = Py::Int(1i64); Py::Bool({ let t998 = v_n.clone(); if py::le(&t997, &t998)? { { let t999 = Py::Int(50i64); if py::le(&t998, &t999)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of iterations must be between 1 and 50"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    let t1000 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[])?, ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?, Py::tuple(vec![c_X()?])], &[])?])).items()?;
    if t1000.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g = t1000[0].clone();
    v_dg = t1000[1].clone();
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        let t1001 = (Py::tuple(vec![ax_engine::pymp::call("mpf", &[v_x0.clone()], &[])?, Py::list(vec![])])).items()?;
        if t1001.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t1001[0].clone();
        v_its = t1001[1].clone();
        for t1002 in py::range_iter(&[(v_n.clone()).i()?]) {
            v__ = t1002;
            v_d = py::call(&v_dg.clone(), &[v_x.clone()])?;
            if ({ let t1003 = v_d.clone(); Py::Bool({ let t1004 = Py::Int(0i64); if py::eq(&t1003, &t1004) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("f'(x) = 0 at an iterate, so Newton's method cannot continue"))));
            }
            v_x = py::sub(&v_x.clone(), &py::truediv(&py::call(&v_g.clone(), &[v_x.clone()])?, &v_d.clone())?)?;
            let _ = py::method(&v_its.clone(), "append", &[py::float(&v_x.clone())?])?;
        }
        v_fx_ = py::float(&py::call(&v_g.clone(), &[v_x.clone()])?)?;
    }
    return Ok(Py::dict(vec![(Py::str("x_n"), py::float(&v_x.clone())?), (Py::str("f_x_n"), v_fx_.clone()), (Py::str("iterates"), v_its.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_normal_line(mut v_f: Py, mut v_a: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_t = f_tangent_line(v_f.clone(), v_a.clone())?;
    if ({ let t1015 = py::abs(&py::getitem(&v_t.clone(), &Py::str("slope"))?)?; Py::Bool({ let t1016 = Py::Float(1e-12); if py::lt(&t1015, &t1016)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the tangent is horizontal there, so the normal line is vertical (x = a) and has no slope"))));
    }
    v_m = py::truediv(&py::neg(&Py::Float(1.0))?, &py::getitem(&v_t.clone(), &Py::str("slope"))?)?;
    return Ok(Py::dict(vec![(Py::str("slope"), v_m.clone()), (Py::str("intercept"), py::sub(&py::getitem(&v_t.clone(), &Py::str("y0"))?, &py::mul(&v_m.clone(), &v_a.clone())?)?), (Py::str("y0"), py::getitem(&v_t.clone(), &Py::str("y0"))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nth_derivative_at_point(mut v_f: Py, mut v_n: Py, mut v_a: Py) -> R {
    let mut v_dn: Py = Py::None;
    let mut v_e: Py = Py::None;
    v_n = py::int(&v_n.clone())?;
    if (py::not(&{ let t1025 = Py::Int(1i64); Py::Bool({ let t1026 = v_n.clone(); if py::le(&t1025, &t1026)? { { let t1027 = Py::Int(12i64); if py::le(&t1026, &t1027)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the order n must be between 1 and 12"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    let _ = ax_engine::pysym::helper("calculus:at", &[v_e.clone(), Py::tuple(vec![c_X()?]), v_a.clone()], &[])?;
    v_dn = ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?, v_n.clone()], &[])?;
    return Ok(Py::dict(vec![(Py::str("nth_derivative"), ax_engine::pysym::helper("calculus:at", &[v_dn.clone(), Py::tuple(vec![c_X()?]), v_a.clone()], &[])?), (Py::str("nth_derivative_expression"), ax_engine::pysym::helper("calculus:show", &[v_dn.clone()], &[])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_numerical_derivative(mut v_f: Py, mut v_a: Py, mut v_h: Py, mut v_method: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_exact: Py = Py::None;
    let mut v_g: Py = Py::None;
    if (py::not(&{ let t1036 = v_h.clone(); Py::Bool({ let t1037 = Py::Int(0i64); if py::gt(&t1036, &t1037)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the step h must be positive"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    v_g = ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[])?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        let t1038 = (Py::tuple(vec![ax_engine::pymp::call("mpf", &[v_a.clone()], &[])?, ax_engine::pymp::call("mpf", &[v_h.clone()], &[])?])).items()?;
        if t1038.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_A = t1038[0].clone();
        v_H = t1038[1].clone();
        if ({ let t1039 = v_method.clone(); Py::Bool({ let t1040 = Py::str("forward"); if py::eq(&t1039, &t1040) { true } else { false } }) }).truthy() {
            v_d = py::truediv(&py::sub(&py::call(&v_g.clone(), &[py::add(&v_A.clone(), &v_H.clone())?])?, &py::call(&v_g.clone(), &[v_A.clone()])?)?, &v_H.clone())?;
        } else {
            if ({ let t1041 = v_method.clone(); Py::Bool({ let t1042 = Py::str("backward"); if py::eq(&t1041, &t1042) { true } else { false } }) }).truthy() {
                v_d = py::truediv(&py::sub(&py::call(&v_g.clone(), &[v_A.clone()])?, &py::call(&v_g.clone(), &[py::sub(&v_A.clone(), &v_H.clone())?])?)?, &v_H.clone())?;
            } else {
                v_d = py::truediv(&py::sub(&py::call(&v_g.clone(), &[py::add(&v_A.clone(), &v_H.clone())?])?, &py::call(&v_g.clone(), &[py::sub(&v_A.clone(), &v_H.clone())?])?)?, &py::mul(&Py::Int(2i64), &v_H.clone())?)?;
            }
        }
    }
    v_exact = ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?, Py::tuple(vec![c_X()?]), v_a.clone()], &[])?;
    return Ok(Py::dict(vec![(Py::str("approximation"), py::float(&v_d.clone())?), (Py::str("exact"), v_exact.clone()), (Py::str("error"), py::sub(&py::float(&v_d.clone())?, &v_exact.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_parametric_arc_length(mut v_xt: Py, mut v_yt: Py, mut v_t0: Py, mut v_t1: Py) -> R {
    let mut v_ex: Py = Py::None;
    let mut v_ey: Py = Py::None;
    if (py::not(&{ let t1051 = v_t0.clone(); Py::Bool({ let t1052 = v_t1.clone(); if py::lt(&t1051, &t1052)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the parameter interval needs t0 < t1"))));
    }
    let t1053 = (f__path(v_xt.clone(), v_yt.clone())?).items()?;
    if t1053.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ex = t1053[0].clone();
    v_ey = t1053[1].clone();
    return Ok(Py::dict(vec![(Py::str("arc_length"), f__q_t(ax_engine::pysym::call("sqrt", &[py::add(&py::pow(&ax_engine::pysym::call("diff", &[v_ex.clone(), c_T()?], &[])?, &Py::Int(2i64))?, &py::pow(&ax_engine::pysym::call("diff", &[v_ey.clone(), c_T()?], &[])?, &Py::Int(2i64))?)?], &[])?, v_t0.clone(), v_t1.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_parametric_slope(mut v_xt: Py, mut v_yt: Py, mut v_t0: Py) -> R {
    let mut v_dx: Py = Py::None;
    let mut v_dy: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let mut v_ey: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_vx: Py = Py::None;
    let t1067 = (f__path(v_xt.clone(), v_yt.clone())?).items()?;
    if t1067.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ex = t1067[0].clone();
    v_ey = t1067[1].clone();
    let t1068 = (Py::tuple(vec![ax_engine::pysym::call("diff", &[v_ex.clone(), c_T()?], &[])?, ax_engine::pysym::call("diff", &[v_ey.clone(), c_T()?], &[])?])).items()?;
    if t1068.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dx = t1068[0].clone();
    v_dy = t1068[1].clone();
    v_vx = ax_engine::pysym::helper("calculus:at", &[v_dx.clone(), Py::tuple(vec![c_T()?]), v_t0.clone()], &[])?;
    if ({ let t1069 = py::abs(&v_vx.clone())?; Py::Bool({ let t1070 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&ax_engine::pysym::helper("calculus:at", &[v_dy.clone(), Py::tuple(vec![c_T()?]), v_t0.clone()], &[])?)?], true)?)?; if py::le(&t1069, &t1070)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("dx/dt = 0 at this parameter: the tangent is vertical (dy/dx undefined)"))));
    }
    v_m = py::truediv(&v_dy.clone(), &v_dx.clone())?;
    return Ok(Py::dict(vec![(Py::str("dy_dx"), py::truediv(&ax_engine::pysym::helper("calculus:at", &[v_dy.clone(), Py::tuple(vec![c_T()?]), v_t0.clone()], &[])?, &v_vx.clone())?), (Py::str("d2y_dx2"), ax_engine::pysym::helper("calculus:at", &[py::truediv(&ax_engine::pysym::call("diff", &[v_m.clone(), c_T()?], &[])?, &v_dx.clone())?, Py::tuple(vec![c_T()?]), v_t0.clone()], &[])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_partial_derivative(mut v_f: Py, mut v_var: Py, mut v_p: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_names: Py = Py::None;
    v_names = f__vars(v_p.clone())?;
    if ({ let t1084 = v_var.clone(); Py::Bool({ let t1085 = v_names.clone(); if !py::contains(&t1085, &t1084)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the variable must be one of ".to_string(), py::format(&py::method(&Py::str(", "), "join", &[v_names.clone()])?, "")?].concat()))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), v_names.clone()], &[])?;
    let _ = { let mut v: Vec<Py> = vec![]; v.push(v_e.clone()); v.push(v_names.clone()); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? };
    return Ok(Py::dict(vec![(Py::str("partial"), { let mut v: Vec<Py> = vec![]; v.push(ax_engine::pysym::call("diff", &[v_e.clone(), py::getitem(&c__SYMS()?, &v_var.clone())?], &[])?); v.push(v_names.clone()); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_polar_area(mut v_r: Py, mut v_al: Py, mut v_be: Py) -> R {
    let mut v_e: Py = Py::None;
    if (py::not(&{ let t1098 = v_al.clone(); Py::Bool({ let t1099 = v_be.clone(); if py::lt(&t1098, &t1099)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the angle interval needs alpha < beta"))));
    }
    if ({ let t1100 = py::sub(&v_be.clone(), &v_al.clone())?; Py::Bool({ let t1101 = py::add(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &Py::Float(1e-12))?; if py::gt(&t1100, &t1101)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the angle interval may span at most one full turn (2 pi)"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_r.clone(), Py::tuple(vec![Py::str("theta")])], &[])?;
    let _ = ax_engine::pysym::helper("calculus:continuous_on", &[v_e.clone(), c_TH()?, v_al.clone(), v_be.clone()], &[])?;
    return Ok(Py::dict(vec![(Py::str("area"), ax_engine::pysym::helper("calculus:quad", &[ax_engine::pysym::helper("calculus:_compile", &[py::truediv(&py::pow(&v_e.clone(), &Py::Int(2i64))?, &Py::Int(2i64))?, Py::tuple(vec![c_TH()?])], &[])?, v_al.clone(), v_be.clone()], &[])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ratio_test(mut v_a: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_a.clone(), Py::tuple(vec![Py::str("n")])], &[])?;
    if ({ let t1114 = { let t1110 = py::method(&v_e.clone(), "subs", &[c_NN()?, Py::Int(1i64)])?; Py::Bool({ let t1111 = Py::Int(0i64); if py::eq(&t1110, &t1111) { true } else { false } }) }; if !t1114.truthy() { t1114 } else { { let t1112 = py::method(&v_e.clone(), "subs", &[c_NN()?, Py::Int(2i64)])?; Py::Bool({ let t1113 = Py::Int(0i64); if py::eq(&t1112, &t1113) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the terms are zero"))));
    }
    v_r = ax_engine::pysym::call("simplify", &[py::truediv(&py::method(&v_e.clone(), "subs", &[c_NN()?, py::add(&c_NN()?, &Py::Int(1i64))?])?, &v_e.clone())?], &[])?;
    v_L = f__limval(f__sympy_limit(ax_engine::pysym::call("Abs", &[v_r.clone()], &[])?, c_NN()?, ax_engine::pysym::constant("oo")?, Py::None)?)?;
    if ({ let t1126 = py::abs(&py::sub(&v_L.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t1127 = Py::Float(1e-12); if py::lt(&t1126, &t1127)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the ratio limit is 1: the ratio test is inconclusive"))));
    }
    return Ok(Py::dict(vec![(Py::str("ratio_limit"), v_L.clone()), (Py::str("verdict"), (if ({ let t1128 = v_L.clone(); Py::Bool({ let t1129 = Py::Int(1i64); if py::lt(&t1128, &t1129)? { true } else { false } }) }).truthy() { Py::str("converges") } else { Py::str("diverges") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_regularized_gamma(mut v_a: Py, mut v_x: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    if ({ let t1138 = py::not(&{ let t1134 = v_a.clone(); Py::Bool({ let t1135 = Py::Int(0i64); if py::gt(&t1134, &t1135)? { true } else { false } }) }); if t1138.truthy() { t1138 } else { { let t1136 = v_x.clone(); Py::Bool({ let t1137 = Py::Int(0i64); if py::lt(&t1136, &t1137)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("this capability needs a > 0 and x >= 0"))));
    }
    if ({ let t1143 = { let t1139 = v_a.clone(); Py::Bool({ let t1140 = Py::Float(10000.0); if py::gt(&t1139, &t1140)? { true } else { false } }) }; if t1143.truthy() { t1143 } else { { let t1141 = v_x.clone(); Py::Bool({ let t1142 = Py::Float(100000.0); if py::gt(&t1141, &t1142)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a and x are limited to 1e4 and 1e5"))));
    }
    let t1160 = (f__reg_gamma(v_a.clone(), v_x.clone())?).items()?;
    if t1160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1160[0].clone();
    v_Q = t1160[1].clone();
    return Ok(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("Q"), v_Q.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rel_err(mut v_got: Py, mut v_want: Py, mut v_slack: Py) -> R {
    let mut v_d: Py = Py::None;
    let t1528: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_d = py::truediv(&py::abs(&py::sub(&py::complex(&[v_got.clone()])?, &py::complex(&[v_want.clone()])?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::complex(&[v_want.clone()])?)?], true)?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1528 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1529 = v_d.clone(); Py::Bool({ let t1530 = v_d.clone(); if py::ne(&t1529, &t1530) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::minmax(&[Py::Float(0.0), py::sub(&v_d.clone(), &v_slack.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_riemann_zeta(mut v_s: Py) -> R {
    if (py::not(&{ let t1169 = v_s.clone(); Py::Bool({ let t1170 = Py::Int(1i64); if py::gt(&t1169, &t1170)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("this capability covers s > 1 (where the defining series converges)"))));
    }
    if ({ let t1171 = v_s.clone(); Py::Bool({ let t1172 = Py::Float(10000.0); if py::gt(&t1171, &t1172)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("zeta"), Py::Float(1.0))]));
    }
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        return Ok(Py::dict(vec![(Py::str("zeta"), f__fin(ax_engine::pymp::call("zeta", &[v_s.clone()], &[])?, Py::str("the result"))?)]));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_right_riemann_sum(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_h: Py = Py::None;
    let t1192 = (f__grid(v_f.clone(), v_a.clone(), v_b.clone(), v_n.clone())?).items()?;
    if t1192.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_e = t1192[0].clone();
    v_h = t1192[1].clone();
    v_n = t1192[2].clone();
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        return Ok(Py::dict(vec![(Py::str("approximation"), py::float(&py::mul(&v_h.clone(), &ax_engine::pymp::call("fsum", &[f__values(v_e.clone(), { let mut out = vec![]; for t1194 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1194;
out.push(py::add(&v_a.clone(), &py::mul(&v_k.clone(), &v_h.clone())?)?); } Py::list(out) })?], &[])?)?)?)]));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_second_derivative_at_point(mut v_f: Py, mut v_a: Py) -> R {
    let mut v_d2: Py = Py::None;
    let mut v_e: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    let _ = ax_engine::pysym::helper("calculus:at", &[v_e.clone(), Py::tuple(vec![c_X()?]), v_a.clone()], &[])?;
    v_d2 = ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?, Py::Int(2i64)], &[])?;
    return Ok(Py::dict(vec![(Py::str("second_derivative"), ax_engine::pysym::helper("calculus:at", &[v_d2.clone(), Py::tuple(vec![c_X()?]), v_a.clone()], &[])?), (Py::str("second_derivative_expression"), ax_engine::pysym::helper("calculus:show", &[v_d2.clone()], &[])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_series_sum(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_e: Py = Py::None;
    v_k = py::int(&v_k.clone())?;
    if (py::not(&{ let t1214 = Py::Int(0i64); Py::Bool({ let t1215 = v_k.clone(); if py::le(&t1214, &t1215)? { { let t1216 = Py::Int(1000i64); if py::le(&t1215, &t1216)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the starting index must be between 0 and 1000"))));
    }
    v_e = f__series_terms(v_a.clone())?;
    let _ = ax_engine::pysym::helper("calculus:at", &[py::method(&v_e.clone(), "subs", &[c__KS()?, v_k.clone()])?, Py::tuple(vec![])], &[])?;
    let t1217: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_S = ax_engine::pysym::call("summation", &[v_e.clone(), Py::tuple(vec![c__KS()?, v_k.clone(), ax_engine::pysym::constant("oo")?])], &[])?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1217 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(&e, PyErr::Other(c) if c.as_str() == "NotImplementedError")) || (matches!(e, PyErr::Value(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(&e, PyErr::Other(c) if c.as_str() == "RecursionError")) || (matches!(e, PyErr::ZeroDivision)) => {
            return Err(py::value_error(py::to_str(&Py::str("the sum could not be found in closed form"))));
        }
        Err(e) => return Err(e),
    }
    if (py::method(&v_S.clone(), "has", &[ax_engine::pysym::constant("oo")?, py::neg(&ax_engine::pysym::constant("oo")?)?, ax_engine::pysym::constant("zoo")?])?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the series diverges"))));
    }
    if ({ let t1218 = py::method(&v_S.clone(), "has", &[ax_engine::pysym::constant("Sum")?])?; if t1218.truthy() { t1218 } else { py::method(&v_S.clone(), "has", &[ax_engine::pysym::constant("Piecewise")?])? } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no closed form was found for this series (or it does not converge)"))));
    }
    return Ok(Py::dict(vec![(Py::str("sum"), f__limval(v_S.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_shell_volume(mut v_f: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v_p: Py = Py::None;
    if (py::not(&{ let t1234 = Py::Int(0i64); Py::Bool({ let t1235 = v_a.clone(); if py::le(&t1234, &t1235)? { { let t1236 = v_b.clone(); if py::lt(&t1235, &t1236)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the shell method about the y-axis needs 0 <= a < b"))));
    }
    v_p = ax_engine::pysym::helper("calculus:poly_of", &[v_f.clone()], &[])?;
    if (py::not(&f__nonneg_on(v_p.clone(), v_a.clone(), v_b.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the shell method needs f(x) >= 0 on [a, b]"))));
    }
    v_W = py::method(&py::method(&py::mul(&ax_engine::pysym::call("Poly", &[c_X()?, c_X()?], &[])?, &v_p.clone())?, "integrate", &[])?, "as_expr", &[])?;
    let t1243 = (Py::tuple(vec![ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_a.clone())?))], &[])?, ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_b.clone())?))], &[])?])).items()?;
    if t1243.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1243[0].clone();
    v_B = t1243[1].clone();
    return Ok(Py::dict(vec![(Py::str("volume"), py::float(&ax_engine::pysym::call("N", &[py::mul(&py::mul(&Py::Int(2i64), &ax_engine::pysym::constant("pi")?)?, &py::sub(&py::method(&v_W.clone(), "subs", &[c_X()?, v_B.clone()])?, &py::method(&v_W.clone(), "subs", &[c_X()?, v_A.clone()])?)?)?, Py::Int(30i64)], &[])?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_surface_area_graph(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py) -> R {
    let mut v__S: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_v: Py = Py::None;
    if (py::not(&{ let t1296 = { let t1292 = v_a.clone(); Py::Bool({ let t1293 = v_b.clone(); if py::lt(&t1292, &t1293)? { true } else { false } }) }; if !t1296.truthy() { t1296 } else { { let t1294 = v_c.clone(); Py::Bool({ let t1295 = v_d.clone(); if py::lt(&t1294, &t1295)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the rectangle needs a < b and c < d"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), c__XY()?], &[])?;
    v_g = f__fl(ax_engine::pysym::call("sqrt", &[py::add(&py::add(&Py::Int(1i64), &py::pow(&ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?, &Py::Int(2i64))?)?, &py::pow(&ax_engine::pysym::call("diff", &[v_e.clone(), c_Y()?], &[])?, &Py::Int(2i64))?)?], &[])?, c__XY()?)?;
    let t1324 = (f__settled({ let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_e = v_e.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__gl2(v_g.clone(), v_a.clone(), v_b.clone(), { let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_e = v_e.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_c.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_e = v_e.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_d.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_e = v_e.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::Float(1.0));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::tuple(vec![Py::Int(24i64), Py::Int(40i64), Py::Int(64i64)]))?).items()?;
    if t1324.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t1324[0].clone();
    v__S = t1324[1].clone();
    return Ok(Py::dict(vec![(Py::str("surface_area"), v_v.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_surface_area_revolution(mut v_f: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_cuts: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_ys: Py = Py::None;
    if (py::not(&{ let t1330 = v_a.clone(); Py::Bool({ let t1331 = v_b.clone(); if py::lt(&t1330, &t1331)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    v_d = ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?;
    let _ = ax_engine::pysym::helper("calculus:continuous_on", &[v_d.clone(), c_X()?, v_a.clone(), v_b.clone()], &[])?;
    v_g = ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[(String::from("use_mp"), Py::Bool(false))])?;
    let t1332 = (Py::tuple(vec![Py::list(vec![]), Py::Int(400i64)])).items()?;
    if t1332.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cuts = t1332[0].clone();
    v_n = t1332[1].clone();
    v_xs = { let mut out = vec![]; for t1333 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1333;
out.push(py::add(&v_a.clone(), &py::truediv(&py::mul(&py::sub(&v_b.clone(), &v_a.clone())?, &v_k.clone())?, &v_n.clone())?)?); } Py::list(out) };
    v_ys = { let mut out = vec![]; for t1334 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t1334;
out.push(py::call(&v_g.clone(), &[v_x.clone()])?); } Py::list(out) };
    for t1335 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t1335;
        if ({ let t1340 = { let t1336 = py::getitem(&v_ys.clone(), &v_k.clone())?; Py::Bool({ let t1337 = Py::Int(0i64); if py::eq(&t1336, &t1337) { true } else { false } }) }; if !t1340.truthy() { t1340 } else { { let t1338 = Py::Int(0i64); Py::Bool({ let t1339 = v_k.clone(); if py::lt(&t1338, &t1339)? { true } else { false } }) } } }).truthy() {
            let _ = py::method(&v_cuts.clone(), "append", &[py::getitem(&v_xs.clone(), &v_k.clone())?])?;
        } else {
            if ({ let t1341 = py::mul(&py::getitem(&v_ys.clone(), &v_k.clone())?, &py::getitem(&v_ys.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t1342 = Py::Int(0i64); if py::lt(&t1341, &t1342)? { true } else { false } }) }).truthy() {
                {
                    let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
                    let _ = py::method(&v_cuts.clone(), "append", &[py::float(&ax_engine::pymp::call("findroot", &[ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[])?, Py::tuple(vec![py::getitem(&v_xs.clone(), &v_k.clone())?, py::getitem(&v_xs.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?])], &[(String::from("solver"), Py::str("anderson"))])?)?])?;
                }
            }
        }
    }
    v_F = ax_engine::pysym::helper("calculus:_compile", &[py::mul(&ax_engine::pysym::call("Abs", &[v_e.clone()], &[])?, &ax_engine::pysym::call("sqrt", &[py::add(&Py::Int(1i64), &py::pow(&v_d.clone(), &Py::Int(2i64))?)?], &[])?)?, Py::tuple(vec![c_X()?])], &[])?;
    return Ok(Py::dict(vec![(Py::str("surface_area"), py::mul(&py::mul(&Py::Int(2i64), &py::float(&ax_engine::pymp::constant("pi")?)?)?, &ax_engine::pysym::helper("calculus:quad", &[v_F.clone(), v_a.clone(), v_b.clone()], &[(String::from("points"), v_cuts.clone())])?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tangent_line(mut v_f: Py, mut v_a: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_y0: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    v_y0 = ax_engine::pysym::helper("calculus:at", &[v_e.clone(), Py::tuple(vec![c_X()?]), v_a.clone()], &[])?;
    v_m = ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?, Py::tuple(vec![c_X()?]), v_a.clone()], &[])?;
    v_b = py::sub(&v_y0.clone(), &py::mul(&v_m.clone(), &v_a.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("slope"), v_m.clone()), (Py::str("intercept"), v_b.clone()), (Py::str("y0"), v_y0.clone()), (Py::str("equation"), py::method(&Py::str(&["y = ".to_string(), py::format(&v_m.clone(), ".12g")?, "*x + ".to_string(), py::format(&v_b.clone(), ".12g")?].concat()), "replace", &[Py::str("+ -"), Py::str("- ")])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tangent_plane(mut v_f: Py, mut v_p: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_z0: Py = Py::None;
    v_g = py::getitem(&f_gradient(v_f.clone(), v_p.clone())?, &Py::str("gradient"))?;
    if ({ let t1387 = py::len(&v_p.clone())?; Py::Bool({ let t1388 = Py::Int(2i64); if py::ne(&t1387, &t1388) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a tangent plane needs a surface z = f(x, y) and a point (x, y)"))));
    }
    v_z0 = { let mut v: Vec<Py> = vec![]; v.push(ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), c__XY()?], &[])?); v.push(c__XY()?); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? };
    return Ok(Py::dict(vec![(Py::str("z0"), v_z0.clone()), (Py::str("fx"), py::getitem(&v_g.clone(), &Py::Int(0i64))?), (Py::str("fy"), py::getitem(&v_g.clone(), &Py::Int(1i64))?), (Py::str("c"), py::sub(&py::sub(&v_z0.clone(), &py::mul(&py::getitem(&v_g.clone(), &Py::Int(0i64))?, &py::getitem(&v_p.clone(), &Py::Int(0i64))?)?)?, &py::mul(&py::getitem(&v_g.clone(), &Py::Int(1i64))?, &py::getitem(&v_p.clone(), &Py::Int(1i64))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_taylor_polynomial(mut v_f: Py, mut v_a: Py, mut v_n: Py, mut v_x1: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_co: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_fx1: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_val: Py = Py::None;
    v_n = py::int(&v_n.clone())?;
    if (py::not(&{ let t1398 = Py::Int(0i64); Py::Bool({ let t1399 = v_n.clone(); if py::le(&t1398, &t1399)? { { let t1400 = Py::Int(12i64); if py::le(&t1399, &t1400)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the degree must be between 0 and 12"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    let t1401 = (Py::tuple(vec![Py::list(vec![]), v_e.clone()])).items()?;
    if t1401.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_co = t1401[0].clone();
    v_d = t1401[1].clone();
    for t1402 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t1402;
        let _ = py::method(&v_co.clone(), "append", &[py::truediv(&ax_engine::pysym::helper("calculus:at", &[v_d.clone(), Py::tuple(vec![c_X()?]), v_a.clone()], &[])?, &py::factorial(&v_k.clone())?)?])?;
        v_d = ax_engine::pysym::call("diff", &[v_d.clone(), c_X()?], &[])?;
    }
    let t1403 = (Py::tuple(vec![py::sub(&v_x1.clone(), &v_a.clone())?, Py::Float(0.0)])).items()?;
    if t1403.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_h = t1403[0].clone();
    v_val = t1403[1].clone();
    for t1404 in py::iter(&(py::reversed(&v_co.clone())?))? {
        v_c = t1404;
        v_val = py::add(&py::mul(&v_val.clone(), &v_h.clone())?, &v_c.clone())?;
    }
    v_fx1 = ax_engine::pysym::helper("calculus:at", &[v_e.clone(), Py::tuple(vec![c_X()?]), v_x1.clone()], &[])?;
    return Ok(Py::dict(vec![(Py::str("coefficients"), v_co.clone()), (Py::str("value"), v_val.clone()), (Py::str("function_value"), v_fx1.clone()), (Py::str("error"), py::sub(&v_fx1.clone(), &v_val.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_triple_integral_box(mut v_f: Py, mut v_x0: Py, mut v_x1: Py, mut v_y0: Py, mut v_y1: Py, mut v_z0: Py, mut v_z1: Py) -> R {
    let mut v__S: Py = Py::None;
    let mut v_bx: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_v: Py = Py::None;
    if (py::not(&{ let t1469 = { let t1462 = v_x0.clone(); Py::Bool({ let t1463 = v_x1.clone(); if py::lt(&t1462, &t1463)? { true } else { false } }) }; if !t1469.truthy() { t1469 } else { { let t1468 = { let t1464 = v_y0.clone(); Py::Bool({ let t1465 = v_y1.clone(); if py::lt(&t1464, &t1465)? { true } else { false } }) }; if !t1468.truthy() { t1468 } else { { let t1466 = v_z0.clone(); Py::Bool({ let t1467 = v_z1.clone(); if py::lt(&t1466, &t1467)? { true } else { false } }) } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the box needs x0 < x1, y0 < y1, z0 < z1"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), c__XYZ()?], &[])?;
    if (py::method(&v_e.clone(), "is_polynomial", &[c_X()?, c_Y()?, c_Z()?])?).truthy() {
        v_v = ax_engine::pysym::call("integrate", &[v_e.clone(), Py::tuple(vec![c_X()?, f__rat(v_x0.clone())?, f__rat(v_x1.clone())?]), Py::tuple(vec![c_Y()?, f__rat(v_y0.clone())?, f__rat(v_y1.clone())?]), Py::tuple(vec![c_Z()?, f__rat(v_z0.clone())?, f__rat(v_z1.clone())?])], &[])?;
        return Ok(Py::dict(vec![(Py::str("integral"), f__pfloat(v_v.clone())?)]));
    }
    v_g = f__fl(v_e.clone(), c__XYZ()?)?;
    v_bx = Py::tuple(vec![Py::tuple(vec![v_x0.clone(), v_x1.clone()]), Py::tuple(vec![v_y0.clone(), v_y1.clone()]), Py::tuple(vec![v_z0.clone(), v_z1.clone()])]);
    let t1501 = (f__settled({ let v_bx = v_bx.clone(); let v_e = v_e.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_v = v_v.clone(); let v_x0 = v_x0.clone(); let v_x1 = v_x1.clone(); let v_y0 = v_y0.clone(); let v_y1 = v_y1.clone(); let v_z0 = v_z0.clone(); let v_z1 = v_z1.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__gl3(v_g.clone(), v_bx.clone(), v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_bx = v_bx.clone(); let v_e = v_e.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_v = v_v.clone(); let v_x0 = v_x0.clone(); let v_x1 = v_x1.clone(); let v_y0 = v_y0.clone(); let v_y1 = v_y1.clone(); let v_z0 = v_z0.clone(); let v_z1 = v_z1.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__gl3({ let v_bx = v_bx.clone(); let v_e = v_e.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_n = v_n.clone(); let v_v = v_v.clone(); let v_x0 = v_x0.clone(); let v_x1 = v_x1.clone(); let v_y0 = v_y0.clone(); let v_y1 = v_y1.clone(); let v_z0 = v_z0.clone(); let v_z1 = v_z1.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_y: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_z: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::call(&v_g.clone(), &[v_x.clone(), v_y.clone(), v_z.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_bx.clone(), v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::tuple(vec![Py::Int(12i64), Py::Int(20i64), Py::Int(28i64)]))?).items()?;
    if t1501.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t1501[0].clone();
    v__S = t1501[1].clone();
    return Ok(Py::dict(vec![(Py::str("integral"), v_v.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_velocity_acceleration(mut v_s: Py, mut v_t0: Py) -> R {
    let mut v_acc: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vel: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_s.clone(), Py::tuple(vec![Py::str("t")])], &[])?;
    let _ = ax_engine::pysym::helper("calculus:at", &[v_e.clone(), Py::tuple(vec![c_T()?]), v_t0.clone()], &[])?;
    let t1506 = (Py::tuple(vec![ax_engine::pysym::call("diff", &[v_e.clone(), c_T()?], &[])?, ax_engine::pysym::call("diff", &[v_e.clone(), c_T()?, Py::Int(2i64)], &[])?])).items()?;
    if t1506.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t1506[0].clone();
    v_acc = t1506[1].clone();
    v_vel = ax_engine::pysym::helper("calculus:at", &[v_v.clone(), Py::tuple(vec![c_T()?]), v_t0.clone()], &[])?;
    return Ok(Py::dict(vec![(Py::str("velocity"), v_vel.clone()), (Py::str("acceleration"), ax_engine::pysym::helper("calculus:at", &[v_acc.clone(), Py::tuple(vec![c_T()?]), v_t0.clone()], &[])?), (Py::str("speed"), py::abs(&v_vel.clone())?), (Py::str("position"), ax_engine::pysym::helper("calculus:at", &[v_e.clone(), Py::tuple(vec![c_T()?]), v_t0.clone()], &[])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_washer_volume(mut v_f: Py, mut v_g: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v_pf: Py = Py::None;
    let mut v_pg: Py = Py::None;
    if (py::not(&{ let t1515 = v_a.clone(); Py::Bool({ let t1516 = v_b.clone(); if py::lt(&t1515, &t1516)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    let t1517 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:poly_of", &[v_f.clone()], &[])?, ax_engine::pysym::helper("calculus:poly_of", &[v_g.clone()], &[])?])).items()?;
    if t1517.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pf = t1517[0].clone();
    v_pg = t1517[1].clone();
    if ({ let t1524 = py::not(&f__nonneg_on(v_pg.clone(), v_a.clone(), v_b.clone())?); if t1524.truthy() { t1524 } else { py::not(&f__nonneg_on(py::sub(&v_pf.clone(), &v_pg.clone())?, v_a.clone(), v_b.clone())?) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the washer method needs outer radius f >= inner radius g >= 0 on [a, b]"))));
    }
    v_W = py::method(&py::method(&py::sub(&py::pow(&v_pf.clone(), &Py::Int(2i64))?, &py::pow(&v_pg.clone(), &Py::Int(2i64))?)?, "integrate", &[])?, "as_expr", &[])?;
    let t1525 = (Py::tuple(vec![ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_a.clone())?))], &[])?, ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_b.clone())?))], &[])?])).items()?;
    if t1525.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1525[0].clone();
    v_B = t1525[1].clone();
    return Ok(Py::dict(vec![(Py::str("volume"), py::float(&ax_engine::pysym::call("N", &[py::mul(&ax_engine::pysym::constant("pi")?, &py::sub(&py::method(&v_W.clone(), "subs", &[c_X()?, v_B.clone()])?, &py::method(&v_W.clone(), "subs", &[c_X()?, v_A.clone()])?)?)?, Py::Int(30i64)], &[])?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_worst(mut v_residuals: Py) -> R {
    return Ok((if (v_residuals.clone()).truthy() { py::minmax(&[v_residuals.clone()], true)? } else { Py::Float(0.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1005(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1012: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1006(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1012 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1013 = v_r.clone(); Py::Bool({ let t1014 = v_r.clone(); if py::eq(&t1013, &t1014) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1006(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__verify_newton(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("x0"))?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1017(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1022: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1018(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1022 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1023 = v_r.clone(); Py::Bool({ let t1024 = v_r.clone(); if py::eq(&t1023, &t1024) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1018(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::mul(&py::getitem(&v_o.clone(), &Py::str("slope"))?, &py::float(&ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?], &[])?)?)?, py::neg(&Py::Float(1.0))?, Py::Float(1e-11))?, f_rel_err(py::add(&py::getitem(&v_o.clone(), &Py::str("intercept"))?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("slope"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?, Py::tuple(vec![c_X()?]), py::getitem(&v_i.clone(), &Py::str("a"))?], &[])?, Py::Float(1e-12))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1028(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1033: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1029(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1033 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1034 = v_r.clone(); Py::Bool({ let t1035 = v_r.clone(); if py::eq(&t1034, &t1035) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1029(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("nth_derivative"))?, ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?], &[])?, Py::Float(1e-09))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1043(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1048: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1044(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1048 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1049 = v_r.clone(); Py::Bool({ let t1050 = v_r.clone(); if py::eq(&t1049, &t1050) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1044(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("exact"))?, ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?], &[])?, Py::Float(1e-12))?, py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("error"))?, &py::sub(&py::getitem(&v_o.clone(), &Py::str("approximation"))?, &py::getitem(&v_o.clone(), &Py::str("exact"))?)?)?)?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("approximation"))?, py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_g: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_a: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_h: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&Py::dict(vec![(Py::str("forward"), py::truediv(&py::sub(&py::call(&v_g.clone(), &[py::add(&v_a.clone(), &v_h.clone())?])?, &py::call(&v_g.clone(), &[v_a.clone()])?)?, &v_h.clone())?), (Py::str("backward"), py::truediv(&py::sub(&py::call(&v_g.clone(), &[v_a.clone()])?, &py::call(&v_g.clone(), &[py::sub(&v_a.clone(), &v_h.clone())?])?)?, &v_h.clone())?), (Py::str("central"), py::truediv(&py::sub(&py::call(&v_g.clone(), &[py::add(&v_a.clone(), &v_h.clone())?])?, &py::call(&v_g.clone(), &[py::sub(&v_a.clone(), &v_h.clone())?])?)?, &py::mul(&Py::Int(2i64), &v_h.clone())?)?)]), &py::getitem(&v_i.clone(), &Py::str("method"))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?, Py::tuple(vec![c_X()?])], &[(String::from("use_mp"), Py::Bool(false))])?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("h"))?])?, Py::Float(1e-09))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1056(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1064: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1057(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1064 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1065 = v_r.clone(); Py::Bool({ let t1066 = v_r.clone(); if py::eq(&t1065, &t1066) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1057(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_v: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        v_v = (if ({ let t1059 = py::getitem(&v_i.clone(), &Py::str("t0"))?; Py::Bool({ let t1060 = py::getitem(&v_i.clone(), &Py::str("t1"))?; if py::ne(&t1059, &t1060) { true } else { false } }) }).truthy() { ax_engine::pysym::helper("calculus:_gl", &[ax_engine::pysym::helper("calculus:_compile", &[s_1058(v_i.clone())?, Py::tuple(vec![c_T()?])], &[])?, py::getitem(&v_i.clone(), &Py::str("t0"))?, py::getitem(&v_i.clone(), &Py::str("t1"))?], &[])? } else { Py::Int(0i64) });
    }
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("arc_length"))?, py::float(&v_v.clone())?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1058(mut v_i: Py) -> R {
    return Ok(ax_engine::pysym::call("sqrt", &[py::add(&py::pow(&ax_engine::pysym::call("diff", &[py::getitem(&f__path(py::getitem(&v_i.clone(), &Py::str("xt"))?, py::getitem(&v_i.clone(), &Py::str("yt"))?)?, &Py::Int(0i64))?, c_T()?], &[])?, &Py::Int(2i64))?, &py::pow(&ax_engine::pysym::call("diff", &[py::getitem(&f__path(py::getitem(&v_i.clone(), &Py::str("xt"))?, py::getitem(&v_i.clone(), &Py::str("yt"))?)?, &Py::Int(1i64))?, c_T()?], &[])?, &Py::Int(2i64))?)?], &[])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_107(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t112: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_108(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t112 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t113 = v_r.clone(); Py::Bool({ let t114 = v_r.clone(); if py::eq(&t113, &t114) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1071(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1077: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_pslope(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1077 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1078 = v_r.clone(); Py::Bool({ let t1079 = v_r.clone(); if py::eq(&t1078, &t1079) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_108(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("beta"))?, py::float(&ax_engine::pymp::call("beta", &[py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?], &[])?)?, Py::Float(1e-12))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1086(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1095: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1087(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1095 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1096 = v_r.clone(); Py::Bool({ let t1097 = v_r.clone(); if py::eq(&t1096, &t1097) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1087(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("partial"))?, py::float(&f__mp_partial(py::getitem(&v_i.clone(), &Py::str("f"))?, f__vars(py::getitem(&v_i.clone(), &Py::str("p"))?)?, py::getitem(&v_i.clone(), &Py::str("p"))?, { let mut out = vec![]; for t1092 in py::iter(&(f__vars(py::getitem(&v_i.clone(), &Py::str("p"))?)?))? { let mut v_v: Py; 
v_v = t1092;
out.push(py::int(&{ let t1093 = v_v.clone(); Py::Bool({ let t1094 = py::getitem(&v_i.clone(), &Py::str("var"))?; if py::eq(&t1093, &t1094) { true } else { false } }) })?); } Py::list(out) })?)?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1102(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1107: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1103(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1107 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1108 = v_r.clone(); Py::Bool({ let t1109 = v_r.clone(); if py::eq(&t1108, &t1109) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1103(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("area"))?, f__gl_check(Py::str(&["(".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("r"))?, "")?, ")^2/2".to_string()].concat()), py::getitem(&v_i.clone(), &Py::str("al"))?, py::getitem(&v_i.clone(), &Py::str("be"))?, Py::tuple(vec![Py::str("theta")]), Py::None)?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1130(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1131: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_ratio(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1131 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1132 = v_r.clone(); Py::Bool({ let t1133 = v_r.clone(); if py::eq(&t1132, &t1133) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1161(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1166: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1162(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1166 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1167 = v_r.clone(); Py::Bool({ let t1168 = v_r.clone(); if py::eq(&t1167, &t1168) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1162(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("P"))?, py::float(&ax_engine::pymp::call("gammainc", &[py::getitem(&v_i.clone(), &Py::str("a"))?, Py::Int(0i64), py::getitem(&v_i.clone(), &Py::str("x"))?], &[(String::from("regularized"), Py::Bool(true))])?)?, Py::Float(1e-11))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("Q"))?, py::float(&ax_engine::pymp::call("gammainc", &[py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("x"))?, ax_engine::pymp::constant("inf")?], &[(String::from("regularized"), Py::Bool(true))])?)?, Py::Float(1e-11))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1173(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1181: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1174(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1181 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1182 = v_r.clone(); Py::Bool({ let t1183 = v_r.clone(); if py::eq(&t1182, &t1183) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1174(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("zeta"))?, f__zeta_em(py::getitem(&v_i.clone(), &Py::str("s"))?, Py::Int(12i64))?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1195(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1203: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1196(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1203 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1204 = v_r.clone(); Py::Bool({ let t1205 = v_r.clone(); if py::eq(&t1204, &t1205) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1196(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("approximation"))?, f__float_sum(py::getitem(&v_i.clone(), &Py::str("f"))?, { let mut out = vec![]; for t1202 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::getitem(&v_i.clone(), &Py::str("n"))?, &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1202;
out.push(py::add(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::truediv(&py::mul(&v_k.clone(), &py::sub(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?)?); } Py::list(out) }, py::mul(&Py::list(vec![py::truediv(&py::sub(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?]), &py::getitem(&v_i.clone(), &Py::str("n"))?)?)?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1206(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1211: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1207(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1211 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1212 = v_r.clone(); Py::Bool({ let t1213 = v_r.clone(); if py::eq(&t1212, &t1213) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1207(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("second_derivative"))?, ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, Py::Int(2i64)], &[])?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1226(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1231: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1227(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1231 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1232 = v_r.clone(); Py::Bool({ let t1233 = v_r.clone(); if py::eq(&t1232, &t1233) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1227(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("sum"))?, py::float(&ax_engine::pysym::helper("calculus:_nsum", &[py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("k"))?], &[])?)?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1244(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1249: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1245(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1249 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1250 = v_r.clone(); Py::Bool({ let t1251 = v_r.clone(); if py::eq(&t1250, &t1251) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1245(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("volume"))?, py::mul(&py::mul(&Py::Int(2i64), &ax_engine::pymp::constant("pi")?)?, &f__gl_check(Py::str(&["x*(".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("f"))?, "")?, ")".to_string()].concat()), py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?)?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1252(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v_approx: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_exact: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t1261 = (f__grid(v_f.clone(), v_a.clone(), v_b.clone(), v_n.clone())?).items()?;
    if t1261.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_e = t1261[0].clone();
    v_h = t1261[1].clone();
    v_n = t1261[2].clone();
    if ({ let t1264 = { let t1262 = Py::str("simpson"); Py::Bool({ let t1263 = Py::str("simpson"); if py::eq(&t1262, &t1263) { true } else { false } }) }; if !t1264.truthy() { t1264 } else { py::modulo(&v_n.clone(), &Py::Int(2i64))? } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Simpson's rule needs an even number of subintervals"))));
    }
    let t1277 = (f__rule_n(v_e.clone(), v_a.clone(), v_b.clone(), v_n.clone(), Py::str("simpson"))?).items()?;
    if t1277.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t1277[0].clone();
    v_w = t1277[1].clone();
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        v_approx = py::float(&ax_engine::pymp::call("fsum", &[{ let mut out = vec![]; for t1279 in py::iter(&(py::zip(&[v_w.clone(), f__values(v_e.clone(), v_pts.clone())?])?))? { let mut v_v: Py; let mut v_wi: Py; 
let t1280 = (t1279).items()?;
if t1280.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_wi = t1280[0].clone();
v_v = t1280[1].clone();
out.push(py::mul(&v_wi.clone(), &v_v.clone())?); } Py::list(out) }], &[])?)?;
    }
    v_exact = f__exact(v_e.clone(), v_a.clone(), v_b.clone())?;
    return Ok(Py::dict(vec![(Py::str("approximation"), v_approx.clone()), (Py::str("exact"), v_exact.clone()), (Py::str("error"), py::sub(&v_approx.clone(), &v_exact.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_127(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t132: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_128(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t132 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t133 = v_r.clone(); Py::Bool({ let t134 = v_r.clone(); if py::eq(&t133, &t134) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_128(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("area"))?, f__gl_check(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?, Py::Float(1e-10))?, f_rel_err(py::mul(&py::getitem(&v_o.clone(), &Py::str("x_bar"))?, &py::getitem(&v_o.clone(), &Py::str("area"))?)?, f__gl_check(Py::str(&["x*(".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("f"))?, "")?, ")".to_string()].concat()), py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?, Py::Float(1e-10))?, f_rel_err(py::mul(&py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("y_bar"))?)?, &py::getitem(&v_o.clone(), &Py::str("area"))?)?, f__gl_check(Py::str(&["(".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("f"))?, "")?, ")^2".to_string()].concat()), py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?, Py::Float(1e-10))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1281(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1289: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1282(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1289 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1290 = v_r.clone(); Py::Bool({ let t1291 = v_r.clone(); if py::eq(&t1290, &t1291) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1282(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?;
    let t1283 = (f__rule_n(v_e.clone(), py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, Py::str("simpson"))?).items()?;
    if t1283.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t1283[0].clone();
    v_w = t1283[1].clone();
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("approximation"))?, f__float_sum(py::getitem(&v_i.clone(), &Py::str("f"))?, v_pts.clone(), v_w.clone())?, Py::Float(1e-11))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("exact"))?, f__gl_check(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?, Py::Float(1e-10))?, py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("error"))?, &py::sub(&py::getitem(&v_o.clone(), &Py::str("approximation"))?, &py::getitem(&v_o.clone(), &Py::str("exact"))?)?)?)?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1325(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1327: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1326(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1327 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1328 = v_r.clone(); Py::Bool({ let t1329 = v_r.clone(); if py::eq(&t1328, &t1329) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1326(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_e: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__scaled_err(py::getitem(&v_o.clone(), &Py::str("surface_area"))?, f__gl2(f__fl(ax_engine::pysym::call("sqrt", &[py::add(&py::add(&Py::Int(1i64), &py::pow(&ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?, &Py::Int(2i64))?)?, &py::pow(&ax_engine::pysym::call("diff", &[v_e.clone(), c_Y()?], &[])?, &Py::Int(2i64))?)?], &[])?, c__XY()?)?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, { let v_e = v_e.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&v_i.clone(), &Py::str("c"))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_e = v_e.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&v_i.clone(), &Py::str("d"))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Int(32i64))?, Py::Float(1.0), Py::Float(1e-10))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?, c__XY()?], &[])?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1343(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1370: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1344(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1370 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1371 = v_r.clone(); Py::Bool({ let t1372 = v_r.clone(); if py::eq(&t1371, &t1372) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1344(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("surface_area"))?, f__verify_surface(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?)?, Py::Float(1e-08))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1373(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1378: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1374(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1378 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1379 = v_r.clone(); Py::Bool({ let t1380 = v_r.clone(); if py::eq(&t1379, &t1380) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1374(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("slope"))?, ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?], &[])?, Py::Float(1e-12))?, f_rel_err(py::add(&py::getitem(&v_o.clone(), &Py::str("intercept"))?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("slope"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?, Py::tuple(vec![c_X()?]), py::getitem(&v_i.clone(), &Py::str("a"))?], &[])?, Py::Float(1e-12))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1389(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1395: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_tplane(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1395 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1396 = v_r.clone(); Py::Bool({ let t1397 = v_r.clone(); if py::eq(&t1396, &t1397) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1405(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1419: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_taylor(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1419 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1420 = v_r.clone(); Py::Bool({ let t1421 = v_r.clone(); if py::eq(&t1420, &t1421) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1422(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v_approx: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_exact: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t1431 = (f__grid(v_f.clone(), v_a.clone(), v_b.clone(), v_n.clone())?).items()?;
    if t1431.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_e = t1431[0].clone();
    v_h = t1431[1].clone();
    v_n = t1431[2].clone();
    if ({ let t1434 = { let t1432 = Py::str("trapezoid"); Py::Bool({ let t1433 = Py::str("simpson"); if py::eq(&t1432, &t1433) { true } else { false } }) }; if !t1434.truthy() { t1434 } else { py::modulo(&v_n.clone(), &Py::Int(2i64))? } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Simpson's rule needs an even number of subintervals"))));
    }
    let t1447 = (f__rule_n(v_e.clone(), v_a.clone(), v_b.clone(), v_n.clone(), Py::str("trapezoid"))?).items()?;
    if t1447.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t1447[0].clone();
    v_w = t1447[1].clone();
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        v_approx = py::float(&ax_engine::pymp::call("fsum", &[{ let mut out = vec![]; for t1449 in py::iter(&(py::zip(&[v_w.clone(), f__values(v_e.clone(), v_pts.clone())?])?))? { let mut v_v: Py; let mut v_wi: Py; 
let t1450 = (t1449).items()?;
if t1450.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_wi = t1450[0].clone();
v_v = t1450[1].clone();
out.push(py::mul(&v_wi.clone(), &v_v.clone())?); } Py::list(out) }], &[])?)?;
    }
    v_exact = f__exact(v_e.clone(), v_a.clone(), v_b.clone())?;
    return Ok(Py::dict(vec![(Py::str("approximation"), v_approx.clone()), (Py::str("exact"), v_exact.clone()), (Py::str("error"), py::sub(&v_approx.clone(), &v_exact.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_143(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t148: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_144(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t148 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t149 = v_r.clone(); Py::Bool({ let t150 = v_r.clone(); if py::eq(&t149, &t150) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_144(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("T"))?, py::float(&ax_engine::pymp::call("chebyt", &[py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::getitem(&v_i.clone(), &Py::str("x"))?], &[])?)?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1451(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1459: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1452(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1459 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1460 = v_r.clone(); Py::Bool({ let t1461 = v_r.clone(); if py::eq(&t1460, &t1461) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1452(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?;
    let t1453 = (f__rule_n(v_e.clone(), py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, Py::str("trapezoid"))?).items()?;
    if t1453.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t1453[0].clone();
    v_w = t1453[1].clone();
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("approximation"))?, f__float_sum(py::getitem(&v_i.clone(), &Py::str("f"))?, v_pts.clone(), v_w.clone())?, Py::Float(1e-11))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("exact"))?, f__gl_check(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?, Py::Float(1e-10))?, py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("error"))?, &py::sub(&py::getitem(&v_o.clone(), &Py::str("approximation"))?, &py::getitem(&v_o.clone(), &Py::str("exact"))?)?)?)?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1502(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1503: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_tpl(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1503 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1504 = v_r.clone(); Py::Bool({ let t1505 = v_r.clone(); if py::eq(&t1504, &t1505) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1507(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1512: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1508(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1512 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1513 = v_r.clone(); Py::Bool({ let t1514 = v_r.clone(); if py::eq(&t1513, &t1514) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1508(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("velocity"))?, ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("s"))?, py::getitem(&v_i.clone(), &Py::str("t0"))?, Py::Int(1i64), Py::tuple(vec![Py::str("t")])], &[])?, Py::Float(1e-12))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("acceleration"))?, ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("s"))?, py::getitem(&v_i.clone(), &Py::str("t0"))?, Py::Int(2i64), Py::tuple(vec![Py::str("t")])], &[])?, Py::Float(1e-11))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1526(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1531: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1527(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1531 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1532 = v_r.clone(); Py::Bool({ let t1533 = v_r.clone(); if py::eq(&t1532, &t1533) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1527(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("volume"))?, py::mul(&ax_engine::pymp::constant("pi")?, &f__gl_check(Py::str(&["(".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("f"))?, "")?, ")^2 - (".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("g"))?, "")?, ")^2".to_string()].concat()), py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?)?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_16(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t35: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_17(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t35 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t36 = v_r.clone(); Py::Bool({ let t37 = v_r.clone(); if py::eq(&t36, &t37) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_168(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t205: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_classify(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t205 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t206 = v_r.clone(); Py::Bool({ let t207 = v_r.clone(); if py::eq(&t206, &t207) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_17(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("area"))?, f__verify_abs_area(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("g"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?)?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_218(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t232: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_219(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t232 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t233 = v_r.clone(); Py::Bool({ let t234 = v_r.clone(); if py::eq(&t233, &t234) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_219(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__vec_err(py::getitem(&v_o.clone(), &Py::str("curl"))?, f__curl_from({ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_c: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_j: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::float(&f__mp_partial(py::getitem(&Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("P"))?, py::getitem(&v_i.clone(), &Py::str("Q"))?, py::getitem(&v_i.clone(), &Py::str("R"))?]), &v_c.clone())?, c__XYZ()?, py::getitem(&v_i.clone(), &Py::str("p"))?, { let mut out = vec![]; for t229 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_k: Py; 
v_k = t229;
out.push(py::int(&{ let t230 = v_k.clone(); Py::Bool({ let t231 = v_j.clone(); if py::eq(&t230, &t231) { true } else { false } }) })?); } Py::list(out) })?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) })?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_238(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t243: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_239(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t243 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t244 = v_r.clone(); Py::Bool({ let t245 = v_r.clone(); if py::eq(&t244, &t245) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_239(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("curvature"))?, py::truediv(&py::abs(&ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, Py::Int(2i64)], &[])?)?, &py::pow(&py::add(&Py::Int(1i64), &py::pow(&ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?], &[])?, &Py::Int(2i64))?)?, &Py::Float(1.5))?)?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_248(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t253: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_249(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t253 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t254 = v_r.clone(); Py::Bool({ let t255 = v_r.clone(); if py::eq(&t254, &t255) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_249(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("integral"))?, f__gl_check(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_256(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t261: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_257(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t261 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t262 = v_r.clone(); Py::Bool({ let t263 = v_r.clone(); if py::eq(&t262, &t263) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_257(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("derivative"))?, ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?], &[])?, Py::Float(1e-12))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_277(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t290: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_dirder(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t290 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t291 = v_r.clone(); Py::Bool({ let t292 = v_r.clone(); if py::eq(&t291, &t292) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_295(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t300: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_296(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t300 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t301 = v_r.clone(); Py::Bool({ let t302 = v_r.clone(); if py::eq(&t301, &t302) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_296(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("volume"))?, py::mul(&ax_engine::pymp::constant("pi")?, &f__gl_check(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), { let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_F: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_x: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::pow(&py::call(&v_F.clone(), &[v_x.clone()])?, &Py::Int(2i64))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) })?)?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t8: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_4(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t8 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t9 = v_r.clone(); Py::Bool({ let t10 = v_r.clone(); if py::eq(&t9, &t10) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_314(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t325: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_315(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t325 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t326 = v_r.clone(); Py::Bool({ let t327 = v_r.clone(); if py::eq(&t326, &t327) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_315(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("divergence"))?, py::fsum(&{ let mut out = vec![]; for t319 in py::iter(&(py::enumerate(&Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("P"))?, py::getitem(&v_i.clone(), &Py::str("Q"))?, py::getitem(&v_i.clone(), &Py::str("R"))?]), 0)?))? { let mut v_c: Py; let mut v_j: Py; 
let t320 = (t319).items()?;
if t320.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t320[0].clone();
v_c = t320[1].clone();
out.push(py::float(&f__mp_partial(v_c.clone(), c__XYZ()?, py::getitem(&v_i.clone(), &Py::str("p"))?, { let mut out = vec![]; for t322 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_k: Py; 
v_k = t322;
out.push(py::int(&{ let t323 = v_k.clone(); Py::Bool({ let t324 = v_j.clone(); if py::eq(&t323, &t324) { true } else { false } }) })?); } Py::list(out) })?)?); } Py::list(out) })?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_374(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t377: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_divthm(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t377 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t378 = v_r.clone(); Py::Bool({ let t379 = v_r.clone(); if py::eq(&t378, &t379) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("arc_length"))?, ax_engine::pysym::helper("calculus:_gl", &[ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::call("sqrt", &[py::add(&Py::Int(1i64), &py::pow(&ax_engine::pysym::call("diff", &[ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?, c_X()?], &[])?, &Py::Int(2i64))?)?], &[])?, Py::tuple(vec![c_X()?])], &[])?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?], &[])?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_413(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t415: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_dbl(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t415 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t416 = v_r.clone(); Py::Bool({ let t417 = v_r.clone(); if py::eq(&t416, &t417) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_42(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t50: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_43(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t50 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t51 = v_r.clone(); Py::Bool({ let t52 = v_r.clone(); if py::eq(&t51, &t52) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_43(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("area"))?, py::sum(&{ let mut out = vec![]; for t47 in py::iter(&(py::zip(&[py::slice(&py::getitem(&v_o.clone(), &Py::str("intersections"))?, None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, py::slice(&py::getitem(&v_o.clone(), &Py::str("intersections"))?, Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_hi: Py; let mut v_lo: Py; 
let t48 = (t47).items()?;
if t48.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_lo = t48[0].clone();
v_hi = t48[1].clone();
out.push(py::abs(&f__gl_check(Py::str(&["(".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("f"))?, "")?, ") - (".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("g"))?, "")?, ")".to_string()].concat()), v_lo.clone(), v_hi.clone(), Py::tuple(vec![Py::str("x")]), Py::None)?)?); } Py::list(out) }, None)?, Py::Float(1e-10))?, py::truediv(&py::minmax(&[{ let mut out = vec![]; for t49 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("intersections"))?))? { let mut v_r: Py; 
v_r = t49;
out.push(py::abs(&ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::helper("calculus:parse_expr", &[Py::str(&["(".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("f"))?, "")?, ") - (".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("g"))?, "")?, ")".to_string()].concat())], &[])?, Py::tuple(vec![c_X()?]), v_r.clone()], &[])?)?); } Py::list(out) }], true)?, &Py::Float(1000.0))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_453(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t455: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_dblr(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t455 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t456 = v_r.clone(); Py::Bool({ let t457 = v_r.clone(); if py::eq(&t456, &t457) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_458(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t463: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_459(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t463 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t464 = v_r.clone(); Py::Bool({ let t465 = v_r.clone(); if py::eq(&t464, &t465) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_459(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("erf"))?, py::float(&ax_engine::pymp::call("erf", &[py::getitem(&v_i.clone(), &Py::str("x"))?], &[])?)?, Py::Float(1e-13))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("erfc"))?, py::float(&ax_engine::pymp::call("erfc", &[py::getitem(&v_i.clone(), &Py::str("x"))?], &[])?)?, Py::Float(1e-13))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_472(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t503: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_extrema(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t503 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t504 = v_r.clone(); Py::Bool({ let t505 = v_r.clone(); if py::eq(&t504, &t505) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_512(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t517: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_fourier(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t517 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t518 = v_r.clone(); Py::Bool({ let t519 = v_r.clone(); if py::eq(&t518, &t519) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_521(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t526: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_ftc(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t526 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t527 = v_r.clone(); Py::Bool({ let t528 = v_r.clone(); if py::eq(&t527, &t528) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_530(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t535: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_531(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t535 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t536 = v_r.clone(); Py::Bool({ let t537 = v_r.clone(); if py::eq(&t536, &t537) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_531(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("gamma"))?, py::float(&ax_engine::pymp::call("gamma", &[py::getitem(&v_i.clone(), &Py::str("x"))?], &[])?)?, Py::Float(1e-13))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_563(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t574: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_564(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t574 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t575 = v_r.clone(); Py::Bool({ let t576 = v_r.clone(); if py::eq(&t575, &t576) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_564(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("exact"))?, f__gl_check(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?, Py::Float(1e-10))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("approximation"))?, f__float_sum(py::getitem(&v_i.clone(), &Py::str("f"))?, { let mut out = vec![]; for t570 in py::iter(&(f__gauss_nodes(py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?))? { let mut v__w: Py; let mut v_x: Py; 
let t571 = (t570).items()?;
if t571.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t571[0].clone();
v__w = t571[1].clone();
out.push(py::add(&py::truediv(&py::add(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?, &Py::Int(2i64))?, &py::mul(&py::truediv(&py::sub(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?, &Py::Int(2i64))?, &py::float(&v_x.clone())?)?)?); } Py::list(out) }, { let mut out = vec![]; for t572 in py::iter(&(f__gauss_nodes(py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?))? { let mut v__x: Py; let mut v_w: Py; 
let t573 = (t572).items()?;
if t573.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__x = t573[0].clone();
v_w = t573[1].clone();
out.push(py::mul(&py::truediv(&py::sub(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?, &Py::Int(2i64))?, &py::float(&v_w.clone())?)?); } Py::list(out) })?, Py::Float(1e-11))?, py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("error"))?, &py::sub(&py::getitem(&v_o.clone(), &Py::str("approximation"))?, &py::getitem(&v_o.clone(), &Py::str("exact"))?)?)?)?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_57(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t62: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_58(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t62 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t63 = v_r.clone(); Py::Bool({ let t64 = v_r.clone(); if py::eq(&t63, &t64) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_58(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("last_term"))?, py::add(&py::getitem(&v_i.clone(), &Py::str("a1"))?, &py::mul(&py::sub(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &Py::Int(1i64))?, &py::getitem(&v_i.clone(), &Py::str("d"))?)?)?, Py::Float(1e-12))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("sum"))?, py::truediv(&py::mul(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &py::add(&py::getitem(&v_i.clone(), &Py::str("a1"))?, &py::getitem(&v_o.clone(), &Py::str("last_term"))?)?)?, &Py::Int(2i64))?, Py::Float(1e-12))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_583(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t589: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_584(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t589 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t590 = v_r.clone(); Py::Bool({ let t591 = v_r.clone(); if py::eq(&t590, &t591) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_584(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("sum"))?, py::fsum(&{ let mut out = vec![]; for t588 in py::range_iter(&[(py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?).i()?]) { let mut v_k: Py; 
v_k = t588;
out.push(py::mul(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("r"))?, &v_k.clone())?)?); } Py::list(out) })?, Py::Float(1e-12))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_594(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t599: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_595(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t599 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t600 = v_r.clone(); Py::Bool({ let t601 = v_r.clone(); if py::eq(&t600, &t601) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_595(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::mul(&py::getitem(&v_o.clone(), &Py::str("sum"))?, &py::sub(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("r"))?)?)?, py::getitem(&v_i.clone(), &Py::str("a"))?, Py::Float(1e-14))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_608(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t623: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_gradient(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t623 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t624 = v_r.clone(); Py::Bool({ let t625 = v_r.clone(); if py::eq(&t624, &t625) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_666(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t669: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_green(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t669 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t670 = v_r.clone(); Py::Bool({ let t671 = v_r.clone(); if py::eq(&t670, &t671) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_68(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t73: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_69(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t73 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t74 = v_r.clone(); Py::Bool({ let t75 = v_r.clone(); if py::eq(&t74, &t75) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_685(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t697: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_hessian(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t697 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t698 = v_r.clone(); Py::Bool({ let t699 = v_r.clone(); if py::eq(&t698, &t699) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_69(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::mul(&py::getitem(&v_o.clone(), &Py::str("average_rate"))?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, py::sub(&py::getitem(&v_o.clone(), &Py::str("fb"))?, &py::getitem(&v_o.clone(), &Py::str("fa"))?)?, Py::Float(1e-13))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_711(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t718: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_implicit(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t718 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t719 = v_r.clone(); Py::Bool({ let t720 = v_r.clone(); if py::eq(&t719, &t720) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_721(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t726: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_722(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t726 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t727 = v_r.clone(); Py::Bool({ let t728 = v_r.clone(); if py::eq(&t727, &t728) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_722(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("integral"))?, py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_F: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(ax_engine::pymp::call("quad", &[v_F.clone(), Py::list(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::add(&py::getitem(&v_i.clone(), &Py::str("a"))?, &Py::Int(1i64))?, py::add(&py::getitem(&v_i.clone(), &Py::str("a"))?, &Py::Int(10i64))?, ax_engine::pymp::constant("inf")?])], &[])?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?, Py::tuple(vec![c_X()?])], &[])?])?, Py::Float(1e-09))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_743(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t771: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_inflection(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t771 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t772 = v_r.clone(); Py::Bool({ let t773 = v_r.clone(); if py::eq(&t772, &t773) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_776(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t782: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_777(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t782 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t783 = v_r.clone(); Py::Bool({ let t784 = v_r.clone(); if py::eq(&t783, &t784) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_777(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("jacobian"))?, py::float(&py::sub(&py::mul(&f__mp_partial(py::getitem(&v_i.clone(), &Py::str("xu"))?, Py::tuple(vec![Py::str("u"), Py::str("v")]), py::getitem(&v_i.clone(), &Py::str("p"))?, Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)]))?, &f__mp_partial(py::getitem(&v_i.clone(), &Py::str("yv"))?, Py::tuple(vec![Py::str("u"), Py::str("v")]), py::getitem(&v_i.clone(), &Py::str("p"))?, Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)]))?)?, &py::mul(&f__mp_partial(py::getitem(&v_i.clone(), &Py::str("xu"))?, Py::tuple(vec![Py::str("u"), Py::str("v")]), py::getitem(&v_i.clone(), &Py::str("p"))?, Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)]))?, &f__mp_partial(py::getitem(&v_i.clone(), &Py::str("yv"))?, Py::tuple(vec![Py::str("u"), Py::str("v")]), py::getitem(&v_i.clone(), &Py::str("p"))?, Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)]))?)?)?)?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_790(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t800: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_791(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t800 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t801 = v_r.clone(); Py::Bool({ let t802 = v_r.clone(); if py::eq(&t801, &t802) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_791(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("laplacian"))?, py::fsum(&{ let mut out = vec![]; for t795 in py::range_iter(&[(py::len(&py::getitem(&v_i.clone(), &Py::str("p"))?)?).i()?]) { let mut v_j: Py; 
v_j = t795;
out.push(py::float(&f__mp_partial(py::getitem(&v_i.clone(), &Py::str("f"))?, f__vars(py::getitem(&v_i.clone(), &Py::str("p"))?)?, py::getitem(&v_i.clone(), &Py::str("p"))?, { let mut out = vec![]; for t797 in py::range_iter(&[(py::len(&py::getitem(&v_i.clone(), &Py::str("p"))?)?).i()?]) { let mut v_k: Py; 
v_k = t797;
out.push(py::mul(&Py::Int(2i64), &py::int(&{ let t798 = v_k.clone(); Py::Bool({ let t799 = v_j.clone(); if py::eq(&t798, &t799) { true } else { false } }) })?)?); } Py::list(out) })?)?); } Py::list(out) })?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_80(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t85: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_81(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t85 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t86 = v_r.clone(); Py::Bool({ let t87 = v_r.clone(); if py::eq(&t86, &t87) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_81(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("average_value"))?, py::truediv(&f__gl_check(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_814(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t822: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_815(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t822 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t823 = v_r.clone(); Py::Bool({ let t824 = v_r.clone(); if py::eq(&t823, &t824) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_815(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("approximation"))?, f__float_sum(py::getitem(&v_i.clone(), &Py::str("f"))?, { let mut out = vec![]; for t821 in py::range_iter(&[(py::getitem(&v_i.clone(), &Py::str("n"))?).i()?]) { let mut v_k: Py; 
v_k = t821;
out.push(py::add(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::truediv(&py::mul(&v_k.clone(), &py::sub(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?)?); } Py::list(out) }, py::mul(&Py::list(vec![py::truediv(&py::sub(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?]), &py::getitem(&v_i.clone(), &Py::str("n"))?)?)?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_833(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t838: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_834(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t838 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t839 = v_r.clone(); Py::Bool({ let t840 = v_r.clone(); if py::eq(&t839, &t840) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_834(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("P"))?, py::float(&ax_engine::pymp::call("legendre", &[py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::getitem(&v_i.clone(), &Py::str("x"))?], &[])?)?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_852(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t856: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_limit_inf(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t856 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t857 = v_r.clone(); Py::Bool({ let t858 = v_r.clone(); if py::eq(&t857, &t858) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_870(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t875: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_limit_pt(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t875 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t876 = v_r.clone(); Py::Bool({ let t877 = v_r.clone(); if py::eq(&t876, &t877) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_883(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t891: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_884(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t891 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t892 = v_r.clone(); Py::Bool({ let t893 = v_r.clone(); if py::eq(&t892, &t893) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_884(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_v: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        v_v = (if ({ let t886 = py::getitem(&v_i.clone(), &Py::str("t0"))?; Py::Bool({ let t887 = py::getitem(&v_i.clone(), &Py::str("t1"))?; if py::ne(&t886, &t887) { true } else { false } }) }).truthy() { ax_engine::pysym::helper("calculus:_gl", &[ax_engine::pysym::helper("calculus:_compile", &[f__scal_integrand(v_i.clone())?, Py::tuple(vec![c_T()?])], &[])?, py::getitem(&v_i.clone(), &Py::str("t0"))?, py::getitem(&v_i.clone(), &Py::str("t1"))?], &[])? } else { Py::Int(0i64) });
    }
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("line_integral"))?, py::float(&v_v.clone())?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_899(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t909: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_900(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t909 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t910 = v_r.clone(); Py::Bool({ let t911 = v_r.clone(); if py::eq(&t910, &t911) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_900(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_v: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        v_v = (if ({ let t904 = py::getitem(&v_i.clone(), &Py::str("t0"))?; Py::Bool({ let t905 = py::getitem(&v_i.clone(), &Py::str("t1"))?; if py::ne(&t904, &t905) { true } else { false } }) }).truthy() { ax_engine::pysym::helper("calculus:_gl", &[ax_engine::pysym::helper("calculus:_compile", &[f__vec_integrand(v_i.clone())?, Py::tuple(vec![c_T()?])], &[])?, py::getitem(&v_i.clone(), &Py::str("t0"))?, py::getitem(&v_i.clone(), &Py::str("t1"))?], &[])? } else { Py::Int(0i64) });
    }
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("line_integral"))?, py::float(&v_v.clone())?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_913(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t918: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_914(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t918 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t919 = v_r.clone(); Py::Bool({ let t920 = v_r.clone(); if py::eq(&t919, &t920) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_914(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("slope"))?, ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?], &[])?, Py::Float(1e-12))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("approximation"))?, py::add(&ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?, Py::tuple(vec![c_X()?]), py::getitem(&v_i.clone(), &Py::str("a"))?], &[])?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("slope"))?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("x1"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?)?, Py::Float(1e-13))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_925(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t954: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_mvt(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t954 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t955 = v_r.clone(); Py::Bool({ let t956 = v_r.clone(); if py::eq(&t955, &t956) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_94(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t99: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_95(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t99 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t100 = v_r.clone(); Py::Bool({ let t101 = v_r.clone(); if py::eq(&t100, &t101) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_95(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("J"))?, py::float(&py::truediv(&ax_engine::pymp::call("quad", &[{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(ax_engine::pymp::call("cos", &[py::sub(&py::mul(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &v_t.clone())?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("x"))?, &ax_engine::pymp::call("sin", &[v_t.clone()], &[])?)?)?], &[])?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::list(vec![Py::Int(0i64), ax_engine::pymp::constant("pi")?])], &[])?, &ax_engine::pymp::constant("pi")?)?)?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_957(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v_approx: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_exact: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t966 = (f__grid(v_f.clone(), v_a.clone(), v_b.clone(), v_n.clone())?).items()?;
    if t966.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_e = t966[0].clone();
    v_h = t966[1].clone();
    v_n = t966[2].clone();
    if ({ let t969 = { let t967 = Py::str("midpoint"); Py::Bool({ let t968 = Py::str("simpson"); if py::eq(&t967, &t968) { true } else { false } }) }; if !t969.truthy() { t969 } else { py::modulo(&v_n.clone(), &Py::Int(2i64))? } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Simpson's rule needs an even number of subintervals"))));
    }
    let t982 = (f__rule_n(v_e.clone(), v_a.clone(), v_b.clone(), v_n.clone(), Py::str("midpoint"))?).items()?;
    if t982.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t982[0].clone();
    v_w = t982[1].clone();
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        v_approx = py::float(&ax_engine::pymp::call("fsum", &[{ let mut out = vec![]; for t984 in py::iter(&(py::zip(&[v_w.clone(), f__values(v_e.clone(), v_pts.clone())?])?))? { let mut v_v: Py; let mut v_wi: Py; 
let t985 = (t984).items()?;
if t985.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_wi = t985[0].clone();
v_v = t985[1].clone();
out.push(py::mul(&v_wi.clone(), &v_v.clone())?); } Py::list(out) }], &[])?)?;
    }
    v_exact = f__exact(v_e.clone(), v_a.clone(), v_b.clone())?;
    return Ok(Py::dict(vec![(Py::str("approximation"), v_approx.clone()), (Py::str("exact"), v_exact.clone()), (Py::str("error"), py::sub(&v_approx.clone(), &v_exact.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_986(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t994: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_987(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t994 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t995 = v_r.clone(); Py::Bool({ let t996 = v_r.clone(); if py::eq(&t995, &t996) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_987(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?;
    let t988 = (f__rule_n(v_e.clone(), py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, Py::str("midpoint"))?).items()?;
    if t988.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t988[0].clone();
    v_w = t988[1].clone();
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("approximation"))?, f__float_sum(py::getitem(&v_i.clone(), &Py::str("f"))?, v_pts.clone(), v_w.clone())?, Py::Float(1e-11))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("exact"))?, f__gl_check(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?, Py::Float(1e-10))?, py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("error"))?, &py::sub(&py::getitem(&v_o.clone(), &Py::str("approximation"))?, &py::getitem(&v_o.clone(), &Py::str("exact"))?)?)?)?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
