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
    "antiderivative",
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
    "conservative_field",
    "coordinate_conversion",
    "critical_points_classify",
    "curl",
    "curvature",
    "curvilinear_triple_integral",
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
    "lagrange_multipliers",
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
    "polar_double_integral",
    "power_series_convergence",
    "ratio_test",
    "regularized_gamma",
    "riemann_zeta",
    "right_riemann_sum",
    "rms_value",
    "second_derivative_at_point",
    "series_sum",
    "shell_volume",
    "simpson_rule",
    "space_curve_arc_length",
    "space_curve_curvature",
    "surface_area_graph",
    "surface_area_revolution",
    "tangent_line",
    "tangent_plane",
    "taylor_polynomial",
    "total_differential",
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
        "antiderivative" => {
            let r = f_antiderivative(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("F0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_42(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "arc_length" => {
            let r = f_arc_length(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_60(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "area_between_curves" => {
            let r = f_area_between_curves(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_73(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "area_enclosed_curves" => {
            let r = f_area_enclosed_curves(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_99(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "arithmetic_series_sum" => {
            let r = f_arithmetic_series_sum(match v.get("a1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_114(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "average_rate_of_change" => {
            let r = f_average_rate_of_change(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_125(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "average_value" => {
            let r = f_average_value(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_137(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bessel_j" => {
            let r = f_bessel_j(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_151(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "beta_function" => {
            let r = f_beta_function(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_164(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "centroid_region" => {
            let r = f_centroid_region(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_184(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chebyshev_polynomial" => {
            let r = f_chebyshev_polynomial(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_200(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "conservative_field" => {
            let r = f_conservative_field(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("B") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_236(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "coordinate_conversion" => {
            let r = f_coordinate_conversion(match v.get("system") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_289(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "critical_points_classify" => {
            let r = f_critical_points_classify(match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_330(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "curl" => {
            let r = f_curl(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_380(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "curvature" => {
            let r = f_curvature(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_400(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "curvilinear_triple_integral" => {
            let r = f_curvilinear_triple_integral(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("system") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("th1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("th2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_460(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "definite_integral" => {
            let r = f_definite_integral(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_474(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "derivative_at_point" => {
            let r = f_derivative_at_point(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_482(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "directional_derivative" => {
            let r = f_directional_derivative(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_503(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "disk_volume" => {
            let r = f_disk_volume(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_521(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "divergence" => {
            let r = f_divergence(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_540(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "divergence_theorem_box" => {
            let r = f_divergence_theorem_box(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_600(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "double_integral_rectangle" => {
            let r = f_double_integral_rectangle(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_639(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "double_integral_region" => {
            let r = f_double_integral_region(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_679(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "error_function" => {
            let r = f_error_function(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_684(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "extrema_on_interval" => {
            let r = f_extrema_on_interval(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_698(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fourier_coefficients" => {
            let r = f_fourier_coefficients(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_738(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ftc_derivative" => {
            let r = f_ftc_derivative(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_747(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gamma_function" => {
            let r = f_gamma_function(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_756(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gauss_legendre" => {
            let r = f_gauss_legendre(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_789(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geometric_partial_sum" => {
            let r = f_geometric_partial_sum(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_809(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "geometric_series_sum" => {
            let r = f_geometric_series_sum(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_820(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gradient" => {
            let r = f_gradient(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_834(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "greens_theorem" => {
            let r = f_greens_theorem(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_892(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hessian_classification" => {
            let r = f_hessian_classification(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_911(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "implicit_derivative" => {
            let r = f_implicit_derivative(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_937(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "improper_integral" => {
            let r = f_improper_integral(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_947(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inflection_points" => {
            let r = f_inflection_points(match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_969(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "jacobian_determinant" => {
            let r = f_jacobian_determinant(match v.get("xu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1002(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lagrange_multipliers" => {
            let r = f_lagrange_multipliers(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1138(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "laplacian" => {
            let r = f_laplacian(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1178(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "left_riemann_sum" => {
            let r = f_left_riemann_sum(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1202(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "legendre_polynomial" => {
            let r = f_legendre_polynomial(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1221(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "limit_at_infinity" => {
            let r = f_limit_at_infinity(match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1240(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "limit_at_point" => {
            let r = f_limit_at_point(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1258(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "line_integral_scalar" => {
            let r = f_line_integral_scalar(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1271(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "line_integral_vector" => {
            let r = f_line_integral_vector(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1287(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_approximation" => {
            let r = f_linear_approximation(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1301(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mean_value_theorem" => {
            let r = f_mean_value_theorem(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1313(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "midpoint_rule" => {
            let r = s_1345(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1374(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "newton_method" => {
            let r = f_newton_method(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1393(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "normal_line" => {
            let r = f_normal_line(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1405(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nth_derivative_at_point" => {
            let r = f_nth_derivative_at_point(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1416(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "numerical_derivative" => {
            let r = f_numerical_derivative(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("method") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1431(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parametric_arc_length" => {
            let r = f_parametric_arc_length(match v.get("xt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1444(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parametric_slope" => {
            let r = f_parametric_slope(match v.get("xt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1459(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "partial_derivative" => {
            let r = f_partial_derivative(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("var") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1474(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polar_area" => {
            let r = f_polar_area(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("al") { Some(x) => py::from_val(x), None => { return None } }, match v.get("be") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1490(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polar_double_integral" => {
            let r = f_polar_double_integral(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("al") { Some(x) => py::from_val(x), None => { return None } }, match v.get("be") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1535(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "power_series_convergence" => {
            let r = f_power_series_convergence(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1598(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ratio_test" => {
            let r = f_ratio_test(match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1645(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "regularized_gamma" => {
            let r = f_regularized_gamma(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1676(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "riemann_zeta" => {
            let r = f_riemann_zeta(match v.get("s") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1688(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "right_riemann_sum" => {
            let r = f_right_riemann_sum(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1710(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rms_value" => {
            let r = f_rms_value(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1723(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "second_derivative_at_point" => {
            let r = f_second_derivative_at_point(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1731(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "series_sum" => {
            let r = f_series_sum(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1751(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shell_volume" => {
            let r = f_shell_volume(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1769(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "simpson_rule" => {
            let r = s_1777(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1806(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "space_curve_arc_length" => {
            let r = f_space_curve_arc_length(match v.get("xt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1822(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "space_curve_curvature" => {
            let r = f_space_curve_curvature(match v.get("xt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1843(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "surface_area_graph" => {
            let r = f_surface_area_graph(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1895(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "surface_area_revolution" => {
            let r = f_surface_area_revolution(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1913(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "tangent_line" => {
            let r = f_tangent_line(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1943(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "tangent_plane" => {
            let r = f_tangent_plane(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1959(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "taylor_polynomial" => {
            let r = f_taylor_polynomial(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1975(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "total_differential" => {
            let r = f_total_differential(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mode") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("change")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2010(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "trapezoid_rule" => {
            let r = s_2035(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2064(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "triple_integral_box" => {
            let r = f_triple_integral_box(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2115(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "velocity_acceleration" => {
            let r = f_velocity_acceleration(match v.get("s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2120(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "washer_volume" => {
            let r = f_washer_volume(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2139(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
fn c__ANTI_PTS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::Float(0.37), Py::Float(1.21), Py::Float(2.53), Py::Float(-0.83), Py::Float(-1.67), Py::Float(3.4)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__B2K() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![py::fraction(&[Py::Int(1i64), Py::Int(6i64)])?, py::fraction(&[py::neg(&Py::Int(1i64))?, Py::Int(30i64)])?, py::fraction(&[Py::Int(1i64), Py::Int(42i64)])?, py::fraction(&[py::neg(&Py::Int(1i64))?, Py::Int(30i64)])?, py::fraction(&[Py::Int(5i64), Py::Int(66i64)])?, py::fraction(&[py::neg(&Py::Int(691i64))?, Py::Int(2730i64)])?, py::fraction(&[Py::Int(7i64), Py::Int(6i64)])?, py::fraction(&[py::neg(&Py::Int(3617i64))?, Py::Int(510i64)])?])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__CPTS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::tuple(vec![Py::Float(0.3), Py::Float(-0.7), Py::Float(1.1)]), Py::tuple(vec![Py::Float(1.3), Py::Float(0.4), Py::Float(-0.6)]), Py::tuple(vec![Py::Float(-0.9), Py::Float(1.7), Py::Float(0.5)])])) })());
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
fn f__anti_term(mut v_t: Py, mut v_var: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_args: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_dep: Py = Py::None;
    let mut v_fac: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_u: Py = Py::None;
    if (py::method(&v_t.clone(), "is_polynomial", &[v_var.clone()])?).truthy() {
        return Ok(ax_engine::pysym::call("integrate", &[v_t.clone(), v_var.clone()], &[])?);
    }
    let t8 = (Py::tuple(vec![ax_engine::pysym::call("Integer", &[Py::Int(1i64)], &[])?, Py::list(vec![])])).items()?;
    if t8.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t8[0].clone();
    v_dep = t8[1].clone();
    for t9 in py::iter(&(ax_engine::pysym::call("Mul.make_args", &[v_t.clone()], &[])?))? {
        v_fac = t9;
        if (py::method(&v_fac.clone(), "has", &[v_var.clone()])?).truthy() {
            let _ = py::method(&v_dep.clone(), "append", &[v_fac.clone()])?;
        } else {
            v_c = py::mul(&v_c.clone(), &v_fac.clone())?;
        }
    }
    if ({ let t10 = py::len(&v_dep.clone())?; Py::Bool({ let t11 = Py::Int(1i64); if py::ne(&t10, &t11) { true } else { false } }) }).truthy() {
        return Ok(Py::None);
    }
    v_g = py::getitem(&v_dep.clone(), &Py::Int(0i64))?;
    if ({ let t15 = py::getattr(&v_g.clone(), "is_Pow")?; if !t15.truthy() { t15 } else { { let t14 = { let t12 = py::getattr(&v_g.clone(), "base")?; Py::Bool({ let t13 = v_var.clone(); if py::eq(&t12, &t13) { true } else { false } }) }; if !t14.truthy() { t14 } else { py::getattr(&py::getattr(&v_g.clone(), "exp")?, "is_Rational")? } } } }).truthy() {
        v_n = py::getattr(&v_g.clone(), "exp")?;
        if ({ let t16 = v_n.clone(); Py::Bool({ let t17 = py::neg(&Py::Int(1i64))?; if py::eq(&t16, &t17) { true } else { false } }) }).truthy() {
            return Ok(py::mul(&v_c.clone(), &ax_engine::pysym::call("log", &[ax_engine::pysym::call("Abs", &[v_var.clone()], &[])?], &[])?)?);
        }
        return Ok(py::truediv(&py::mul(&v_c.clone(), &py::pow(&v_var.clone(), &py::add(&v_n.clone(), &Py::Int(1i64))?)?)?, &py::add(&v_n.clone(), &Py::Int(1i64))?)?);
    }
    if ({ let t20 = py::getattr(&v_g.clone(), "is_Pow")?; if !t20.truthy() { t20 } else { { let t18 = py::getattr(&v_g.clone(), "base")?; Py::Bool({ let t19 = ax_engine::pysym::constant("E")?; if py::eq(&t18, &t19) { true } else { false } }) } } }).truthy() {
        v_a = f__lin_coef(py::getattr(&v_g.clone(), "exp")?, v_var.clone())?;
        if ({ let t24 = v_a.clone(); Py::Bool({ let t25 = Py::None; if py::is(&t24, &t25) { true } else { false } }) }).truthy() {
            return Ok(Py::None);
        }
        return Ok(py::truediv(&py::mul(&v_c.clone(), &v_g.clone())?, &v_a.clone())?);
    }
    if (py::getattr(&v_g.clone(), "is_Pow")?).truthy() {
        return Ok(Py::None);
    }
    v_args = py::getattr(&v_g.clone(), "args")?;
    if ({ let t26 = py::len(&v_args.clone())?; Py::Bool({ let t27 = Py::Int(1i64); if py::ne(&t26, &t27) { true } else { false } }) }).truthy() {
        return Ok(Py::None);
    }
    v_u = py::getitem(&v_args.clone(), &Py::Int(0i64))?;
    v_a = f__lin_coef(v_u.clone(), v_var.clone())?;
    if ({ let t28 = v_a.clone(); Py::Bool({ let t29 = Py::None; if py::is(&t28, &t29) { true } else { false } }) }).truthy() {
        return Ok(Py::None);
    }
    if ({ let t30 = v_g.clone(); Py::Bool({ let t31 = ax_engine::pysym::call("sin", &[v_u.clone()], &[])?; if py::eq(&t30, &t31) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&py::mul(&py::neg(&v_c.clone())?, &ax_engine::pysym::call("cos", &[v_u.clone()], &[])?)?, &v_a.clone())?);
    }
    if ({ let t32 = v_g.clone(); Py::Bool({ let t33 = ax_engine::pysym::call("cos", &[v_u.clone()], &[])?; if py::eq(&t32, &t33) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&py::mul(&v_c.clone(), &ax_engine::pysym::call("sin", &[v_u.clone()], &[])?)?, &v_a.clone())?);
    }
    if ({ let t34 = v_g.clone(); Py::Bool({ let t35 = ax_engine::pysym::call("exp", &[v_u.clone()], &[])?; if py::eq(&t34, &t35) { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&py::mul(&v_c.clone(), &v_g.clone())?, &v_a.clone())?);
    }
    return Ok(Py::None);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__antiderivative(mut v_e: Py, mut v_var: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_out = ax_engine::pysym::call("Integer", &[Py::Int(0i64)], &[])?;
    for t7 in py::iter(&(ax_engine::pysym::call("Add.make_args", &[ax_engine::pysym::call("expand", &[v_e.clone()], &[])?], &[])?))? {
        v_t = t7;
        v_a = f__anti_term(v_t.clone(), v_var.clone())?;
        if ({ let t36 = v_a.clone(); Py::Bool({ let t37 = Py::None; if py::is(&t36, &t37) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("this integrand is outside the documented table: sums of c*x^n (n rational) and of c*sin, c*cos, c*exp of a linear argument a*x + b (products, quotients and compositions that need substitution, parts or partial fractions are refused)"))));
        }
        v_out = py::add(&v_out.clone(), &v_a.clone())?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__arc3_integrand(mut v_i: Py) -> R {
    let mut v_ex: Py = Py::None;
    let mut v_ey: Py = Py::None;
    let mut v_ez: Py = Py::None;
    let t1824 = (f__path3(py::getitem(&v_i.clone(), &Py::str("xt"))?, py::getitem(&v_i.clone(), &Py::str("yt"))?, py::getitem(&v_i.clone(), &Py::str("zt"))?)?).items()?;
    if t1824.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ex = t1824[0].clone();
    v_ey = t1824[1].clone();
    v_ez = t1824[2].clone();
    return Ok(ax_engine::pysym::call("sqrt", &[py::add(&py::add(&py::pow(&ax_engine::pysym::call("diff", &[v_ex.clone(), c_T()?], &[])?, &Py::Int(2i64))?, &py::pow(&ax_engine::pysym::call("diff", &[v_ey.clone(), c_T()?], &[])?, &Py::Int(2i64))?)?, &py::pow(&ax_engine::pysym::call("diff", &[v_ez.clone(), c_T()?], &[])?, &Py::Int(2i64))?)?], &[])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__check_nondegenerate(mut v_e: Py, mut v_A: Py, mut v_B: Py, mut v_D: Py, mut v_E: Py, mut v_F: Py, mut v_x: Py, mut v_y: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_fx: Py = Py::None;
    let mut v_fxx: Py = Py::None;
    let mut v_fxy: Py = Py::None;
    let mut v_fy: Py = Py::None;
    let mut v_fyy: Py = Py::None;
    let mut v_h0: Py = Py::None;
    let mut v_h2: Py = Py::None;
    let mut v_k0: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t1124 = (f__ellipse_params(v_A.clone(), v_B.clone(), v_D.clone(), v_E.clone(), v_F.clone())?).items()?;
    if t1124.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_h0 = t1124[0].clone();
    v_k0 = t1124[1].clone();
    v_a = t1124[2].clone();
    v_b = t1124[3].clone();
    let t1125 = (Py::tuple(vec![py::truediv(&py::sub(&v_x.clone(), &v_h0.clone())?, &v_a.clone())?, py::truediv(&py::sub(&v_y.clone(), &v_k0.clone())?, &v_b.clone())?])).items()?;
    if t1125.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t1125[0].clone();
    v_s = t1125[1].clone();
    let t1126 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?, c__XY()?, v_x.clone(), v_y.clone()], &[])?, ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_Y()?], &[])?, c__XY()?, v_x.clone(), v_y.clone()], &[])?])).items()?;
    if t1126.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fx = t1126[0].clone();
    v_fy = t1126[1].clone();
    let t1127 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?, Py::Int(2i64)], &[])?, c__XY()?, v_x.clone(), v_y.clone()], &[])?, ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?, c_Y()?], &[])?, c__XY()?, v_x.clone(), v_y.clone()], &[])?, ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_Y()?, Py::Int(2i64)], &[])?, c__XY()?, v_x.clone(), v_y.clone()], &[])?])).items()?;
    if t1127.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_fxx = t1127[0].clone();
    v_fxy = t1127[1].clone();
    v_fyy = t1127[2].clone();
    v_h2 = py::sub(&py::sub(&py::add(&py::sub(&py::mul(&py::mul(&py::mul(&py::mul(&v_a.clone(), &v_a.clone())?, &v_s.clone())?, &v_s.clone())?, &v_fxx.clone())?, &py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_b.clone())?, &v_s.clone())?, &v_c.clone())?, &v_fxy.clone())?)?, &py::mul(&py::mul(&py::mul(&py::mul(&v_b.clone(), &v_b.clone())?, &v_c.clone())?, &v_c.clone())?, &v_fyy.clone())?)?, &py::mul(&py::mul(&v_a.clone(), &v_c.clone())?, &v_fx.clone())?)?, &py::mul(&py::mul(&v_b.clone(), &v_s.clone())?, &v_fy.clone())?)?;
    v_S = py::add(&py::add(&py::add(&py::add(&py::mul(&py::mul(&v_a.clone(), &v_a.clone())?, &py::abs(&v_fxx.clone())?)?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_b.clone())?, &py::abs(&v_fxy.clone())?)?)?, &py::mul(&py::mul(&v_b.clone(), &v_b.clone())?, &py::abs(&v_fyy.clone())?)?)?, &py::mul(&v_a.clone(), &py::abs(&v_fx.clone())?)?)?, &py::mul(&v_b.clone(), &py::abs(&v_fy.clone())?)?)?;
    if ({ let t1128 = py::abs(&v_h2.clone())?; Py::Bool({ let t1129 = py::mul(&Py::Float(1e-06), &py::minmax(&[v_S.clone(), Py::Float(1e-300)], true)?)?; if py::le(&t1128, &t1129)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("degenerate constrained extremum (the second derivative of f along the constraint vanishes there): its location cannot be resolved numerically"))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__conic(mut v_text: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_G: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_G = ax_engine::pysym::call("expand", &[f__eqn(v_text.clone(), c__XY()?)?], &[])?;
    if (py::not(&py::method(&v_G.clone(), "is_polynomial", &[c_X()?, c_Y()?])?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the constraint must be a line or a circle/ellipse in x and y"))));
    }
    v_z = Py::dict(vec![(c_X()?, Py::Int(0i64)), (c_Y()?, Py::Int(0i64))]);
    let t1015 = (Py::tuple(vec![py::truediv(&ax_engine::pysym::call("diff", &[v_G.clone(), c_X()?, Py::Int(2i64)], &[])?, &Py::Int(2i64))?, py::truediv(&ax_engine::pysym::call("diff", &[v_G.clone(), c_Y()?, Py::Int(2i64)], &[])?, &Py::Int(2i64))?, ax_engine::pysym::call("diff", &[v_G.clone(), c_X()?, c_Y()?], &[])?])).items()?;
    if t1015.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1015[0].clone();
    v_B = t1015[1].clone();
    v_C = t1015[2].clone();
    let t1016 = (Py::tuple(vec![py::method(&ax_engine::pysym::call("diff", &[v_G.clone(), c_X()?], &[])?, "subs", &[v_z.clone()])?, py::method(&ax_engine::pysym::call("diff", &[v_G.clone(), c_Y()?], &[])?, "subs", &[v_z.clone()])?, py::method(&v_G.clone(), "subs", &[v_z.clone()])?])).items()?;
    if t1016.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_D = t1016[0].clone();
    v_E = t1016[1].clone();
    v_F = t1016[2].clone();
    for t1017 in py::iter(&(Py::tuple(vec![v_A.clone(), v_B.clone(), v_C.clone(), v_D.clone(), v_E.clone(), v_F.clone()])))? {
        v_v = t1017;
        if (py::not(&py::getattr(&v_v.clone(), "is_number")?)).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the constraint must be a line or a circle/ellipse in x and y"))));
        }
    }
    if ({ let t1018 = ax_engine::pysym::call("expand", &[py::sub(&v_G.clone(), &py::add(&py::add(&py::add(&py::add(&py::add(&py::mul(&v_A.clone(), &py::pow(&c_X()?, &Py::Int(2i64))?)?, &py::mul(&v_B.clone(), &py::pow(&c_Y()?, &Py::Int(2i64))?)?)?, &py::mul(&py::mul(&v_C.clone(), &c_X()?)?, &c_Y()?)?)?, &py::mul(&v_D.clone(), &c_X()?)?)?, &py::mul(&v_E.clone(), &c_Y()?)?)?, &v_F.clone())?)?], &[])?; Py::Bool({ let t1019 = Py::Int(0i64); if py::ne(&t1018, &t1019) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the constraint must be a line or a circle/ellipse in x and y"))));
    }
    return Ok(Py::tuple(vec![v_G.clone(), v_A.clone(), v_B.clone(), v_C.clone(), v_D.clone(), v_E.clone(), v_F.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__constraint_samples(mut v_A: Py, mut v_B: Py, mut v_D: Py, mut v_E: Py, mut v_F: Py) -> R {
    let mut v_Df: Py = Py::None;
    let mut v_Ef: Py = Py::None;
    let mut v_Ff: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_h0: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k0: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_out = Py::list(vec![]);
    if ({ let t1156 = { let t1152 = v_A.clone(); Py::Bool({ let t1153 = Py::Int(0i64); if py::eq(&t1152, &t1153) { true } else { false } }) }; if !t1156.truthy() { t1156 } else { { let t1154 = v_B.clone(); Py::Bool({ let t1155 = Py::Int(0i64); if py::eq(&t1154, &t1155) { true } else { false } }) } } }).truthy() {
        let t1157 = (Py::tuple(vec![f__pfloat(v_D.clone())?, f__pfloat(v_E.clone())?, f__pfloat(v_F.clone())?])).items()?;
        if t1157.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_Df = t1157[0].clone();
        v_Ef = t1157[1].clone();
        v_Ff = t1157[2].clone();
        for t1158 in py::range_iter(&[(Py::Int(401i64)).i()?]) {
            v_j = t1158;
            v_s = py::add(&py::neg(&Py::Float(50.0))?, &py::mul(&v_j.clone(), &Py::Float(0.25))?)?;
            if ({ let t1159 = v_Ef.clone(); Py::Bool({ let t1160 = Py::Int(0i64); if py::ne(&t1159, &t1160) { true } else { false } }) }).truthy() {
                let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![v_s.clone(), py::truediv(&py::neg(&py::add(&py::mul(&v_Df.clone(), &v_s.clone())?, &v_Ff.clone())?)?, &v_Ef.clone())?])])?;
            } else {
                let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![py::truediv(&py::neg(&v_Ff.clone())?, &v_Df.clone())?, v_s.clone()])])?;
            }
        }
        return Ok(v_out.clone());
    }
    let t1161 = (f__ellipse_params(v_A.clone(), v_B.clone(), v_D.clone(), v_E.clone(), v_F.clone())?).items()?;
    if t1161.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_h0 = t1161[0].clone();
    v_k0 = t1161[1].clone();
    v_a = t1161[2].clone();
    v_b = t1161[3].clone();
    for t1162 in py::range_iter(&[(Py::Int(720i64)).i()?]) {
        v_j = t1162;
        v_t = py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_j.clone())?, &Py::Int(720i64))?;
        let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![py::add(&v_h0.clone(), &py::mul(&v_a.clone(), &py::math1("cos", &v_t.clone())?)?)?, py::add(&v_k0.clone(), &py::mul(&v_b.clone(), &py::math1("sin", &v_t.clone())?)?)?])])?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__curl_from(mut v_part: Py) -> R {
    return Ok(Py::list(vec![py::sub(&py::call(&v_part.clone(), &[Py::Int(2i64), Py::Int(1i64)])?, &py::call(&v_part.clone(), &[Py::Int(1i64), Py::Int(2i64)])?)?, py::sub(&py::call(&v_part.clone(), &[Py::Int(0i64), Py::Int(2i64)])?, &py::call(&v_part.clone(), &[Py::Int(2i64), Py::Int(0i64)])?)?, py::sub(&py::call(&v_part.clone(), &[Py::Int(1i64), Py::Int(0i64)])?, &py::call(&v_part.clone(), &[Py::Int(0i64), Py::Int(1i64)])?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__curvi_integrand(mut v_f: Py, mut v_system: Py, mut v_jac: Py) -> R {
    let mut v_g: Py = Py::None;
    v_g = f__fl(ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), c__XYZ()?], &[])?, c__XYZ()?)?;
    if ({ let t461 = v_system.clone(); Py::Bool({ let t462 = Py::str("cylindrical"); if py::eq(&t461, &t462) { true } else { false } }) }).truthy() {
        return Ok({ let v_f = v_f.clone(); let v_g = v_g.clone(); let v_jac = v_jac.clone(); let v_system = v_system.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_t: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_w: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&py::call(&v_g.clone(), &[py::mul(&v_u.clone(), &py::math1("cos", &v_t.clone())?)?, py::mul(&v_u.clone(), &py::math1("sin", &v_t.clone())?)?, v_w.clone()])?, &(if (v_jac.clone()).truthy() { v_u.clone() } else { Py::Float(1.0) }))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) });
    }
    return Ok({ let v_f = v_f.clone(); let v_g = v_g.clone(); let v_jac = v_jac.clone(); let v_system = v_system.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_t: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_w: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&py::call(&v_g.clone(), &[py::mul(&py::mul(&v_u.clone(), &py::math1("sin", &v_w.clone())?)?, &py::math1("cos", &v_t.clone())?)?, py::mul(&py::mul(&v_u.clone(), &py::math1("sin", &v_w.clone())?)?, &py::math1("sin", &v_t.clone())?)?, py::mul(&v_u.clone(), &py::math1("cos", &v_w.clone())?)?])?, &(if (v_jac.clone()).truthy() { py::mul(&py::mul(&v_u.clone(), &v_u.clone())?, &py::math1("sin", &v_w.clone())?)? } else { Py::Float(1.0) }))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dh(mut v_fx: Py, mut v_fy: Py, mut v_h0: Py, mut v_k0: Py, mut v_a: Py, mut v_b: Py, mut v_t: Py) -> R {
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t1069 = (Py::tuple(vec![py::add(&v_h0.clone(), &py::mul(&v_a.clone(), &py::math1("cos", &v_t.clone())?)?)?, py::add(&v_k0.clone(), &py::mul(&v_b.clone(), &py::math1("sin", &v_t.clone())?)?)?])).items()?;
    if t1069.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1069[0].clone();
    v_y = t1069[1].clone();
    return Ok(py::add(&py::mul(&py::mul(&py::neg(&v_a.clone())?, &py::math1("sin", &v_t.clone())?)?, &py::call(&v_fx.clone(), &[v_x.clone(), v_y.clone()])?)?, &py::mul(&py::mul(&v_b.clone(), &py::math1("cos", &v_t.clone())?)?, &py::call(&v_fy.clone(), &[v_x.clone(), v_y.clone()])?)?)?);
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
fn f__ellipse_params(mut v_A: Py, mut v_B: Py, mut v_D: Py, mut v_E: Py, mut v_F: Py) -> R {
    let mut v_Af: Py = Py::None;
    let mut v_Bf: Py = Py::None;
    let mut v_Df: Py = Py::None;
    let mut v_Ef: Py = Py::None;
    let mut v_Ff: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_h0: Py = Py::None;
    let mut v_k0: Py = Py::None;
    let t1061 = (Py::tuple(vec![f__pfloat(v_A.clone())?, f__pfloat(v_B.clone())?, f__pfloat(v_D.clone())?, f__pfloat(v_E.clone())?, f__pfloat(v_F.clone())?])).items()?;
    if t1061.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_Af = t1061[0].clone();
    v_Bf = t1061[1].clone();
    v_Df = t1061[2].clone();
    v_Ef = t1061[3].clone();
    v_Ff = t1061[4].clone();
    let t1062 = (Py::tuple(vec![py::truediv(&py::neg(&v_Df.clone())?, &py::mul(&Py::Int(2i64), &v_Af.clone())?)?, py::truediv(&py::neg(&v_Ef.clone())?, &py::mul(&Py::Int(2i64), &v_Bf.clone())?)?])).items()?;
    if t1062.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_h0 = t1062[0].clone();
    v_k0 = t1062[1].clone();
    v_K = py::sub(&py::add(&py::mul(&py::mul(&v_Af.clone(), &v_h0.clone())?, &v_h0.clone())?, &py::mul(&py::mul(&v_Bf.clone(), &v_k0.clone())?, &v_k0.clone())?)?, &v_Ff.clone())?;
    if (py::not(&{ let t1063 = py::truediv(&v_K.clone(), &v_Af.clone())?; Py::Bool({ let t1064 = Py::Int(0i64); if py::gt(&t1063, &t1064)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the constraint curve is empty or a single point"))));
    }
    return Ok(Py::tuple(vec![v_h0.clone(), v_k0.clone(), py::math1("sqrt", &py::truediv(&v_K.clone(), &v_Af.clone())?)?, py::math1("sqrt", &py::truediv(&v_K.clone(), &v_Bf.clone())?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__end_probe(mut v_g: Py, mut v_xe: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_b0: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_sg: Py = Py::None;
    let mut v_y1: Py = Py::None;
    let mut v_y2: Py = Py::None;
    let mut v_ys: Py = Py::None;
    let t1606 = (Py::tuple(vec![Py::list(vec![]), Py::Int(1i64)])).items()?;
    if t1606.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ys = t1606[0].clone();
    v_sg = t1606[1].clone();
    for t1607 in py::iter(&(Py::tuple(vec![Py::Int(4000i64), Py::Int(8000i64), Py::Int(16000i64), Py::Int(32000i64)])))? {
        v_N = t1607;
        v_b0 = py::call(&v_g.clone(), &[ax_engine::pymp::call("mpf", &[v_N.clone()], &[])?, v_xe.clone()])?;
        if ({ let t1608 = v_b0.clone(); Py::Bool({ let t1609 = Py::Int(0i64); if py::eq(&t1608, &t1609) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a term vanishes at the endpoint"))));
        }
        v_q = py::truediv(&py::call(&v_g.clone(), &[ax_engine::pymp::call("mpf", &[py::add(&v_N.clone(), &Py::Int(1i64))?], &[])?, v_xe.clone()])?, &v_b0.clone())?;
        v_sg = (if ({ let t1610 = v_q.clone(); Py::Bool({ let t1611 = Py::Int(0i64); if py::gt(&t1610, &t1611)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? });
        let _ = py::method(&v_ys.clone(), "append", &[py::mul(&v_N.clone(), &py::sub(&py::abs(&v_q.clone())?, &Py::Int(1i64))?)?])?;
    }
    v_y1 = { let mut out = vec![]; for t1612 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t1612;
out.push(py::sub(&py::mul(&Py::Int(2i64), &py::getitem(&v_ys.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?)?, &py::getitem(&v_ys.clone(), &v_j.clone())?)?); } Py::list(out) };
    v_y2 = { let mut out = vec![]; for t1613 in py::range_iter(&[(Py::Int(2i64)).i()?]) { let mut v_j: Py; 
v_j = t1613;
out.push(py::truediv(&py::sub(&py::mul(&Py::Int(4i64), &py::getitem(&v_y1.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?)?, &py::getitem(&v_y1.clone(), &v_j.clone())?)?, &Py::Int(3i64))?); } Py::list(out) };
    return Ok(Py::tuple(vec![v_sg.clone(), py::truediv(&py::sub(&py::mul(&Py::Int(8i64), &py::getitem(&v_y2.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_y2.clone(), &Py::Int(0i64))?)?, &Py::Int(7i64))?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__end_verdict(mut v_sigma: Py, mut v_d: Py) -> R {
    if ({ let t1586 = v_d.clone(); Py::Bool({ let t1587 = py::neg(&Py::Int(1i64))?; if py::lt(&t1586, &t1587)? { true } else { false } }) }).truthy() {
        return Ok(Py::str("converges absolutely"));
    }
    if ({ let t1592 = { let t1588 = v_sigma.clone(); Py::Bool({ let t1589 = Py::Int(0i64); if py::lt(&t1588, &t1589)? { true } else { false } }) }; if !t1592.truthy() { t1592 } else { { let t1590 = v_d.clone(); Py::Bool({ let t1591 = Py::Int(0i64); if py::lt(&t1590, &t1591)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::str("converges conditionally"));
    }
    return Ok(Py::str("diverges"));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__eqn(mut v_text: Py, mut v_names: Py) -> R {
    let mut v_left: Py = Py::None;
    let mut v_right: Py = Py::None;
    if ({ let t1013 = py::not(&Py::Bool(matches!(v_text.clone(), Py::Str(_)))); if t1013.truthy() { t1013 } else { { let t1011 = py::method(&v_text.clone(), "count", &[Py::str("=")])?; Py::Bool({ let t1012 = Py::Int(1i64); if py::ne(&t1011, &t1012) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("write the curve as one equation, e.g. x^2 + y^2 = 25"))));
    }
    let t1014 = (py::method(&v_text.clone(), "split", &[Py::str("=")])?).items()?;
    if t1014.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_left = t1014[0].clone();
    v_right = t1014[1].clone();
    return Ok(py::sub(&ax_engine::pysym::helper("calculus:parse_expr", &[py::method(&v_left.clone(), "strip", &[])?, v_names.clone()], &[])?, &ax_engine::pysym::helper("calculus:parse_expr", &[py::method(&v_right.clone(), "strip", &[])?, v_names.clone()], &[])?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__eqn_text(mut v_text: Py) -> R {
    let mut v_left: Py = Py::None;
    let mut v_right: Py = Py::None;
    let t938 = (py::method(&v_text.clone(), "split", &[Py::str("=")])?).items()?;
    if t938.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_left = t938[0].clone();
    v_right = t938[1].clone();
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
fn f__extreme(mut v_cands: Py, mut v_sg: Py) -> R {
    let mut v_best: Py = Py::None;
    let mut v_tol: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_best = Py::None;
    for t1104 in py::iter(&(v_cands.clone()))? {
        let t1120 = (t1104).items()?;
        if t1120.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_v = t1120[0].clone();
        v_x = t1120[1].clone();
        v_y = t1120[2].clone();
        if ({ let t1105 = v_best.clone(); Py::Bool({ let t1106 = Py::None; if py::is(&t1105, &t1106) { true } else { false } }) }).truthy() {
            v_best = Py::tuple(vec![v_v.clone(), v_x.clone(), v_y.clone()]);
            continue;
        }
        v_tol = py::mul(&Py::Float(1e-10), &py::minmax(&[Py::Float(1.0), py::abs(&v_v.clone())?, py::abs(&py::getitem(&v_best.clone(), &Py::Int(0i64))?)?], true)?)?;
        if ({ let t1107 = py::mul(&v_sg.clone(), &py::sub(&v_v.clone(), &py::getitem(&v_best.clone(), &Py::Int(0i64))?)?)?; Py::Bool({ let t1108 = v_tol.clone(); if py::gt(&t1107, &t1108)? { true } else { false } }) }).truthy() {
            v_best = Py::tuple(vec![v_v.clone(), v_x.clone(), v_y.clone()]);
        } else {
            if ({ let t1119 = { let t1109 = py::abs(&py::sub(&v_v.clone(), &py::getitem(&v_best.clone(), &Py::Int(0i64))?)?)?; Py::Bool({ let t1110 = v_tol.clone(); if py::le(&t1109, &t1110)? { true } else { false } }) }; if !t1119.truthy() { t1119 } else { { let t1118 = { let t1111 = v_x.clone(); Py::Bool({ let t1112 = py::sub(&py::getitem(&v_best.clone(), &Py::Int(1i64))?, &Py::Float(1e-09))?; if py::lt(&t1111, &t1112)? { true } else { false } }) }; if t1118.truthy() { t1118 } else { { let t1117 = { let t1113 = py::abs(&py::sub(&v_x.clone(), &py::getitem(&v_best.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t1114 = Py::Float(1e-09); if py::le(&t1113, &t1114)? { true } else { false } }) }; if !t1117.truthy() { t1117 } else { { let t1115 = v_y.clone(); Py::Bool({ let t1116 = py::getitem(&v_best.clone(), &Py::Int(2i64))?; if py::lt(&t1115, &t1116)? { true } else { false } }) } } } } } } }).truthy() {
                v_best = Py::tuple(vec![v_v.clone(), v_x.clone(), v_y.clone()]);
            }
        }
    }
    return Ok(v_best.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__field(mut v_P: Py, mut v_Q: Py, mut v_R: Py) -> R {
    let mut v_names: Py = Py::None;
    v_names = (if ({ let t562 = v_R.clone(); Py::Bool({ let t563 = Py::None; if !py::is(&t562, &t563) { true } else { false } }) }).truthy() { c__XYZ()? } else { c__XY()? });
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t566 in py::iter(&((if ({ let t564 = v_R.clone(); Py::Bool({ let t565 = Py::None; if !py::is(&t564, &t565) { true } else { false } }) }).truthy() { Py::list(vec![v_P.clone(), v_Q.clone(), v_R.clone()]) } else { Py::list(vec![v_P.clone(), v_Q.clone()]) })))? { let mut v_c: Py; 
v_c = t566;
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
    return Ok(py::fsum(&{ let mut out = vec![]; for t2070 in py::iter(&(py::zip(&[v_w.clone(), v_pts.clone()])?))? { let mut v_p: Py; let mut v_wi: Py; 
let t2071 = (t2070).items()?;
if t2071.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_wi = t2071[0].clone();
v_p = t2071[1].clone();
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
        for t2093 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_i = t2093;
            v_x = ax_engine::pymp::call("cos", &[py::truediv(&py::mul(&ax_engine::pymp::constant("pi")?, &py::sub(&v_i.clone(), &py::truediv(&ax_engine::pymp::call("mpf", &[Py::Int(1i64)], &[])?, &Py::Int(4i64))?)?)?, &py::add(&v_n.clone(), &py::truediv(&ax_engine::pymp::call("mpf", &[Py::Int(1i64)], &[])?, &Py::Int(2i64))?)?)?], &[])?;
            for t2094 in py::range_iter(&[(Py::Int(100i64)).i()?]) {
                v__ = t2094;
                let t2095 = (Py::tuple(vec![ax_engine::pymp::call("mpf", &[Py::Int(1i64)], &[])?, v_x.clone()])).items()?;
                if t2095.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_p0 = t2095[0].clone();
                v_p1 = t2095[1].clone();
                for t2096 in py::range_iter(&[(Py::Int(2i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                    v_k = t2096;
                    let t2097 = (Py::tuple(vec![v_p1.clone(), py::truediv(&py::sub(&py::mul(&py::mul(&py::sub(&py::mul(&Py::Int(2i64), &v_k.clone())?, &Py::Int(1i64))?, &v_x.clone())?, &v_p1.clone())?, &py::mul(&py::sub(&v_k.clone(), &Py::Int(1i64))?, &v_p0.clone())?)?, &v_k.clone())?])).items()?;
                    if t2097.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_p0 = t2097[0].clone();
                    v_p1 = t2097[1].clone();
                }
                v_dp = (if ({ let t2098 = v_n.clone(); Py::Bool({ let t2099 = Py::Int(1i64); if py::gt(&t2098, &t2099)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&v_n.clone(), &py::sub(&py::mul(&v_x.clone(), &v_p1.clone())?, &v_p0.clone())?)?, &py::sub(&py::mul(&v_x.clone(), &v_x.clone())?, &Py::Int(1i64))?)? } else { ax_engine::pymp::call("mpf", &[Py::Int(1i64)], &[])? });
                if ({ let t2100 = v_n.clone(); Py::Bool({ let t2101 = Py::Int(1i64); if py::eq(&t2100, &t2101) { true } else { false } }) }).truthy() {
                    let t2102 = (Py::tuple(vec![v_x.clone(), ax_engine::pymp::call("mpf", &[Py::Int(1i64)], &[])?])).items()?;
                    if t2102.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_p1 = t2102[0].clone();
                    v_dp = t2102[1].clone();
                }
                v_dx = py::truediv(&v_p1.clone(), &v_dp.clone())?;
                v_x = py::sub(&v_x, &v_dx.clone())?;
                if ({ let t2103 = py::abs(&v_dx.clone())?; Py::Bool({ let t2104 = py::pow(&ax_engine::pymp::call("mpf", &[Py::Int(10i64)], &[])?, &py::neg(&Py::Int(35i64))?)?; if py::lt(&t2103, &t2104)? { true } else { false } }) }).truthy() {
                    break;
                }
            }
            let t2105 = (Py::tuple(vec![ax_engine::pymp::call("mpf", &[Py::Int(1i64)], &[])?, v_x.clone()])).items()?;
            if t2105.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_p0 = t2105[0].clone();
            v_p1 = t2105[1].clone();
            for t2106 in py::range_iter(&[(Py::Int(2i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                v_k = t2106;
                let t2107 = (Py::tuple(vec![v_p1.clone(), py::truediv(&py::sub(&py::mul(&py::mul(&py::sub(&py::mul(&Py::Int(2i64), &v_k.clone())?, &Py::Int(1i64))?, &v_x.clone())?, &v_p1.clone())?, &py::mul(&py::sub(&v_k.clone(), &Py::Int(1i64))?, &v_p0.clone())?)?, &v_k.clone())?])).items()?;
                if t2107.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_p0 = t2107[0].clone();
                v_p1 = t2107[1].clone();
            }
            v_dp = (if ({ let t2108 = v_n.clone(); Py::Bool({ let t2109 = Py::Int(1i64); if py::gt(&t2108, &t2109)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&v_n.clone(), &py::sub(&py::mul(&v_x.clone(), &v_p1.clone())?, &v_p0.clone())?)?, &py::sub(&py::mul(&v_x.clone(), &v_x.clone())?, &Py::Int(1i64))?)? } else { ax_engine::pymp::call("mpf", &[Py::Int(1i64)], &[])? });
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
    let t2090 = (Py::tuple(vec![py::truediv(&py::add(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?, py::truediv(&py::sub(&v_b.clone(), &v_a.clone())?, &Py::Int(2i64))?])).items()?;
    if t2090.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t2090[0].clone();
    v_r = t2090[1].clone();
    return Ok(py::mul(&v_r.clone(), &py::fsum(&{ let mut out = vec![]; for t2112 in py::iter(&(f__glf(v_n.clone())?))? { let mut v_w: Py; let mut v_x: Py; 
let t2113 = (t2112).items()?;
if t2113.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2113[0].clone();
v_w = t2113[1].clone();
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
    let t2086 = (v_bx.clone()).items()?;
    if t2086.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    let t2087 = (t2086[0].clone()).items()?;
    if t2087.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2087[0].clone();
    v_b = t2087[1].clone();
    let t2088 = (t2086[1].clone()).items()?;
    if t2088.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t2088[0].clone();
    v_d = t2088[1].clone();
    let t2089 = (t2086[2].clone()).items()?;
    if t2089.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_e = t2089[0].clone();
    v_f = t2089[1].clone();
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
    if ({ let t2091 = v_n.clone(); Py::Bool({ let t2092 = c__GLF()?; if !py::contains(&t2092, &t2091)? { true } else { false } }) }).truthy() {
        py::setitem(&c__GLF()?, &v_n.clone(), { let mut out = vec![]; for t2110 in py::iter(&(f__gauss_nodes(v_n.clone())?))? { let mut v_w: Py; let mut v_x: Py; 
let t2111 = (t2110).items()?;
if t2111.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2111[0].clone();
v_w = t2111[1].clone();
out.push(Py::tuple(vec![py::float(&v_x.clone())?, py::float(&v_w.clone())?])); } Py::list(out) })?;
    }
    return Ok(py::getitem(&c__GLF()?, &v_n.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__grid(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    if ({ let t2041 = { let t2036 = py::int(&v_n.clone())?; Py::Bool({ let t2037 = v_n.clone(); if py::ne(&t2036, &t2037) { true } else { false } }) }; if t2041.truthy() { t2041 } else { py::not(&{ let t2038 = Py::Int(1i64); Py::Bool({ let t2039 = v_n.clone(); if py::le(&t2038, &t2039)? { { let t2040 = Py::Int(100000i64); if py::le(&t2039, &t2040)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be a whole number of subintervals between 1 and 100000"))));
    }
    if (py::not(&{ let t2042 = v_a.clone(); Py::Bool({ let t2043 = v_b.clone(); if py::lt(&t2042, &t2043)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    return Ok(Py::tuple(vec![ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?, py::truediv(&py::sub(&v_b.clone(), &v_a.clone())?, &py::int(&v_n.clone())?)?, py::int(&v_n.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lagrange_ellipse(mut v_e: Py, mut v_A: Py, mut v_B: Py, mut v_D: Py, mut v_E: Py, mut v_F: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_cands: Py = Py::None;
    let mut v_cd: Py = Py::None;
    let mut v_fl: Py = Py::None;
    let mut v_fx: Py = Py::None;
    let mut v_fy: Py = Py::None;
    let mut v_h0: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k0: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_roots: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_slo: Py = Py::None;
    let mut v_sm: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_ts: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vmax: Py = Py::None;
    let mut v_vmin: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t1065 = (f__ellipse_params(v_A.clone(), v_B.clone(), v_D.clone(), v_E.clone(), v_F.clone())?).items()?;
    if t1065.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_h0 = t1065[0].clone();
    v_k0 = t1065[1].clone();
    v_a = t1065[2].clone();
    v_b = t1065[3].clone();
    let t1066 = (Py::tuple(vec![f__fl(ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?, c__XY()?)?, f__fl(ax_engine::pysym::call("diff", &[v_e.clone(), c_Y()?], &[])?, c__XY()?)?, f__fl(v_e.clone(), c__XY()?)?])).items()?;
    if t1066.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_fx = t1066[0].clone();
    v_fy = t1066[1].clone();
    v_fl = t1066[2].clone();
    v_N = Py::Int(1440i64);
    let t1067 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t1067.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ts = t1067[0].clone();
    v_s = t1067[1].clone();
    for t1068 in py::range_iter(&[(py::add(&v_N.clone(), &Py::Int(1i64))?).i()?]) {
        v_j = t1068;
        v_t = py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::add(&v_j.clone(), &Py::Float(0.37))?)?, &v_N.clone())?;
        let _ = py::method(&v_ts.clone(), "append", &[v_t.clone()])?;
        let _ = py::method(&v_s.clone(), "append", &[f__dh(v_fx.clone(), v_fy.clone(), v_h0.clone(), v_k0.clone(), v_a.clone(), v_b.clone(), v_t.clone())?])?;
    }
    v_roots = Py::list(vec![]);
    for t1070 in py::range_iter(&[(v_N.clone()).i()?]) {
        v_j = t1070;
        if ({ let t1071 = py::getitem(&v_s.clone(), &v_j.clone())?; Py::Bool({ let t1072 = Py::Int(0i64); if py::eq(&t1071, &t1072) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_roots.clone(), "append", &[py::getitem(&v_ts.clone(), &v_j.clone())?])?;
        } else {
            if ({ let t1073 = py::mul(&py::getitem(&v_s.clone(), &v_j.clone())?, &py::getitem(&v_s.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t1074 = Py::Int(0i64); if py::lt(&t1073, &t1074)? { true } else { false } }) }).truthy() {
                let t1075 = (Py::tuple(vec![py::getitem(&v_ts.clone(), &v_j.clone())?, py::getitem(&v_ts.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?, py::getitem(&v_s.clone(), &v_j.clone())?])).items()?;
                if t1075.len() != 3 { return Err(PyErr::Value("unpack".into())); }
                v_lo = t1075[0].clone();
                v_hi = t1075[1].clone();
                v_slo = t1075[2].clone();
                for t1076 in py::range_iter(&[(Py::Int(80i64)).i()?]) {
                    v__ = t1076;
                    v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
                    v_sm = f__dh(v_fx.clone(), v_fy.clone(), v_h0.clone(), v_k0.clone(), v_a.clone(), v_b.clone(), v_mid.clone())?;
                    if ({ let t1077 = v_sm.clone(); Py::Bool({ let t1078 = Py::Int(0i64); if py::eq(&t1077, &t1078) { true } else { false } }) }).truthy() {
                        v_lo = v_mid.clone();
                        v_hi = v_mid.clone();
                        break;
                    }
                    if ({ let t1079 = { let t1081 = v_sm.clone(); Py::Bool({ let t1082 = Py::Int(0i64); if py::gt(&t1081, &t1082)? { true } else { false } }) }; Py::Bool({ let t1080 = { let t1083 = v_slo.clone(); Py::Bool({ let t1084 = Py::Int(0i64); if py::gt(&t1083, &t1084)? { true } else { false } }) }; if py::eq(&t1079, &t1080) { true } else { false } }) }).truthy() {
                        let t1085 = (Py::tuple(vec![v_mid.clone(), v_sm.clone()])).items()?;
                        if t1085.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                        v_lo = t1085[0].clone();
                        v_slo = t1085[1].clone();
                    } else {
                        v_hi = v_mid.clone();
                    }
                }
                let _ = py::method(&v_roots.clone(), "append", &[py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?])?;
            }
        }
    }
    v_cands = Py::list(vec![]);
    for t1086 in py::iter(&(v_roots.clone()))? {
        v_t = t1086;
        let t1087 = (Py::tuple(vec![py::add(&v_h0.clone(), &py::mul(&v_a.clone(), &py::math1("cos", &v_t.clone())?)?)?, py::add(&v_k0.clone(), &py::mul(&v_b.clone(), &py::math1("sin", &v_t.clone())?)?)?])).items()?;
        if t1087.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t1087[0].clone();
        v_y = t1087[1].clone();
        let _ = py::method(&v_cands.clone(), "append", &[Py::tuple(vec![ax_engine::pysym::helper("calculus:at", &[v_e.clone(), c__XY()?, v_x.clone(), v_y.clone()], &[])?, v_x.clone(), v_y.clone()])])?;
    }
    if (py::not(&v_cands.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("f is constant on the constraint"))));
    }
    let t1088 = (Py::tuple(vec![py::getitem(&py::getitem(&v_cands.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, py::getitem(&py::getitem(&v_cands.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?])).items()?;
    if t1088.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vmax = t1088[0].clone();
    v_vmin = t1088[1].clone();
    for t1089 in py::iter(&(v_cands.clone()))? {
        v_cd = t1089;
        let t1090 = (Py::tuple(vec![py::minmax(&[v_vmax.clone(), py::getitem(&v_cd.clone(), &Py::Int(0i64))?], true)?, py::minmax(&[v_vmin.clone(), py::getitem(&v_cd.clone(), &Py::Int(0i64))?], false)?])).items()?;
        if t1090.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_vmax = t1090[0].clone();
        v_vmin = t1090[1].clone();
    }
    if ({ let t1091 = py::sub(&v_vmax.clone(), &v_vmin.clone())?; Py::Bool({ let t1092 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_vmax.clone())?], true)?)?; if py::le(&t1091, &t1092)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("f is constant on the constraint (every point is both a maximum and a minimum)"))));
    }
    for t1093 in py::iter(&(v_ts.clone()))? {
        v_t = t1093;
        v_v = py::call(&v_fl.clone(), &[py::add(&v_h0.clone(), &py::mul(&v_a.clone(), &py::math1("cos", &v_t.clone())?)?)?, py::add(&v_k0.clone(), &py::mul(&v_b.clone(), &py::math1("sin", &v_t.clone())?)?)?])?;
        if ({ let t1098 = { let t1094 = v_v.clone(); Py::Bool({ let t1095 = py::add(&v_vmax.clone(), &py::mul(&Py::Float(1e-09), &py::minmax(&[Py::Float(1.0), py::abs(&v_vmax.clone())?], true)?)?)?; if py::gt(&t1094, &t1095)? { true } else { false } }) }; if t1098.truthy() { t1098 } else { { let t1096 = v_v.clone(); Py::Bool({ let t1097 = py::sub(&v_vmin.clone(), &py::mul(&Py::Float(1e-09), &py::minmax(&[Py::Float(1.0), py::abs(&v_vmin.clone())?], true)?)?)?; if py::lt(&t1096, &t1097)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a critical point could not be resolved numerically"))));
        }
    }
    return Ok(v_cands.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lagrange_line(mut v_e: Py, mut v_D: Py, mut v_E: Py, mut v_F: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_cands: Py = Py::None;
    let mut v_cf: Py = Py::None;
    let mut v_co: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_prev: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_xt: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_yt: Py = Py::None;
    if ({ let t1034 = v_E.clone(); Py::Bool({ let t1035 = Py::Int(0i64); if py::ne(&t1034, &t1035) { true } else { false } }) }).truthy() {
        let t1036 = (Py::tuple(vec![c_T()?, py::truediv(&py::neg(&py::add(&py::mul(&v_D.clone(), &c_T()?)?, &v_F.clone())?)?, &v_E.clone())?])).items()?;
        if t1036.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_xt = t1036[0].clone();
        v_yt = t1036[1].clone();
    } else {
        let t1037 = (Py::tuple(vec![py::truediv(&py::neg(&v_F.clone())?, &v_D.clone())?, c_T()?])).items()?;
        if t1037.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_xt = t1037[0].clone();
        v_yt = t1037[1].clone();
    }
    v_h = ax_engine::pysym::call("expand", &[py::method(&v_e.clone(), "subs", &[Py::dict(vec![(c_X()?, v_xt.clone()), (c_Y()?, v_yt.clone())])])?], &[])?;
    if (py::not(&py::method(&v_h.clone(), "is_polynomial", &[c_T()?])?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("with a line constraint the function must be a polynomial in x and y"))));
    }
    v_P = ax_engine::pysym::call("Poly", &[v_h.clone(), c_T()?], &[])?;
    v_co = py::method(&v_P.clone(), "all_coeffs", &[])?;
    for t1038 in py::iter(&(v_co.clone()))? {
        v_cf = t1038;
        if (py::not(&py::getattr(&v_cf.clone(), "is_Rational")?)).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("with a line constraint the polynomial needs rational coefficients"))));
        }
    }
    v_n = py::method(&v_P.clone(), "degree", &[])?;
    if ({ let t1039 = v_n.clone(); Py::Bool({ let t1040 = Py::Int(1i64); if py::lt(&t1039, &t1040)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("f is constant on the constraint"))));
    }
    if ({ let t1041 = py::modulo(&v_n.clone(), &Py::Int(2i64))?; Py::Bool({ let t1042 = Py::Int(1i64); if py::eq(&t1041, &t1042) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("f is unbounded above and below along the line: there is no constrained maximum or minimum"))));
    }
    let t1043 = (Py::tuple(vec![Py::list(vec![]), Py::None])).items()?;
    if t1043.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cands = t1043[0].clone();
    v_prev = t1043[1].clone();
    for t1044 in py::iter(&(ax_engine::pysym::call("real_roots", &[py::method(&v_P.clone(), "diff", &[])?], &[])?))? {
        v_r = t1044;
        v_t = py::float(&v_r.clone())?;
        if ({ let t1049 = { let t1045 = v_prev.clone(); Py::Bool({ let t1046 = Py::None; if !py::is(&t1045, &t1046) { true } else { false } }) }; if !t1049.truthy() { t1049 } else { { let t1047 = py::abs(&py::sub(&v_t.clone(), &v_prev.clone())?)?; Py::Bool({ let t1048 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_t.clone())?], true)?)?; if py::le(&t1047, &t1048)? { true } else { false } }) } } }).truthy() {
            continue;
        }
        v_prev = v_t.clone();
        let t1050 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:at", &[v_xt.clone(), Py::tuple(vec![c_T()?]), v_t.clone()], &[])?, ax_engine::pysym::helper("calculus:at", &[v_yt.clone(), Py::tuple(vec![c_T()?]), v_t.clone()], &[])?])).items()?;
        if t1050.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t1050[0].clone();
        v_y = t1050[1].clone();
        let _ = py::method(&v_cands.clone(), "append", &[Py::tuple(vec![ax_engine::pysym::helper("calculus:at", &[v_e.clone(), c__XY()?, v_x.clone(), v_y.clone()], &[])?, v_x.clone(), v_y.clone()])])?;
    }
    return Ok(Py::tuple(vec![v_cands.clone(), { let t1051 = py::getitem(&v_co.clone(), &Py::Int(0i64))?; Py::Bool({ let t1052 = Py::Int(0i64); if py::lt(&t1051, &t1052)? { true } else { false } }) }, { let t1053 = py::getitem(&v_co.clone(), &Py::Int(0i64))?; Py::Bool({ let t1054 = Py::Int(0i64); if py::gt(&t1053, &t1054)? { true } else { false } }) }]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__limval(mut v_L: Py) -> R {
    let mut v_v: Py = Py::None;
    if ({ let t1747 = py::not(&Py::Bool(matches!(v_L.clone(), Py::Sym(_)))); if t1747.truthy() { t1747 } else { { let t1746 = py::getattr(&v_L.clone(), "free_symbols")?; if t1746.truthy() { t1746 } else { { let t1745 = py::method(&v_L.clone(), "has", &[ax_engine::pysym::constant("Limit")?])?; if t1745.truthy() { t1745 } else { { let t1744 = py::method(&v_L.clone(), "has", &[ax_engine::pysym::constant("Sum")?])?; if t1744.truthy() { t1744 } else { py::method(&v_L.clone(), "has", &[ax_engine::pysym::constant("AccumBounds")?])? } } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the limit does not exist or could not be determined"))));
    }
    if (py::method(&v_L.clone(), "has", &[ax_engine::pysym::constant("oo")?, py::neg(&ax_engine::pysym::constant("oo")?)?, ax_engine::pysym::constant("zoo")?, ax_engine::pysym::constant("nan")?])?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the limit is infinite (it does not exist as a finite number)"))));
    }
    v_v = py::complex(&[ax_engine::pysym::call("N", &[v_L.clone(), Py::Int(30i64)], &[])?])?;
    if ({ let t1750 = { let t1748 = py::abs(&py::imag(&v_v.clone())?)?; Py::Bool({ let t1749 = py::mul(&Py::Float(1e-20), &py::minmax(&[Py::Float(1.0), py::abs(&py::real(&v_v.clone())?)?], true)?)?; if py::gt(&t1748, &t1749)? { true } else { false } }) }; if t1750.truthy() { t1750 } else { py::not(&py::math1("isfinite", &py::real(&v_v.clone())?)?) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the limit is not a finite real number"))));
    }
    return Ok(py::real(&v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lin_coef(mut v_arg: Py, mut v_var: Py) -> R {
    let mut v_d: Py = Py::None;
    if (py::not(&py::method(&v_arg.clone(), "is_polynomial", &[v_var.clone()])?)).truthy() {
        return Ok(Py::None);
    }
    v_d = ax_engine::pysym::call("diff", &[v_arg.clone(), v_var.clone()], &[])?;
    if ({ let t23 = py::not(&py::getattr(&v_d.clone(), "is_number")?); if t23.truthy() { t23 } else { { let t21 = v_d.clone(); Py::Bool({ let t22 = Py::Int(0i64); if py::eq(&t21, &t22) { true } else { false } }) } } }).truthy() {
        return Ok(Py::None);
    }
    return Ok(v_d.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mp_partial(mut v_text: Py, mut v_names: Py, mut v_point: Py, mut v_orders: Py) -> R {
    let mut v_g: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        v_g = ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::helper("calculus:parse_expr", &[v_text.clone(), v_names.clone()], &[])?, v_names.clone()], &[])?;
        return Ok(ax_engine::pymp::call("diff", &[v_g.clone(), Py::tuple(({ let mut out = vec![]; for t2016 in py::iter(&(v_point.clone()))? { let mut v_v: Py; 
v_v = t2016;
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
    if ({ let t1318 = py::method(&v_s.clone(), "degree", &[])?; Py::Bool({ let t1319 = Py::Int(1i64); if py::lt(&t1318, &t1319)? { true } else { false } }) }).truthy() {
        return Ok(Py::Int(0i64));
    }
    if ({ let t1320 = v_a.clone(); Py::Bool({ let t1321 = Py::None; if py::is(&t1320, &t1321) { true } else { false } }) }).truthy() {
        return Ok(py::method(&v_s.clone(), "count_roots", &[])?);
    }
    let t1322 = (Py::tuple(vec![f__rat(v_a.clone())?, f__rat(v_b.clone())?])).items()?;
    if t1322.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1322[0].clone();
    v_B = t1322[1].clone();
    return Ok(py::sub(&py::sub(&py::method(&v_s.clone(), "count_roots", &[v_A.clone(), v_B.clone()])?, &py::int(&{ let t1323 = py::method(&v_s.clone(), "eval", &[v_A.clone()])?; Py::Bool({ let t1324 = Py::Int(0i64); if py::eq(&t1323, &t1324) { true } else { false } }) })?)?, &py::int(&{ let t1325 = py::method(&v_s.clone(), "eval", &[v_B.clone()])?; Py::Bool({ let t1326 = Py::Int(0i64); if py::eq(&t1325, &t1326) { true } else { false } }) })?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg_on(mut v_p: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_pts: Py = Py::None;
    let mut v_tests: Py = Py::None;
    v_pts = py::add(&py::add(&Py::list(vec![ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_a.clone())?))], &[])?]), &ax_engine::pysym::helper("calculus:real_roots_in", &[v_p.clone(), v_a.clone(), v_b.clone()], &[])?)?, &Py::list(vec![ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_b.clone())?))], &[])?]))?;
    v_tests = py::add(&Py::list(vec![py::getitem(&v_pts.clone(), &Py::Int(0i64))?, py::getitem(&v_pts.clone(), &py::neg(&Py::Int(1i64))?)?]), &{ let mut out = vec![]; for t2131 in py::range_iter(&[(py::sub(&py::len(&v_pts.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t2131;
out.push(py::truediv(&py::add(&py::getitem(&v_pts.clone(), &v_k.clone())?, &py::getitem(&v_pts.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?, &Py::Int(2i64))?); } Py::list(out) })?;
    return Ok(Py::Bool(({ let mut out = vec![]; for t2132 in py::iter(&(v_tests.clone()))? { let mut v_t: Py; 
v_t = t2132;
out.push((if (py::getattr(&v_t.clone(), "is_Rational")?).truthy() { { let t2133 = py::method(&v_p.clone(), "eval", &[ax_engine::pysym::call("nsimplify", &[v_t.clone()], &[])?])?; Py::Bool({ let t2134 = Py::Int(0i64); if py::ge(&t2133, &t2134)? { true } else { false } }) } } else { { let t2135 = py::float(&ax_engine::pysym::call("N", &[py::method(&py::method(&v_p.clone(), "as_expr", &[])?, "subs", &[py::getattr(&v_p.clone(), "gen")?, v_t.clone()])?, Py::Int(30i64)], &[])?)?; Py::Bool({ let t2136 = Py::Int(0i64); if py::ge(&t2135, &t2136)? { true } else { false } }) } })); } Py::list(out) }).items()?.iter().all(|x| x.truthy())));
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
fn f__path3(mut v_xt: Py, mut v_yt: Py, mut v_zt: Py) -> R {
    return Ok(Py::tuple(vec![ax_engine::pysym::helper("calculus:parse_expr", &[v_xt.clone(), Py::tuple(vec![Py::str("t")])], &[])?, ax_engine::pysym::helper("calculus:parse_expr", &[v_yt.clone(), Py::tuple(vec![Py::str("t")])], &[])?, ax_engine::pysym::helper("calculus:parse_expr", &[v_zt.clone(), Py::tuple(vec![Py::str("t")])], &[])?]));
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
fn f__pfield(mut v_P: Py, mut v_Q: Py, mut v_R: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_ok: Py = Py::None;
    let mut v_three: Py = Py::None;
    v_three = { let t208 = v_R.clone(); Py::Bool({ let t209 = Py::None; if !py::is(&t208, &t209) { true } else { false } }) };
    v_names = (if (v_three.clone()).truthy() { c__XYZ()? } else { c__XY()? });
    v_F = Py::list(vec![ax_engine::pysym::helper("calculus:parse_expr", &[v_P.clone(), v_names.clone()], &[])?, ax_engine::pysym::helper("calculus:parse_expr", &[v_Q.clone(), v_names.clone()], &[])?]);
    if (v_three.clone()).truthy() {
        let _ = py::method(&v_F.clone(), "append", &[ax_engine::pysym::helper("calculus:parse_expr", &[v_R.clone(), v_names.clone()], &[])?])?;
    }
    for t210 in py::iter(&(v_F.clone()))? {
        v_c = t210;
        v_ok = (if (v_three.clone()).truthy() { py::method(&v_c.clone(), "is_polynomial", &[c_X()?, c_Y()?, c_Z()?])? } else { py::method(&v_c.clone(), "is_polynomial", &[c_X()?, c_Y()?])? });
        if (py::not(&v_ok.clone())).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the documented scope is a field with polynomial components (other fields are refused)"))));
        }
    }
    return Ok(Py::tuple(vec![v_F.clone(), v_names.clone()]));
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
    for t70 in py::range_iter(&[(py::sub(&py::len(&v_pts.clone())?, &Py::Int(1i64))?).i()?]) {
        v_k = t70;
        v_tot = py::iadd(&v_tot, &ax_engine::pysym::call("Abs", &[py::sub(&py::method(&py::method(&v_H.clone(), "as_expr", &[])?, "subs", &[py::getattr(&v_h.clone(), "gen")?, py::getitem(&v_pts.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?])?, &py::method(&py::method(&v_H.clone(), "as_expr", &[])?, "subs", &[py::getattr(&v_h.clone(), "gen")?, py::getitem(&v_pts.clone(), &v_k.clone())?])?)?], &[])?)?;
    }
    return Ok(Py::tuple(vec![py::float(&ax_engine::pysym::call("N", &[v_tot.clone(), Py::Int(30i64)], &[])?)?, { let mut out = vec![]; for t71 in py::iter(&(py::slice(&v_pts.clone(), Some((Py::Int(1i64)).i()?), Some((py::neg(&Py::Int(1i64))?).i()?), None)?))? { let mut v_r: Py; 
v_r = t71;
out.push(py::float(&ax_engine::pysym::call("N", &[v_r.clone(), Py::Int(30i64)], &[])?)?); } Py::list(out) }]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pseries_parts(mut v_e: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_base: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let mut v_fac: Py = Py::None;
    let mut v_hn: Py = Py::None;
    let mut v_hx: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_kk: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t1543 = (Py::tuple(vec![Py::None, Py::None])).items()?;
    if t1543.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k = t1543[0].clone();
    v_m = t1543[1].clone();
    v_rho = ax_engine::pysym::call("Integer", &[Py::Int(1i64)], &[])?;
    v_d = py::fraction(&[Py::Int(0i64)])?;
    for t1544 in py::iter(&(ax_engine::pysym::call("Mul.make_args", &[v_e.clone()], &[])?))? {
        v_fac = t1544;
        let t1545 = (Py::tuple(vec![py::method(&v_fac.clone(), "has", &[c_X()?])?, py::method(&v_fac.clone(), "has", &[c_NN()?])?])).items()?;
        if t1545.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_hx = t1545[0].clone();
        v_hn = t1545[1].clone();
        if ({ let t1546 = py::not(&v_hx.clone()); if !t1546.truthy() { t1546 } else { py::not(&v_hn.clone()) } }).truthy() {
            if ({ let t1547 = v_fac.clone(); Py::Bool({ let t1548 = Py::Int(0i64); if py::eq(&t1547, &t1548) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("the term is zero"))));
            }
            continue;
        }
        if (v_hx.clone()).truthy() {
            if ({ let t1551 = { let t1549 = v_k.clone(); Py::Bool({ let t1550 = Py::None; if !py::is(&t1549, &t1550) { true } else { false } }) }; if t1551.truthy() { t1551 } else { py::not(&py::getattr(&v_fac.clone(), "is_Pow")?) } }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("write the term as c_n * (x - a)^n, with x only in that power"))));
            }
            let t1552 = (Py::tuple(vec![py::getattr(&v_fac.clone(), "base")?, py::getattr(&v_fac.clone(), "exp")?])).items()?;
            if t1552.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_base = t1552[0].clone();
            v_ex = t1552[1].clone();
            if ({ let t1553 = py::method(&v_base.clone(), "has", &[c_NN()?])?; if t1553.truthy() { t1553 } else { py::not(&py::method(&v_base.clone(), "is_polynomial", &[c_X()?])?) } }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("the power must be of a linear expression k*x + m"))));
            }
            v_kk = ax_engine::pysym::call("diff", &[v_base.clone(), c_X()?], &[])?;
            if ({ let t1556 = py::not(&py::getattr(&v_kk.clone(), "is_number")?); if t1556.truthy() { t1556 } else { { let t1554 = v_kk.clone(); Py::Bool({ let t1555 = Py::Int(0i64); if py::eq(&t1554, &t1555) { true } else { false } }) } } }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("the power must be of a linear expression k*x + m"))));
            }
            v_j = ax_engine::pysym::call("expand", &[py::sub(&v_ex.clone(), &c_NN()?)?], &[])?;
            if (py::not(&py::getattr(&v_j.clone(), "is_Integer")?)).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("the power of (x - a) must be n (or n plus a whole number)"))));
            }
            let t1557 = (Py::tuple(vec![v_kk.clone(), py::method(&v_base.clone(), "subs", &[c_X()?, Py::Int(0i64)])?])).items()?;
            if t1557.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_k = t1557[0].clone();
            v_m = t1557[1].clone();
            continue;
        }
        if (py::method(&v_fac.clone(), "is_polynomial", &[c_NN()?])?).truthy() {
            v_d = py::add(&v_d.clone(), &py::method(&ax_engine::pysym::call("Poly", &[v_fac.clone(), c_NN()?], &[])?, "degree", &[])?)?;
            continue;
        }
        if (py::not(&py::getattr(&v_fac.clone(), "is_Pow")?)).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the coefficient may contain powers and polynomials of n and constants to the power n only"))));
        }
        let t1558 = (Py::tuple(vec![py::getattr(&v_fac.clone(), "base")?, py::getattr(&v_fac.clone(), "exp")?])).items()?;
        if t1558.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_base = t1558[0].clone();
        v_ex = t1558[1].clone();
        if ({ let t1559 = py::method(&v_base.clone(), "has", &[c_NN()?])?; if !t1559.truthy() { t1559 } else { py::not(&py::method(&v_ex.clone(), "has", &[c_NN()?])?) } }).truthy() {
            if ({ let t1560 = py::not(&py::method(&v_base.clone(), "is_polynomial", &[c_NN()?])?); if t1560.truthy() { t1560 } else { py::not(&py::getattr(&v_ex.clone(), "is_Rational")?) } }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("a power of a polynomial in n needs a rational exponent"))));
            }
            v_P = ax_engine::pysym::call("Poly", &[v_base.clone(), c_NN()?], &[])?;
            if ({ let t1563 = py::not(&py::getattr(&v_ex.clone(), "is_Integer")?); if !t1563.truthy() { t1563 } else { { let t1561 = py::method(&v_P.clone(), "LC", &[])?; Py::Bool({ let t1562 = Py::Int(0i64); if py::lt(&t1561, &t1562)? { true } else { false } }) } } }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("the term is not real for large n"))));
            }
            v_d = py::add(&v_d.clone(), &py::mul(&py::method(&v_P.clone(), "degree", &[])?, &py::fraction(&[py::int(&py::getattr(&v_ex.clone(), "p")?)?, py::int(&py::getattr(&v_ex.clone(), "q")?)?])?)?)?;
            continue;
        }
        if ({ let t1565 = py::not(&py::method(&v_base.clone(), "has", &[c_NN()?])?); if !t1565.truthy() { t1565 } else { { let t1564 = py::getattr(&v_base.clone(), "is_number")?; if !t1564.truthy() { t1564 } else { py::method(&v_ex.clone(), "is_polynomial", &[c_NN()?])? } } } }).truthy() {
            v_s = ax_engine::pysym::call("diff", &[v_ex.clone(), c_NN()?], &[])?;
            if ({ let t1568 = py::not(&py::getattr(&v_s.clone(), "is_Integer")?); if t1568.truthy() { t1568 } else { { let t1566 = v_s.clone(); Py::Bool({ let t1567 = Py::Int(0i64); if py::eq(&t1566, &t1567) { true } else { false } }) } } }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("a constant to a power of n needs an exponent s*n + t with s a whole number"))));
            }
            if ({ let t1569 = v_base.clone(); Py::Bool({ let t1570 = Py::Int(0i64); if py::eq(&t1569, &t1570) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("the term is zero"))));
            }
            v_rho = py::mul(&v_rho.clone(), &py::pow(&v_base.clone(), &v_s.clone())?)?;
            continue;
        }
        return Err(py::value_error(py::to_str(&Py::str("the coefficient may contain powers and polynomials of n and constants to the power n only"))));
    }
    if ({ let t1571 = v_k.clone(); Py::Bool({ let t1572 = Py::None; if py::is(&t1571, &t1572) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("write the series term with a power (x - a)^n, e.g. (x - 2)^n/(n*3^n)"))));
    }
    return Ok(Py::tuple(vec![v_k.clone(), v_m.clone(), v_rho.clone(), v_d.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__q_t(mut v_e: Py, mut v_t0: Py, mut v_t1: Py) -> R {
    if ({ let t1820 = v_t0.clone(); Py::Bool({ let t1821 = v_t1.clone(); if py::eq(&t1820, &t1821) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    let _ = ax_engine::pysym::helper("calculus:continuous_on", &[v_e.clone(), c_T()?, v_t0.clone(), v_t1.clone()], &[])?;
    return Ok(ax_engine::pysym::helper("calculus:quad", &[ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_T()?])], &[])?, v_t0.clone(), v_t1.clone()], &[])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r12(mut v_v: Py) -> R {
    let mut v_w: Py = Py::None;
    v_w = py::float(&Py::str(&[py::format(&py::float(&v_v.clone())?, ".12g")?].concat()))?;
    return Ok(py::add(&v_w.clone(), &Py::Float(0.0))?);
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
    if ({ let t1659 = v_x.clone(); Py::Bool({ let t1660 = Py::Int(0i64); if py::eq(&t1659, &t1660) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)]));
    }
    v_lead = py::math1("exp", &py::sub(&py::add(&py::neg(&v_x.clone())?, &py::mul(&v_a.clone(), &py::math1("log", &v_x.clone())?)?)?, &py::math1("lgamma", &v_a.clone())?)?)?;
    if ({ let t1661 = v_x.clone(); Py::Bool({ let t1662 = py::add(&v_a.clone(), &Py::Int(1i64))?; if py::lt(&t1661, &t1662)? { true } else { false } }) }).truthy() {
        let t1663 = (Py::tuple(vec![v_a.clone(), py::truediv(&Py::Float(1.0), &v_a.clone())?])).items()?;
        if t1663.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ap = t1663[0].clone();
        v_d = t1663[1].clone();
        v_s = v_d.clone();
        for t1664 in py::range_iter(&[(Py::Int(100000i64)).i()?]) {
            v__ = t1664;
            v_ap = py::iadd(&v_ap, &Py::Int(1i64))?;
            v_d = py::mul(&v_d, &py::truediv(&v_x.clone(), &v_ap.clone())?)?;
            v_s = py::iadd(&v_s, &v_d.clone())?;
            if ({ let t1665 = py::abs(&v_d.clone())?; Py::Bool({ let t1666 = py::mul(&py::abs(&v_s.clone())?, &Py::Float(1e-17))?; if py::lt(&t1665, &t1666)? { true } else { false } }) }).truthy() {
                break;
            }
        }
        v_P = py::mul(&v_s.clone(), &v_lead.clone())?;
        return Ok(Py::tuple(vec![v_P.clone(), py::sub(&Py::Float(1.0), &v_P.clone())?]));
    }
    v_tiny = Py::Float(1e-300);
    v_b = py::sub(&py::add(&v_x.clone(), &Py::Int(1i64))?, &v_a.clone())?;
    let t1667 = (Py::tuple(vec![py::truediv(&Py::Int(1i64), &v_tiny.clone())?, py::truediv(&Py::Int(1i64), &v_b.clone())?])).items()?;
    if t1667.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t1667[0].clone();
    v_d = t1667[1].clone();
    v_h = v_d.clone();
    for t1668 in py::range_iter(&[(Py::Int(1i64)).i()?, (Py::Int(100000i64)).i()?]) {
        v_k = t1668;
        v_an = py::mul(&py::neg(&v_k.clone())?, &py::sub(&v_k.clone(), &v_a.clone())?)?;
        v_b = py::iadd(&v_b, &Py::Int(2i64))?;
        v_d = py::add(&py::mul(&v_an.clone(), &v_d.clone())?, &v_b.clone())?;
        v_d = (if ({ let t1669 = py::abs(&v_d.clone())?; Py::Bool({ let t1670 = v_tiny.clone(); if py::lt(&t1669, &t1670)? { true } else { false } }) }).truthy() { v_tiny.clone() } else { v_d.clone() });
        v_c = py::add(&v_b.clone(), &py::truediv(&v_an.clone(), &v_c.clone())?)?;
        v_c = (if ({ let t1671 = py::abs(&v_c.clone())?; Py::Bool({ let t1672 = v_tiny.clone(); if py::lt(&t1671, &t1672)? { true } else { false } }) }).truthy() { v_tiny.clone() } else { v_c.clone() });
        v_d = py::truediv(&Py::Int(1i64), &v_d.clone())?;
        v_de = py::mul(&v_d.clone(), &v_c.clone())?;
        v_h = py::mul(&v_h, &v_de.clone())?;
        if ({ let t1673 = py::abs(&py::sub(&v_de.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t1674 = Py::Float(3e-16); if py::lt(&t1673, &t1674)? { true } else { false } }) }).truthy() {
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
    v_scale = { let t1338 = py::sum(&{ let mut out = vec![]; for t1336 in py::iter(&(py::enumerate(&v_co.clone(), 0)?))? { let mut v_j: Py; let mut v_k: Py; 
let t1337 = (t1336).items()?;
if t1337.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t1337[0].clone();
v_k = t1337[1].clone();
out.push(py::mul(&py::abs(&py::float(&v_k.clone())?)?, &py::pow(&py::minmax(&[Py::Float(1.0), py::abs(&v_c.clone())?], true)?, &py::sub(&v_deg.clone(), &v_j.clone())?)?)?); } Py::list(out) }, None)?; if t1338.truthy() { t1338 } else { Py::Float(1.0) } };
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
    if ({ let t2048 = v_kind.clone(); Py::Bool({ let t2049 = Py::str("trapezoid"); if py::eq(&t2048, &t2049) { true } else { false } }) }).truthy() {
        let t2051 = (Py::tuple(vec![{ let mut out = vec![]; for t2050 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t2050;
out.push(py::add(&v_a.clone(), &py::mul(&v_k.clone(), &v_h.clone())?)?); } Py::list(out) }, py::add(&py::add(&Py::list(vec![py::truediv(&v_h.clone(), &Py::Int(2i64))?]), &py::mul(&Py::list(vec![v_h.clone()]), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?, &Py::list(vec![py::truediv(&v_h.clone(), &Py::Int(2i64))?]))?])).items()?;
        if t2051.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_pts = t2051[0].clone();
        v_w = t2051[1].clone();
    } else {
        if ({ let t2052 = v_kind.clone(); Py::Bool({ let t2053 = Py::str("simpson"); if py::eq(&t2052, &t2053) { true } else { false } }) }).truthy() {
            v_pts = { let mut out = vec![]; for t2054 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t2054;
out.push(py::add(&v_a.clone(), &py::mul(&v_k.clone(), &v_h.clone())?)?); } Py::list(out) };
            v_w = { let mut out = vec![]; for t2055 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t2055;
out.push(py::mul(&py::truediv(&v_h.clone(), &Py::Int(3i64))?, &(if ({ let t2056 = v_k.clone(); Py::Bool({ let t2057 = Py::tuple(vec![Py::Int(0i64), v_n.clone()]); if py::contains(&t2057, &t2056)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { (if (py::modulo(&v_k.clone(), &Py::Int(2i64))?).truthy() { Py::Int(4i64) } else { Py::Int(2i64) }) }))?); } Py::list(out) };
        } else {
            let t2059 = (Py::tuple(vec![{ let mut out = vec![]; for t2058 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t2058;
out.push(py::add(&v_a.clone(), &py::mul(&py::add(&v_k.clone(), &Py::Float(0.5))?, &v_h.clone())?)?); } Py::list(out) }, py::mul(&Py::list(vec![v_h.clone()]), &v_n.clone())?])).items()?;
            if t2059.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_pts = t2059[0].clone();
            v_w = t2059[1].clone();
        }
    }
    return Ok(Py::tuple(vec![v_pts.clone(), v_w.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__same(mut v_a: Py, mut v_b: Py) -> R {
    return Ok((if ({ let t1616 = v_a.clone(); Py::Bool({ let t1617 = v_b.clone(); if py::eq(&t1616, &t1617) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__scal_integrand(mut v_i: Py) -> R {
    let mut v_ef: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let mut v_ey: Py = Py::None;
    let t1273 = (f__path(py::getitem(&v_i.clone(), &Py::str("xt"))?, py::getitem(&v_i.clone(), &Py::str("yt"))?)?).items()?;
    if t1273.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ex = t1273[0].clone();
    v_ey = t1273[1].clone();
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
    for t2083 in py::iter(&(py::slice(&v_ns.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? {
        v_n = t2083;
        v_v = py::call(&v_fn.clone(), &[v_n.clone()])?;
        if ({ let t2084 = py::abs(&py::sub(&v_v.clone(), &v_prev.clone())?)?; Py::Bool({ let t2085 = py::mul(&Py::Float(1e-12), &v_S.clone())?; if py::le(&t2084, &t2085)? { true } else { false } }) }).truthy() {
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
    v_fl = { let mut out = vec![]; for t960 in py::iter(&(v_xs.clone()))? { let mut v_v: Py; 
v_v = t960;
out.push(py::float(&v_v.clone())?); } Py::list(out) };
    v_cuts = py::add(&py::add(&Py::list(vec![py::sub(&py::getitem(&v_fl.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?]), &{ let mut out = vec![]; for t961 in py::range_iter(&[(py::sub(&py::len(&v_fl.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t961;
out.push(py::truediv(&py::add(&py::getitem(&v_fl.clone(), &v_k.clone())?, &py::getitem(&v_fl.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?, &Py::Int(2i64))?); } Py::list(out) })?, &Py::list(vec![py::add(&py::getitem(&v_fl.clone(), &py::neg(&Py::Int(1i64))?)?, &Py::Int(1i64))?]))?;
    v_s = { let mut out = vec![]; for t962 in py::iter(&(v_cuts.clone()))? { let mut v_c: Py; 
v_c = t962;
out.push(py::int(&ax_engine::pysym::call("sign", &[py::method(&v_q.clone(), "eval", &[f__rat(v_c.clone())?])?], &[])?)?); } Py::list(out) };
    return Ok({ let mut out = vec![]; for t963 in py::range_iter(&[(py::len(&v_fl.clone())?).i()?]) { let mut v_k: Py; 
v_k = t963;
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
    v_xs = { let mut out = vec![]; for t1919 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1919;
out.push(py::add(&v_a.clone(), &py::truediv(&py::mul(&py::sub(&v_b.clone(), &v_a.clone())?, &v_k.clone())?, &v_n.clone())?)?); } Py::list(out) };
    v_ys = { let mut out = vec![]; for t1920 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t1920;
out.push(py::call(&v_g.clone(), &[v_x.clone()])?); } Py::list(out) };
    v_out = Py::list(vec![]);
    for t1921 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t1921;
        let t1922 = (Py::tuple(vec![py::getitem(&v_xs.clone(), &v_k.clone())?, py::getitem(&v_xs.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, py::getitem(&v_ys.clone(), &v_k.clone())?])).items()?;
        if t1922.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_lo = t1922[0].clone();
        v_hi = t1922[1].clone();
        v_flo = t1922[2].clone();
        if ({ let t1925 = { let t1923 = v_flo.clone(); Py::Bool({ let t1924 = Py::Int(0i64); if py::eq(&t1923, &t1924) { true } else { false } }) }; if !t1925.truthy() { t1925 } else { v_k.clone() } }).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[v_lo.clone()])?;
        } else {
            if ({ let t1926 = py::mul(&v_flo.clone(), &py::getitem(&v_ys.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t1927 = Py::Int(0i64); if py::lt(&t1926, &t1927)? { true } else { false } }) }).truthy() {
                for t1928 in py::range_iter(&[(Py::Int(80i64)).i()?]) {
                    v__ = t1928;
                    v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
                    v_fm = py::call(&v_g.clone(), &[v_mid.clone()])?;
                    if ({ let t1929 = v_fm.clone(); Py::Bool({ let t1930 = Py::Int(0i64); if py::eq(&t1929, &t1930) { true } else { false } }) }).truthy() {
                        let t1931 = v_mid.clone();
                        v_lo = t1931.clone();
                        v_hi = t1931.clone();
                        break;
                    }
                    if ({ let t1932 = { let t1934 = v_fm.clone(); Py::Bool({ let t1935 = Py::Int(0i64); if py::lt(&t1934, &t1935)? { true } else { false } }) }; Py::Bool({ let t1933 = { let t1936 = v_flo.clone(); Py::Bool({ let t1937 = Py::Int(0i64); if py::lt(&t1936, &t1937)? { true } else { false } }) }; if py::eq(&t1932, &t1933) { true } else { false } }) }).truthy() {
                        let t1938 = (Py::tuple(vec![v_mid.clone(), v_fm.clone()])).items()?;
                        if t1938.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                        v_lo = t1938[0].clone();
                        v_flo = t1938[1].clone();
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
fn f__snap_int(mut v_v: Py, mut v_S: Py) -> R {
    return Ok((if ({ let t1533 = py::abs(&v_v.clone())?; Py::Bool({ let t1534 = py::mul(&Py::Float(1e-12), &v_S.clone())?; if py::le(&t1533, &t1534)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { f__r12(v_v.clone())? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sympy_limit(mut v_e: Py, mut v_var: Py, mut v_point: Py, mut v_d: Py) -> R {
    let mut v_exc: Py = Py::None;
    let t1639: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            return Ok(Some((if ({ let t1637 = v_d.clone(); Py::Bool({ let t1638 = Py::None; if py::is(&t1637, &t1638) { true } else { false } }) }).truthy() { ax_engine::pysym::call("limit", &[v_e.clone(), v_var.clone(), v_point.clone()], &[])? } else { ax_engine::pysym::call("limit", &[v_e.clone(), v_var.clone(), v_point.clone(), v_d.clone()], &[])? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1639 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            v_exc = Py::str(&match &e { PyErr::Value(m) => m.clone(), other => format!("{:?}", other) });
            return Err(py::value_error(py::to_str(&{ let t1640 = py::getitem(&py::method(&Py::str(&py::to_str(&v_exc.clone())), "split", &[Py::str(".")])?, &Py::Int(0i64))?; if t1640.truthy() { t1640 } else { Py::str("the limit does not exist") } })));
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
fn f__to_rect(mut v_system: Py, mut v_p: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let t263 = (Py::tuple(vec![py::float(&py::getitem(&v_p.clone(), &Py::Int(0i64))?)?, py::float(&py::getitem(&v_p.clone(), &Py::Int(1i64))?)?, py::float(&py::getitem(&v_p.clone(), &Py::Int(2i64))?)?])).items()?;
    if t263.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t263[0].clone();
    v_b = t263[1].clone();
    v_c = t263[2].clone();
    if ({ let t264 = v_system.clone(); Py::Bool({ let t265 = Py::str("rectangular"); if py::eq(&t264, &t265) { true } else { false } }) }).truthy() {
        return Ok(Py::list(vec![v_a.clone(), v_b.clone(), v_c.clone()]));
    }
    if ({ let t266 = v_system.clone(); Py::Bool({ let t267 = Py::str("cylindrical"); if py::eq(&t266, &t267) { true } else { false } }) }).truthy() {
        if ({ let t268 = v_a.clone(); Py::Bool({ let t269 = Py::Int(0i64); if py::lt(&t268, &t269)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the radial distance r must be at least 0"))));
        }
        return Ok(Py::list(vec![py::mul(&v_a.clone(), &py::math1("cos", &v_b.clone())?)?, py::mul(&v_a.clone(), &py::math1("sin", &v_b.clone())?)?, v_c.clone()]));
    }
    if ({ let t270 = v_system.clone(); Py::Bool({ let t271 = Py::str("spherical"); if py::eq(&t270, &t271) { true } else { false } }) }).truthy() {
        if ({ let t272 = v_a.clone(); Py::Bool({ let t273 = Py::Int(0i64); if py::lt(&t272, &t273)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("rho must be at least 0"))));
        }
        if (py::not(&{ let t274 = Py::Int(0i64); Py::Bool({ let t275 = v_c.clone(); if py::le(&t274, &t275)? { { let t276 = py::add(&Py::Float(std::f64::consts::PI), &Py::Float(1e-12))?; if py::le(&t275, &t276)? { true } else { false } } } else { false } }) })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the polar angle phi must lie between 0 and pi (measured from the positive z-axis)"))));
        }
        return Ok(Py::list(vec![py::mul(&py::mul(&v_a.clone(), &py::math1("sin", &v_c.clone())?)?, &py::math1("cos", &v_b.clone())?)?, py::mul(&py::mul(&v_a.clone(), &py::math1("sin", &v_c.clone())?)?, &py::math1("sin", &v_b.clone())?)?, py::mul(&v_a.clone(), &py::math1("cos", &v_c.clone())?)?]));
    }
    return Err(py::value_error(py::to_str(&Py::str("the system must be rectangular, cylindrical or spherical"))));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__values(mut v_e: Py, mut v_pts: Py) -> R {
    let mut v_g: Py = Py::None;
    v_g = ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[])?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        return Ok({ let mut out = vec![]; for t2061 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t2061;
out.push(py::call(&v_g.clone(), &[ax_engine::pymp::call("mpf", &[v_p.clone()], &[])?])?); } Py::list(out) });
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vars(mut v_p: Py) -> R {
    if ({ let t1992 = py::len(&v_p.clone())?; Py::Bool({ let t1993 = Py::Int(2i64); if py::eq(&t1992, &t1993) { true } else { false } }) }).truthy() {
        return Ok(c__XY()?);
    }
    if ({ let t1994 = py::len(&v_p.clone())?; Py::Bool({ let t1995 = Py::Int(3i64); if py::eq(&t1994, &t1995) { true } else { false } }) }).truthy() {
        return Ok(c__XYZ()?);
    }
    return Err(py::value_error(py::to_str(&Py::str("the point needs 2 or 3 coordinates"))));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vec_err(mut v_got: Py, mut v_want: Py, mut v_slack: Py) -> R {
    let t2022 = (Py::tuple(vec![Py::list((v_got.clone()).items()?), Py::list((v_want.clone()).items()?)])).items()?;
    if t2022.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_got = t2022[0].clone();
    v_want = t2022[1].clone();
    if ({ let t2023 = py::len(&v_got.clone())?; Py::Bool({ let t2024 = py::len(&v_want.clone())?; if py::ne(&t2023, &t2024) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(f_worst({ let mut v = vec![]; v.extend(({ let mut out = vec![]; for t2025 in py::iter(&(py::zip(&[v_got.clone(), v_want.clone()])?))? { let mut v_g: Py; let mut v_w: Py; 
let t2026 = (t2025).items()?;
if t2026.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_g = t2026[0].clone();
v_w = t2026[1].clone();
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
    let t1289 = (f__path(py::getitem(&v_i.clone(), &Py::str("xt"))?, py::getitem(&v_i.clone(), &Py::str("yt"))?)?).items()?;
    if t1289.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ex = t1289[0].clone();
    v_ey = t1289[1].clone();
    let t1291 = ({ let mut out = vec![]; for t1290 in py::iter(&(Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("P"))?, py::getitem(&v_i.clone(), &Py::str("Q"))?])))? { let mut v_c: Py; 
v_c = t1290;
out.push(py::method(&ax_engine::pysym::helper("calculus:parse_expr", &[v_c.clone(), c__XY()?], &[])?, "subs", &[Py::dict(vec![(c_X()?, v_ex.clone()), (c_Y()?, v_ey.clone())])])?); } Py::list(out) }).items()?;
    if t1291.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_eP = t1291[0].clone();
    v_eQ = t1291[1].clone();
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
    v_co = { let mut out = vec![]; for t78 in py::iter(&(py::method(&py::sub(&ax_engine::pysym::helper("calculus:poly_of", &[v_f.clone()], &[])?, &ax_engine::pysym::helper("calculus:poly_of", &[v_g.clone()], &[])?)?, "all_coeffs", &[])?))? { let mut v_c: Py; 
v_c = t78;
out.push(py::truediv(&ax_engine::pymp::call("mpf", &[py::getattr(&v_c.clone(), "p")?], &[])?, &py::getattr(&v_c.clone(), "q")?)?); } Py::list(out) };
    let t88: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            {
                let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
                v_rs = (if ({ let t79 = py::len(&v_co.clone())?; Py::Bool({ let t80 = Py::Int(1i64); if py::gt(&t79, &t80)? { true } else { false } }) }).truthy() { ax_engine::pymp::call("polyroots", &[v_co.clone()], &[(String::from("maxsteps"), Py::Int(400i64)), (String::from("extraprec"), Py::Int(120i64))])? } else { Py::list(vec![]) });
            }
            v_cuts = py::sorted(&{ let mut out = vec![]; for t81 in py::iter(&(v_rs.clone()))? { let mut v_r: Py; 
v_r = t81;
if ({ let t87 = { let t82 = py::abs(&ax_engine::pymp::call("im", &[v_r.clone()], &[])?)?; Py::Bool({ let t83 = Py::Float(1e-20); if py::lt(&t82, &t83)? { true } else { false } }) }; if !t87.truthy() { t87 } else { { let t84 = v_a.clone(); Py::Bool({ let t85 = py::float(&ax_engine::pymp::call("re", &[v_r.clone()], &[])?)?; if py::lt(&t84, &t85)? { { let t86 = v_b.clone(); if py::lt(&t85, &t86)? { true } else { false } } } else { false } }) } } }).truthy() { out.push(py::float(&ax_engine::pymp::call("re", &[v_r.clone()], &[])?)?); }} Py::list(out) }, (Py::Bool(false)).truthy())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t88 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            v_cuts = { let mut out = vec![]; for t89 in py::iter(&(ax_engine::pysym::helper("calculus:real_roots_in", &[py::sub(&ax_engine::pysym::helper("calculus:poly_of", &[v_f.clone()], &[])?, &ax_engine::pysym::helper("calculus:poly_of", &[v_g.clone()], &[])?)?, v_a.clone(), v_b.clone()], &[])?))? { let mut v_r: Py; 
v_r = t89;
out.push(py::float(&v_r.clone())?); } Py::list(out) };
        }
        Err(e) => return Err(e),
    }
    v_knots = py::add(&py::add(&Py::list(vec![v_a.clone()]), &v_cuts.clone())?, &Py::list(vec![v_b.clone()]))?;
    v_h = Py::str(&["(".to_string(), py::format(&v_f.clone(), "")?, ") - (".to_string(), py::format(&v_g.clone(), "")?, ")".to_string()].concat());
    return Ok(py::sum(&{ let mut out = vec![]; for t90 in py::iter(&(py::zip(&[py::slice(&v_knots.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, py::slice(&v_knots.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_hi: Py; let mut v_lo: Py; 
let t91 = (t90).items()?;
if t91.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_lo = t91[0].clone();
v_hi = t91[1].clone();
out.push(py::abs(&f__gl_check(v_h.clone(), v_lo.clone(), v_hi.clone(), Py::tuple(vec![Py::str("x")]), Py::None)?)?); } Py::list(out) }, None)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_anti(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_G: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_want: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?;
    v_G = f__antiderivative(v_e.clone(), c_X()?)?;
    if ({ let t43 = py::getitem(&v_o.clone(), &Py::str("antiderivative"))?; Py::Bool({ let t44 = py::add(&ax_engine::pysym::helper("calculus:show", &[v_G.clone()], &[])?, &Py::str(" + C"))?; if py::ne(&t43, &t44) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_res = Py::list(vec![]);
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        let t45 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:_compile", &[v_G.clone(), Py::tuple(vec![c_X()?])], &[])?, ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[])?])).items()?;
        if t45.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_g = t45[0].clone();
        v_h = t45[1].clone();
        for t46 in py::iter(&(c__ANTI_PTS()?))? {
            v_p = t46;
            let t47: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                    v_d = ax_engine::pymp::call("diff", &[v_g.clone(), ax_engine::pymp::call("mpf", &[v_p.clone()], &[])?], &[])?;
                    v_want = py::call(&v_h.clone(), &[ax_engine::pymp::call("mpf", &[v_p.clone()], &[])?])?;
                    #[allow(unreachable_code)]
                    Ok(None)
                })();
            match t47 {
                Ok(Some(r)) => { return Ok(r); }
                Ok(None) => {}
                Err(e) if (matches!(e, PyErr::Value(_))) => {
                    continue;
                }
                Err(e) => return Err(e),
            }
            let _ = py::method(&v_res.clone(), "append", &[f_rel_err(py::float(&v_d.clone())?, py::float(&v_want.clone())?, Py::Float(1e-12))?])?;
        }
    }
    if ({ let t51 = py::len(&v_res.clone())?; Py::Bool({ let t52 = Py::Int(2i64); if py::lt(&t51, &t52)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    if ({ let t53 = py::method(&v_i.clone(), "get", &[Py::str("x0")])?; Py::Bool({ let t54 = Py::None; if !py::is(&t53, &t54) { true } else { false } }) }).truthy() {
        let _ = py::method(&v_res.clone(), "append", &[f_rel_err(py::getitem(&v_o.clone(), &Py::str("constant"))?, py::sub(&py::getitem(&v_i.clone(), &Py::str("F0"))?, &ax_engine::pysym::helper("calculus:at", &[v_G.clone(), Py::tuple(vec![c_X()?]), py::getitem(&v_i.clone(), &Py::str("x0"))?], &[])?)?, Py::Float(1e-12))?])?;
    }
    return Ok(f_worst({ let mut v = vec![]; v.extend((v_res.clone()).items()?); Py::tuple(v) })?);
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
    if ({ let t344 = { let t331 = py::len(&v_cs.clone())?; Py::Bool({ let t332 = f__n_roots_open(v_dp.clone(), Py::None, Py::None)?; if py::ne(&t331, &t332) { true } else { false } }) }; if t344.truthy() { t344 } else { { let t342 = v_cs.clone(); Py::Bool({ let t343 = py::sorted(&v_cs.clone(), (Py::Bool(false)).truthy())?; if py::ne(&t342, &t343) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    let t345 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t345.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t345[0].clone();
    v_mn = t345[1].clone();
    for t346 in py::iter(&(py::zip(&[v_cs.clone(), (if (v_cs.clone()).truthy() { f__side_signs(v_dp.clone(), v_cs.clone())? } else { Py::list(vec![]) })])?))? {
        let t353 = (t346).items()?;
        if t353.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t353[0].clone();
        let t354 = (t353[1].clone()).items()?;
        if t354.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_left = t354[0].clone();
        v_right = t354[1].clone();
        let _ = py::method(&(if ({ let t347 = v_left.clone(); Py::Bool({ let t348 = Py::Int(0i64); if py::gt(&t347, &t348)? { { let t349 = v_right.clone(); if py::gt(&t348, &t349)? { true } else { false } } } else { false } }) }).truthy() { v_mx.clone() } else { (if ({ let t350 = v_left.clone(); Py::Bool({ let t351 = Py::Int(0i64); if py::lt(&t350, &t351)? { { let t352 = v_right.clone(); if py::lt(&t351, &t352)? { true } else { false } } } else { false } }) }).truthy() { v_mn.clone() } else { Py::list(vec![]) }) }), "append", &[v_c.clone()])?;
    }
    return Ok({ let mut v: Vec<Py> = vec![]; v.extend(({ let mut out = vec![]; for t355 in py::iter(&(v_cs.clone()))? { let mut v_c: Py; 
v_c = t355;
out.push(f__root_res(v_dp.clone(), v_c.clone())?); } Py::list(out) }).items()?); v.push(f__vec_err(py::getitem(&v_o.clone(), &Py::str("local_maxima"))?, v_mx.clone(), Py::Int(0i64))?); v.push(f__vec_err(py::getitem(&v_o.clone(), &Py::str("local_minima"))?, v_mn.clone(), Py::Int(0i64))?); let mut d: Vec<(Py, Py)> = vec![];   if !d.is_empty() { return Err(PyErr::Type("worst() got an unexpected keyword argument".into())) } f_worst(Py::tuple(v.get(0..).map(|s| s.to_vec()).unwrap_or_default()))? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_cons(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_Phi: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_dPy: Py = Py::None;
    let mut v_dPz: Py = Py::None;
    let mut v_dQx: Py = Py::None;
    let mut v_dQz: Py = Py::None;
    let mut v_dRx: Py = Py::None;
    let mut v_dRy: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_orders: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_ptext: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_text: Py = Py::None;
    let mut v_texts: Py = Py::None;
    v_R = py::method(&v_i.clone(), "get", &[Py::str("R")])?;
    let t237 = (f__pfield(py::getitem(&v_i.clone(), &Py::str("P"))?, py::getitem(&v_i.clone(), &Py::str("Q"))?, v_R.clone())?).items()?;
    if t237.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t237[0].clone();
    v_names = t237[1].clone();
    v_k = py::len(&v_names.clone())?;
    v_texts = Py::list(vec![py::getitem(&v_i.clone(), &Py::str("P"))?, py::getitem(&v_i.clone(), &Py::str("Q"))?]);
    if ({ let t238 = v_R.clone(); Py::Bool({ let t239 = Py::None; if !py::is(&t238, &t239) { true } else { false } }) }).truthy() {
        let _ = py::method(&v_texts.clone(), "append", &[v_R.clone()])?;
    }
    if ({ let t240 = py::getitem(&v_o.clone(), &Py::str("conservative"))?; Py::Bool({ let t241 = Py::str("no"); if py::eq(&t240, &t241) { true } else { false } }) }).truthy() {
        v_best = Py::Float(0.0);
        for t242 in py::iter(&(c__CPTS()?))? {
            v_p = t242;
            v_q = Py::list((py::slice(&v_p.clone(), None, Some((v_k.clone()).i()?), None)?).items()?);
            v_dQx = py::float(&f__mp_partial(py::getitem(&v_texts.clone(), &Py::Int(1i64))?, v_names.clone(), v_q.clone(), py::slice(&Py::list(vec![Py::Int(1i64), Py::Int(0i64), Py::Int(0i64)]), None, Some((v_k.clone()).i()?), None)?)?)?;
            v_dPy = py::float(&f__mp_partial(py::getitem(&v_texts.clone(), &Py::Int(0i64))?, v_names.clone(), v_q.clone(), py::slice(&Py::list(vec![Py::Int(0i64), Py::Int(1i64), Py::Int(0i64)]), None, Some((v_k.clone()).i()?), None)?)?)?;
            v_best = py::minmax(&[v_best.clone(), py::abs(&py::sub(&v_dQx.clone(), &v_dPy.clone())?)?], true)?;
            if ({ let t244 = v_k.clone(); Py::Bool({ let t245 = Py::Int(3i64); if py::eq(&t244, &t245) { true } else { false } }) }).truthy() {
                v_dRy = py::float(&f__mp_partial(py::getitem(&v_texts.clone(), &Py::Int(2i64))?, v_names.clone(), v_q.clone(), Py::list(vec![Py::Int(0i64), Py::Int(1i64), Py::Int(0i64)]))?)?;
                v_dQz = py::float(&f__mp_partial(py::getitem(&v_texts.clone(), &Py::Int(1i64))?, v_names.clone(), v_q.clone(), Py::list(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(1i64)]))?)?;
                v_dPz = py::float(&f__mp_partial(py::getitem(&v_texts.clone(), &Py::Int(0i64))?, v_names.clone(), v_q.clone(), Py::list(vec![Py::Int(0i64), Py::Int(0i64), Py::Int(1i64)]))?)?;
                v_dRx = py::float(&f__mp_partial(py::getitem(&v_texts.clone(), &Py::Int(2i64))?, v_names.clone(), v_q.clone(), Py::list(vec![Py::Int(1i64), Py::Int(0i64), Py::Int(0i64)]))?)?;
                v_best = py::minmax(&[v_best.clone(), py::abs(&py::sub(&v_dRy.clone(), &v_dQz.clone())?)?, py::abs(&py::sub(&v_dPz.clone(), &v_dRx.clone())?)?], true)?;
            }
        }
        return Ok((if ({ let t246 = v_best.clone(); Py::Bool({ let t247 = Py::Float(1e-06); if py::gt(&t246, &t247)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    v_text = py::getitem(&v_o.clone(), &Py::str("potential"))?;
    if ({ let t250 = { let t248 = py::getitem(&v_o.clone(), &Py::str("conservative"))?; Py::Bool({ let t249 = Py::str("yes"); if py::ne(&t248, &t249) { true } else { false } }) }; if t250.truthy() { t250 } else { py::not(&py::method(&v_text.clone(), "endswith", &[Py::str(" + C")])?) } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_ptext = py::slice(&v_text.clone(), None, Some((py::neg(&Py::Int(4i64))?).i()?), None)?;
    v_Phi = ax_engine::pysym::helper("calculus:parse_expr", &[v_ptext.clone(), v_names.clone()], &[])?;
    v_res = Py::list(vec![]);
    for t251 in py::iter(&(c__CPTS()?))? {
        v_p = t251;
        v_q = Py::list((py::slice(&v_p.clone(), None, Some((v_k.clone()).i()?), None)?).items()?);
        for t252 in py::range_iter(&[(v_k.clone()).i()?]) {
            v_j = t252;
            v_orders = py::mul(&Py::list(vec![Py::Int(0i64)]), &v_k.clone())?;
            py::setitem(&v_orders.clone(), &v_j.clone(), Py::Int(1i64))?;
            let _ = py::method(&v_res.clone(), "append", &[f_rel_err(py::float(&f__mp_partial(v_ptext.clone(), v_names.clone(), v_q.clone(), v_orders.clone())?)?, { let mut v: Vec<Py> = vec![]; v.push(py::getitem(&v_F.clone(), &v_j.clone())?); v.push(v_names.clone()); v.extend((v_q.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? }, Py::Float(1e-10))?])?;
        }
    }
    if ({ let t256 = Py::str("line_integral"); Py::Bool({ let t257 = v_o.clone(); if py::contains(&t257, &t256)? { true } else { false } }) }).truthy() {
        let _ = py::method(&v_res.clone(), "append", &[f_rel_err(py::getitem(&v_o.clone(), &Py::str("line_integral"))?, py::sub(&{ let mut v: Vec<Py> = vec![]; v.push(v_Phi.clone()); v.push(v_names.clone()); v.extend((py::getitem(&v_i.clone(), &Py::str("B"))?).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? }, &{ let mut v: Vec<Py> = vec![]; v.push(v_Phi.clone()); v.push(v_names.clone()); v.extend((py::getitem(&v_i.clone(), &Py::str("A"))?).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? })?, Py::Float(1e-12))?])?;
    }
    return Ok(f_worst({ let mut v = vec![]; v.extend((v_res.clone()).items()?); Py::tuple(v) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_coords(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_bad: Py = Py::None;
    let mut v_pairs: Py = Py::None;
    let mut v_ph: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_src: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_th2: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    let mut v_zc: Py = Py::None;
    let t290 = (py::getitem(&v_o.clone(), &Py::str("rectangular"))?).items()?;
    if t290.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t290[0].clone();
    v_y = t290[1].clone();
    v_z = t290[2].clone();
    let t291 = (py::getitem(&v_o.clone(), &Py::str("cylindrical"))?).items()?;
    if t291.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_r = t291[0].clone();
    v_th = t291[1].clone();
    v_zc = t291[2].clone();
    let t292 = (py::getitem(&v_o.clone(), &Py::str("spherical"))?).items()?;
    if t292.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_rho = t292[0].clone();
    v_th2 = t292[1].clone();
    v_ph = t292[2].clone();
    v_S = py::minmax(&[Py::Float(1.0), py::abs(&v_x.clone())?, py::abs(&v_y.clone())?, py::abs(&v_z.clone())?], true)?;
    v_src = f__to_rect(py::getitem(&v_i.clone(), &Py::str("system"))?, py::getitem(&v_i.clone(), &Py::str("p"))?)?;
    v_bad = Py::Float(0.0);
    if ({ let t305 = { let t293 = v_r.clone(); Py::Bool({ let t294 = Py::Int(0i64); if py::lt(&t293, &t294)? { true } else { false } }) }; if t305.truthy() { t305 } else { { let t304 = { let t295 = v_rho.clone(); Py::Bool({ let t296 = Py::Int(0i64); if py::lt(&t295, &t296)? { true } else { false } }) }; if t304.truthy() { t304 } else { { let t303 = py::not(&{ let t297 = Py::Int(0i64); Py::Bool({ let t298 = v_th.clone(); if py::le(&t297, &t298)? { { let t299 = py::add(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &Py::Float(1e-09))?; if py::lt(&t298, &t299)? { true } else { false } } } else { false } }) }); if t303.truthy() { t303 } else { py::not(&{ let t300 = Py::Int(0i64); Py::Bool({ let t301 = v_ph.clone(); if py::le(&t300, &t301)? { { let t302 = py::add(&Py::Float(std::f64::consts::PI), &Py::Float(1e-09))?; if py::le(&t301, &t302)? { true } else { false } } } else { false } }) }) } } } } } }).truthy() {
        v_bad = Py::Float(1.0);
    }
    v_res = Py::list(vec![v_bad.clone(), f__same(v_th.clone(), v_th2.clone())?, py::truediv(&py::abs(&py::sub(&v_zc.clone(), &v_z.clone())?)?, &v_S.clone())?]);
    v_pairs = Py::tuple(vec![Py::tuple(vec![py::mul(&v_r.clone(), &py::math1("cos", &v_th.clone())?)?, v_x.clone()]), Py::tuple(vec![py::mul(&v_r.clone(), &py::math1("sin", &v_th.clone())?)?, v_y.clone()]), Py::tuple(vec![py::mul(&py::mul(&v_rho.clone(), &py::math1("sin", &v_ph.clone())?)?, &py::math1("cos", &v_th.clone())?)?, v_x.clone()]), Py::tuple(vec![py::mul(&py::mul(&v_rho.clone(), &py::math1("sin", &v_ph.clone())?)?, &py::math1("sin", &v_th.clone())?)?, v_y.clone()]), Py::tuple(vec![py::mul(&v_rho.clone(), &py::math1("cos", &v_ph.clone())?)?, v_z.clone()]), Py::tuple(vec![py::getitem(&v_src.clone(), &Py::Int(0i64))?, v_x.clone()]), Py::tuple(vec![py::getitem(&v_src.clone(), &Py::Int(1i64))?, v_y.clone()]), Py::tuple(vec![py::getitem(&v_src.clone(), &Py::Int(2i64))?, v_z.clone()])]);
    for t308 in py::iter(&(v_pairs.clone()))? {
        let t309 = (t308).items()?;
        if t309.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t309[0].clone();
        v_b = t309[1].clone();
        let _ = py::method(&v_res.clone(), "append", &[py::minmax(&[Py::Float(0.0), py::sub(&py::truediv(&py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?, &v_S.clone())?, &Py::Float(1e-10))?], true)?])?;
    }
    return Ok(f_worst({ let mut v = vec![]; v.extend((v_res.clone()).items()?); Py::tuple(v) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_curv3(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_cr: Py = Py::None;
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_texts: Py = Py::None;
    v_texts = Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("xt"))?, py::getitem(&v_i.clone(), &Py::str("yt"))?, py::getitem(&v_i.clone(), &Py::str("zt"))?]);
    v_d1 = { let mut out = vec![]; for t1844 in py::iter(&(v_texts.clone()))? { let mut v_c: Py; 
v_c = t1844;
out.push(py::float(&ax_engine::pysym::helper("calculus:_mp_diff", &[v_c.clone(), py::getitem(&v_i.clone(), &Py::str("t0"))?, Py::Int(1i64), Py::tuple(vec![Py::str("t")])], &[])?)?); } Py::list(out) };
    v_d2 = { let mut out = vec![]; for t1845 in py::iter(&(v_texts.clone()))? { let mut v_c: Py; 
v_c = t1845;
out.push(py::float(&ax_engine::pysym::helper("calculus:_mp_diff", &[v_c.clone(), py::getitem(&v_i.clone(), &Py::str("t0"))?, Py::Int(2i64), Py::tuple(vec![Py::str("t")])], &[])?)?); } Py::list(out) };
    v_s = py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t1846 in py::iter(&(v_d1.clone()))? { let mut v_v: Py; 
v_v = t1846;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?)?;
    v_cr = Py::list(vec![py::sub(&py::mul(&py::getitem(&v_d1.clone(), &Py::Int(1i64))?, &py::getitem(&v_d2.clone(), &Py::Int(2i64))?)?, &py::mul(&py::getitem(&v_d1.clone(), &Py::Int(2i64))?, &py::getitem(&v_d2.clone(), &Py::Int(1i64))?)?)?, py::sub(&py::mul(&py::getitem(&v_d1.clone(), &Py::Int(2i64))?, &py::getitem(&v_d2.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&v_d1.clone(), &Py::Int(0i64))?, &py::getitem(&v_d2.clone(), &Py::Int(2i64))?)?)?, py::sub(&py::mul(&py::getitem(&v_d1.clone(), &Py::Int(0i64))?, &py::getitem(&v_d2.clone(), &Py::Int(1i64))?)?, &py::mul(&py::getitem(&v_d1.clone(), &Py::Int(1i64))?, &py::getitem(&v_d2.clone(), &Py::Int(0i64))?)?)?]);
    v_k = py::truediv(&py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t1847 in py::iter(&(v_cr.clone()))? { let mut v_v: Py; 
v_v = t1847;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?)?, &py::pow(&v_s.clone(), &Py::Int(3i64))?)?;
    v_res = Py::list(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("curvature"))?, v_k.clone(), Py::Float(1e-09))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("speed"))?, v_s.clone(), Py::Float(1e-11))?, f__vec_err(py::getitem(&v_o.clone(), &Py::str("unit_tangent"))?, { let mut out = vec![]; for t1856 in py::iter(&(v_d1.clone()))? { let mut v_v: Py; 
v_v = t1856;
out.push(py::truediv(&v_v.clone(), &v_s.clone())?); } Py::list(out) }, Py::Float(1e-11))?]);
    if ({ let t1857 = Py::str("radius_of_curvature"); Py::Bool({ let t1858 = v_o.clone(); if py::contains(&t1858, &t1857)? { true } else { false } }) }).truthy() {
        let _ = py::method(&v_res.clone(), "append", &[f_rel_err(py::mul(&py::getitem(&v_o.clone(), &Py::str("radius_of_curvature"))?, &v_k.clone())?, Py::Float(1.0), Py::Float(1e-09))?])?;
    }
    return Ok(f_worst({ let mut v = vec![]; v.extend((v_res.clone()).items()?); Py::tuple(v) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_curvi(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_bx: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_u1: Py = Py::None;
    let mut v_u2: Py = Py::None;
    let mut v_vol: Py = Py::None;
    let mut v_w1: Py = Py::None;
    let mut v_w2: Py = Py::None;
    v_h = f__curvi_integrand(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("system"))?, Py::Bool(true))?;
    v_bx = Py::tuple(vec![Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("u1"))?, py::getitem(&v_i.clone(), &Py::str("u2"))?]), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("th1"))?, py::getitem(&v_i.clone(), &Py::str("th2"))?]), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("w1"))?, py::getitem(&v_i.clone(), &Py::str("w2"))?])]);
    let t463 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("u1"))?, py::getitem(&v_i.clone(), &Py::str("u2"))?, py::sub(&py::getitem(&v_i.clone(), &Py::str("th2"))?, &py::getitem(&v_i.clone(), &Py::str("th1"))?)?, py::getitem(&v_i.clone(), &Py::str("w1"))?, py::getitem(&v_i.clone(), &Py::str("w2"))?])).items()?;
    if t463.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_u1 = t463[0].clone();
    v_u2 = t463[1].clone();
    v_th = t463[2].clone();
    v_w1 = t463[3].clone();
    v_w2 = t463[4].clone();
    v_vol = (if ({ let t464 = py::getitem(&v_i.clone(), &Py::str("system"))?; Py::Bool({ let t465 = Py::str("cylindrical"); if py::eq(&t464, &t465) { true } else { false } }) }).truthy() { py::mul(&py::mul(&py::truediv(&py::sub(&py::mul(&v_u2.clone(), &v_u2.clone())?, &py::mul(&v_u1.clone(), &v_u1.clone())?)?, &Py::Int(2i64))?, &v_th.clone())?, &py::sub(&v_w2.clone(), &v_w1.clone())?)? } else { py::mul(&py::mul(&py::truediv(&py::sub(&py::pow(&v_u2.clone(), &Py::Int(3i64))?, &py::pow(&v_u1.clone(), &Py::Int(3i64))?)?, &Py::Int(3i64))?, &v_th.clone())?, &py::sub(&py::math1("cos", &v_w1.clone())?, &py::math1("cos", &v_w2.clone())?)?)? });
    return Ok(f_worst(Py::tuple(vec![f__scaled_err(py::getitem(&v_o.clone(), &Py::str("integral"))?, f__gl3(v_h.clone(), v_bx.clone(), Py::Int(16i64))?, f__gl3({ let v_bx = v_bx.clone(); let v_h = v_h.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_th = v_th.clone(); let v_u1 = v_u1.clone(); let v_u2 = v_u2.clone(); let v_vol = v_vol.clone(); let v_w1 = v_w1.clone(); let v_w2 = v_w2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_t: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_w: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::call(&v_h.clone(), &[v_u.clone(), v_t.clone(), v_w.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_bx.clone(), Py::Int(16i64))?, Py::Float(1e-10))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("region_volume"))?, v_vol.clone(), Py::Float(1e-11))?]))?);
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
    let t640 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("c"))?, py::getitem(&v_i.clone(), &Py::str("d"))?])).items()?;
    if t640.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t640[0].clone();
    v_d = t640[1].clone();
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
    let t680 = (Py::tuple(vec![f__fl(ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("g1"))?], &[])?, Py::tuple(vec![Py::str("x")]))?, f__fl(ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("g2"))?], &[])?, Py::tuple(vec![Py::str("x")]))?])).items()?;
    if t680.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t680[0].clone();
    v_hi = t680[1].clone();
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
    let t504 = (Py::tuple(vec![f__vars(py::getitem(&v_i.clone(), &Py::str("p"))?)?, py::getitem(&v_i.clone(), &Py::str("u"))?])).items()?;
    if t504.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_names = t504[0].clone();
    v_u = t504[1].clone();
    v_nu = py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t505 in py::iter(&(v_u.clone()))? { let mut v_c: Py; 
v_c = t505;
out.push(py::mul(&v_c.clone(), &v_c.clone())?); } Py::list(out) })?)?;
    v_g = { let mut out = vec![]; for t506 in py::range_iter(&[(py::len(&v_names.clone())?).i()?]) { let mut v_j: Py; 
v_j = t506;
out.push(py::float(&f__mp_partial(py::getitem(&v_i.clone(), &Py::str("f"))?, v_names.clone(), py::getitem(&v_i.clone(), &Py::str("p"))?, { let mut out = vec![]; for t508 in py::range_iter(&[(py::len(&v_names.clone())?).i()?]) { let mut v_k: Py; 
v_k = t508;
out.push(py::int(&{ let t509 = v_k.clone(); Py::Bool({ let t510 = v_j.clone(); if py::eq(&t509, &t510) { true } else { false } }) })?); } Py::list(out) })?)?); } Py::list(out) };
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("directional_derivative"))?, py::truediv(&py::fsum(&{ let mut out = vec![]; for t514 in py::iter(&(py::zip(&[v_g.clone(), v_u.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t515 = (t514).items()?;
if t515.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t515[0].clone();
v_b = t515[1].clone();
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
    let t601 = (f__field(py::getitem(&v_i.clone(), &Py::str("P"))?, py::getitem(&v_i.clone(), &Py::str("Q"))?, py::getitem(&v_i.clone(), &Py::str("R"))?)?).items()?;
    if t601.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t602 = (t601[0].clone()).items()?;
    if t602.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_eP = t602[0].clone();
    v_eQ = t602[1].clone();
    v_eR = t602[2].clone();
    v__n = t601[1].clone();
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
    let t699 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:poly_of", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::list((py::getitem(&v_o.clone(), &Py::str("critical_points"))?).items()?)])).items()?;
    if t699.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_p = t699[0].clone();
    v_a = t699[1].clone();
    v_b = t699[2].clone();
    v_cs = t699[3].clone();
    v_dp = py::method(&v_p.clone(), "diff", &[c_X()?])?;
    v_n = (if ({ let t700 = py::method(&v_p.clone(), "degree", &[])?; Py::Bool({ let t701 = Py::Int(2i64); if py::ge(&t700, &t701)? { true } else { false } }) }).truthy() { f__n_roots_open(v_dp.clone(), v_a.clone(), v_b.clone())? } else { Py::Int(0i64) });
    if ({ let t720 = { let t711 = py::len(&v_cs.clone())?; Py::Bool({ let t712 = v_n.clone(); if py::ne(&t711, &t712) { true } else { false } }) }; if t720.truthy() { t720 } else { { let t719 = { let t713 = v_cs.clone(); Py::Bool({ let t714 = py::sorted(&v_cs.clone(), (Py::Bool(false)).truthy())?; if py::ne(&t713, &t714) { true } else { false } }) }; if t719.truthy() { t719 } else { Py::Bool(({ let mut out = vec![]; for t715 in py::iter(&(v_cs.clone()))? { let mut v_c: Py; 
v_c = t715;
out.push(py::not(&{ let t716 = v_a.clone(); Py::Bool({ let t717 = v_c.clone(); if py::lt(&t716, &t717)? { { let t718 = v_b.clone(); if py::lt(&t717, &t718)? { true } else { false } } } else { false } }) })); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_vals = { let mut out = vec![]; for t721 in py::iter(&(py::add(&Py::list(vec![v_a.clone(), v_b.clone()]), &v_cs.clone())?))? { let mut v_v: Py; 
v_v = t721;
out.push(f__peval(v_p.clone(), v_v.clone())?); } Py::list(out) };
    return Ok({ let mut v: Vec<Py> = vec![]; v.extend(({ let mut out = vec![]; for t722 in py::iter(&(v_cs.clone()))? { let mut v_c: Py; 
v_c = t722;
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
    let t739 = (f__fourier_gl(py::getitem(&v_i.clone(), &Py::str("f"))?, py::float(&py::getitem(&v_i.clone(), &Py::str("L"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?)?).items()?;
    if t739.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a0 = t739[0].clone();
    v_ak = t739[1].clone();
    v_bk = t739[2].clone();
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
        let t748 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?, Py::tuple(vec![Py::str("t")])], &[])?, Py::tuple(vec![c_T()?])], &[])?, ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("g"))?], &[])?, Py::tuple(vec![c_X()?])], &[])?])).items()?;
        if t748.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_F = t748[0].clone();
        v_G = t748[1].clone();
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
    v_g = { let mut out = vec![]; for t835 in py::range_iter(&[(py::len(&v_names.clone())?).i()?]) { let mut v_j: Py; 
v_j = t835;
out.push(py::float(&f__mp_partial(py::getitem(&v_i.clone(), &Py::str("f"))?, v_names.clone(), py::getitem(&v_i.clone(), &Py::str("p"))?, { let mut out = vec![]; for t837 in py::range_iter(&[(py::len(&v_names.clone())?).i()?]) { let mut v_k: Py; 
v_k = t837;
out.push(py::int(&{ let t838 = v_k.clone(); Py::Bool({ let t839 = v_j.clone(); if py::eq(&t838, &t839) { true } else { false } }) })?); } Py::list(out) })?)?); } Py::list(out) };
    return Ok(f_worst(Py::tuple(vec![f__vec_err(py::getitem(&v_o.clone(), &Py::str("gradient"))?, v_g.clone(), Py::Float(1e-11))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("magnitude"))?, py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t848 in py::iter(&(v_g.clone()))? { let mut v_c: Py; 
v_c = t848;
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
    let t893 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("P"))?, c__XY()?], &[])?, ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("Q"))?, c__XY()?], &[])?])).items()?;
    if t893.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_eP = t893[0].clone();
    v_eQ = t893[1].clone();
    v_g = f__fl(py::sub(&ax_engine::pysym::call("diff", &[v_eQ.clone(), c_X()?], &[])?, &ax_engine::pysym::call("diff", &[v_eP.clone(), c_Y()?], &[])?)?, c__XY()?)?;
    let t894 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("c"))?, py::getitem(&v_i.clone(), &Py::str("d"))?])).items()?;
    if t894.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t894[0].clone();
    v_d = t894[1].clone();
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
    v_h = { let mut out = vec![]; for t912 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::Int(2i64), Py::Int(0i64)]), Py::tuple(vec![Py::Int(1i64), Py::Int(1i64)]), Py::tuple(vec![Py::Int(0i64), Py::Int(2i64)])])))? { let mut v_v: Py; 
v_v = t912;
out.push(py::float(&f__mp_partial(py::getitem(&v_i.clone(), &Py::str("f"))?, c__XY()?, py::getitem(&v_i.clone(), &Py::str("p"))?, v_v.clone())?)?); } Py::list(out) };
    v_D = py::sub(&py::mul(&py::getitem(&v_h.clone(), &Py::Int(0i64))?, &py::getitem(&v_h.clone(), &Py::Int(2i64))?)?, &py::pow(&py::getitem(&v_h.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?;
    v_kind = (if ({ let t914 = py::getitem(&v_o.clone(), &Py::str("D"))?; Py::Bool({ let t915 = Py::Int(0i64); if py::lt(&t914, &t915)? { true } else { false } }) }).truthy() { Py::str("saddle point") } else { (if ({ let t916 = py::getitem(&v_o.clone(), &Py::str("fxx"))?; Py::Bool({ let t917 = Py::Int(0i64); if py::gt(&t916, &t917)? { true } else { false } }) }).truthy() { Py::str("local minimum") } else { Py::str("local maximum") }) });
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
    let t940 = (Py::tuple(vec![f__mp_partial(v_t.clone(), c__XY()?, py::getitem(&v_i.clone(), &Py::str("p"))?, Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)]))?, f__mp_partial(v_t.clone(), c__XY()?, py::getitem(&v_i.clone(), &Py::str("p"))?, Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)]))?])).items()?;
    if t940.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fx = t940[0].clone();
    v_fy = t940[1].clone();
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
    if ({ let t972 = py::getattr(&v_d2.clone(), "is_zero")?; if t972.truthy() { t972 } else { { let t970 = py::method(&v_d2.clone(), "degree", &[])?; Py::Bool({ let t971 = Py::Int(1i64); if py::lt(&t970, &t971)? { true } else { false } }) } } }).truthy() {
        return Ok((if (py::not(&v_pts.clone())).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    v_odd = ax_engine::pysym::call("prod", &[{ let mut out = vec![]; for t973 in py::iter(&(py::getitem(&ax_engine::pysym::call("sqf_list", &[py::method(&v_d2.clone(), "as_expr", &[])?], &[])?, &Py::Int(1i64))?))? { let mut v_fac: Py; let mut v_m: Py; 
let t974 = (t973).items()?;
if t974.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_fac = t974[0].clone();
v_m = t974[1].clone();
if ({ let t975 = py::modulo(&v_m.clone(), &Py::Int(2i64))?; Py::Bool({ let t976 = Py::Int(1i64); if py::eq(&t975, &t976) { true } else { false } }) }).truthy() { out.push(v_fac.clone()); }} Py::list(out) }], &[])?;
    v_q = ax_engine::pysym::call("Poly", &[v_odd.clone(), c_X()?], &[])?;
    if ({ let t992 = { let t977 = py::len(&v_pts.clone())?; Py::Bool({ let t978 = (if ({ let t979 = py::method(&v_q.clone(), "degree", &[])?; Py::Bool({ let t980 = Py::Int(1i64); if py::ge(&t979, &t980)? { true } else { false } }) }).truthy() { f__n_roots_open(v_q.clone(), Py::None, Py::None)? } else { Py::Int(0i64) }); if py::ne(&t977, &t978) { true } else { false } }) }; if t992.truthy() { t992 } else { { let t990 = v_pts.clone(); Py::Bool({ let t991 = py::sorted(&v_pts.clone(), (Py::Bool(false)).truthy())?; if py::ne(&t990, &t991) { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(f_worst({ let mut v = vec![]; v.extend(({ let mut out = vec![]; for t993 in py::iter(&(v_pts.clone()))? { let mut v_c: Py; 
v_c = t993;
out.push(f__root_res(v_q.clone(), v_c.clone())?); } Py::list(out) }).items()?); Py::tuple(v) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_lagrange(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_G: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_fl: Py = Py::None;
    let mut v_fx: Py = Py::None;
    let mut v_fy: Py = Py::None;
    let mut v_gx: Py = Py::None;
    let mut v_gy: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pt: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?, c__XY()?], &[])?;
    let t1139 = (f__conic(py::getitem(&v_i.clone(), &Py::str("g"))?)?).items()?;
    if t1139.len() != 7 { return Err(PyErr::Value("unpack".into())); }
    v_G = t1139[0].clone();
    v_A = t1139[1].clone();
    v_B = t1139[2].clone();
    v_C = t1139[3].clone();
    v_D = t1139[4].clone();
    v_E = t1139[5].clone();
    v_F = t1139[6].clone();
    v_fl = f__fl(v_e.clone(), c__XY()?)?;
    let t1140 = (Py::tuple(vec![Py::list(vec![]), Py::Int(0i64)])).items()?;
    if t1140.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_res = t1140[0].clone();
    v_n = t1140[1].clone();
    for t1141 in py::iter(&(Py::tuple(vec![Py::str("max"), Py::str("min")])))? {
        v_key = t1141;
        if ({ let t1142 = py::add(&v_key.clone(), &Py::str("_value"))?; Py::Bool({ let t1143 = v_o.clone(); if !py::contains(&t1143, &t1142)? { true } else { false } }) }).truthy() {
            continue;
        }
        v_n = py::iadd(&v_n, &Py::Int(1i64))?;
        v_pt = py::getitem(&v_o.clone(), &py::add(&v_key.clone(), &Py::str("_point"))?)?;
        let t1144 = (Py::tuple(vec![py::getitem(&v_pt.clone(), &Py::Int(0i64))?, py::getitem(&v_pt.clone(), &Py::Int(1i64))?, py::getitem(&v_o.clone(), &py::add(&v_key.clone(), &Py::str("_lambda"))?)?])).items()?;
        if t1144.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_x = t1144[0].clone();
        v_y = t1144[1].clone();
        v_lam = t1144[2].clone();
        let t1145 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?, c__XY()?, v_x.clone(), v_y.clone()], &[])?, ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_Y()?], &[])?, c__XY()?, v_x.clone(), v_y.clone()], &[])?])).items()?;
        if t1145.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_fx = t1145[0].clone();
        v_fy = t1145[1].clone();
        let t1146 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_G.clone(), c_X()?], &[])?, c__XY()?, v_x.clone(), v_y.clone()], &[])?, ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_G.clone(), c_Y()?], &[])?, c__XY()?, v_x.clone(), v_y.clone()], &[])?])).items()?;
        if t1146.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_gx = t1146[0].clone();
        v_gy = t1146[1].clone();
        v_S = py::minmax(&[Py::Float(1.0), py::abs(&v_fx.clone())?, py::abs(&v_fy.clone())?], true)?;
        let _ = py::method(&v_res.clone(), "append", &[py::minmax(&[Py::Float(0.0), py::sub(&py::truediv(&py::abs(&ax_engine::pysym::helper("calculus:at", &[v_G.clone(), c__XY()?, v_x.clone(), v_y.clone()], &[])?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_gx.clone())?, py::abs(&v_gy.clone())?], true)?)?, &Py::Float(1e-09))?], true)?])?;
        let _ = py::method(&v_res.clone(), "append", &[py::minmax(&[Py::Float(0.0), py::sub(&py::truediv(&py::abs(&py::sub(&v_fx.clone(), &py::mul(&v_lam.clone(), &v_gx.clone())?)?)?, &v_S.clone())?, &Py::Float(1e-09))?], true)?])?;
        let _ = py::method(&v_res.clone(), "append", &[py::minmax(&[Py::Float(0.0), py::sub(&py::truediv(&py::abs(&py::sub(&v_fy.clone(), &py::mul(&v_lam.clone(), &v_gy.clone())?)?)?, &v_S.clone())?, &Py::Float(1e-09))?], true)?])?;
        let _ = py::method(&v_res.clone(), "append", &[f_rel_err(py::getitem(&v_o.clone(), &py::add(&v_key.clone(), &Py::str("_value"))?)?, ax_engine::pysym::helper("calculus:at", &[v_e.clone(), c__XY()?, v_x.clone(), v_y.clone()], &[])?, Py::Float(1e-09))?])?;
    }
    if ({ let t1150 = v_n.clone(); Py::Bool({ let t1151 = Py::Int(0i64); if py::eq(&t1150, &t1151) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    for t1163 in py::iter(&(f__constraint_samples(v_A.clone(), v_B.clone(), v_D.clone(), v_E.clone(), v_F.clone())?))? {
        let t1169 = (t1163).items()?;
        if t1169.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t1169[0].clone();
        v_y = t1169[1].clone();
        let t1164: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                v_v = py::call(&v_fl.clone(), &[v_x.clone(), v_y.clone()])?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1164 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (matches!(e, PyErr::Value(_))) => {
                continue;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1165 = Py::str("max_value"); Py::Bool({ let t1166 = v_o.clone(); if py::contains(&t1166, &t1165)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_res.clone(), "append", &[py::minmax(&[Py::Float(0.0), py::sub(&py::truediv(&py::sub(&v_v.clone(), &py::getitem(&v_o.clone(), &Py::str("max_value"))?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_o.clone(), &Py::str("max_value"))?)?], true)?)?, &Py::Float(1e-09))?], true)?])?;
        }
        if ({ let t1167 = Py::str("min_value"); Py::Bool({ let t1168 = v_o.clone(); if py::contains(&t1168, &t1167)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_res.clone(), "append", &[py::minmax(&[Py::Float(0.0), py::sub(&py::truediv(&py::sub(&py::getitem(&v_o.clone(), &Py::str("min_value"))?, &v_v.clone())?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_o.clone(), &Py::str("min_value"))?)?], true)?)?, &Py::Float(1e-09))?], true)?])?;
        }
    }
    return Ok(f_worst({ let mut v = vec![]; v.extend((v_res.clone()).items()?); Py::tuple(v) })?);
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
        let t1259 = (Py::tuple(vec![f__mpq(f__rat(py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, py::pow(&ax_engine::pymp::call("mpf", &[Py::Int(10i64)], &[])?, &py::neg(&Py::Int(12i64))?)?])).items()?;
        if t1259.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_A = t1259[0].clone();
        v_h = t1259[1].clone();
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
    let t1314 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:poly_of", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?])).items()?;
    if t1314.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p = t1314[0].clone();
    v_a = t1314[1].clone();
    v_b = t1314[2].clone();
    v_m = py::truediv(&py::sub(&f__peval(v_p.clone(), v_b.clone())?, &f__peval(v_p.clone(), v_a.clone())?)?, &py::sub(&v_b.clone(), &v_a.clone())?)?;
    let t1315 = (Py::tuple(vec![f__rat(v_a.clone())?, f__rat(v_b.clone())?])).items()?;
    if t1315.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1315[0].clone();
    v_B = t1315[1].clone();
    v_q = py::sub(&py::method(&v_p.clone(), "diff", &[c_X()?])?, &py::truediv(&py::sub(&py::method(&v_p.clone(), "eval", &[v_B.clone()])?, &py::method(&v_p.clone(), "eval", &[v_A.clone()])?)?, &py::sub(&v_B.clone(), &v_A.clone())?)?)?;
    v_cs = Py::list((py::getitem(&v_o.clone(), &Py::str("c_values"))?).items()?);
    if ({ let t1334 = { let t1316 = py::len(&v_cs.clone())?; Py::Bool({ let t1317 = f__n_roots_open(v_q.clone(), v_a.clone(), v_b.clone())?; if py::ne(&t1316, &t1317) { true } else { false } }) }; if t1334.truthy() { t1334 } else { { let t1333 = { let t1327 = v_cs.clone(); Py::Bool({ let t1328 = py::sorted(&v_cs.clone(), (Py::Bool(false)).truthy())?; if py::ne(&t1327, &t1328) { true } else { false } }) }; if t1333.truthy() { t1333 } else { Py::Bool(({ let mut out = vec![]; for t1329 in py::iter(&(v_cs.clone()))? { let mut v_c: Py; 
v_c = t1329;
out.push(py::not(&{ let t1330 = v_a.clone(); Py::Bool({ let t1331 = v_c.clone(); if py::lt(&t1330, &t1331)? { { let t1332 = v_b.clone(); if py::lt(&t1331, &t1332)? { true } else { false } } } else { false } }) })); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(f_worst({ let mut v = vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("secant_slope"))?, v_m.clone(), Py::Float(1e-12))?]; v.extend(({ let mut out = vec![]; for t1335 in py::iter(&(v_cs.clone()))? { let mut v_c: Py; 
v_c = t1335;
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
        v_steps = { let mut out = vec![]; for t1395 in py::iter(&(py::zip(&[py::add(&Py::list(vec![v_x0.clone()]), &py::slice(&v_its.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?)?, v_its.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1396 = (t1395).items()?;
if t1396.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1396[0].clone();
v_b = t1396[1].clone();
out.push(f_rel_err(v_b.clone(), py::float(&py::sub(&ax_engine::pymp::call("mpf", &[v_a.clone()], &[])?, &py::truediv(&py::call(&v_g.clone(), &[ax_engine::pymp::call("mpf", &[v_a.clone()], &[])?])?, &ax_engine::pymp::call("diff", &[v_g.clone(), ax_engine::pymp::call("mpf", &[v_a.clone()], &[])?], &[])?)?)?)?, Py::Float(1e-10))?); } Py::list(out) };
    }
    return Ok(py::minmax(&[py::add(&v_steps.clone(), &Py::list(vec![f_rel_err(py::getitem(&v_its.clone(), &py::neg(&Py::Int(1i64))?)?, py::getitem(&v_o.clone(), &Py::str("x_n"))?, Py::Float(0.0))?]))?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_polar(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_al: Py = Py::None;
    let mut v_be: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_r2: Py = Py::None;
    v_g = f__fl(ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?, c__XY()?], &[])?, c__XY()?)?;
    v_h = { let v_g = v_g.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_th: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&py::call(&v_g.clone(), &[py::mul(&v_r.clone(), &py::math1("cos", &v_th.clone())?)?, py::mul(&v_r.clone(), &py::math1("sin", &v_th.clone())?)?])?, &v_r.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t1536 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("r1"))?, py::getitem(&v_i.clone(), &Py::str("r2"))?, py::getitem(&v_i.clone(), &Py::str("al"))?, py::getitem(&v_i.clone(), &Py::str("be"))?])).items()?;
    if t1536.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t1536[0].clone();
    v_r2 = t1536[1].clone();
    v_al = t1536[2].clone();
    v_be = t1536[3].clone();
    v_S = f__gl2({ let v_al = v_al.clone(); let v_be = v_be.clone(); let v_g = v_g.clone(); let v_h = v_h.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_r1 = v_r1.clone(); let v_r2 = v_r2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_th: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::call(&v_h.clone(), &[v_r.clone(), v_th.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_r1.clone(), v_r2.clone(), { let v_al = v_al.clone(); let v_be = v_be.clone(); let v_g = v_g.clone(); let v_h = v_h.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_r1 = v_r1.clone(); let v_r2 = v_r2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_al.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_al = v_al.clone(); let v_be = v_be.clone(); let v_g = v_g.clone(); let v_h = v_h.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_r1 = v_r1.clone(); let v_r2 = v_r2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_be.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Int(32i64))?;
    return Ok(f_worst(Py::tuple(vec![f__scaled_err(py::getitem(&v_o.clone(), &Py::str("integral"))?, f__gl2(v_h.clone(), v_r1.clone(), v_r2.clone(), { let v_S = v_S.clone(); let v_al = v_al.clone(); let v_be = v_be.clone(); let v_g = v_g.clone(); let v_h = v_h.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_r1 = v_r1.clone(); let v_r2 = v_r2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_al.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_S = v_S.clone(); let v_al = v_al.clone(); let v_be = v_be.clone(); let v_g = v_g.clone(); let v_h = v_h.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_r1 = v_r1.clone(); let v_r2 = v_r2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_be.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Int(32i64))?, v_S.clone(), Py::Float(1e-10))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("region_area"))?, py::truediv(&py::mul(&py::sub(&py::mul(&v_r2.clone(), &v_r2.clone())?, &py::mul(&v_r1.clone(), &v_r1.clone())?)?, &py::sub(&v_be.clone(), &v_al.clone())?)?, &Py::Int(2i64))?, Py::Float(1e-11))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_pseries(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dl: Py = Py::None;
    let mut v_dr: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lb: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_rb: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_sl: Py = Py::None;
    let mut v_sr: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("a"))?, Py::tuple(vec![Py::str("n"), Py::str("x")])], &[])?;
    let t1599 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("endpoint_exponent"))?, py::getitem(&v_o.clone(), &Py::str("radius"))?, py::getitem(&v_o.clone(), &Py::str("centre"))?])).items()?;
    if t1599.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_d = t1599[0].clone();
    v_R = t1599[1].clone();
    v_c = t1599[2].clone();
    let t1600 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("left_end"))?, py::getitem(&v_o.clone(), &Py::str("right_end"))?])).items()?;
    if t1600.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t1600[0].clone();
    v_hi = t1600[1].clone();
    v_res = Py::list(vec![f_rel_err(v_lo.clone(), py::sub(&v_c.clone(), &v_R.clone())?, Py::Float(1e-12))?, f_rel_err(v_hi.clone(), py::add(&v_c.clone(), &v_R.clone())?, Py::Float(1e-12))?]);
    if (py::not(&{ let t1604 = v_R.clone(); Py::Bool({ let t1605 = Py::Int(0i64); if py::gt(&t1604, &t1605)? { true } else { false } }) })).truthy() {
        return Ok(Py::Float(1.0));
    }
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(40i64))?;
        v_g = ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![Py::str("n"), Py::str("x")])], &[])?;
        let t1614 = (f__end_probe(v_g.clone(), py::add(&ax_engine::pymp::call("mpf", &[v_c.clone()], &[])?, &ax_engine::pymp::call("mpf", &[v_R.clone()], &[])?)?)?).items()?;
        if t1614.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_sr = t1614[0].clone();
        v_dr = t1614[1].clone();
        let t1615 = (f__end_probe(v_g.clone(), py::sub(&ax_engine::pymp::call("mpf", &[v_c.clone()], &[])?, &ax_engine::pymp::call("mpf", &[v_R.clone()], &[])?)?)?).items()?;
        if t1615.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_sl = t1615[0].clone();
        v_dl = t1615[1].clone();
    }
    let _ = py::method(&v_res.clone(), "append", &[py::minmax(&[Py::Float(0.0), py::sub(&py::abs(&py::sub(&py::float(&v_dr.clone())?, &v_d.clone())?)?, &Py::Float(1e-06))?], true)?])?;
    let _ = py::method(&v_res.clone(), "append", &[py::minmax(&[Py::Float(0.0), py::sub(&py::abs(&py::sub(&py::float(&v_dl.clone())?, &v_d.clone())?)?, &Py::Float(1e-06))?], true)?])?;
    let _ = py::method(&v_res.clone(), "append", &[f__same(py::getitem(&v_o.clone(), &Py::str("right_behaviour"))?, f__end_verdict(v_sr.clone(), v_d.clone())?)?])?;
    let _ = py::method(&v_res.clone(), "append", &[f__same(py::getitem(&v_o.clone(), &Py::str("left_behaviour"))?, f__end_verdict(v_sl.clone(), v_d.clone())?)?])?;
    v_lb = (if ({ let t1618 = py::getitem(&v_o.clone(), &Py::str("left_behaviour"))?; Py::Bool({ let t1619 = Py::str("diverges"); if py::eq(&t1618, &t1619) { true } else { false } }) }).truthy() { Py::str("(") } else { Py::str("[") });
    v_rb = (if ({ let t1620 = py::getitem(&v_o.clone(), &Py::str("right_behaviour"))?; Py::Bool({ let t1621 = Py::str("diverges"); if py::eq(&t1620, &t1621) { true } else { false } }) }).truthy() { Py::str(")") } else { Py::str("]") });
    let _ = py::method(&v_res.clone(), "append", &[f__same(py::getitem(&v_o.clone(), &Py::str("interval"))?, py::add(&py::add(&v_lb.clone(), &Py::str(&[py::format(&v_lo.clone(), ".12g")?, ", ".to_string(), py::format(&v_hi.clone(), ".12g")?].concat()))?, &v_rb.clone())?)?])?;
    return Ok(f_worst({ let mut v = vec![]; v.extend((v_res.clone()).items()?); Py::tuple(v) })?);
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
        let t1461 = ({ let mut out = vec![]; for t1460 in py::iter(&(f__path(py::getitem(&v_i.clone(), &Py::str("xt"))?, py::getitem(&v_i.clone(), &Py::str("yt"))?)?))? { let mut v_e: Py; 
v_e = t1460;
out.push(ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_T()?])], &[])?); } Py::list(out) }).items()?;
        if t1461.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_gx = t1461[0].clone();
        v_gy = t1461[1].clone();
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
    let t1918 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[])?, ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?, Py::tuple(vec![c_X()?])], &[])?])).items()?;
    if t1918.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t1918[0].clone();
    v_D = t1918[1].clone();
    v_cuts = f__sign_changes(ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[(String::from("use_mp"), Py::Bool(false))])?, v_a.clone(), v_b.clone(), Py::Int(1000i64))?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        v_knots = py::add(&py::add(&Py::list(vec![ax_engine::pymp::call("mpf", &[v_a.clone()], &[])?]), &{ let mut out = vec![]; for t1939 in py::iter(&(v_cuts.clone()))? { let mut v_c: Py; 
v_c = t1939;
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
    let t1976 = (Py::tuple(vec![Py::list((py::getitem(&v_o.clone(), &Py::str("coefficients"))?).items()?), py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::sub(&py::getitem(&v_i.clone(), &Py::str("x1"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?])).items()?;
    if t1976.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_co = t1976[0].clone();
    v_n = t1976[1].clone();
    v_h = t1976[2].clone();
    if ({ let t1977 = py::len(&v_co.clone())?; Py::Bool({ let t1978 = py::add(&v_n.clone(), &Py::Int(1i64))?; if py::ne(&t1977, &t1978) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        v_g = ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?, Py::tuple(vec![c_X()?])], &[])?;
        v_ref = ax_engine::pymp::call("taylor", &[v_g.clone(), ax_engine::pymp::call("mpf", &[py::getitem(&v_i.clone(), &Py::str("a"))?], &[])?, v_n.clone()], &[])?;
        v_fx1 = py::call(&v_g.clone(), &[ax_engine::pymp::call("mpf", &[py::getitem(&v_i.clone(), &Py::str("x1"))?], &[])?])?;
    }
    v_val = Py::Float(0.0);
    for t1979 in py::iter(&(py::reversed(&v_co.clone())?))? {
        v_c = t1979;
        v_val = py::add(&py::mul(&v_val.clone(), &v_h.clone())?, &v_c.clone())?;
    }
    return Ok(f_worst(Py::tuple(vec![f__vec_err(v_co.clone(), { let mut out = vec![]; for t1988 in py::iter(&(v_ref.clone()))? { let mut v_c: Py; 
v_c = t1988;
out.push(py::float(&v_c.clone())?); } Py::list(out) }, Py::Float(1e-11))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("value"))?, v_val.clone(), Py::Float(1e-12))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("function_value"))?, py::float(&v_fx1.clone())?, Py::Float(1e-12))?, py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("error"))?, &py::sub(&py::getitem(&v_o.clone(), &Py::str("function_value"))?, &py::getitem(&v_o.clone(), &Py::str("value"))?)?)?)?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_totdiff(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_df: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_res: Py = Py::None;
    v_p = py::getitem(&v_i.clone(), &Py::str("p"))?;
    v_names = f__vars(v_p.clone())?;
    v_k = py::len(&v_names.clone())?;
    v_dx = (if ({ let t2011 = py::method(&v_i.clone(), "get", &[Py::str("mode"), Py::str("change")])?; Py::Bool({ let t2012 = Py::str("change"); if py::eq(&t2011, &t2012) { true } else { false } }) }).truthy() { { let mut out = vec![]; for t2013 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("d"))?))? { let mut v_v: Py; 
v_v = t2013;
out.push(py::float(&v_v.clone())?); } Py::list(out) } } else { { let mut out = vec![]; for t2014 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t2014;
out.push(py::truediv(&py::mul(&py::getitem(&v_p.clone(), &v_j.clone())?, &py::getitem(&py::getitem(&v_i.clone(), &Py::str("d"))?, &v_j.clone())?)?, &Py::Int(100i64))?); } Py::list(out) } });
    v_g = { let mut out = vec![]; for t2015 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t2015;
out.push(py::float(&f__mp_partial(py::getitem(&v_i.clone(), &Py::str("f"))?, v_names.clone(), v_p.clone(), { let mut out = vec![]; for t2017 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_m: Py; 
v_m = t2017;
out.push(py::int(&{ let t2018 = v_m.clone(); Py::Bool({ let t2019 = v_j.clone(); if py::eq(&t2018, &t2019) { true } else { false } }) })?); } Py::list(out) })?)?); } Py::list(out) };
    v_df = py::fsum(&{ let mut out = vec![]; for t2020 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t2020;
out.push(py::mul(&py::getitem(&v_g.clone(), &v_j.clone())?, &py::getitem(&v_dx.clone(), &v_j.clone())?)?); } Py::list(out) })?;
    v_S = py::fsum(&{ let mut out = vec![]; for t2021 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t2021;
out.push(py::abs(&py::mul(&py::getitem(&v_g.clone(), &v_j.clone())?, &py::getitem(&v_dx.clone(), &v_j.clone())?)?)?); } Py::list(out) })?;
    v_res = Py::list(vec![py::minmax(&[Py::Float(0.0), py::sub(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("df"))?, &v_df.clone())?)?, &py::minmax(&[py::abs(&v_df.clone())?, v_S.clone(), Py::Float(1e-300)], true)?)?, &Py::Float(1e-10))?], true)?, f__vec_err(py::getitem(&v_o.clone(), &Py::str("partials"))?, v_g.clone(), Py::Float(1e-10))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("f_value"))?, { let mut v: Vec<Py> = vec![]; v.push(ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?, v_names.clone()], &[])?); v.push(v_names.clone()); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? }, Py::Float(1e-12))?]);
    if ({ let t2030 = Py::str("percent_change"); Py::Bool({ let t2031 = v_o.clone(); if py::contains(&t2031, &t2030)? { true } else { false } }) }).truthy() {
        let _ = py::method(&v_res.clone(), "append", &[f_rel_err(py::getitem(&v_o.clone(), &Py::str("percent_change"))?, py::truediv(&py::mul(&Py::Int(100i64), &py::getitem(&v_o.clone(), &Py::str("df"))?)?, &py::getitem(&v_o.clone(), &Py::str("f_value"))?)?, Py::Float(1e-12))?])?;
    }
    return Ok(f_worst({ let mut v = vec![]; v.extend((v_res.clone()).items()?); Py::tuple(v) })?);
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
    let t1961 = (Py::tuple(vec![py::float(&f__mp_partial(py::getitem(&v_i.clone(), &Py::str("f"))?, c__XY()?, v_p.clone(), Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)]))?)?, py::float(&f__mp_partial(py::getitem(&v_i.clone(), &Py::str("f"))?, c__XY()?, v_p.clone(), Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)]))?)?])).items()?;
    if t1961.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fx = t1961[0].clone();
    v_fy = t1961[1].clone();
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
        v_v = py::add(&py::add(&ax_engine::pymp::call("fsum", &[{ let mut out = vec![]; for t1693 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_N.clone()).i()?]) { let mut v_n: Py; 
v_n = t1693;
out.push(py::pow(&ax_engine::pymp::call("mpf", &[v_n.clone()], &[])?, &py::neg(&v_s.clone())?)?); } Py::list(out) }], &[])?, &py::truediv(&py::pow(&ax_engine::pymp::call("mpf", &[v_N.clone()], &[])?, &py::sub(&Py::Int(1i64), &v_s.clone())?)?, &py::sub(&v_s.clone(), &Py::Int(1i64))?)?)?, &py::truediv(&py::pow(&ax_engine::pymp::call("mpf", &[v_N.clone()], &[])?, &py::neg(&v_s.clone())?)?, &Py::Int(2i64))?)?;
        v_rise = v_s.clone();
        for t1694 in py::iter(&(py::enumerate(&c__B2K()?, (Py::Int(1i64)).i()?)?))? {
            let t1695 = (t1694).items()?;
            if t1695.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_k = t1695[0].clone();
            v_B = t1695[1].clone();
            v_v = py::iadd(&v_v, &py::mul(&py::mul(&py::truediv(&py::truediv(&ax_engine::pymp::call("mpf", &[py::numerator(&v_B.clone())?], &[])?, &py::denominator(&v_B.clone())?)?, &ax_engine::pymp::call("factorial", &[py::mul(&Py::Int(2i64), &v_k.clone())?], &[])?)?, &v_rise.clone())?, &py::pow(&ax_engine::pymp::call("mpf", &[v_N.clone()], &[])?, &py::add(&py::sub(&py::neg(&v_s.clone())?, &py::mul(&Py::Int(2i64), &v_k.clone())?)?, &Py::Int(1i64))?)?)?)?;
            v_rise = py::mul(&v_rise, &py::mul(&py::sub(&py::add(&v_s.clone(), &py::mul(&Py::Int(2i64), &v_k.clone())?)?, &Py::Int(1i64))?, &py::add(&v_s.clone(), &py::mul(&Py::Int(2i64), &v_k.clone())?)?)?)?;
        }
        return Ok(py::float(&v_v.clone())?);
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_antiderivative(mut v_f: Py, mut v_x0: Py, mut v_F0: Py) -> R {
    let mut v_G: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_sgn: Py = Py::None;
    if ({ let t1 = { let t3 = v_x0.clone(); Py::Bool({ let t4 = Py::None; if py::is(&t3, &t4) { true } else { false } }) }; Py::Bool({ let t2 = { let t5 = v_F0.clone(); Py::Bool({ let t6 = Py::None; if py::is(&t5, &t6) { true } else { false } }) }; if py::ne(&t1, &t2) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("an initial condition needs both the point x0 and the value F(x0)"))));
    }
    v_G = f__antiderivative(ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?, c_X()?)?;
    v_out = Py::dict(vec![(Py::str("antiderivative"), py::add(&ax_engine::pysym::helper("calculus:show", &[v_G.clone()], &[])?, &Py::str(" + C"))?)]);
    if ({ let t38 = v_x0.clone(); Py::Bool({ let t39 = Py::None; if !py::is(&t38, &t39) { true } else { false } }) }).truthy() {
        v_c = py::sub(&v_F0.clone(), &ax_engine::pysym::helper("calculus:at", &[v_G.clone(), Py::tuple(vec![c_X()?]), v_x0.clone()], &[])?)?;
        v_sgn = (if ({ let t40 = v_c.clone(); Py::Bool({ let t41 = Py::Int(0i64); if py::lt(&t40, &t41)? { true } else { false } }) }).truthy() { Py::str(" - ") } else { Py::str(" + ") });
        py::setitem(&v_out.clone(), &Py::str("constant"), v_c.clone())?;
        py::setitem(&v_out.clone(), &Py::str("particular"), py::add(&py::add(&ax_engine::pysym::helper("calculus:show", &[v_G.clone()], &[])?, &v_sgn.clone())?, &Py::str(&[py::format(&py::abs(&v_c.clone())?, ".12g")?].concat()))?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_arc_length(mut v_f: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    if (py::not(&{ let t58 = v_a.clone(); Py::Bool({ let t59 = v_b.clone(); if py::lt(&t58, &t59)? { true } else { false } }) })).truthy() {
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
    if (py::not(&{ let t68 = v_a.clone(); Py::Bool({ let t69 = v_b.clone(); if py::lt(&t68, &t69)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    v_h = py::sub(&ax_engine::pysym::helper("calculus:poly_of", &[v_f.clone()], &[])?, &ax_engine::pysym::helper("calculus:poly_of", &[v_g.clone()], &[])?)?;
    let t72 = (f__poly_abs_integral(v_h.clone(), v_a.clone(), v_b.clone())?).items()?;
    if t72.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_area = t72[0].clone();
    v_cross = t72[1].clone();
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
    if ({ let t95 = py::len(&v_roots.clone())?; Py::Bool({ let t96 = Py::Int(2i64); if py::lt(&t95, &t96)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the curves meet in fewer than two points, so they enclose no bounded region"))));
    }
    v_H = py::method(&py::method(&v_h.clone(), "integrate", &[])?, "as_expr", &[])?;
    v_tot = py::sum(&{ let mut out = vec![]; for t97 in py::range_iter(&[(py::sub(&py::len(&v_roots.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t97;
out.push(ax_engine::pysym::call("Abs", &[py::sub(&py::method(&v_H.clone(), "subs", &[c_X()?, py::getitem(&v_roots.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?])?, &py::method(&v_H.clone(), "subs", &[c_X()?, py::getitem(&v_roots.clone(), &v_k.clone())?])?)?], &[])?); } Py::list(out) }, None)?;
    return Ok(Py::dict(vec![(Py::str("area"), py::float(&ax_engine::pysym::call("N", &[v_tot.clone(), Py::Int(30i64)], &[])?)?), (Py::str("x_left"), py::float(&ax_engine::pysym::call("N", &[py::getitem(&v_roots.clone(), &Py::Int(0i64))?, Py::Int(30i64)], &[])?)?), (Py::str("x_right"), py::float(&ax_engine::pysym::call("N", &[py::getitem(&v_roots.clone(), &py::neg(&Py::Int(1i64))?)?, Py::Int(30i64)], &[])?)?), (Py::str("intersections"), { let mut out = vec![]; for t98 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t98;
out.push(py::float(&ax_engine::pysym::call("N", &[v_r.clone(), Py::Int(30i64)], &[])?)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_arithmetic_series_sum(mut v_a1: Py, mut v_d: Py, mut v_n: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_D: Py = Py::None;
    v_n = py::int(&v_n.clone())?;
    if (py::not(&{ let t110 = Py::Int(1i64); Py::Bool({ let t111 = v_n.clone(); if py::le(&t110, &t111)? { { let t112 = py::pow(&Py::Int(10i64), &Py::Int(6i64))?; if py::le(&t111, &t112)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be between 1 and 1000000"))));
    }
    let t113 = (Py::tuple(vec![f__frac(v_a1.clone())?, f__frac(v_d.clone())?])).items()?;
    if t113.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t113[0].clone();
    v_D = t113[1].clone();
    return Ok(Py::dict(vec![(Py::str("sum"), py::float(&py::mul(&py::fraction(&[v_n.clone(), Py::Int(2i64)])?, &py::add(&py::mul(&Py::Int(2i64), &v_A.clone())?, &py::mul(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_D.clone())?)?)?)?), (Py::str("last_term"), py::float(&py::add(&v_A.clone(), &py::mul(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_D.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_average_rate_of_change(mut v_f: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_fa: Py = Py::None;
    let mut v_fb: Py = Py::None;
    if ({ let t122 = v_a.clone(); Py::Bool({ let t123 = v_b.clone(); if py::eq(&t122, &t123) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two points must differ"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    let t124 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:at", &[v_e.clone(), Py::tuple(vec![c_X()?]), v_a.clone()], &[])?, ax_engine::pysym::helper("calculus:at", &[v_e.clone(), Py::tuple(vec![c_X()?]), v_b.clone()], &[])?])).items()?;
    if t124.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t124[0].clone();
    v_fb = t124[1].clone();
    return Ok(Py::dict(vec![(Py::str("average_rate"), py::truediv(&py::sub(&v_fb.clone(), &v_fa.clone())?, &py::sub(&v_b.clone(), &v_a.clone())?)?), (Py::str("fa"), v_fa.clone()), (Py::str("fb"), v_fb.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_average_value(mut v_f: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_i: Py = Py::None;
    if (py::not(&{ let t133 = v_a.clone(); Py::Bool({ let t134 = v_b.clone(); if py::lt(&t133, &t134)? { true } else { false } }) })).truthy() {
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
    if ({ let t150 = py::not(&{ let t145 = Py::Int(0i64); Py::Bool({ let t146 = v_n.clone(); if py::le(&t145, &t146)? { { let t147 = Py::Int(50i64); if py::le(&t146, &t147)? { true } else { false } } } else { false } }) }); if t150.truthy() { t150 } else { { let t148 = py::abs(&v_x.clone())?; Py::Bool({ let t149 = Py::Int(1000i64); if py::gt(&t148, &t149)? { true } else { false } }) } } }).truthy() {
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
    if (py::not(&{ let t163 = { let t159 = v_a.clone(); Py::Bool({ let t160 = Py::Int(0i64); if py::gt(&t159, &t160)? { true } else { false } }) }; if !t163.truthy() { t163 } else { { let t161 = v_b.clone(); Py::Bool({ let t162 = Py::Int(0i64); if py::gt(&t161, &t162)? { true } else { false } }) } } })).truthy() {
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
    if (py::not(&{ let t172 = v_a.clone(); Py::Bool({ let t173 = v_b.clone(); if py::lt(&t172, &t173)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    v_p = ax_engine::pysym::helper("calculus:poly_of", &[v_f.clone()], &[])?;
    if (py::not(&f__nonneg_on(v_p.clone(), v_a.clone(), v_b.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the region under the curve needs f(x) >= 0 on [a, b]"))));
    }
    let t180 = (Py::tuple(vec![ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_a.clone())?))], &[])?, ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_b.clone())?))], &[])?])).items()?;
    if t180.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A_ = t180[0].clone();
    v_B_ = t180[1].clone();
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
    if ({ let t181 = v_area.clone(); Py::Bool({ let t182 = Py::Int(0i64); if py::eq(&t181, &t182) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the region has zero area"))));
    }
    let t183 = (Py::tuple(vec![py::truediv(&py::call(&v_integ.clone(), &[py::mul(&ax_engine::pysym::call("Poly", &[c_X()?, c_X()?], &[])?, &v_p.clone())?])?, &v_area.clone())?, py::truediv(&py::call(&v_integ.clone(), &[py::pow(&v_p.clone(), &Py::Int(2i64))?])?, &py::mul(&Py::Int(2i64), &v_area.clone())?)?])).items()?;
    if t183.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xb = t183[0].clone();
    v_yb = t183[1].clone();
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
    if (py::not(&{ let t192 = Py::Int(0i64); Py::Bool({ let t193 = v_n.clone(); if py::le(&t192, &t193)? { { let t194 = Py::Int(500i64); if py::le(&t193, &t194)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the degree must be between 0 and 500"))));
    }
    let t195 = (Py::tuple(vec![Py::Float(1.0), v_x.clone()])).items()?;
    if t195.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t0 = t195[0].clone();
    v_t1 = t195[1].clone();
    if ({ let t196 = v_n.clone(); Py::Bool({ let t197 = Py::Int(0i64); if py::eq(&t196, &t197) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("T"), Py::Float(1.0))]));
    }
    for t198 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_n.clone()).i()?]) {
        v__ = t198;
        let t199 = (Py::tuple(vec![v_t1.clone(), py::sub(&py::mul(&py::mul(&Py::Int(2i64), &v_x.clone())?, &v_t1.clone())?, &v_t0.clone())?])).items()?;
        if t199.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_t0 = t199[0].clone();
        v_t1 = t199[1].clone();
    }
    return Ok(Py::dict(vec![(Py::str("T"), v_t1.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_conservative_field(mut v_P: Py, mut v_Q: Py, mut v_R: Py, mut v_A: Py, mut v_B: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_curl: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_nonzero: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_text: Py = Py::None;
    let mut v_three: Py = Py::None;
    let t211 = (f__pfield(v_P.clone(), v_Q.clone(), v_R.clone())?).items()?;
    if t211.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t211[0].clone();
    v_names = t211[1].clone();
    v_three = { let t212 = v_R.clone(); Py::Bool({ let t213 = Py::None; if !py::is(&t212, &t213) { true } else { false } }) };
    if ({ let t214 = { let t216 = v_A.clone(); Py::Bool({ let t217 = Py::None; if py::is(&t216, &t217) { true } else { false } }) }; Py::Bool({ let t215 = { let t218 = v_B.clone(); Py::Bool({ let t219 = Py::None; if py::is(&t218, &t219) { true } else { false } }) }; if py::ne(&t214, &t215) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a line integral needs both the start point A and the end point B"))));
    }
    if ({ let t227 = { let t220 = v_A.clone(); Py::Bool({ let t221 = Py::None; if !py::is(&t220, &t221) { true } else { false } }) }; if !t227.truthy() { t227 } else { { let t226 = { let t222 = py::len(&v_A.clone())?; Py::Bool({ let t223 = py::len(&v_names.clone())?; if py::ne(&t222, &t223) { true } else { false } }) }; if t226.truthy() { t226 } else { { let t224 = py::len(&v_B.clone())?; Py::Bool({ let t225 = py::len(&v_names.clone())?; if py::ne(&t224, &t225) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the points need as many coordinates as the field has components"))));
    }
    if (v_three.clone()).truthy() {
        v_curl = Py::list(vec![ax_engine::pysym::call("expand", &[py::sub(&ax_engine::pysym::call("diff", &[py::getitem(&v_F.clone(), &Py::Int(2i64))?, c_Y()?], &[])?, &ax_engine::pysym::call("diff", &[py::getitem(&v_F.clone(), &Py::Int(1i64))?, c_Z()?], &[])?)?], &[])?, ax_engine::pysym::call("expand", &[py::sub(&ax_engine::pysym::call("diff", &[py::getitem(&v_F.clone(), &Py::Int(0i64))?, c_Z()?], &[])?, &ax_engine::pysym::call("diff", &[py::getitem(&v_F.clone(), &Py::Int(2i64))?, c_X()?], &[])?)?], &[])?, ax_engine::pysym::call("expand", &[py::sub(&ax_engine::pysym::call("diff", &[py::getitem(&v_F.clone(), &Py::Int(1i64))?, c_X()?], &[])?, &ax_engine::pysym::call("diff", &[py::getitem(&v_F.clone(), &Py::Int(0i64))?, c_Y()?], &[])?)?], &[])?]);
    } else {
        v_curl = Py::list(vec![ax_engine::pysym::call("expand", &[py::sub(&ax_engine::pysym::call("diff", &[py::getitem(&v_F.clone(), &Py::Int(1i64))?, c_X()?], &[])?, &ax_engine::pysym::call("diff", &[py::getitem(&v_F.clone(), &Py::Int(0i64))?, c_Y()?], &[])?)?], &[])?]);
    }
    v_nonzero = Py::Bool(false);
    for t228 in py::iter(&(v_curl.clone()))? {
        v_c = t228;
        if ({ let t229 = v_c.clone(); Py::Bool({ let t230 = Py::Int(0i64); if py::ne(&t229, &t230) { true } else { false } }) }).truthy() {
            v_nonzero = Py::Bool(true);
        }
    }
    if (v_nonzero.clone()).truthy() {
        v_text = ax_engine::pysym::helper("calculus:show", &[py::getitem(&v_curl.clone(), &Py::Int(0i64))?], &[])?;
        if (v_three.clone()).truthy() {
            v_text = py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&Py::str("("), &ax_engine::pysym::helper("calculus:show", &[py::getitem(&v_curl.clone(), &Py::Int(0i64))?], &[])?)?, &Py::str(", "))?, &ax_engine::pysym::helper("calculus:show", &[py::getitem(&v_curl.clone(), &Py::Int(1i64))?], &[])?)?, &Py::str(", "))?, &ax_engine::pysym::helper("calculus:show", &[py::getitem(&v_curl.clone(), &Py::Int(2i64))?], &[])?)?, &Py::str(")"))?;
        }
        return Ok(Py::dict(vec![(Py::str("conservative"), Py::str("no")), (Py::str("curl"), v_text.clone())]));
    }
    v_phi = ax_engine::pysym::call("integrate", &[py::getitem(&v_F.clone(), &Py::Int(0i64))?, c_X()?], &[])?;
    v_phi = py::add(&v_phi.clone(), &ax_engine::pysym::call("integrate", &[ax_engine::pysym::call("expand", &[py::sub(&py::getitem(&v_F.clone(), &Py::Int(1i64))?, &ax_engine::pysym::call("diff", &[v_phi.clone(), c_Y()?], &[])?)?], &[])?, c_Y()?], &[])?)?;
    if (v_three.clone()).truthy() {
        v_phi = py::add(&v_phi.clone(), &ax_engine::pysym::call("integrate", &[ax_engine::pysym::call("expand", &[py::sub(&py::getitem(&v_F.clone(), &Py::Int(2i64))?, &ax_engine::pysym::call("diff", &[v_phi.clone(), c_Z()?], &[])?)?], &[])?, c_Z()?], &[])?)?;
    }
    v_phi = ax_engine::pysym::call("expand", &[v_phi.clone()], &[])?;
    for t231 in py::range_iter(&[(py::len(&v_names.clone())?).i()?]) {
        v_k = t231;
        if ({ let t232 = ax_engine::pysym::call("expand", &[py::sub(&ax_engine::pysym::call("diff", &[v_phi.clone(), py::getitem(&c__SYMS()?, &py::getitem(&v_names.clone(), &v_k.clone())?)?], &[])?, &py::getitem(&v_F.clone(), &v_k.clone())?)?], &[])?; Py::Bool({ let t233 = Py::Int(0i64); if py::ne(&t232, &t233) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the potential function could not be confirmed"))));
        }
    }
    v_out = Py::dict(vec![(Py::str("conservative"), Py::str("yes")), (Py::str("potential"), py::add(&ax_engine::pysym::helper("calculus:show", &[v_phi.clone()], &[])?, &Py::str(" + C"))?)]);
    if ({ let t234 = v_A.clone(); Py::Bool({ let t235 = Py::None; if !py::is(&t234, &t235) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("line_integral"), py::sub(&{ let mut v: Vec<Py> = vec![]; v.push(v_phi.clone()); v.push(v_names.clone()); v.extend((v_B.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? }, &{ let mut v: Vec<Py> = vec![]; v.push(v_phi.clone()); v.push(v_names.clone()); v.extend((v_A.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? })?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_coordinate_conversion(mut v_system: Py, mut v_p: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_ph: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    if ({ let t261 = py::len(&v_p.clone())?; Py::Bool({ let t262 = Py::Int(3i64); if py::ne(&t261, &t262) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the point needs three coordinates"))));
    }
    v_v = f__to_rect(v_system.clone(), v_p.clone())?;
    v_S = py::minmax(&[py::abs(&py::getitem(&v_v.clone(), &Py::Int(0i64))?)?, py::abs(&py::getitem(&v_v.clone(), &Py::Int(1i64))?)?, py::abs(&py::getitem(&v_v.clone(), &Py::Int(2i64))?)?], true)?;
    v_x = (if ({ let t277 = py::abs(&py::getitem(&v_v.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t278 = py::mul(&Py::Float(1e-12), &v_S.clone())?; if py::le(&t277, &t278)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::getitem(&v_v.clone(), &Py::Int(0i64))? });
    v_y = (if ({ let t279 = py::abs(&py::getitem(&v_v.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t280 = py::mul(&Py::Float(1e-12), &v_S.clone())?; if py::le(&t279, &t280)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::getitem(&v_v.clone(), &Py::Int(1i64))? });
    v_z = (if ({ let t281 = py::abs(&py::getitem(&v_v.clone(), &Py::Int(2i64))?)?; Py::Bool({ let t282 = py::mul(&Py::Float(1e-12), &v_S.clone())?; if py::le(&t281, &t282)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::getitem(&v_v.clone(), &Py::Int(2i64))? });
    v_r = py::hypot(&[v_x.clone(), v_y.clone()])?;
    v_rho = py::hypot(&[v_r.clone(), v_z.clone()])?;
    v_th = (if ({ let t283 = v_r.clone(); Py::Bool({ let t284 = Py::Int(0i64); if py::gt(&t283, &t284)? { true } else { false } }) }).truthy() { py::atan2(&v_y.clone(), &v_x.clone())? } else { Py::Float(0.0) });
    if ({ let t285 = v_th.clone(); Py::Bool({ let t286 = Py::Int(0i64); if py::lt(&t285, &t286)? { true } else { false } }) }).truthy() {
        v_th = py::add(&v_th.clone(), &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?;
    }
    v_ph = (if ({ let t287 = v_rho.clone(); Py::Bool({ let t288 = Py::Int(0i64); if py::gt(&t287, &t288)? { true } else { false } }) }).truthy() { py::atan2(&v_r.clone(), &v_z.clone())? } else { Py::Float(0.0) });
    return Ok(Py::dict(vec![(Py::str("rectangular"), Py::list(vec![f__r12(v_x.clone())?, f__r12(v_y.clone())?, f__r12(v_z.clone())?])), (Py::str("cylindrical"), Py::list(vec![f__r12(v_r.clone())?, f__r12(v_th.clone())?, f__r12(v_z.clone())?])), (Py::str("spherical"), Py::list(vec![f__r12(v_rho.clone())?, f__r12(v_th.clone())?, f__r12(v_ph.clone())?]))]));
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
    if ({ let t313 = py::method(&v_p.clone(), "degree", &[])?; Py::Bool({ let t314 = Py::Int(2i64); if py::lt(&t313, &t314)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs a polynomial of degree 2 or more (a line has no isolated critical points)"))));
    }
    v_dp = py::method(&v_p.clone(), "diff", &[c_X()?])?;
    v_rs = f__distinct_real_roots(v_dp.clone())?;
    let t315 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t315.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t315[0].clone();
    v_mn = t315[1].clone();
    for t320 in py::iter(&(py::zip(&[v_rs.clone(), (if (v_rs.clone()).truthy() { f__side_signs(v_dp.clone(), v_rs.clone())? } else { Py::list(vec![]) })])?))? {
        let t327 = (t320).items()?;
        if t327.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_r = t327[0].clone();
        let t328 = (t327[1].clone()).items()?;
        if t328.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_left = t328[0].clone();
        v_right = t328[1].clone();
        if ({ let t321 = v_left.clone(); Py::Bool({ let t322 = Py::Int(0i64); if py::gt(&t321, &t322)? { { let t323 = v_right.clone(); if py::gt(&t322, &t323)? { true } else { false } } } else { false } }) }).truthy() {
            let _ = py::method(&v_mx.clone(), "append", &[f__pfloat(v_r.clone())?])?;
        } else {
            if ({ let t324 = v_left.clone(); Py::Bool({ let t325 = Py::Int(0i64); if py::lt(&t324, &t325)? { { let t326 = v_right.clone(); if py::lt(&t325, &t326)? { true } else { false } } } else { false } }) }).truthy() {
                let _ = py::method(&v_mn.clone(), "append", &[f__pfloat(v_r.clone())?])?;
            }
        }
    }
    return Ok(Py::dict(vec![(Py::str("critical_points"), { let mut out = vec![]; for t329 in py::iter(&(v_rs.clone()))? { let mut v_r: Py; 
v_r = t329;
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
    if ({ let t370 = py::len(&v_p.clone())?; Py::Bool({ let t371 = Py::Int(3i64); if py::ne(&t370, &t371) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the point needs 3 coordinates (x, y, z)"))));
    }
    let t377 = (f__field(v_P.clone(), v_Q.clone(), v_R.clone())?).items()?;
    if t377.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t378 = (t377[0].clone()).items()?;
    if t378.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_eP = t378[0].clone();
    v_eQ = t378[1].clone();
    v_eR = t378[2].clone();
    v_names = t377[1].clone();
    for t379 in py::iter(&(Py::tuple(vec![v_eP.clone(), v_eQ.clone(), v_eR.clone()])))? {
        v_c = t379;
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
    let t397 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?, Py::tuple(vec![c_X()?]), v_a.clone()], &[])?, ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?, Py::Int(2i64)], &[])?, Py::tuple(vec![c_X()?]), v_a.clone()], &[])?])).items()?;
    if t397.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d1 = t397[0].clone();
    v_d2 = t397[1].clone();
    v_k = py::truediv(&py::abs(&v_d2.clone())?, &py::pow(&py::add(&Py::Int(1i64), &py::mul(&v_d1.clone(), &v_d1.clone())?)?, &Py::Float(1.5))?)?;
    v_out = Py::dict(vec![(Py::str("curvature"), v_k.clone())]);
    if ({ let t398 = v_k.clone(); Py::Bool({ let t399 = Py::Int(0i64); if py::gt(&t398, &t399)? { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("radius_of_curvature"), py::truediv(&Py::Int(1i64), &v_k.clone())?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_curvilinear_triple_integral(mut v_f: Py, mut v_system: Py, mut v_u1: Py, mut v_u2: Py, mut v_th1: Py, mut v_th2: Py, mut v_w1: Py, mut v_w2: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_bx: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vol: Py = Py::None;
    if ({ let t408 = v_system.clone(); Py::Bool({ let t409 = Py::tuple(vec![Py::str("cylindrical"), Py::str("spherical")]); if !py::contains(&t409, &t408)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the system must be cylindrical or spherical"))));
    }
    if (py::not(&{ let t410 = Py::Int(0i64); Py::Bool({ let t411 = v_u1.clone(); if py::le(&t410, &t411)? { { let t412 = v_u2.clone(); if py::lt(&t411, &t412)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the radial limits need 0 <= lower < upper"))));
    }
    if (py::not(&{ let t413 = v_th1.clone(); Py::Bool({ let t414 = v_th2.clone(); if py::lt(&t413, &t414)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the angles need theta1 < theta2"))));
    }
    if ({ let t415 = py::sub(&v_th2.clone(), &v_th1.clone())?; Py::Bool({ let t416 = py::add(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &Py::Float(1e-12))?; if py::gt(&t415, &t416)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the theta range may span at most one full turn (2 pi)"))));
    }
    if (py::not(&{ let t417 = v_w1.clone(); Py::Bool({ let t418 = v_w2.clone(); if py::lt(&t417, &t418)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the third limits need lower < upper"))));
    }
    v_g = f__fl(ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), c__XYZ()?], &[])?, c__XYZ()?)?;
    if ({ let t419 = v_system.clone(); Py::Bool({ let t420 = Py::str("cylindrical"); if py::eq(&t419, &t420) { true } else { false } }) }).truthy() {
        v_h = { let v_f = v_f.clone(); let v_g = v_g.clone(); let v_system = v_system.clone(); let v_th1 = v_th1.clone(); let v_th2 = v_th2.clone(); let v_u1 = v_u1.clone(); let v_u2 = v_u2.clone(); let v_w1 = v_w1.clone(); let v_w2 = v_w2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_t: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_w: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&py::call(&v_g.clone(), &[py::mul(&v_u.clone(), &py::math1("cos", &v_t.clone())?)?, py::mul(&v_u.clone(), &py::math1("sin", &v_t.clone())?)?, v_w.clone()])?, &v_u.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
        v_vol = py::mul(&py::mul(&py::truediv(&py::sub(&py::mul(&v_u2.clone(), &v_u2.clone())?, &py::mul(&v_u1.clone(), &v_u1.clone())?)?, &Py::Int(2i64))?, &py::sub(&v_th2.clone(), &v_th1.clone())?)?, &py::sub(&v_w2.clone(), &v_w1.clone())?)?;
    } else {
        if ({ let t425 = { let t421 = v_w1.clone(); Py::Bool({ let t422 = Py::Int(0i64); if py::lt(&t421, &t422)? { true } else { false } }) }; if t425.truthy() { t425 } else { { let t423 = v_w2.clone(); Py::Bool({ let t424 = py::add(&Py::Float(std::f64::consts::PI), &Py::Float(1e-12))?; if py::gt(&t423, &t424)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("phi (from the positive z-axis) must stay between 0 and pi"))));
        }
        v_h = { let v_f = v_f.clone(); let v_g = v_g.clone(); let v_h = v_h.clone(); let v_system = v_system.clone(); let v_th1 = v_th1.clone(); let v_th2 = v_th2.clone(); let v_u1 = v_u1.clone(); let v_u2 = v_u2.clone(); let v_vol = v_vol.clone(); let v_w1 = v_w1.clone(); let v_w2 = v_w2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_t: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_w: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&py::mul(&py::mul(&py::call(&v_g.clone(), &[py::mul(&py::mul(&v_u.clone(), &py::math1("sin", &v_w.clone())?)?, &py::math1("cos", &v_t.clone())?)?, py::mul(&py::mul(&v_u.clone(), &py::math1("sin", &v_w.clone())?)?, &py::math1("sin", &v_t.clone())?)?, py::mul(&v_u.clone(), &py::math1("cos", &v_w.clone())?)?])?, &v_u.clone())?, &v_u.clone())?, &py::math1("sin", &v_w.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
        v_vol = py::mul(&py::mul(&py::truediv(&py::sub(&py::pow(&v_u2.clone(), &Py::Int(3i64))?, &py::pow(&v_u1.clone(), &Py::Int(3i64))?)?, &Py::Int(3i64))?, &py::sub(&v_th2.clone(), &v_th1.clone())?)?, &py::sub(&py::math1("cos", &v_w1.clone())?, &py::math1("cos", &v_w2.clone())?)?)?;
    }
    v_bx = Py::tuple(vec![Py::tuple(vec![v_u1.clone(), v_u2.clone()]), Py::tuple(vec![v_th1.clone(), v_th2.clone()]), Py::tuple(vec![v_w1.clone(), v_w2.clone()])]);
    let t457 = (f__settled({ let v_bx = v_bx.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_h = v_h.clone(); let v_system = v_system.clone(); let v_th1 = v_th1.clone(); let v_th2 = v_th2.clone(); let v_u1 = v_u1.clone(); let v_u2 = v_u2.clone(); let v_vol = v_vol.clone(); let v_w1 = v_w1.clone(); let v_w2 = v_w2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__gl3(v_h.clone(), v_bx.clone(), v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_bx = v_bx.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_h = v_h.clone(); let v_system = v_system.clone(); let v_th1 = v_th1.clone(); let v_th2 = v_th2.clone(); let v_u1 = v_u1.clone(); let v_u2 = v_u2.clone(); let v_vol = v_vol.clone(); let v_w1 = v_w1.clone(); let v_w2 = v_w2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__gl3({ let v_bx = v_bx.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_h = v_h.clone(); let v_n = v_n.clone(); let v_system = v_system.clone(); let v_th1 = v_th1.clone(); let v_th2 = v_th2.clone(); let v_u1 = v_u1.clone(); let v_u2 = v_u2.clone(); let v_vol = v_vol.clone(); let v_w1 = v_w1.clone(); let v_w2 = v_w2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_t: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_w: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::call(&v_h.clone(), &[v_u.clone(), v_t.clone(), v_w.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_bx.clone(), v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::tuple(vec![Py::Int(12i64), Py::Int(20i64), Py::Int(28i64)]))?).items()?;
    if t457.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t457[0].clone();
    v_S = t457[1].clone();
    return Ok(Py::dict(vec![(Py::str("integral"), f__snap_int(v_v.clone(), v_S.clone())?), (Py::str("region_volume"), f__r12(v_vol.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_definite_integral(mut v_f: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_e: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    if ({ let t472 = v_a.clone(); Py::Bool({ let t473 = v_b.clone(); if py::eq(&t472, &t473) { true } else { false } }) }).truthy() {
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
    if ({ let t490 = py::len(&v_u.clone())?; Py::Bool({ let t491 = py::len(&v_p.clone())?; if py::ne(&t490, &t491) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the direction needs as many components as the point"))));
    }
    v_nu = py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t492 in py::iter(&(v_u.clone()))? { let mut v_c: Py; 
v_c = t492;
out.push(py::mul(&v_c.clone(), &v_c.clone())?); } Py::list(out) })?)?;
    if ({ let t493 = v_nu.clone(); Py::Bool({ let t494 = Py::Int(0i64); if py::eq(&t493, &t494) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the direction vector must not be zero"))));
    }
    v_g = py::getitem(&f_gradient(v_f.clone(), v_p.clone())?, &Py::str("gradient"))?;
    return Ok(Py::dict(vec![(Py::str("directional_derivative"), py::truediv(&py::fsum(&{ let mut out = vec![]; for t501 in py::iter(&(py::zip(&[v_g.clone(), v_u.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t502 = (t501).items()?;
if t502.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t502[0].clone();
v_b = t502[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?, &v_nu.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_disk_volume(mut v_f: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_e: Py = Py::None;
    if (py::not(&{ let t519 = v_a.clone(); Py::Bool({ let t520 = v_b.clone(); if py::lt(&t519, &t520)? { true } else { false } }) })).truthy() {
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
    if ({ let t529 = py::len(&v_p.clone())?; Py::Bool({ let t530 = Py::Int(3i64); if py::ne(&t529, &t530) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the point needs 3 coordinates (x, y, z)"))));
    }
    let t536 = (f__field(v_P.clone(), v_Q.clone(), v_R.clone())?).items()?;
    if t536.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t536[0].clone();
    v_names = t536[1].clone();
    for t537 in py::iter(&(v_F.clone()))? {
        v_c = t537;
        let _ = { let mut v: Vec<Py> = vec![]; v.push(v_c.clone()); v.push(v_names.clone()); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? };
    }
    return Ok(Py::dict(vec![(Py::str("divergence"), py::fsum(&{ let mut out = vec![]; for t538 in py::iter(&(py::zip(&[v_F.clone(), Py::tuple(vec![c_X()?, c_Y()?, c_Z()?])])?))? { let mut v_c: Py; let mut v_s: Py; 
let t539 = (t538).items()?;
if t539.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t539[0].clone();
v_s = t539[1].clone();
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
    if (py::not(&{ let t561 = { let t554 = v_x0.clone(); Py::Bool({ let t555 = v_x1.clone(); if py::lt(&t554, &t555)? { true } else { false } }) }; if !t561.truthy() { t561 } else { { let t560 = { let t556 = v_y0.clone(); Py::Bool({ let t557 = v_y1.clone(); if py::lt(&t556, &t557)? { true } else { false } }) }; if !t560.truthy() { t560 } else { { let t558 = v_z0.clone(); Py::Bool({ let t559 = v_z1.clone(); if py::lt(&t558, &t559)? { true } else { false } }) } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the box needs x0 < x1, y0 < y1, z0 < z1"))));
    }
    let t567 = (f__field(v_P.clone(), v_Q.clone(), v_R.clone())?).items()?;
    if t567.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t568 = (t567[0].clone()).items()?;
    if t568.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_eP = t568[0].clone();
    v_eQ = t568[1].clone();
    v_eR = t568[2].clone();
    v__n = t567[1].clone();
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
    if (py::not(&{ let t610 = { let t606 = v_a.clone(); Py::Bool({ let t607 = v_b.clone(); if py::lt(&t606, &t607)? { true } else { false } }) }; if !t610.truthy() { t610 } else { { let t608 = v_c.clone(); Py::Bool({ let t609 = v_d.clone(); if py::lt(&t608, &t609)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the rectangle needs a < b and c < d"))));
    }
    v_g = f__fl(ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), c__XY()?], &[])?, c__XY()?)?;
    let t638 = (f__settled({ let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); py::func(move |a: &[Py]| -> R {
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
    if t638.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t638[0].clone();
    v__S = t638[1].clone();
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
    if (py::not(&{ let t644 = v_a.clone(); Py::Bool({ let t645 = v_b.clone(); if py::lt(&t644, &t645)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the x-interval needs a < b"))));
    }
    v_g = f__fl(ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), c__XY()?], &[])?, c__XY()?)?;
    let t646 = (Py::tuple(vec![f__fl(ax_engine::pysym::helper("calculus:parse_expr", &[v_g1.clone()], &[])?, Py::tuple(vec![Py::str("x")]))?, f__fl(ax_engine::pysym::helper("calculus:parse_expr", &[v_g2.clone()], &[])?, Py::tuple(vec![Py::str("x")]))?])).items()?;
    if t646.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t646[0].clone();
    v_hi = t646[1].clone();
    for t668 in py::iter(&(f__glf(Py::Int(64i64))?))? {
        let t671 = (t668).items()?;
        if t671.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t671[0].clone();
        v__w = t671[1].clone();
        v_xx = py::add(&py::truediv(&py::add(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?, &py::mul(&py::truediv(&py::sub(&v_b.clone(), &v_a.clone())?, &Py::Int(2i64))?, &v_x.clone())?)?;
        if ({ let t669 = py::call(&v_lo.clone(), &[v_xx.clone()])?; Py::Bool({ let t670 = py::add(&py::call(&v_hi.clone(), &[v_xx.clone()])?, &py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&py::call(&v_hi.clone(), &[v_xx.clone()])?)?], true)?)?)?; if py::gt(&t669, &t670)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the lower boundary must lie below the upper boundary on [a, b]"))));
        }
    }
    let t678 = (f__settled({ let v__w = v__w.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_g1 = v_g1.clone(); let v_g2 = v_g2.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_x = v_x.clone(); let v_xx = v_xx.clone(); py::func(move |a: &[Py]| -> R {
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
    if t678.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t678[0].clone();
    v__S = t678[1].clone();
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
    if (py::not(&{ let t692 = v_a.clone(); Py::Bool({ let t693 = v_b.clone(); if py::lt(&t692, &t693)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    v_p = ax_engine::pysym::helper("calculus:poly_of", &[v_f.clone()], &[])?;
    v_crit = (if ({ let t694 = py::method(&v_p.clone(), "degree", &[])?; Py::Bool({ let t695 = Py::Int(2i64); if py::ge(&t694, &t695)? { true } else { false } }) }).truthy() { { let mut out = vec![]; for t696 in py::iter(&(ax_engine::pysym::helper("calculus:real_roots_in", &[py::method(&v_p.clone(), "diff", &[c_X()?])?, v_a.clone(), v_b.clone()], &[])?))? { let mut v_r: Py; 
v_r = t696;
out.push(f__pfloat(v_r.clone())?); } Py::list(out) } } else { Py::list(vec![]) });
    v_vals = { let mut out = vec![]; for t697 in py::iter(&(py::add(&Py::list(vec![v_a.clone(), v_b.clone()]), &v_crit.clone())?))? { let mut v_v: Py; 
v_v = t697;
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
    if (py::not(&{ let t732 = v_L.clone(); Py::Bool({ let t733 = Py::Int(0i64); if py::gt(&t732, &t733)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the half-period L must be positive"))));
    }
    if (py::not(&{ let t734 = Py::Int(1i64); Py::Bool({ let t735 = v_k.clone(); if py::le(&t734, &t735)? { { let t736 = Py::Int(50i64); if py::le(&t735, &t736)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the order must be between 1 and 50"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    let _ = ax_engine::pysym::helper("calculus:continuous_on", &[v_e.clone(), c_X()?, py::neg(&v_L.clone())?, v_L.clone()], &[])?;
    v_g = ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[])?;
    v_pts = { let mut out = vec![]; for t737 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::mul(&Py::Int(2i64), &v_k.clone())?).i()?]) { let mut v_j: Py; 
v_j = t737;
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
    let t746 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), Py::tuple(vec![Py::str("t")])], &[])?, ax_engine::pysym::helper("calculus:parse_expr", &[v_g.clone()], &[])?])).items()?;
    if t746.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ef = t746[0].clone();
    v_eg = t746[1].clone();
    v_u = ax_engine::pysym::helper("calculus:at", &[v_eg.clone(), Py::tuple(vec![c_X()?]), v_x0.clone()], &[])?;
    let _ = ax_engine::pysym::helper("calculus:continuous_on", &[v_ef.clone(), c_T()?, v_a.clone(), v_u.clone()], &[])?;
    return Ok(Py::dict(vec![(Py::str("derivative"), py::mul(&ax_engine::pysym::helper("calculus:at", &[v_ef.clone(), Py::tuple(vec![c_T()?]), v_u.clone()], &[])?, &ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_eg.clone(), c_X()?], &[])?, Py::tuple(vec![c_X()?]), v_x0.clone()], &[])?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gamma_function(mut v_x: Py) -> R {
    let mut v_v: Py = Py::None;
    let t755: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_v = py::math1("gamma", &v_x.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t755 {
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
    if (py::not(&{ let t764 = Py::Int(1i64); Py::Bool({ let t765 = v_n.clone(); if py::le(&t764, &t765)? { { let t766 = Py::Int(64i64); if py::le(&t765, &t766)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of nodes must be between 1 and 64"))));
    }
    if (py::not(&{ let t767 = v_a.clone(); Py::Bool({ let t768 = v_b.clone(); if py::lt(&t767, &t768)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    v_g = ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[])?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        let t769 = (Py::tuple(vec![py::truediv(&py::add(&ax_engine::pymp::call("mpf", &[v_a.clone()], &[])?, &v_b.clone())?, &Py::Int(2i64))?, py::truediv(&py::sub(&ax_engine::pymp::call("mpf", &[v_b.clone()], &[])?, &v_a.clone())?, &Py::Int(2i64))?])).items()?;
        if t769.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t769[0].clone();
        v_r = t769[1].clone();
        v_approx = py::float(&py::mul(&v_r.clone(), &ax_engine::pymp::call("fsum", &[{ let mut out = vec![]; for t787 in py::iter(&(f__gauss_nodes(v_n.clone())?))? { let mut v_w: Py; let mut v_x: Py; 
let t788 = (t787).items()?;
if t788.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t788[0].clone();
v_w = t788[1].clone();
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
    if (py::not(&{ let t803 = Py::Int(1i64); Py::Bool({ let t804 = v_n.clone(); if py::le(&t803, &t804)? { { let t805 = Py::Int(500i64); if py::le(&t804, &t805)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be between 1 and 500"))));
    }
    let t806 = (Py::tuple(vec![f__frac(v_a.clone())?, f__frac(v_r.clone())?])).items()?;
    if t806.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t806[0].clone();
    v_R = t806[1].clone();
    return Ok(Py::dict(vec![(Py::str("sum"), py::float(&(if ({ let t807 = v_R.clone(); Py::Bool({ let t808 = Py::Int(1i64); if py::eq(&t807, &t808) { true } else { false } }) }).truthy() { py::mul(&v_n.clone(), &v_A.clone())? } else { py::truediv(&py::mul(&v_A.clone(), &py::sub(&Py::Int(1i64), &py::pow(&v_R.clone(), &v_n.clone())?)?)?, &py::sub(&Py::Int(1i64), &v_R.clone())?)? }))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_geometric_series_sum(mut v_a: Py, mut v_r: Py) -> R {
    if (py::not(&{ let t818 = py::abs(&v_r.clone())?; Py::Bool({ let t819 = Py::Int(1i64); if py::lt(&t818, &t819)? { true } else { false } }) })).truthy() {
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
    v_g = { let mut out = vec![]; for t1955 in py::iter(&(v_names.clone()))? { let mut v_v: Py; 
v_v = t1955;
out.push({ let mut v: Vec<Py> = vec![]; v.push(ax_engine::pysym::call("diff", &[v_e.clone(), py::getitem(&c__SYMS()?, &v_v.clone())?], &[])?); v.push(v_names.clone()); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? }); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("gradient"), v_g.clone()), (Py::str("magnitude"), py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t1956 in py::iter(&(v_g.clone()))? { let mut v_c: Py; 
v_c = t1956;
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
    if (py::not(&{ let t856 = { let t852 = v_a.clone(); Py::Bool({ let t853 = v_b.clone(); if py::lt(&t852, &t853)? { true } else { false } }) }; if !t856.truthy() { t856 } else { { let t854 = v_c.clone(); Py::Bool({ let t855 = v_d.clone(); if py::lt(&t854, &t855)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the rectangle needs a < b and c < d"))));
    }
    let t857 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:parse_expr", &[v_P.clone(), c__XY()?], &[])?, ax_engine::pysym::helper("calculus:parse_expr", &[v_Q.clone(), c__XY()?], &[])?])).items()?;
    if t857.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_eP = t857[0].clone();
    v_eQ = t857[1].clone();
    v_g = f__fl(py::sub(&ax_engine::pysym::call("diff", &[v_eQ.clone(), c_X()?], &[])?, &ax_engine::pysym::call("diff", &[v_eP.clone(), c_Y()?], &[])?)?, c__XY()?)?;
    let t885 = (f__settled({ let v_P = v_P.clone(); let v_Q = v_Q.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_eP = v_eP.clone(); let v_eQ = v_eQ.clone(); let v_g = v_g.clone(); py::func(move |a: &[Py]| -> R {
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
    if t885.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dbl = t885[0].clone();
    v_S = t885[1].clone();
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
    for t886 in py::iter(&(Py::tuple(vec![Py::tuple(vec![py::method(&v_eP.clone(), "subs", &[c_Y()?, f__rat(v_c.clone())?])?, c_X()?, v_a.clone(), v_b.clone(), Py::Int(1i64)]), Py::tuple(vec![py::method(&v_eQ.clone(), "subs", &[c_X()?, f__rat(v_b.clone())?])?, c_Y()?, v_c.clone(), v_d.clone(), Py::Int(1i64)]), Py::tuple(vec![py::method(&v_eP.clone(), "subs", &[c_Y()?, f__rat(v_d.clone())?])?, c_X()?, v_a.clone(), v_b.clone(), py::neg(&Py::Int(1i64))?]), Py::tuple(vec![py::method(&v_eQ.clone(), "subs", &[c_X()?, f__rat(v_a.clone())?])?, c_Y()?, v_c.clone(), v_d.clone(), py::neg(&Py::Int(1i64))?])])))? {
        let t889 = (t886).items()?;
        if t889.len() != 5 { return Err(PyErr::Value("unpack".into())); }
        v_e = t889[0].clone();
        v_v = t889[1].clone();
        v_lo = t889[2].clone();
        v_hi = t889[3].clone();
        v_s = t889[4].clone();
        v_k = py::call(&v_const.clone(), &[v_e.clone(), v_lo.clone(), v_hi.clone()])?;
        let _ = py::method(&v_parts.clone(), "append", &[py::mul(&v_s.clone(), &(if ({ let t887 = v_k.clone(); Py::Bool({ let t888 = Py::None; if !py::is(&t887, &t888) { true } else { false } }) }).truthy() { v_k.clone() } else { py::call(&v_side.clone(), &[v_e.clone(), v_v.clone(), v_lo.clone(), v_hi.clone()])? }))?])?;
    }
    v_circ = py::fsum(&v_parts.clone())?;
    if ({ let t890 = py::abs(&py::sub(&v_circ.clone(), &v_dbl.clone())?)?; Py::Bool({ let t891 = py::mul(&Py::Float(1e-08), &py::minmax(&[py::abs(&v_dbl.clone())?, py::mul(&Py::Float(0.001), &v_S.clone())?, Py::Float(1e-12)], true)?)?; if py::gt(&t890, &t891)? { true } else { false } }) }).truthy() {
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
    if ({ let t898 = py::len(&v_p.clone())?; Py::Bool({ let t899 = Py::Int(2i64); if py::ne(&t898, &t899) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("this test is for f(x, y) at a point (x, y)"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), c__XY()?], &[])?;
    let _ = { let mut v: Vec<Py> = vec![]; v.push(v_e.clone()); v.push(c__XY()?); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? };
    let t900 = (Py::tuple(vec![{ let mut v: Vec<Py> = vec![]; v.push(ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?); v.push(c__XY()?); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? }, { let mut v: Vec<Py> = vec![]; v.push(ax_engine::pysym::call("diff", &[v_e.clone(), c_Y()?], &[])?); v.push(c__XY()?); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? }])).items()?;
    if t900.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fx = t900[0].clone();
    v_fy = t900[1].clone();
    let t902 = ({ let mut out = vec![]; for t901 in py::iter(&(Py::tuple(vec![Py::tuple(vec![c_X()?, c_X()?]), Py::tuple(vec![c_Y()?, c_Y()?]), Py::tuple(vec![c_X()?, c_Y()?])])))? { let mut v_v: Py; 
v_v = t901;
out.push({ let mut v: Vec<Py> = vec![]; v.push({ let mut v: Vec<Py> = vec![]; v.push(v_e.clone()); v.extend((v_v.clone()).items()?); ax_engine::pysym::call("diff", &v, &[])? }); v.push(c__XY()?); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? }); } Py::list(out) }).items()?;
    if t902.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_fxx = t902[0].clone();
    v_fyy = t902[1].clone();
    v_fxy = t902[2].clone();
    v_scale = py::minmax(&[Py::Float(1.0), py::abs(&v_fxx.clone())?, py::abs(&v_fyy.clone())?, py::abs(&v_fxy.clone())?], true)?;
    if ({ let t903 = py::minmax(&[py::abs(&v_fx.clone())?, py::abs(&v_fy.clone())?], true)?; Py::Bool({ let t904 = py::mul(&Py::Float(1e-10), &v_scale.clone())?; if py::gt(&t903, &t904)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["(".to_string(), py::format(&py::getitem(&v_p.clone(), &Py::Int(0i64))?, "g")?, ", ".to_string(), py::format(&py::getitem(&v_p.clone(), &Py::Int(1i64))?, "g")?, ") is not a critical point (the gradient is (".to_string(), py::format(&v_fx.clone(), ".6g")?, ", ".to_string(), py::format(&v_fy.clone(), ".6g")?, "))".to_string()].concat()))));
    }
    v_D = py::sub(&py::mul(&v_fxx.clone(), &v_fyy.clone())?, &py::mul(&v_fxy.clone(), &v_fxy.clone())?)?;
    if ({ let t905 = py::abs(&v_D.clone())?; Py::Bool({ let t906 = py::mul(&py::mul(&Py::Float(1e-12), &v_scale.clone())?, &v_scale.clone())?; if py::le(&t905, &t906)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("D = 0: the second derivative test is inconclusive"))));
    }
    v_kind = (if ({ let t907 = v_D.clone(); Py::Bool({ let t908 = Py::Int(0i64); if py::lt(&t907, &t908)? { true } else { false } }) }).truthy() { Py::str("saddle point") } else { (if ({ let t909 = v_fxx.clone(); Py::Bool({ let t910 = Py::Int(0i64); if py::gt(&t909, &t910)? { true } else { false } }) }).truthy() { Py::str("local minimum") } else { Py::str("local maximum") }) });
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
    let t930 = (v_p.clone()).items()?;
    if t930.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x0 = t930[0].clone();
    v_y0 = t930[1].clone();
    v_scale = py::minmax(&[py::add(&Py::list(vec![Py::Float(1.0)]), &{ let mut out = vec![]; for t931 in py::iter(&(ax_engine::pysym::call("Add.make_args", &[ax_engine::pysym::call("expand", &[v_e.clone()], &[])?], &[])?))? { let mut v_t: Py; 
v_t = t931;
out.push(py::abs(&ax_engine::pysym::helper("calculus:at", &[v_t.clone(), c__XY()?, v_x0.clone(), v_y0.clone()], &[])?)?); } Py::list(out) })?], true)?;
    if ({ let t932 = py::abs(&ax_engine::pysym::helper("calculus:at", &[v_e.clone(), c__XY()?, v_x0.clone(), v_y0.clone()], &[])?)?; Py::Bool({ let t933 = py::mul(&Py::Float(1e-09), &v_scale.clone())?; if py::gt(&t932, &t933)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the point (".to_string(), py::format(&v_x0.clone(), "g")?, ", ".to_string(), py::format(&v_y0.clone(), "g")?, ") is not on the curve".to_string()].concat()))));
    }
    let t934 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?, c__XY()?, v_x0.clone(), v_y0.clone()], &[])?, ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_Y()?], &[])?, c__XY()?, v_x0.clone(), v_y0.clone()], &[])?])).items()?;
    if t934.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fx = t934[0].clone();
    v_fy = t934[1].clone();
    if ({ let t935 = py::abs(&v_fy.clone())?; Py::Bool({ let t936 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_fx.clone())?], true)?)?; if py::le(&t935, &t936)? { true } else { false } }) }).truthy() {
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
    if ({ let t955 = py::method(&v_p.clone(), "degree", &[])?; Py::Bool({ let t956 = Py::Int(2i64); if py::lt(&t955, &t956)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a polynomial of degree < 2 has no concavity changes"))));
    }
    v_d2 = py::method(&py::method(&v_p.clone(), "diff", &[c_X()?])?, "diff", &[c_X()?])?;
    if ({ let t959 = py::getattr(&v_d2.clone(), "is_zero")?; if t959.truthy() { t959 } else { { let t957 = py::method(&v_d2.clone(), "degree", &[])?; Py::Bool({ let t958 = Py::Int(1i64); if py::lt(&t957, &t958)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::dict(vec![(Py::str("inflection_points"), Py::list(vec![]))]));
    }
    v_rs = f__distinct_real_roots(v_d2.clone())?;
    return Ok(Py::dict(vec![(Py::str("inflection_points"), { let mut out = vec![]; for t964 in py::iter(&(py::zip(&[v_rs.clone(), (if (v_rs.clone()).truthy() { f__side_signs(v_d2.clone(), v_rs.clone())? } else { Py::list(vec![]) })])?))? { let mut v_left: Py; let mut v_r: Py; let mut v_right: Py; 
let t965 = (t964).items()?;
if t965.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t965[0].clone();
let t966 = (t965[1].clone()).items()?;
if t966.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_left = t966[0].clone();
v_right = t966[1].clone();
if ({ let t967 = py::mul(&v_left.clone(), &v_right.clone())?; Py::Bool({ let t968 = Py::Int(0i64); if py::lt(&t967, &t968)? { true } else { false } }) }).truthy() { out.push(f__pfloat(v_r.clone())?); }} Py::list(out) })]));
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
    let t1000 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:parse_expr", &[v_xu.clone(), v_names.clone()], &[])?, ax_engine::pysym::helper("calculus:parse_expr", &[v_yv.clone(), v_names.clone()], &[])?])).items()?;
    if t1000.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ex = t1000[0].clone();
    v_ey = t1000[1].clone();
    let t1001 = (Py::tuple(vec![py::getitem(&c__SYMS()?, &Py::str("u"))?, py::getitem(&c__SYMS()?, &Py::str("v"))?])).items()?;
    if t1001.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_U_ = t1001[0].clone();
    v_V_s = t1001[1].clone();
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
fn f_lagrange_multipliers(mut v_f: Py, mut v_g: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_G: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_cands: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_ex: Py = Py::None;
    let mut v_ey: Py = Py::None;
    let mut v_fx: Py = Py::None;
    let mut v_fy: Py = Py::None;
    let mut v_gx: Py = Py::None;
    let mut v_gxe: Py = Py::None;
    let mut v_gy: Py = Py::None;
    let mut v_gye: Py = Py::None;
    let mut v_has_max: Py = Py::None;
    let mut v_has_min: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_kind: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_ok: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_sg: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), c__XY()?], &[])?;
    let t1020 = (f__conic(v_g.clone())?).items()?;
    if t1020.len() != 7 { return Err(PyErr::Value("unpack".into())); }
    v_G = t1020[0].clone();
    v_A = t1020[1].clone();
    v_B = t1020[2].clone();
    v_C = t1020[3].clone();
    v_D = t1020[4].clone();
    v_E = t1020[5].clone();
    v_F = t1020[6].clone();
    if ({ let t1028 = { let t1021 = v_A.clone(); Py::Bool({ let t1022 = Py::Int(0i64); if py::eq(&t1021, &t1022) { true } else { false } }) }; if !t1028.truthy() { t1028 } else { { let t1027 = { let t1023 = v_B.clone(); Py::Bool({ let t1024 = Py::Int(0i64); if py::eq(&t1023, &t1024) { true } else { false } }) }; if !t1027.truthy() { t1027 } else { { let t1025 = v_C.clone(); Py::Bool({ let t1026 = Py::Int(0i64); if py::eq(&t1025, &t1026) { true } else { false } }) } } } } }).truthy() {
        if ({ let t1033 = { let t1029 = v_D.clone(); Py::Bool({ let t1030 = Py::Int(0i64); if py::eq(&t1029, &t1030) { true } else { false } }) }; if !t1033.truthy() { t1033 } else { { let t1031 = v_E.clone(); Py::Bool({ let t1032 = Py::Int(0i64); if py::eq(&t1031, &t1032) { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the constraint must involve x or y"))));
        }
        let t1055 = (f__lagrange_line(v_e.clone(), v_D.clone(), v_E.clone(), v_F.clone())?).items()?;
        if t1055.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_cands = t1055[0].clone();
        v_has_max = t1055[1].clone();
        v_has_min = t1055[2].clone();
        v_kind = Py::str("line");
    } else {
        if ({ let t1060 = { let t1056 = v_C.clone(); Py::Bool({ let t1057 = Py::Int(0i64); if py::eq(&t1056, &t1057) { true } else { false } }) }; if !t1060.truthy() { t1060 } else { { let t1058 = py::mul(&v_A.clone(), &v_B.clone())?; Py::Bool({ let t1059 = Py::Int(0i64); if py::gt(&t1058, &t1059)? { true } else { false } }) } } }).truthy() {
            v_cands = f__lagrange_ellipse(v_e.clone(), v_A.clone(), v_B.clone(), v_D.clone(), v_E.clone(), v_F.clone())?;
            let t1099 = (Py::tuple(vec![Py::Bool(true), Py::Bool(true)])).items()?;
            if t1099.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_has_max = t1099[0].clone();
            v_has_min = t1099[1].clone();
            v_kind = (if ({ let t1100 = v_A.clone(); Py::Bool({ let t1101 = v_B.clone(); if py::eq(&t1100, &t1101) { true } else { false } }) }).truthy() { Py::str("circle") } else { Py::str("ellipse") });
        } else {
            return Err(py::value_error(py::to_str(&Py::str("documented constraints: a line a*x + b*y = c, or a circle or ellipse with axes parallel to the coordinate axes"))));
        }
    }
    let t1102 = (Py::tuple(vec![ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?, ax_engine::pysym::call("diff", &[v_e.clone(), c_Y()?], &[])?, ax_engine::pysym::call("diff", &[v_G.clone(), c_X()?], &[])?, ax_engine::pysym::call("diff", &[v_G.clone(), c_Y()?], &[])?])).items()?;
    if t1102.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_ex = t1102[0].clone();
    v_ey = t1102[1].clone();
    v_gxe = t1102[2].clone();
    v_gye = t1102[3].clone();
    v_out = Py::dict(vec![(Py::str("constraint"), v_kind.clone())]);
    for t1103 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("max"), Py::Int(1i64), v_has_max.clone()]), Py::tuple(vec![Py::str("min"), py::neg(&Py::Int(1i64))?, v_has_min.clone()])])))? {
        let t1137 = (t1103).items()?;
        if t1137.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_key = t1137[0].clone();
        v_sg = t1137[1].clone();
        v_ok = t1137[2].clone();
        if (py::not(&v_ok.clone())).truthy() {
            continue;
        }
        v_best = f__extreme(v_cands.clone(), v_sg.clone())?;
        let t1121 = (Py::tuple(vec![py::getitem(&v_best.clone(), &Py::Int(0i64))?, py::getitem(&v_best.clone(), &Py::Int(1i64))?, py::getitem(&v_best.clone(), &Py::Int(2i64))?])).items()?;
        if t1121.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_v = t1121[0].clone();
        v_x = t1121[1].clone();
        v_y = t1121[2].clone();
        if ({ let t1122 = v_kind.clone(); Py::Bool({ let t1123 = Py::str("line"); if py::ne(&t1122, &t1123) { true } else { false } }) }).truthy() {
            let _ = f__check_nondegenerate(v_e.clone(), v_A.clone(), v_B.clone(), v_D.clone(), v_E.clone(), v_F.clone(), v_x.clone(), v_y.clone())?;
        }
        let t1130 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:at", &[v_ex.clone(), c__XY()?, v_x.clone(), v_y.clone()], &[])?, ax_engine::pysym::helper("calculus:at", &[v_ey.clone(), c__XY()?, v_x.clone(), v_y.clone()], &[])?])).items()?;
        if t1130.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_fx = t1130[0].clone();
        v_fy = t1130[1].clone();
        let t1131 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:at", &[v_gxe.clone(), c__XY()?, v_x.clone(), v_y.clone()], &[])?, ax_engine::pysym::helper("calculus:at", &[v_gye.clone(), c__XY()?, v_x.clone(), v_y.clone()], &[])?])).items()?;
        if t1131.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_gx = t1131[0].clone();
        v_gy = t1131[1].clone();
        let t1134 = ((if ({ let t1132 = py::abs(&v_gx.clone())?; Py::Bool({ let t1133 = py::abs(&v_gy.clone())?; if py::ge(&t1132, &t1133)? { true } else { false } }) }).truthy() { Py::tuple(vec![v_fx.clone(), v_gx.clone()]) } else { Py::tuple(vec![v_fy.clone(), v_gy.clone()]) })).items()?;
        if t1134.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_num = t1134[0].clone();
        v_den = t1134[1].clone();
        v_lam = (if ({ let t1135 = py::abs(&v_num.clone())?; Py::Bool({ let t1136 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_fx.clone())?, py::abs(&v_fy.clone())?], true)?)?; if py::le(&t1135, &t1136)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::truediv(&v_num.clone(), &v_den.clone())? });
        py::setitem(&v_out.clone(), &py::add(&v_key.clone(), &Py::str("_value"))?, f__r12(v_v.clone())?)?;
        py::setitem(&v_out.clone(), &py::add(&v_key.clone(), &Py::str("_point"))?, Py::list(vec![f__r12(v_x.clone())?, f__r12(v_y.clone())?]))?;
        py::setitem(&v_out.clone(), &py::add(&v_key.clone(), &Py::str("_lambda"))?, f__r12(v_lam.clone())?)?;
    }
    return Ok(v_out.clone());
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
    return Ok(Py::dict(vec![(Py::str("laplacian"), py::fsum(&{ let mut out = vec![]; for t1177 in py::iter(&(v_names.clone()))? { let mut v_v: Py; 
v_v = t1177;
out.push({ let mut v: Vec<Py> = vec![]; v.push(ax_engine::pysym::call("diff", &[v_e.clone(), py::getitem(&c__SYMS()?, &v_v.clone())?, Py::Int(2i64)], &[])?); v.push(v_names.clone()); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? }); } Py::list(out) })?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_left_riemann_sum(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_h: Py = Py::None;
    let t1199 = (f__grid(v_f.clone(), v_a.clone(), v_b.clone(), v_n.clone())?).items()?;
    if t1199.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_e = t1199[0].clone();
    v_h = t1199[1].clone();
    v_n = t1199[2].clone();
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        return Ok(Py::dict(vec![(Py::str("approximation"), py::float(&py::mul(&v_h.clone(), &ax_engine::pymp::call("fsum", &[f__values(v_e.clone(), { let mut out = vec![]; for t1201 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t1201;
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
    if (py::not(&{ let t1213 = Py::Int(0i64); Py::Bool({ let t1214 = v_n.clone(); if py::le(&t1213, &t1214)? { { let t1215 = Py::Int(200i64); if py::le(&t1214, &t1215)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the degree must be between 0 and 200"))));
    }
    let t1216 = (Py::tuple(vec![Py::Float(1.0), v_x.clone()])).items()?;
    if t1216.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p0 = t1216[0].clone();
    v_p1 = t1216[1].clone();
    if ({ let t1217 = v_n.clone(); Py::Bool({ let t1218 = Py::Int(0i64); if py::eq(&t1217, &t1218) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("P"), Py::Float(1.0))]));
    }
    for t1219 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_n.clone()).i()?]) {
        v_k = t1219;
        let t1220 = (Py::tuple(vec![v_p1.clone(), py::truediv(&py::sub(&py::mul(&py::mul(&py::add(&py::mul(&Py::Int(2i64), &v_k.clone())?, &Py::Int(1i64))?, &v_x.clone())?, &v_p1.clone())?, &py::mul(&v_k.clone(), &v_p0.clone())?)?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?])).items()?;
        if t1220.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p0 = t1220[0].clone();
        v_p1 = t1220[1].clone();
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
    if (py::not(&{ let t1266 = v_t0.clone(); Py::Bool({ let t1267 = v_t1.clone(); if py::lt(&t1266, &t1267)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the parameter must run forward (t0 < t1) for an integral with respect to arc length"))));
    }
    let t1268 = (f__path(v_xt.clone(), v_yt.clone())?).items()?;
    if t1268.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ex = t1268[0].clone();
    v_ey = t1268[1].clone();
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
    let t1282 = (f__path(v_xt.clone(), v_yt.clone())?).items()?;
    if t1282.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ex = t1282[0].clone();
    v_ey = t1282[1].clone();
    let t1284 = ({ let mut out = vec![]; for t1283 in py::iter(&(Py::tuple(vec![v_P.clone(), v_Q.clone()])))? { let mut v_c: Py; 
v_c = t1283;
out.push(py::method(&ax_engine::pysym::helper("calculus:parse_expr", &[v_c.clone(), c__XY()?], &[])?, "subs", &[Py::dict(vec![(c_X()?, v_ex.clone()), (c_Y()?, v_ey.clone())])])?); } Py::list(out) }).items()?;
    if t1284.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_eP = t1284[0].clone();
    v_eQ = t1284[1].clone();
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
    let t1300 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:at", &[v_e.clone(), Py::tuple(vec![c_X()?]), v_a.clone()], &[])?, ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?, Py::tuple(vec![c_X()?]), v_a.clone()], &[])?])).items()?;
    if t1300.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t1300[0].clone();
    v_m = t1300[1].clone();
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
    if (py::not(&{ let t1309 = v_a.clone(); Py::Bool({ let t1310 = v_b.clone(); if py::lt(&t1309, &t1310)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    v_p = ax_engine::pysym::helper("calculus:poly_of", &[v_f.clone()], &[])?;
    let t1311 = (Py::tuple(vec![f__rat(v_a.clone())?, f__rat(v_b.clone())?])).items()?;
    if t1311.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1311[0].clone();
    v_B = t1311[1].clone();
    v_m = py::truediv(&py::sub(&py::method(&v_p.clone(), "eval", &[v_B.clone()])?, &py::method(&v_p.clone(), "eval", &[v_A.clone()])?)?, &py::sub(&v_B.clone(), &v_A.clone())?)?;
    v_q = py::sub(&py::method(&v_p.clone(), "diff", &[c_X()?])?, &v_m.clone())?;
    if (py::getattr(&v_q.clone(), "is_zero")?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("f is linear: every c in (a, b) satisfies the mean value theorem"))));
    }
    return Ok(Py::dict(vec![(Py::str("secant_slope"), py::float(&v_m.clone())?), (Py::str("c_values"), { let mut out = vec![]; for t1312 in py::iter(&(ax_engine::pysym::helper("calculus:real_roots_in", &[v_q.clone(), v_a.clone(), v_b.clone()], &[])?))? { let mut v_c: Py; 
v_c = t1312;
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
    if (py::not(&{ let t1385 = Py::Int(1i64); Py::Bool({ let t1386 = v_n.clone(); if py::le(&t1385, &t1386)? { { let t1387 = Py::Int(50i64); if py::le(&t1386, &t1387)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of iterations must be between 1 and 50"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    let t1388 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[])?, ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?, Py::tuple(vec![c_X()?])], &[])?])).items()?;
    if t1388.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g = t1388[0].clone();
    v_dg = t1388[1].clone();
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        let t1389 = (Py::tuple(vec![ax_engine::pymp::call("mpf", &[v_x0.clone()], &[])?, Py::list(vec![])])).items()?;
        if t1389.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t1389[0].clone();
        v_its = t1389[1].clone();
        for t1390 in py::range_iter(&[(v_n.clone()).i()?]) {
            v__ = t1390;
            v_d = py::call(&v_dg.clone(), &[v_x.clone()])?;
            if ({ let t1391 = v_d.clone(); Py::Bool({ let t1392 = Py::Int(0i64); if py::eq(&t1391, &t1392) { true } else { false } }) }).truthy() {
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
    if ({ let t1403 = py::abs(&py::getitem(&v_t.clone(), &Py::str("slope"))?)?; Py::Bool({ let t1404 = Py::Float(1e-12); if py::lt(&t1403, &t1404)? { true } else { false } }) }).truthy() {
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
    if (py::not(&{ let t1413 = Py::Int(1i64); Py::Bool({ let t1414 = v_n.clone(); if py::le(&t1413, &t1414)? { { let t1415 = Py::Int(12i64); if py::le(&t1414, &t1415)? { true } else { false } } } else { false } }) })).truthy() {
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
    if (py::not(&{ let t1424 = v_h.clone(); Py::Bool({ let t1425 = Py::Int(0i64); if py::gt(&t1424, &t1425)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the step h must be positive"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    v_g = ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[])?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        let t1426 = (Py::tuple(vec![ax_engine::pymp::call("mpf", &[v_a.clone()], &[])?, ax_engine::pymp::call("mpf", &[v_h.clone()], &[])?])).items()?;
        if t1426.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_A = t1426[0].clone();
        v_H = t1426[1].clone();
        if ({ let t1427 = v_method.clone(); Py::Bool({ let t1428 = Py::str("forward"); if py::eq(&t1427, &t1428) { true } else { false } }) }).truthy() {
            v_d = py::truediv(&py::sub(&py::call(&v_g.clone(), &[py::add(&v_A.clone(), &v_H.clone())?])?, &py::call(&v_g.clone(), &[v_A.clone()])?)?, &v_H.clone())?;
        } else {
            if ({ let t1429 = v_method.clone(); Py::Bool({ let t1430 = Py::str("backward"); if py::eq(&t1429, &t1430) { true } else { false } }) }).truthy() {
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
    if (py::not(&{ let t1439 = v_t0.clone(); Py::Bool({ let t1440 = v_t1.clone(); if py::lt(&t1439, &t1440)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the parameter interval needs t0 < t1"))));
    }
    let t1441 = (f__path(v_xt.clone(), v_yt.clone())?).items()?;
    if t1441.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ex = t1441[0].clone();
    v_ey = t1441[1].clone();
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
    let t1455 = (f__path(v_xt.clone(), v_yt.clone())?).items()?;
    if t1455.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ex = t1455[0].clone();
    v_ey = t1455[1].clone();
    let t1456 = (Py::tuple(vec![ax_engine::pysym::call("diff", &[v_ex.clone(), c_T()?], &[])?, ax_engine::pysym::call("diff", &[v_ey.clone(), c_T()?], &[])?])).items()?;
    if t1456.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dx = t1456[0].clone();
    v_dy = t1456[1].clone();
    v_vx = ax_engine::pysym::helper("calculus:at", &[v_dx.clone(), Py::tuple(vec![c_T()?]), v_t0.clone()], &[])?;
    if ({ let t1457 = py::abs(&v_vx.clone())?; Py::Bool({ let t1458 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&ax_engine::pysym::helper("calculus:at", &[v_dy.clone(), Py::tuple(vec![c_T()?]), v_t0.clone()], &[])?)?], true)?)?; if py::le(&t1457, &t1458)? { true } else { false } }) }).truthy() {
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
    if ({ let t1472 = v_var.clone(); Py::Bool({ let t1473 = v_names.clone(); if !py::contains(&t1473, &t1472)? { true } else { false } }) }).truthy() {
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
    if (py::not(&{ let t1486 = v_al.clone(); Py::Bool({ let t1487 = v_be.clone(); if py::lt(&t1486, &t1487)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the angle interval needs alpha < beta"))));
    }
    if ({ let t1488 = py::sub(&v_be.clone(), &v_al.clone())?; Py::Bool({ let t1489 = py::add(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &Py::Float(1e-12))?; if py::gt(&t1488, &t1489)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the angle interval may span at most one full turn (2 pi)"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_r.clone(), Py::tuple(vec![Py::str("theta")])], &[])?;
    let _ = ax_engine::pysym::helper("calculus:continuous_on", &[v_e.clone(), c_TH()?, v_al.clone(), v_be.clone()], &[])?;
    return Ok(Py::dict(vec![(Py::str("area"), ax_engine::pysym::helper("calculus:quad", &[ax_engine::pysym::helper("calculus:_compile", &[py::truediv(&py::pow(&v_e.clone(), &Py::Int(2i64))?, &Py::Int(2i64))?, Py::tuple(vec![c_TH()?])], &[])?, v_al.clone(), v_be.clone()], &[])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_polar_double_integral(mut v_f: Py, mut v_r1: Py, mut v_r2: Py, mut v_al: Py, mut v_be: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_v: Py = Py::None;
    if (py::not(&{ let t1498 = Py::Int(0i64); Py::Bool({ let t1499 = v_r1.clone(); if py::le(&t1498, &t1499)? { { let t1500 = v_r2.clone(); if py::lt(&t1499, &t1500)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the radii need 0 <= r1 < r2"))));
    }
    if (py::not(&{ let t1501 = v_al.clone(); Py::Bool({ let t1502 = v_be.clone(); if py::lt(&t1501, &t1502)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the angles need theta1 < theta2"))));
    }
    if ({ let t1503 = py::sub(&v_be.clone(), &v_al.clone())?; Py::Bool({ let t1504 = py::add(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &Py::Float(1e-12))?; if py::gt(&t1503, &t1504)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the angle range may span at most one full turn (2 pi)"))));
    }
    v_g = f__fl(ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), c__XY()?], &[])?, c__XY()?)?;
    v_h = { let v_al = v_al.clone(); let v_be = v_be.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_r1 = v_r1.clone(); let v_r2 = v_r2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_th: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&py::call(&v_g.clone(), &[py::mul(&v_r.clone(), &py::math1("cos", &v_th.clone())?)?, py::mul(&v_r.clone(), &py::math1("sin", &v_th.clone())?)?])?, &v_r.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t1532 = (f__settled({ let v_al = v_al.clone(); let v_be = v_be.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_h = v_h.clone(); let v_r1 = v_r1.clone(); let v_r2 = v_r2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__gl2(v_h.clone(), v_r1.clone(), v_r2.clone(), { let v_al = v_al.clone(); let v_be = v_be.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_h = v_h.clone(); let v_n = v_n.clone(); let v_r1 = v_r1.clone(); let v_r2 = v_r2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_al.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_al = v_al.clone(); let v_be = v_be.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_h = v_h.clone(); let v_n = v_n.clone(); let v_r1 = v_r1.clone(); let v_r2 = v_r2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_be.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_al = v_al.clone(); let v_be = v_be.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_h = v_h.clone(); let v_r1 = v_r1.clone(); let v_r2 = v_r2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__gl2({ let v_al = v_al.clone(); let v_be = v_be.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_h = v_h.clone(); let v_n = v_n.clone(); let v_r1 = v_r1.clone(); let v_r2 = v_r2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_th: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::call(&v_h.clone(), &[v_r.clone(), v_th.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_r1.clone(), v_r2.clone(), { let v_al = v_al.clone(); let v_be = v_be.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_h = v_h.clone(); let v_n = v_n.clone(); let v_r1 = v_r1.clone(); let v_r2 = v_r2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_al.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_al = v_al.clone(); let v_be = v_be.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_h = v_h.clone(); let v_n = v_n.clone(); let v_r1 = v_r1.clone(); let v_r2 = v_r2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(v_be.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_n.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::tuple(vec![Py::Int(24i64), Py::Int(40i64), Py::Int(64i64)]))?).items()?;
    if t1532.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t1532[0].clone();
    v_S = t1532[1].clone();
    return Ok(Py::dict(vec![(Py::str("integral"), f__snap_int(v_v.clone(), v_S.clone())?), (Py::str("region_area"), f__r12(py::truediv(&py::mul(&py::sub(&py::mul(&v_r2.clone(), &v_r2.clone())?, &py::mul(&v_r1.clone(), &v_r1.clone())?)?, &py::sub(&v_be.clone(), &v_al.clone())?)?, &Py::Int(2i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_power_series_convergence(mut v_a: Py, mut v_k: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_Rx: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cx: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_kf: Py = Py::None;
    let mut v_kx: Py = Py::None;
    let mut v_lb: Py = Py::None;
    let mut v_left: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_rb: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_right: Py = Py::None;
    let mut v_rv: Py = Py::None;
    let mut v_sr: Py = Py::None;
    let mut v_xm: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_a.clone(), Py::tuple(vec![Py::str("n"), Py::str("x")])], &[])?;
    let t1573 = (f__pseries_parts(v_e.clone())?).items()?;
    if t1573.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_kx = t1573[0].clone();
    v_mx = t1573[1].clone();
    v_rho = t1573[2].clone();
    v_d = t1573[3].clone();
    let t1574 = (Py::tuple(vec![f__pfloat(v_rho.clone())?, f__pfloat(v_kx.clone())?])).items()?;
    if t1574.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rv = t1574[0].clone();
    v_kf = t1574[1].clone();
    v_Rx = py::truediv(&Py::Int(1i64), &py::mul(&ax_engine::pysym::call("Abs", &[v_rho.clone()], &[])?, &ax_engine::pysym::call("Abs", &[v_kx.clone()], &[])?)?)?;
    v_cx = py::truediv(&py::neg(&v_mx.clone())?, &v_kx.clone())?;
    let t1575 = (Py::tuple(vec![f__pfloat(v_Rx.clone())?, f__pfloat(v_cx.clone())?, f__pfloat(py::sub(&v_cx.clone(), &v_Rx.clone())?)?, f__pfloat(py::add(&v_cx.clone(), &v_Rx.clone())?)?])).items()?;
    if t1575.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_R = t1575[0].clone();
    v_c = t1575[1].clone();
    v_lo = t1575[2].clone();
    v_hi = t1575[3].clone();
    if ({ let t1576 = v_k.clone(); Py::Bool({ let t1577 = Py::None; if !py::is(&t1576, &t1577) { true } else { false } }) }).truthy() {
        v_k = py::int(&v_k.clone())?;
        if (py::not(&{ let t1578 = Py::Int(0i64); Py::Bool({ let t1579 = v_k.clone(); if py::le(&t1578, &t1579)? { { let t1580 = Py::Int(1000i64); if py::le(&t1579, &t1580)? { true } else { false } } } else { false } }) })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the starting index must be between 0 and 1000"))));
        }
        v_xm = f__pfloat(py::add(&v_cx.clone(), &py::truediv(&v_Rx.clone(), &Py::Int(2i64))?)?)?;
        for t1581 in py::range_iter(&[(v_k.clone()).i()?, (py::add(&v_k.clone(), &Py::Int(4i64))?).i()?]) {
            v_j = t1581;
            let _ = ax_engine::pysym::helper("calculus:at", &[v_e.clone(), Py::tuple(vec![Py::str("n"), Py::str("x")]), v_j.clone(), v_xm.clone()], &[])?;
        }
    }
    v_sr = py::mul(&(if ({ let t1582 = v_rv.clone(); Py::Bool({ let t1583 = Py::Int(0i64); if py::gt(&t1582, &t1583)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? }), &(if ({ let t1584 = v_kf.clone(); Py::Bool({ let t1585 = Py::Int(0i64); if py::gt(&t1584, &t1585)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? }))?;
    let t1593 = (Py::tuple(vec![f__end_verdict(v_sr.clone(), v_d.clone())?, f__end_verdict(py::neg(&v_sr.clone())?, v_d.clone())?])).items()?;
    if t1593.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_right = t1593[0].clone();
    v_left = t1593[1].clone();
    v_lb = (if ({ let t1594 = v_left.clone(); Py::Bool({ let t1595 = Py::str("diverges"); if py::eq(&t1594, &t1595) { true } else { false } }) }).truthy() { Py::str("(") } else { Py::str("[") });
    v_rb = (if ({ let t1596 = v_right.clone(); Py::Bool({ let t1597 = Py::str("diverges"); if py::eq(&t1596, &t1597) { true } else { false } }) }).truthy() { Py::str(")") } else { Py::str("]") });
    return Ok(Py::dict(vec![(Py::str("radius"), v_R.clone()), (Py::str("centre"), v_c.clone()), (Py::str("left_end"), v_lo.clone()), (Py::str("right_end"), v_hi.clone()), (Py::str("left_behaviour"), v_left.clone()), (Py::str("right_behaviour"), v_right.clone()), (Py::str("interval"), py::add(&py::add(&v_lb.clone(), &Py::str(&[py::format(&v_lo.clone(), ".12g")?, ", ".to_string(), py::format(&v_hi.clone(), ".12g")?].concat()))?, &v_rb.clone())?), (Py::str("endpoint_exponent"), py::float(&v_d.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ratio_test(mut v_a: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_a.clone(), Py::tuple(vec![Py::str("n")])], &[])?;
    if ({ let t1629 = { let t1625 = py::method(&v_e.clone(), "subs", &[c_NN()?, Py::Int(1i64)])?; Py::Bool({ let t1626 = Py::Int(0i64); if py::eq(&t1625, &t1626) { true } else { false } }) }; if !t1629.truthy() { t1629 } else { { let t1627 = py::method(&v_e.clone(), "subs", &[c_NN()?, Py::Int(2i64)])?; Py::Bool({ let t1628 = Py::Int(0i64); if py::eq(&t1627, &t1628) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the terms are zero"))));
    }
    v_r = ax_engine::pysym::call("simplify", &[py::truediv(&py::method(&v_e.clone(), "subs", &[c_NN()?, py::add(&c_NN()?, &Py::Int(1i64))?])?, &v_e.clone())?], &[])?;
    v_L = f__limval(f__sympy_limit(ax_engine::pysym::call("Abs", &[v_r.clone()], &[])?, c_NN()?, ax_engine::pysym::constant("oo")?, Py::None)?)?;
    if ({ let t1641 = py::abs(&py::sub(&v_L.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t1642 = Py::Float(1e-12); if py::lt(&t1641, &t1642)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the ratio limit is 1: the ratio test is inconclusive"))));
    }
    return Ok(Py::dict(vec![(Py::str("ratio_limit"), v_L.clone()), (Py::str("verdict"), (if ({ let t1643 = v_L.clone(); Py::Bool({ let t1644 = Py::Int(1i64); if py::lt(&t1643, &t1644)? { true } else { false } }) }).truthy() { Py::str("converges") } else { Py::str("diverges") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_regularized_gamma(mut v_a: Py, mut v_x: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    if ({ let t1653 = py::not(&{ let t1649 = v_a.clone(); Py::Bool({ let t1650 = Py::Int(0i64); if py::gt(&t1649, &t1650)? { true } else { false } }) }); if t1653.truthy() { t1653 } else { { let t1651 = v_x.clone(); Py::Bool({ let t1652 = Py::Int(0i64); if py::lt(&t1651, &t1652)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("this capability needs a > 0 and x >= 0"))));
    }
    if ({ let t1658 = { let t1654 = v_a.clone(); Py::Bool({ let t1655 = Py::Float(10000.0); if py::gt(&t1654, &t1655)? { true } else { false } }) }; if t1658.truthy() { t1658 } else { { let t1656 = v_x.clone(); Py::Bool({ let t1657 = Py::Float(100000.0); if py::gt(&t1656, &t1657)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a and x are limited to 1e4 and 1e5"))));
    }
    let t1675 = (f__reg_gamma(v_a.clone(), v_x.clone())?).items()?;
    if t1675.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t1675[0].clone();
    v_Q = t1675[1].clone();
    return Ok(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("Q"), v_Q.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rel_err(mut v_got: Py, mut v_want: Py, mut v_slack: Py) -> R {
    let mut v_d: Py = Py::None;
    let t2141: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_d = py::truediv(&py::abs(&py::sub(&py::complex(&[v_got.clone()])?, &py::complex(&[v_want.clone()])?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::complex(&[v_want.clone()])?)?], true)?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2141 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if ({ let t2142 = v_d.clone(); Py::Bool({ let t2143 = v_d.clone(); if py::ne(&t2142, &t2143) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::minmax(&[Py::Float(0.0), py::sub(&v_d.clone(), &v_slack.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_riemann_zeta(mut v_s: Py) -> R {
    if (py::not(&{ let t1684 = v_s.clone(); Py::Bool({ let t1685 = Py::Int(1i64); if py::gt(&t1684, &t1685)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("this capability covers s > 1 (where the defining series converges)"))));
    }
    if ({ let t1686 = v_s.clone(); Py::Bool({ let t1687 = Py::Float(10000.0); if py::gt(&t1686, &t1687)? { true } else { false } }) }).truthy() {
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
    let t1707 = (f__grid(v_f.clone(), v_a.clone(), v_b.clone(), v_n.clone())?).items()?;
    if t1707.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_e = t1707[0].clone();
    v_h = t1707[1].clone();
    v_n = t1707[2].clone();
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        return Ok(Py::dict(vec![(Py::str("approximation"), py::float(&py::mul(&v_h.clone(), &ax_engine::pymp::call("fsum", &[f__values(v_e.clone(), { let mut out = vec![]; for t1709 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1709;
out.push(py::add(&v_a.clone(), &py::mul(&v_k.clone(), &v_h.clone())?)?); } Py::list(out) })?], &[])?)?)?)]));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rms_value(mut v_f: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_mean: Py = Py::None;
    let mut v_ms: Py = Py::None;
    if (py::not(&{ let t1721 = v_a.clone(); Py::Bool({ let t1722 = v_b.clone(); if py::lt(&t1721, &t1722)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    let _ = ax_engine::pysym::helper("calculus:continuous_on", &[v_e.clone(), c_X()?, v_a.clone(), v_b.clone()], &[])?;
    v_ms = py::truediv(&ax_engine::pysym::helper("calculus:quad", &[ax_engine::pysym::helper("calculus:_compile", &[py::pow(&v_e.clone(), &Py::Int(2i64))?, Py::tuple(vec![c_X()?])], &[])?, v_a.clone(), v_b.clone()], &[])?, &py::sub(&v_b.clone(), &v_a.clone())?)?;
    v_mean = py::truediv(&ax_engine::pysym::helper("calculus:quad", &[ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[])?, v_a.clone(), v_b.clone()], &[])?, &py::sub(&v_b.clone(), &v_a.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("rms"), py::math1("sqrt", &py::minmax(&[v_ms.clone(), Py::Float(0.0)], true)?)?), (Py::str("mean_square"), v_ms.clone()), (Py::str("mean"), v_mean.clone())]));
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
    if (py::not(&{ let t1739 = Py::Int(0i64); Py::Bool({ let t1740 = v_k.clone(); if py::le(&t1739, &t1740)? { { let t1741 = Py::Int(1000i64); if py::le(&t1740, &t1741)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the starting index must be between 0 and 1000"))));
    }
    v_e = f__series_terms(v_a.clone())?;
    let _ = ax_engine::pysym::helper("calculus:at", &[py::method(&v_e.clone(), "subs", &[c__KS()?, v_k.clone()])?, Py::tuple(vec![])], &[])?;
    let t1742: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_S = ax_engine::pysym::call("summation", &[v_e.clone(), Py::tuple(vec![c__KS()?, v_k.clone(), ax_engine::pysym::constant("oo")?])], &[])?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1742 {
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
    if ({ let t1743 = py::method(&v_S.clone(), "has", &[ax_engine::pysym::constant("Sum")?])?; if t1743.truthy() { t1743 } else { py::method(&v_S.clone(), "has", &[ax_engine::pysym::constant("Piecewise")?])? } }).truthy() {
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
    if (py::not(&{ let t1759 = Py::Int(0i64); Py::Bool({ let t1760 = v_a.clone(); if py::le(&t1759, &t1760)? { { let t1761 = v_b.clone(); if py::lt(&t1760, &t1761)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the shell method about the y-axis needs 0 <= a < b"))));
    }
    v_p = ax_engine::pysym::helper("calculus:poly_of", &[v_f.clone()], &[])?;
    if (py::not(&f__nonneg_on(v_p.clone(), v_a.clone(), v_b.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the shell method needs f(x) >= 0 on [a, b]"))));
    }
    v_W = py::method(&py::method(&py::mul(&ax_engine::pysym::call("Poly", &[c_X()?, c_X()?], &[])?, &v_p.clone())?, "integrate", &[])?, "as_expr", &[])?;
    let t1768 = (Py::tuple(vec![ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_a.clone())?))], &[])?, ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_b.clone())?))], &[])?])).items()?;
    if t1768.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1768[0].clone();
    v_B = t1768[1].clone();
    return Ok(Py::dict(vec![(Py::str("volume"), py::float(&ax_engine::pysym::call("N", &[py::mul(&py::mul(&Py::Int(2i64), &ax_engine::pysym::constant("pi")?)?, &py::sub(&py::method(&v_W.clone(), "subs", &[c_X()?, v_B.clone()])?, &py::method(&v_W.clone(), "subs", &[c_X()?, v_A.clone()])?)?)?, Py::Int(30i64)], &[])?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_space_curve_arc_length(mut v_xt: Py, mut v_yt: Py, mut v_zt: Py, mut v_t0: Py, mut v_t1: Py) -> R {
    let mut v_ex: Py = Py::None;
    let mut v_ey: Py = Py::None;
    let mut v_ez: Py = Py::None;
    if (py::not(&{ let t1817 = v_t0.clone(); Py::Bool({ let t1818 = v_t1.clone(); if py::lt(&t1817, &t1818)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the parameter interval needs t0 < t1"))));
    }
    let t1819 = (f__path3(v_xt.clone(), v_yt.clone(), v_zt.clone())?).items()?;
    if t1819.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ex = t1819[0].clone();
    v_ey = t1819[1].clone();
    v_ez = t1819[2].clone();
    return Ok(Py::dict(vec![(Py::str("arc_length"), f__q_t(ax_engine::pysym::call("sqrt", &[py::add(&py::add(&py::pow(&ax_engine::pysym::call("diff", &[v_ex.clone(), c_T()?], &[])?, &Py::Int(2i64))?, &py::pow(&ax_engine::pysym::call("diff", &[v_ey.clone(), c_T()?], &[])?, &Py::Int(2i64))?)?, &py::pow(&ax_engine::pysym::call("diff", &[v_ez.clone(), c_T()?], &[])?, &Py::Int(2i64))?)?], &[])?, v_t0.clone(), v_t1.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_space_curve_curvature(mut v_xt: Py, mut v_yt: Py, mut v_zt: Py, mut v_t0: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cr: Py = Py::None;
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_E = f__path3(v_xt.clone(), v_yt.clone(), v_zt.clone())?;
    for t1833 in py::iter(&(v_E.clone()))? {
        v_c = t1833;
        let _ = ax_engine::pysym::helper("calculus:at", &[v_c.clone(), Py::tuple(vec![c_T()?]), v_t0.clone()], &[])?;
    }
    v_d1 = { let mut out = vec![]; for t1834 in py::iter(&(v_E.clone()))? { let mut v_c: Py; 
v_c = t1834;
out.push(ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_c.clone(), c_T()?], &[])?, Py::tuple(vec![c_T()?]), v_t0.clone()], &[])?); } Py::list(out) };
    v_d2 = { let mut out = vec![]; for t1835 in py::iter(&(v_E.clone()))? { let mut v_c: Py; 
v_c = t1835;
out.push(ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::call("diff", &[v_c.clone(), c_T()?, Py::Int(2i64)], &[])?, Py::tuple(vec![c_T()?]), v_t0.clone()], &[])?); } Py::list(out) };
    v_s = py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t1836 in py::iter(&(v_d1.clone()))? { let mut v_v: Py; 
v_v = t1836;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?)?;
    if ({ let t1837 = v_s.clone(); Py::Bool({ let t1838 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_d2.clone(), &Py::Int(0i64))?)?, py::abs(&py::getitem(&v_d2.clone(), &Py::Int(1i64))?)?, py::abs(&py::getitem(&v_d2.clone(), &Py::Int(2i64))?)?], true)?)?; if py::le(&t1837, &t1838)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("r'(t) = 0 there: the curve has no tangent direction (curvature undefined)"))));
    }
    v_cr = Py::list(vec![py::sub(&py::mul(&py::getitem(&v_d1.clone(), &Py::Int(1i64))?, &py::getitem(&v_d2.clone(), &Py::Int(2i64))?)?, &py::mul(&py::getitem(&v_d1.clone(), &Py::Int(2i64))?, &py::getitem(&v_d2.clone(), &Py::Int(1i64))?)?)?, py::sub(&py::mul(&py::getitem(&v_d1.clone(), &Py::Int(2i64))?, &py::getitem(&v_d2.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&v_d1.clone(), &Py::Int(0i64))?, &py::getitem(&v_d2.clone(), &Py::Int(2i64))?)?)?, py::sub(&py::mul(&py::getitem(&v_d1.clone(), &Py::Int(0i64))?, &py::getitem(&v_d2.clone(), &Py::Int(1i64))?)?, &py::mul(&py::getitem(&v_d1.clone(), &Py::Int(1i64))?, &py::getitem(&v_d2.clone(), &Py::Int(0i64))?)?)?]);
    v_k = py::truediv(&py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t1839 in py::iter(&(v_cr.clone()))? { let mut v_v: Py; 
v_v = t1839;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?)?, &py::pow(&v_s.clone(), &Py::Int(3i64))?)?;
    v_out = Py::dict(vec![(Py::str("curvature"), v_k.clone()), (Py::str("speed"), v_s.clone()), (Py::str("unit_tangent"), { let mut out = vec![]; for t1840 in py::iter(&(v_d1.clone()))? { let mut v_v: Py; 
v_v = t1840;
out.push(py::truediv(&v_v.clone(), &v_s.clone())?); } Py::list(out) })]);
    if ({ let t1841 = v_k.clone(); Py::Bool({ let t1842 = Py::Int(0i64); if py::gt(&t1841, &t1842)? { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("radius_of_curvature"), py::truediv(&Py::Int(1i64), &v_k.clone())?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_surface_area_graph(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py) -> R {
    let mut v__S: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_v: Py = Py::None;
    if (py::not(&{ let t1866 = { let t1862 = v_a.clone(); Py::Bool({ let t1863 = v_b.clone(); if py::lt(&t1862, &t1863)? { true } else { false } }) }; if !t1866.truthy() { t1866 } else { { let t1864 = v_c.clone(); Py::Bool({ let t1865 = v_d.clone(); if py::lt(&t1864, &t1865)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the rectangle needs a < b and c < d"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), c__XY()?], &[])?;
    v_g = f__fl(ax_engine::pysym::call("sqrt", &[py::add(&py::add(&Py::Int(1i64), &py::pow(&ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?, &Py::Int(2i64))?)?, &py::pow(&ax_engine::pysym::call("diff", &[v_e.clone(), c_Y()?], &[])?, &Py::Int(2i64))?)?], &[])?, c__XY()?)?;
    let t1894 = (f__settled({ let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_e = v_e.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); py::func(move |a: &[Py]| -> R {
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
    if t1894.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t1894[0].clone();
    v__S = t1894[1].clone();
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
    if (py::not(&{ let t1900 = v_a.clone(); Py::Bool({ let t1901 = v_b.clone(); if py::lt(&t1900, &t1901)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    v_d = ax_engine::pysym::call("diff", &[v_e.clone(), c_X()?], &[])?;
    let _ = ax_engine::pysym::helper("calculus:continuous_on", &[v_d.clone(), c_X()?, v_a.clone(), v_b.clone()], &[])?;
    v_g = ax_engine::pysym::helper("calculus:_compile", &[v_e.clone(), Py::tuple(vec![c_X()?])], &[(String::from("use_mp"), Py::Bool(false))])?;
    let t1902 = (Py::tuple(vec![Py::list(vec![]), Py::Int(400i64)])).items()?;
    if t1902.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cuts = t1902[0].clone();
    v_n = t1902[1].clone();
    v_xs = { let mut out = vec![]; for t1903 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1903;
out.push(py::add(&v_a.clone(), &py::truediv(&py::mul(&py::sub(&v_b.clone(), &v_a.clone())?, &v_k.clone())?, &v_n.clone())?)?); } Py::list(out) };
    v_ys = { let mut out = vec![]; for t1904 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t1904;
out.push(py::call(&v_g.clone(), &[v_x.clone()])?); } Py::list(out) };
    for t1905 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t1905;
        if ({ let t1910 = { let t1906 = py::getitem(&v_ys.clone(), &v_k.clone())?; Py::Bool({ let t1907 = Py::Int(0i64); if py::eq(&t1906, &t1907) { true } else { false } }) }; if !t1910.truthy() { t1910 } else { { let t1908 = Py::Int(0i64); Py::Bool({ let t1909 = v_k.clone(); if py::lt(&t1908, &t1909)? { true } else { false } }) } } }).truthy() {
            let _ = py::method(&v_cuts.clone(), "append", &[py::getitem(&v_xs.clone(), &v_k.clone())?])?;
        } else {
            if ({ let t1911 = py::mul(&py::getitem(&v_ys.clone(), &v_k.clone())?, &py::getitem(&v_ys.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t1912 = Py::Int(0i64); if py::lt(&t1911, &t1912)? { true } else { false } }) }).truthy() {
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
    if ({ let t1957 = py::len(&v_p.clone())?; Py::Bool({ let t1958 = Py::Int(2i64); if py::ne(&t1957, &t1958) { true } else { false } }) }).truthy() {
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
    if (py::not(&{ let t1968 = Py::Int(0i64); Py::Bool({ let t1969 = v_n.clone(); if py::le(&t1968, &t1969)? { { let t1970 = Py::Int(12i64); if py::le(&t1969, &t1970)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the degree must be between 0 and 12"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone()], &[])?;
    let t1971 = (Py::tuple(vec![Py::list(vec![]), v_e.clone()])).items()?;
    if t1971.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_co = t1971[0].clone();
    v_d = t1971[1].clone();
    for t1972 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t1972;
        let _ = py::method(&v_co.clone(), "append", &[py::truediv(&ax_engine::pysym::helper("calculus:at", &[v_d.clone(), Py::tuple(vec![c_X()?]), v_a.clone()], &[])?, &py::factorial(&v_k.clone())?)?])?;
        v_d = ax_engine::pysym::call("diff", &[v_d.clone(), c_X()?], &[])?;
    }
    let t1973 = (Py::tuple(vec![py::sub(&v_x1.clone(), &v_a.clone())?, Py::Float(0.0)])).items()?;
    if t1973.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_h = t1973[0].clone();
    v_val = t1973[1].clone();
    for t1974 in py::iter(&(py::reversed(&v_co.clone())?))? {
        v_c = t1974;
        v_val = py::add(&py::mul(&v_val.clone(), &v_h.clone())?, &v_c.clone())?;
    }
    v_fx1 = ax_engine::pysym::helper("calculus:at", &[v_e.clone(), Py::tuple(vec![c_X()?]), v_x1.clone()], &[])?;
    return Ok(Py::dict(vec![(Py::str("coefficients"), v_co.clone()), (Py::str("value"), v_val.clone()), (Py::str("function_value"), v_fx1.clone()), (Py::str("error"), py::sub(&v_fx1.clone(), &v_val.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_total_differential(mut v_f: Py, mut v_p: Py, mut v_d: Py, mut v_mode: Py) -> R {
    let mut v_df: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_f0: Py = Py::None;
    let mut v_grads: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_names = f__vars(v_p.clone())?;
    if ({ let t1996 = py::len(&v_d.clone())?; Py::Bool({ let t1997 = py::len(&v_p.clone())?; if py::ne(&t1996, &t1997) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one change for each variable"))));
    }
    if ({ let t1998 = v_mode.clone(); Py::Bool({ let t1999 = Py::tuple(vec![Py::str("change"), Py::str("percent")]); if !py::contains(&t1999, &t1998)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("mode must be change or percent"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), v_names.clone()], &[])?;
    v_f0 = { let mut v: Vec<Py> = vec![]; v.push(v_e.clone()); v.push(v_names.clone()); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? };
    v_grads = { let mut out = vec![]; for t2000 in py::iter(&(v_names.clone()))? { let mut v_v: Py; 
v_v = t2000;
out.push({ let mut v: Vec<Py> = vec![]; v.push(ax_engine::pysym::call("diff", &[v_e.clone(), py::getitem(&c__SYMS()?, &v_v.clone())?], &[])?); v.push(v_names.clone()); v.extend((v_p.clone()).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? }); } Py::list(out) };
    v_dx = (if ({ let t2001 = v_mode.clone(); Py::Bool({ let t2002 = Py::str("change"); if py::eq(&t2001, &t2002) { true } else { false } }) }).truthy() { { let mut out = vec![]; for t2003 in py::iter(&(v_d.clone()))? { let mut v_v: Py; 
v_v = t2003;
out.push(py::float(&v_v.clone())?); } Py::list(out) } } else { { let mut out = vec![]; for t2004 in py::range_iter(&[(py::len(&v_p.clone())?).i()?]) { let mut v_k: Py; 
v_k = t2004;
out.push(py::truediv(&py::mul(&py::getitem(&v_p.clone(), &v_k.clone())?, &py::getitem(&v_d.clone(), &v_k.clone())?)?, &Py::Int(100i64))?); } Py::list(out) } });
    v_df = py::fsum(&{ let mut out = vec![]; for t2005 in py::range_iter(&[(py::len(&v_p.clone())?).i()?]) { let mut v_k: Py; 
v_k = t2005;
out.push(py::mul(&py::getitem(&v_grads.clone(), &v_k.clone())?, &py::getitem(&v_dx.clone(), &v_k.clone())?)?); } Py::list(out) })?;
    v_out = Py::dict(vec![(Py::str("df"), v_df.clone()), (Py::str("f_value"), v_f0.clone()), (Py::str("partials"), v_grads.clone()), (Py::str("changes"), v_dx.clone())]);
    if ({ let t2006 = v_f0.clone(); Py::Bool({ let t2007 = Py::Int(0i64); if py::ne(&t2006, &t2007) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("percent_change"), py::truediv(&py::mul(&Py::Int(100i64), &v_df.clone())?, &v_f0.clone())?)?;
    }
    let t2009: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            py::setitem(&v_out.clone(), &Py::str("exact_change"), py::sub(&{ let mut v: Vec<Py> = vec![]; v.push(v_e.clone()); v.push(v_names.clone()); v.extend(({ let mut out = vec![]; for t2008 in py::range_iter(&[(py::len(&v_p.clone())?).i()?]) { let mut v_k: Py; 
v_k = t2008;
out.push(py::add(&py::getitem(&v_p.clone(), &v_k.clone())?, &py::getitem(&v_dx.clone(), &v_k.clone())?)?); } Py::list(out) }).items()?); ax_engine::pysym::helper("calculus:at", &v, &[])? }, &v_f0.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2009 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
        }
        Err(e) => return Err(e),
    }
    return Ok(v_out.clone());
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
    if (py::not(&{ let t2082 = { let t2075 = v_x0.clone(); Py::Bool({ let t2076 = v_x1.clone(); if py::lt(&t2075, &t2076)? { true } else { false } }) }; if !t2082.truthy() { t2082 } else { { let t2081 = { let t2077 = v_y0.clone(); Py::Bool({ let t2078 = v_y1.clone(); if py::lt(&t2077, &t2078)? { true } else { false } }) }; if !t2081.truthy() { t2081 } else { { let t2079 = v_z0.clone(); Py::Bool({ let t2080 = v_z1.clone(); if py::lt(&t2079, &t2080)? { true } else { false } }) } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the box needs x0 < x1, y0 < y1, z0 < z1"))));
    }
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[v_f.clone(), c__XYZ()?], &[])?;
    if (py::method(&v_e.clone(), "is_polynomial", &[c_X()?, c_Y()?, c_Z()?])?).truthy() {
        v_v = ax_engine::pysym::call("integrate", &[v_e.clone(), Py::tuple(vec![c_X()?, f__rat(v_x0.clone())?, f__rat(v_x1.clone())?]), Py::tuple(vec![c_Y()?, f__rat(v_y0.clone())?, f__rat(v_y1.clone())?]), Py::tuple(vec![c_Z()?, f__rat(v_z0.clone())?, f__rat(v_z1.clone())?])], &[])?;
        return Ok(Py::dict(vec![(Py::str("integral"), f__pfloat(v_v.clone())?)]));
    }
    v_g = f__fl(v_e.clone(), c__XYZ()?)?;
    v_bx = Py::tuple(vec![Py::tuple(vec![v_x0.clone(), v_x1.clone()]), Py::tuple(vec![v_y0.clone(), v_y1.clone()]), Py::tuple(vec![v_z0.clone(), v_z1.clone()])]);
    let t2114 = (f__settled({ let v_bx = v_bx.clone(); let v_e = v_e.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_v = v_v.clone(); let v_x0 = v_x0.clone(); let v_x1 = v_x1.clone(); let v_y0 = v_y0.clone(); let v_y1 = v_y1.clone(); let v_z0 = v_z0.clone(); let v_z1 = v_z1.clone(); py::func(move |a: &[Py]| -> R {
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
    if t2114.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t2114[0].clone();
    v__S = t2114[1].clone();
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
    let t2119 = (Py::tuple(vec![ax_engine::pysym::call("diff", &[v_e.clone(), c_T()?], &[])?, ax_engine::pysym::call("diff", &[v_e.clone(), c_T()?, Py::Int(2i64)], &[])?])).items()?;
    if t2119.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t2119[0].clone();
    v_acc = t2119[1].clone();
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
    if (py::not(&{ let t2128 = v_a.clone(); Py::Bool({ let t2129 = v_b.clone(); if py::lt(&t2128, &t2129)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval needs a < b"))));
    }
    let t2130 = (Py::tuple(vec![ax_engine::pysym::helper("calculus:poly_of", &[v_f.clone()], &[])?, ax_engine::pysym::helper("calculus:poly_of", &[v_g.clone()], &[])?])).items()?;
    if t2130.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pf = t2130[0].clone();
    v_pg = t2130[1].clone();
    if ({ let t2137 = py::not(&f__nonneg_on(v_pg.clone(), v_a.clone(), v_b.clone())?); if t2137.truthy() { t2137 } else { py::not(&f__nonneg_on(py::sub(&v_pf.clone(), &v_pg.clone())?, v_a.clone(), v_b.clone())?) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the washer method needs outer radius f >= inner radius g >= 0 on [a, b]"))));
    }
    v_W = py::method(&py::method(&py::sub(&py::pow(&v_pf.clone(), &Py::Int(2i64))?, &py::pow(&v_pg.clone(), &Py::Int(2i64))?)?, "integrate", &[])?, "as_expr", &[])?;
    let t2138 = (Py::tuple(vec![ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_a.clone())?))], &[])?, ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_b.clone())?))], &[])?])).items()?;
    if t2138.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2138[0].clone();
    v_B = t2138[1].clone();
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
fn s_100(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("area"))?, py::sum(&{ let mut out = vec![]; for t104 in py::iter(&(py::zip(&[py::slice(&py::getitem(&v_o.clone(), &Py::str("intersections"))?, None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, py::slice(&py::getitem(&v_o.clone(), &Py::str("intersections"))?, Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_hi: Py; let mut v_lo: Py; 
let t105 = (t104).items()?;
if t105.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_lo = t105[0].clone();
v_hi = t105[1].clone();
out.push(py::abs(&f__gl_check(Py::str(&["(".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("f"))?, "")?, ") - (".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("g"))?, "")?, ")".to_string()].concat()), v_lo.clone(), v_hi.clone(), Py::tuple(vec![Py::str("x")]), Py::None)?)?); } Py::list(out) }, None)?, Py::Float(1e-10))?, py::truediv(&py::minmax(&[{ let mut out = vec![]; for t106 in py::iter(&(py::getitem(&v_o.clone(), &Py::str("intersections"))?))? { let mut v_r: Py; 
v_r = t106;
out.push(py::abs(&ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::helper("calculus:parse_expr", &[Py::str(&["(".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("f"))?, "")?, ") - (".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("g"))?, "")?, ")".to_string()].concat())], &[])?, Py::tuple(vec![c_X()?]), v_r.clone()], &[])?)?); } Py::list(out) }], true)?, &Py::Float(1000.0))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1002(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1008: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1003(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1008 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1009 = v_r.clone(); Py::Bool({ let t1010 = v_r.clone(); if py::eq(&t1009, &t1010) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1003(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("jacobian"))?, py::float(&py::sub(&py::mul(&f__mp_partial(py::getitem(&v_i.clone(), &Py::str("xu"))?, Py::tuple(vec![Py::str("u"), Py::str("v")]), py::getitem(&v_i.clone(), &Py::str("p"))?, Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)]))?, &f__mp_partial(py::getitem(&v_i.clone(), &Py::str("yv"))?, Py::tuple(vec![Py::str("u"), Py::str("v")]), py::getitem(&v_i.clone(), &Py::str("p"))?, Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)]))?)?, &py::mul(&f__mp_partial(py::getitem(&v_i.clone(), &Py::str("xu"))?, Py::tuple(vec![Py::str("u"), Py::str("v")]), py::getitem(&v_i.clone(), &Py::str("p"))?, Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)]))?, &f__mp_partial(py::getitem(&v_i.clone(), &Py::str("yv"))?, Py::tuple(vec![Py::str("u"), Py::str("v")]), py::getitem(&v_i.clone(), &Py::str("p"))?, Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)]))?)?)?)?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1138(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1170: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_lagrange(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1170 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1171 = v_r.clone(); Py::Bool({ let t1172 = v_r.clone(); if py::eq(&t1171, &t1172) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_114(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t119: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_115(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t119 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t120 = v_r.clone(); Py::Bool({ let t121 = v_r.clone(); if py::eq(&t120, &t121) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_115(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("last_term"))?, py::add(&py::getitem(&v_i.clone(), &Py::str("a1"))?, &py::mul(&py::sub(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &Py::Int(1i64))?, &py::getitem(&v_i.clone(), &Py::str("d"))?)?)?, Py::Float(1e-12))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("sum"))?, py::truediv(&py::mul(&py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &py::add(&py::getitem(&v_i.clone(), &Py::str("a1"))?, &py::getitem(&v_o.clone(), &Py::str("last_term"))?)?)?, &Py::Int(2i64))?, Py::Float(1e-12))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1178(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1188: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1179(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1188 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1189 = v_r.clone(); Py::Bool({ let t1190 = v_r.clone(); if py::eq(&t1189, &t1190) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1179(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("laplacian"))?, py::fsum(&{ let mut out = vec![]; for t1183 in py::range_iter(&[(py::len(&py::getitem(&v_i.clone(), &Py::str("p"))?)?).i()?]) { let mut v_j: Py; 
v_j = t1183;
out.push(py::float(&f__mp_partial(py::getitem(&v_i.clone(), &Py::str("f"))?, f__vars(py::getitem(&v_i.clone(), &Py::str("p"))?)?, py::getitem(&v_i.clone(), &Py::str("p"))?, { let mut out = vec![]; for t1185 in py::range_iter(&[(py::len(&py::getitem(&v_i.clone(), &Py::str("p"))?)?).i()?]) { let mut v_k: Py; 
v_k = t1185;
out.push(py::mul(&Py::Int(2i64), &py::int(&{ let t1186 = v_k.clone(); Py::Bool({ let t1187 = v_j.clone(); if py::eq(&t1186, &t1187) { true } else { false } }) })?)?); } Py::list(out) })?)?); } Py::list(out) })?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1202(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1210: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1203(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1210 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1211 = v_r.clone(); Py::Bool({ let t1212 = v_r.clone(); if py::eq(&t1211, &t1212) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1203(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("approximation"))?, f__float_sum(py::getitem(&v_i.clone(), &Py::str("f"))?, { let mut out = vec![]; for t1209 in py::range_iter(&[(py::getitem(&v_i.clone(), &Py::str("n"))?).i()?]) { let mut v_k: Py; 
v_k = t1209;
out.push(py::add(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::truediv(&py::mul(&v_k.clone(), &py::sub(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?)?); } Py::list(out) }, py::mul(&Py::list(vec![py::truediv(&py::sub(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?]), &py::getitem(&v_i.clone(), &Py::str("n"))?)?)?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1221(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1226: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1222(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1226 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1227 = v_r.clone(); Py::Bool({ let t1228 = v_r.clone(); if py::eq(&t1227, &t1228) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1222(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("P"))?, py::float(&ax_engine::pymp::call("legendre", &[py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::getitem(&v_i.clone(), &Py::str("x"))?], &[])?)?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1240(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1244: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_limit_inf(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1244 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1245 = v_r.clone(); Py::Bool({ let t1246 = v_r.clone(); if py::eq(&t1245, &t1246) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_125(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t130: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_126(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t130 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t131 = v_r.clone(); Py::Bool({ let t132 = v_r.clone(); if py::eq(&t131, &t132) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1258(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1263: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_limit_pt(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1263 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1264 = v_r.clone(); Py::Bool({ let t1265 = v_r.clone(); if py::eq(&t1264, &t1265) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_126(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::mul(&py::getitem(&v_o.clone(), &Py::str("average_rate"))?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, py::sub(&py::getitem(&v_o.clone(), &Py::str("fb"))?, &py::getitem(&v_o.clone(), &Py::str("fa"))?)?, Py::Float(1e-13))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1271(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1279: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1272(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1279 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1280 = v_r.clone(); Py::Bool({ let t1281 = v_r.clone(); if py::eq(&t1280, &t1281) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1272(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_v: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        v_v = (if ({ let t1274 = py::getitem(&v_i.clone(), &Py::str("t0"))?; Py::Bool({ let t1275 = py::getitem(&v_i.clone(), &Py::str("t1"))?; if py::ne(&t1274, &t1275) { true } else { false } }) }).truthy() { ax_engine::pysym::helper("calculus:_gl", &[ax_engine::pysym::helper("calculus:_compile", &[f__scal_integrand(v_i.clone())?, Py::tuple(vec![c_T()?])], &[])?, py::getitem(&v_i.clone(), &Py::str("t0"))?, py::getitem(&v_i.clone(), &Py::str("t1"))?], &[])? } else { Py::Int(0i64) });
    }
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("line_integral"))?, py::float(&v_v.clone())?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1287(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1297: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1288(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1297 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1298 = v_r.clone(); Py::Bool({ let t1299 = v_r.clone(); if py::eq(&t1298, &t1299) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1288(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_v: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        v_v = (if ({ let t1292 = py::getitem(&v_i.clone(), &Py::str("t0"))?; Py::Bool({ let t1293 = py::getitem(&v_i.clone(), &Py::str("t1"))?; if py::ne(&t1292, &t1293) { true } else { false } }) }).truthy() { ax_engine::pysym::helper("calculus:_gl", &[ax_engine::pysym::helper("calculus:_compile", &[f__vec_integrand(v_i.clone())?, Py::tuple(vec![c_T()?])], &[])?, py::getitem(&v_i.clone(), &Py::str("t0"))?, py::getitem(&v_i.clone(), &Py::str("t1"))?], &[])? } else { Py::Int(0i64) });
    }
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("line_integral"))?, py::float(&v_v.clone())?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1301(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1306: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1302(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1306 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1307 = v_r.clone(); Py::Bool({ let t1308 = v_r.clone(); if py::eq(&t1307, &t1308) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1302(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("slope"))?, ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?], &[])?, Py::Float(1e-12))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("approximation"))?, py::add(&ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?, Py::tuple(vec![c_X()?]), py::getitem(&v_i.clone(), &Py::str("a"))?], &[])?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("slope"))?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("x1"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?)?, Py::Float(1e-13))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1313(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1342: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_mvt(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1342 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1343 = v_r.clone(); Py::Bool({ let t1344 = v_r.clone(); if py::eq(&t1343, &t1344) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1345(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v_approx: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_exact: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t1354 = (f__grid(v_f.clone(), v_a.clone(), v_b.clone(), v_n.clone())?).items()?;
    if t1354.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_e = t1354[0].clone();
    v_h = t1354[1].clone();
    v_n = t1354[2].clone();
    if ({ let t1357 = { let t1355 = Py::str("midpoint"); Py::Bool({ let t1356 = Py::str("simpson"); if py::eq(&t1355, &t1356) { true } else { false } }) }; if !t1357.truthy() { t1357 } else { py::modulo(&v_n.clone(), &Py::Int(2i64))? } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Simpson's rule needs an even number of subintervals"))));
    }
    let t1370 = (f__rule_n(v_e.clone(), v_a.clone(), v_b.clone(), v_n.clone(), Py::str("midpoint"))?).items()?;
    if t1370.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t1370[0].clone();
    v_w = t1370[1].clone();
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        v_approx = py::float(&ax_engine::pymp::call("fsum", &[{ let mut out = vec![]; for t1372 in py::iter(&(py::zip(&[v_w.clone(), f__values(v_e.clone(), v_pts.clone())?])?))? { let mut v_v: Py; let mut v_wi: Py; 
let t1373 = (t1372).items()?;
if t1373.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_wi = t1373[0].clone();
v_v = t1373[1].clone();
out.push(py::mul(&v_wi.clone(), &v_v.clone())?); } Py::list(out) }], &[])?)?;
    }
    v_exact = f__exact(v_e.clone(), v_a.clone(), v_b.clone())?;
    return Ok(Py::dict(vec![(Py::str("approximation"), v_approx.clone()), (Py::str("exact"), v_exact.clone()), (Py::str("error"), py::sub(&v_approx.clone(), &v_exact.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_137(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t142: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_138(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t142 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t143 = v_r.clone(); Py::Bool({ let t144 = v_r.clone(); if py::eq(&t143, &t144) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1374(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1382: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1375(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1382 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1383 = v_r.clone(); Py::Bool({ let t1384 = v_r.clone(); if py::eq(&t1383, &t1384) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1375(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?;
    let t1376 = (f__rule_n(v_e.clone(), py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, Py::str("midpoint"))?).items()?;
    if t1376.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t1376[0].clone();
    v_w = t1376[1].clone();
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("approximation"))?, f__float_sum(py::getitem(&v_i.clone(), &Py::str("f"))?, v_pts.clone(), v_w.clone())?, Py::Float(1e-11))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("exact"))?, f__gl_check(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?, Py::Float(1e-10))?, py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("error"))?, &py::sub(&py::getitem(&v_o.clone(), &Py::str("approximation"))?, &py::getitem(&v_o.clone(), &Py::str("exact"))?)?)?)?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_138(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("average_value"))?, py::truediv(&f__gl_check(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1393(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1400: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1394(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1400 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1401 = v_r.clone(); Py::Bool({ let t1402 = v_r.clone(); if py::eq(&t1401, &t1402) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1394(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__verify_newton(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("x0"))?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1405(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1410: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1406(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1410 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1411 = v_r.clone(); Py::Bool({ let t1412 = v_r.clone(); if py::eq(&t1411, &t1412) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1406(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::mul(&py::getitem(&v_o.clone(), &Py::str("slope"))?, &py::float(&ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?], &[])?)?)?, py::neg(&Py::Float(1.0))?, Py::Float(1e-11))?, f_rel_err(py::add(&py::getitem(&v_o.clone(), &Py::str("intercept"))?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("slope"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?, Py::tuple(vec![c_X()?]), py::getitem(&v_i.clone(), &Py::str("a"))?], &[])?, Py::Float(1e-12))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1416(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1421: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1417(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1421 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1422 = v_r.clone(); Py::Bool({ let t1423 = v_r.clone(); if py::eq(&t1422, &t1423) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1417(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("nth_derivative"))?, ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?], &[])?, Py::Float(1e-09))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1431(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1436: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1432(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1436 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1437 = v_r.clone(); Py::Bool({ let t1438 = v_r.clone(); if py::eq(&t1437, &t1438) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1432(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_1444(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1452: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1445(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1452 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1453 = v_r.clone(); Py::Bool({ let t1454 = v_r.clone(); if py::eq(&t1453, &t1454) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1445(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_v: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        v_v = (if ({ let t1447 = py::getitem(&v_i.clone(), &Py::str("t0"))?; Py::Bool({ let t1448 = py::getitem(&v_i.clone(), &Py::str("t1"))?; if py::ne(&t1447, &t1448) { true } else { false } }) }).truthy() { ax_engine::pysym::helper("calculus:_gl", &[ax_engine::pysym::helper("calculus:_compile", &[s_1446(v_i.clone())?, Py::tuple(vec![c_T()?])], &[])?, py::getitem(&v_i.clone(), &Py::str("t0"))?, py::getitem(&v_i.clone(), &Py::str("t1"))?], &[])? } else { Py::Int(0i64) });
    }
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("arc_length"))?, py::float(&v_v.clone())?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1446(mut v_i: Py) -> R {
    return Ok(ax_engine::pysym::call("sqrt", &[py::add(&py::pow(&ax_engine::pysym::call("diff", &[py::getitem(&f__path(py::getitem(&v_i.clone(), &Py::str("xt"))?, py::getitem(&v_i.clone(), &Py::str("yt"))?)?, &Py::Int(0i64))?, c_T()?], &[])?, &Py::Int(2i64))?, &py::pow(&ax_engine::pysym::call("diff", &[py::getitem(&f__path(py::getitem(&v_i.clone(), &Py::str("xt"))?, py::getitem(&v_i.clone(), &Py::str("yt"))?)?, &Py::Int(1i64))?, c_T()?], &[])?, &Py::Int(2i64))?)?], &[])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1459(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1465: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_pslope(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1465 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1466 = v_r.clone(); Py::Bool({ let t1467 = v_r.clone(); if py::eq(&t1466, &t1467) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1474(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1483: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1475(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1483 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1484 = v_r.clone(); Py::Bool({ let t1485 = v_r.clone(); if py::eq(&t1484, &t1485) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1475(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("partial"))?, py::float(&f__mp_partial(py::getitem(&v_i.clone(), &Py::str("f"))?, f__vars(py::getitem(&v_i.clone(), &Py::str("p"))?)?, py::getitem(&v_i.clone(), &Py::str("p"))?, { let mut out = vec![]; for t1480 in py::iter(&(f__vars(py::getitem(&v_i.clone(), &Py::str("p"))?)?))? { let mut v_v: Py; 
v_v = t1480;
out.push(py::int(&{ let t1481 = v_v.clone(); Py::Bool({ let t1482 = py::getitem(&v_i.clone(), &Py::str("var"))?; if py::eq(&t1481, &t1482) { true } else { false } }) })?); } Py::list(out) })?)?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1490(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1495: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1491(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1495 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1496 = v_r.clone(); Py::Bool({ let t1497 = v_r.clone(); if py::eq(&t1496, &t1497) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1491(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("area"))?, f__gl_check(Py::str(&["(".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("r"))?, "")?, ")^2/2".to_string()].concat()), py::getitem(&v_i.clone(), &Py::str("al"))?, py::getitem(&v_i.clone(), &Py::str("be"))?, Py::tuple(vec![Py::str("theta")]), Py::None)?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_151(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t156: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_152(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t156 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t157 = v_r.clone(); Py::Bool({ let t158 = v_r.clone(); if py::eq(&t157, &t158) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_152(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_1535(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1540: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_polar(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1540 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1541 = v_r.clone(); Py::Bool({ let t1542 = v_r.clone(); if py::eq(&t1541, &t1542) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1598(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1622: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_pseries(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1622 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1623 = v_r.clone(); Py::Bool({ let t1624 = v_r.clone(); if py::eq(&t1623, &t1624) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_164(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t169: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_165(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t169 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t170 = v_r.clone(); Py::Bool({ let t171 = v_r.clone(); if py::eq(&t170, &t171) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1645(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1646: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_ratio(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1646 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1647 = v_r.clone(); Py::Bool({ let t1648 = v_r.clone(); if py::eq(&t1647, &t1648) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_165(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("beta"))?, py::float(&ax_engine::pymp::call("beta", &[py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?], &[])?)?, Py::Float(1e-12))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1676(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1681: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1677(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1681 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1682 = v_r.clone(); Py::Bool({ let t1683 = v_r.clone(); if py::eq(&t1682, &t1683) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1677(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("P"))?, py::float(&ax_engine::pymp::call("gammainc", &[py::getitem(&v_i.clone(), &Py::str("a"))?, Py::Int(0i64), py::getitem(&v_i.clone(), &Py::str("x"))?], &[(String::from("regularized"), Py::Bool(true))])?)?, Py::Float(1e-11))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("Q"))?, py::float(&ax_engine::pymp::call("gammainc", &[py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("x"))?, ax_engine::pymp::constant("inf")?], &[(String::from("regularized"), Py::Bool(true))])?)?, Py::Float(1e-11))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1688(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1696: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1689(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1696 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1697 = v_r.clone(); Py::Bool({ let t1698 = v_r.clone(); if py::eq(&t1697, &t1698) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1689(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("zeta"))?, f__zeta_em(py::getitem(&v_i.clone(), &Py::str("s"))?, Py::Int(12i64))?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1710(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1718: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1711(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1718 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1719 = v_r.clone(); Py::Bool({ let t1720 = v_r.clone(); if py::eq(&t1719, &t1720) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1711(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("approximation"))?, f__float_sum(py::getitem(&v_i.clone(), &Py::str("f"))?, { let mut out = vec![]; for t1717 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::getitem(&v_i.clone(), &Py::str("n"))?, &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1717;
out.push(py::add(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::truediv(&py::mul(&v_k.clone(), &py::sub(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?)?); } Py::list(out) }, py::mul(&Py::list(vec![py::truediv(&py::sub(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?]), &py::getitem(&v_i.clone(), &Py::str("n"))?)?)?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1723(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1728: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1724(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1728 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1729 = v_r.clone(); Py::Bool({ let t1730 = v_r.clone(); if py::eq(&t1729, &t1730) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1724(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("mean_square"))?, py::truediv(&f__gl_check(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), { let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_g: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_x: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::pow(&py::call(&v_g.clone(), &[v_x.clone()])?, &Py::Int(2i64))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) })?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, Py::Float(1e-10))?, f_rel_err(py::pow(&py::getitem(&v_o.clone(), &Py::str("rms"))?, &Py::Int(2i64))?, py::getitem(&v_o.clone(), &Py::str("mean_square"))?, Py::Float(1e-12))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("mean"))?, py::truediv(&f__gl_check(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, Py::Float(1e-10))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1731(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1736: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1732(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1736 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1737 = v_r.clone(); Py::Bool({ let t1738 = v_r.clone(); if py::eq(&t1737, &t1738) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1732(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("second_derivative"))?, ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, Py::Int(2i64)], &[])?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1751(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1756: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1752(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1756 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1757 = v_r.clone(); Py::Bool({ let t1758 = v_r.clone(); if py::eq(&t1757, &t1758) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1752(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("sum"))?, py::float(&ax_engine::pysym::helper("calculus:_nsum", &[py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("k"))?], &[])?)?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1769(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1774: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1770(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1774 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1775 = v_r.clone(); Py::Bool({ let t1776 = v_r.clone(); if py::eq(&t1775, &t1776) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1770(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("volume"))?, py::mul(&py::mul(&Py::Int(2i64), &ax_engine::pymp::constant("pi")?)?, &f__gl_check(Py::str(&["x*(".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("f"))?, "")?, ")".to_string()].concat()), py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?)?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1777(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v_approx: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_exact: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t1786 = (f__grid(v_f.clone(), v_a.clone(), v_b.clone(), v_n.clone())?).items()?;
    if t1786.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_e = t1786[0].clone();
    v_h = t1786[1].clone();
    v_n = t1786[2].clone();
    if ({ let t1789 = { let t1787 = Py::str("simpson"); Py::Bool({ let t1788 = Py::str("simpson"); if py::eq(&t1787, &t1788) { true } else { false } }) }; if !t1789.truthy() { t1789 } else { py::modulo(&v_n.clone(), &Py::Int(2i64))? } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Simpson's rule needs an even number of subintervals"))));
    }
    let t1802 = (f__rule_n(v_e.clone(), v_a.clone(), v_b.clone(), v_n.clone(), Py::str("simpson"))?).items()?;
    if t1802.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t1802[0].clone();
    v_w = t1802[1].clone();
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        v_approx = py::float(&ax_engine::pymp::call("fsum", &[{ let mut out = vec![]; for t1804 in py::iter(&(py::zip(&[v_w.clone(), f__values(v_e.clone(), v_pts.clone())?])?))? { let mut v_v: Py; let mut v_wi: Py; 
let t1805 = (t1804).items()?;
if t1805.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_wi = t1805[0].clone();
v_v = t1805[1].clone();
out.push(py::mul(&v_wi.clone(), &v_v.clone())?); } Py::list(out) }], &[])?)?;
    }
    v_exact = f__exact(v_e.clone(), v_a.clone(), v_b.clone())?;
    return Ok(Py::dict(vec![(Py::str("approximation"), v_approx.clone()), (Py::str("exact"), v_exact.clone()), (Py::str("error"), py::sub(&v_approx.clone(), &v_exact.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1806(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1814: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1807(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1814 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1815 = v_r.clone(); Py::Bool({ let t1816 = v_r.clone(); if py::eq(&t1815, &t1816) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1807(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?;
    let t1808 = (f__rule_n(v_e.clone(), py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, Py::str("simpson"))?).items()?;
    if t1808.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t1808[0].clone();
    v_w = t1808[1].clone();
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("approximation"))?, f__float_sum(py::getitem(&v_i.clone(), &Py::str("f"))?, v_pts.clone(), v_w.clone())?, Py::Float(1e-11))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("exact"))?, f__gl_check(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?, Py::Float(1e-10))?, py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("error"))?, &py::sub(&py::getitem(&v_o.clone(), &Py::str("approximation"))?, &py::getitem(&v_o.clone(), &Py::str("exact"))?)?)?)?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1822(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1830: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1823(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1830 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1831 = v_r.clone(); Py::Bool({ let t1832 = v_r.clone(); if py::eq(&t1831, &t1832) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1823(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_v: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        v_v = (if ({ let t1825 = py::getitem(&v_i.clone(), &Py::str("t0"))?; Py::Bool({ let t1826 = py::getitem(&v_i.clone(), &Py::str("t1"))?; if py::ne(&t1825, &t1826) { true } else { false } }) }).truthy() { ax_engine::pysym::helper("calculus:_gl", &[ax_engine::pysym::helper("calculus:_compile", &[f__arc3_integrand(v_i.clone())?, Py::tuple(vec![c_T()?])], &[])?, py::getitem(&v_i.clone(), &Py::str("t0"))?, py::getitem(&v_i.clone(), &Py::str("t1"))?], &[])? } else { Py::Int(0i64) });
    }
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("arc_length"))?, py::float(&v_v.clone())?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_184(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t189: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_185(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t189 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t190 = v_r.clone(); Py::Bool({ let t191 = v_r.clone(); if py::eq(&t190, &t191) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1843(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1859: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_curv3(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1859 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1860 = v_r.clone(); Py::Bool({ let t1861 = v_r.clone(); if py::eq(&t1860, &t1861) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_185(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("area"))?, f__gl_check(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?, Py::Float(1e-10))?, f_rel_err(py::mul(&py::getitem(&v_o.clone(), &Py::str("x_bar"))?, &py::getitem(&v_o.clone(), &Py::str("area"))?)?, f__gl_check(Py::str(&["x*(".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("f"))?, "")?, ")".to_string()].concat()), py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?, Py::Float(1e-10))?, f_rel_err(py::mul(&py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("y_bar"))?)?, &py::getitem(&v_o.clone(), &Py::str("area"))?)?, f__gl_check(Py::str(&["(".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("f"))?, "")?, ")^2".to_string()].concat()), py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?, Py::Float(1e-10))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1895(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1897: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1896(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1897 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1898 = v_r.clone(); Py::Bool({ let t1899 = v_r.clone(); if py::eq(&t1898, &t1899) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1896(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_1913(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1940: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1914(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1940 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1941 = v_r.clone(); Py::Bool({ let t1942 = v_r.clone(); if py::eq(&t1941, &t1942) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1914(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("surface_area"))?, f__verify_surface(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?)?, Py::Float(1e-08))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1943(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1948: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1944(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1948 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1949 = v_r.clone(); Py::Bool({ let t1950 = v_r.clone(); if py::eq(&t1949, &t1950) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1944(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("slope"))?, ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?], &[])?, Py::Float(1e-12))?, f_rel_err(py::add(&py::getitem(&v_o.clone(), &Py::str("intercept"))?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("slope"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?)?, ax_engine::pysym::helper("calculus:at", &[ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?, Py::tuple(vec![c_X()?]), py::getitem(&v_i.clone(), &Py::str("a"))?], &[])?, Py::Float(1e-12))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1959(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1965: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_tplane(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1965 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1966 = v_r.clone(); Py::Bool({ let t1967 = v_r.clone(); if py::eq(&t1966, &t1967) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1975(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1989: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_taylor(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1989 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t1990 = v_r.clone(); Py::Bool({ let t1991 = v_r.clone(); if py::eq(&t1990, &t1991) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_200(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t205: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_201(v_inputs.clone(), v_outputs.clone())?)?;
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
fn s_201(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("T"))?, py::float(&ax_engine::pymp::call("chebyt", &[py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::getitem(&v_i.clone(), &Py::str("x"))?], &[])?)?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2010(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2032: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_totdiff(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2032 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t2033 = v_r.clone(); Py::Bool({ let t2034 = v_r.clone(); if py::eq(&t2033, &t2034) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2035(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v_approx: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_exact: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t2044 = (f__grid(v_f.clone(), v_a.clone(), v_b.clone(), v_n.clone())?).items()?;
    if t2044.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_e = t2044[0].clone();
    v_h = t2044[1].clone();
    v_n = t2044[2].clone();
    if ({ let t2047 = { let t2045 = Py::str("trapezoid"); Py::Bool({ let t2046 = Py::str("simpson"); if py::eq(&t2045, &t2046) { true } else { false } }) }; if !t2047.truthy() { t2047 } else { py::modulo(&v_n.clone(), &Py::Int(2i64))? } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Simpson's rule needs an even number of subintervals"))));
    }
    let t2060 = (f__rule_n(v_e.clone(), v_a.clone(), v_b.clone(), v_n.clone(), Py::str("trapezoid"))?).items()?;
    if t2060.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t2060[0].clone();
    v_w = t2060[1].clone();
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        v_approx = py::float(&ax_engine::pymp::call("fsum", &[{ let mut out = vec![]; for t2062 in py::iter(&(py::zip(&[v_w.clone(), f__values(v_e.clone(), v_pts.clone())?])?))? { let mut v_v: Py; let mut v_wi: Py; 
let t2063 = (t2062).items()?;
if t2063.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_wi = t2063[0].clone();
v_v = t2063[1].clone();
out.push(py::mul(&v_wi.clone(), &v_v.clone())?); } Py::list(out) }], &[])?)?;
    }
    v_exact = f__exact(v_e.clone(), v_a.clone(), v_b.clone())?;
    return Ok(Py::dict(vec![(Py::str("approximation"), v_approx.clone()), (Py::str("exact"), v_exact.clone()), (Py::str("error"), py::sub(&v_approx.clone(), &v_exact.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2064(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2072: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_2065(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2072 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t2073 = v_r.clone(); Py::Bool({ let t2074 = v_r.clone(); if py::eq(&t2073, &t2074) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2065(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_e = ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?;
    let t2066 = (f__rule_n(v_e.clone(), py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, Py::str("trapezoid"))?).items()?;
    if t2066.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t2066[0].clone();
    v_w = t2066[1].clone();
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("approximation"))?, f__float_sum(py::getitem(&v_i.clone(), &Py::str("f"))?, v_pts.clone(), v_w.clone())?, Py::Float(1e-11))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("exact"))?, f__gl_check(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?, Py::Float(1e-10))?, py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("error"))?, &py::sub(&py::getitem(&v_o.clone(), &Py::str("approximation"))?, &py::getitem(&v_o.clone(), &Py::str("exact"))?)?)?)?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2115(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2116: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_tpl(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2116 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t2117 = v_r.clone(); Py::Bool({ let t2118 = v_r.clone(); if py::eq(&t2117, &t2118) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2120(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2125: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_2121(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2125 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t2126 = v_r.clone(); Py::Bool({ let t2127 = v_r.clone(); if py::eq(&t2126, &t2127) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2121(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("velocity"))?, ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("s"))?, py::getitem(&v_i.clone(), &Py::str("t0"))?, Py::Int(1i64), Py::tuple(vec![Py::str("t")])], &[])?, Py::Float(1e-12))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("acceleration"))?, ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("s"))?, py::getitem(&v_i.clone(), &Py::str("t0"))?, Py::Int(2i64), Py::tuple(vec![Py::str("t")])], &[])?, Py::Float(1e-11))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2139(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2144: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_2140(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2144 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t2145 = v_r.clone(); Py::Bool({ let t2146 = v_r.clone(); if py::eq(&t2145, &t2146) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2140(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("volume"))?, py::mul(&ax_engine::pymp::constant("pi")?, &f__gl_check(Py::str(&["(".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("f"))?, "")?, ")^2 - (".to_string(), py::format(&py::getitem(&v_i.clone(), &Py::str("g"))?, "")?, ")^2".to_string()].concat()), py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?)?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_236(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t258: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_cons(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t258 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t259 = v_r.clone(); Py::Bool({ let t260 = v_r.clone(); if py::eq(&t259, &t260) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_289(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t310: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_coords(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t310 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t311 = v_r.clone(); Py::Bool({ let t312 = v_r.clone(); if py::eq(&t311, &t312) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_330(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t367: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_classify(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t367 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t368 = v_r.clone(); Py::Bool({ let t369 = v_r.clone(); if py::eq(&t368, &t369) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_380(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t394: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_381(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t394 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t395 = v_r.clone(); Py::Bool({ let t396 = v_r.clone(); if py::eq(&t395, &t396) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_381(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__vec_err(py::getitem(&v_o.clone(), &Py::str("curl"))?, f__curl_from({ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_c: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_j: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::float(&f__mp_partial(py::getitem(&Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("P"))?, py::getitem(&v_i.clone(), &Py::str("Q"))?, py::getitem(&v_i.clone(), &Py::str("R"))?]), &v_c.clone())?, c__XYZ()?, py::getitem(&v_i.clone(), &Py::str("p"))?, { let mut out = vec![]; for t391 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_k: Py; 
v_k = t391;
out.push(py::int(&{ let t392 = v_k.clone(); Py::Bool({ let t393 = v_j.clone(); if py::eq(&t392, &t393) { true } else { false } }) })?); } Py::list(out) })?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) })?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_400(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t405: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_401(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t405 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t406 = v_r.clone(); Py::Bool({ let t407 = v_r.clone(); if py::eq(&t406, &t407) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_401(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("curvature"))?, py::truediv(&py::abs(&ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, Py::Int(2i64)], &[])?)?, &py::pow(&py::add(&Py::Int(1i64), &py::pow(&ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?], &[])?, &Py::Int(2i64))?)?, &Py::Float(1.5))?)?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_42(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t55: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_anti(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t55 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t56 = v_r.clone(); Py::Bool({ let t57 = v_r.clone(); if py::eq(&t56, &t57) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_460(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t469: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_curvi(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t469 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t470 = v_r.clone(); Py::Bool({ let t471 = v_r.clone(); if py::eq(&t470, &t471) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_474(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t479: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_475(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t479 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t480 = v_r.clone(); Py::Bool({ let t481 = v_r.clone(); if py::eq(&t480, &t481) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_475(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("integral"))?, f__gl_check(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_482(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t487: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_483(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t487 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t488 = v_r.clone(); Py::Bool({ let t489 = v_r.clone(); if py::eq(&t488, &t489) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_483(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("derivative"))?, ax_engine::pysym::helper("calculus:_mp_diff", &[py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?], &[])?, Py::Float(1e-12))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_503(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t516: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_dirder(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t516 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t517 = v_r.clone(); Py::Bool({ let t518 = v_r.clone(); if py::eq(&t517, &t518) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_521(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t526: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_522(v_inputs.clone(), v_outputs.clone())?)?;
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
fn s_522(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_540(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t551: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_541(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t551 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t552 = v_r.clone(); Py::Bool({ let t553 = v_r.clone(); if py::eq(&t552, &t553) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_541(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("divergence"))?, py::fsum(&{ let mut out = vec![]; for t545 in py::iter(&(py::enumerate(&Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("P"))?, py::getitem(&v_i.clone(), &Py::str("Q"))?, py::getitem(&v_i.clone(), &Py::str("R"))?]), 0)?))? { let mut v_c: Py; let mut v_j: Py; 
let t546 = (t545).items()?;
if t546.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t546[0].clone();
v_c = t546[1].clone();
out.push(py::float(&f__mp_partial(v_c.clone(), c__XYZ()?, py::getitem(&v_i.clone(), &Py::str("p"))?, { let mut out = vec![]; for t548 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_k: Py; 
v_k = t548;
out.push(py::int(&{ let t549 = v_k.clone(); Py::Bool({ let t550 = v_j.clone(); if py::eq(&t549, &t550) { true } else { false } }) })?); } Py::list(out) })?)?); } Py::list(out) })?, Py::Float(1e-11))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_60(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t65: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_61(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t65 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t66 = v_r.clone(); Py::Bool({ let t67 = v_r.clone(); if py::eq(&t66, &t67) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_600(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t603: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_divthm(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t603 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t604 = v_r.clone(); Py::Bool({ let t605 = v_r.clone(); if py::eq(&t604, &t605) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_61(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("arc_length"))?, ax_engine::pysym::helper("calculus:_gl", &[ax_engine::pysym::helper("calculus:_compile", &[ax_engine::pysym::call("sqrt", &[py::add(&Py::Int(1i64), &py::pow(&ax_engine::pysym::call("diff", &[ax_engine::pysym::helper("calculus:parse_expr", &[py::getitem(&v_i.clone(), &Py::str("f"))?], &[])?, c_X()?], &[])?, &Py::Int(2i64))?)?], &[])?, Py::tuple(vec![c_X()?])], &[])?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?], &[])?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_639(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t641: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_dbl(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t641 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t642 = v_r.clone(); Py::Bool({ let t643 = v_r.clone(); if py::eq(&t642, &t643) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_679(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t681: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_dblr(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t681 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t682 = v_r.clone(); Py::Bool({ let t683 = v_r.clone(); if py::eq(&t682, &t683) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_684(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t689: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_685(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t689 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t690 = v_r.clone(); Py::Bool({ let t691 = v_r.clone(); if py::eq(&t690, &t691) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_685(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("erf"))?, py::float(&ax_engine::pymp::call("erf", &[py::getitem(&v_i.clone(), &Py::str("x"))?], &[])?)?, Py::Float(1e-13))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("erfc"))?, py::float(&ax_engine::pymp::call("erfc", &[py::getitem(&v_i.clone(), &Py::str("x"))?], &[])?)?, Py::Float(1e-13))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_698(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t729: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_extrema(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t729 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t730 = v_r.clone(); Py::Bool({ let t731 = v_r.clone(); if py::eq(&t730, &t731) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_73(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t92: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_74(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t92 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t93 = v_r.clone(); Py::Bool({ let t94 = v_r.clone(); if py::eq(&t93, &t94) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_738(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t743: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_fourier(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t743 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t744 = v_r.clone(); Py::Bool({ let t745 = v_r.clone(); if py::eq(&t744, &t745) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_74(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("area"))?, f__verify_abs_area(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("g"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?)?, Py::Float(1e-10))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_747(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t752: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_ftc(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t752 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t753 = v_r.clone(); Py::Bool({ let t754 = v_r.clone(); if py::eq(&t753, &t754) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_756(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t761: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_757(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t761 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t762 = v_r.clone(); Py::Bool({ let t763 = v_r.clone(); if py::eq(&t762, &t763) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_757(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("gamma"))?, py::float(&ax_engine::pymp::call("gamma", &[py::getitem(&v_i.clone(), &Py::str("x"))?], &[])?)?, Py::Float(1e-13))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_789(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t800: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_790(v_inputs.clone(), v_outputs.clone())?)?;
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
fn s_790(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_worst(Py::tuple(vec![f_rel_err(py::getitem(&v_o.clone(), &Py::str("exact"))?, f__gl_check(py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, Py::tuple(vec![Py::str("x")]), Py::None)?, Py::Float(1e-10))?, f_rel_err(py::getitem(&v_o.clone(), &Py::str("approximation"))?, f__float_sum(py::getitem(&v_i.clone(), &Py::str("f"))?, { let mut out = vec![]; for t796 in py::iter(&(f__gauss_nodes(py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?))? { let mut v__w: Py; let mut v_x: Py; 
let t797 = (t796).items()?;
if t797.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t797[0].clone();
v__w = t797[1].clone();
out.push(py::add(&py::truediv(&py::add(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?, &Py::Int(2i64))?, &py::mul(&py::truediv(&py::sub(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?, &Py::Int(2i64))?, &py::float(&v_x.clone())?)?)?); } Py::list(out) }, { let mut out = vec![]; for t798 in py::iter(&(f__gauss_nodes(py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?))? { let mut v__x: Py; let mut v_w: Py; 
let t799 = (t798).items()?;
if t799.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__x = t799[0].clone();
v_w = t799[1].clone();
out.push(py::mul(&py::truediv(&py::sub(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?, &Py::Int(2i64))?, &py::float(&v_w.clone())?)?); } Py::list(out) })?, Py::Float(1e-11))?, py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("error"))?, &py::sub(&py::getitem(&v_o.clone(), &Py::str("approximation"))?, &py::getitem(&v_o.clone(), &Py::str("exact"))?)?)?)?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_809(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t815: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_810(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t815 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t816 = v_r.clone(); Py::Bool({ let t817 = v_r.clone(); if py::eq(&t816, &t817) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_810(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::getitem(&v_o.clone(), &Py::str("sum"))?, py::fsum(&{ let mut out = vec![]; for t814 in py::range_iter(&[(py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?).i()?]) { let mut v_k: Py; 
v_k = t814;
out.push(py::mul(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("r"))?, &v_k.clone())?)?); } Py::list(out) })?, Py::Float(1e-12))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_820(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t825: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_821(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t825 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t826 = v_r.clone(); Py::Bool({ let t827 = v_r.clone(); if py::eq(&t826, &t827) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_821(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f_rel_err(py::mul(&py::getitem(&v_o.clone(), &Py::str("sum"))?, &py::sub(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("r"))?)?)?, py::getitem(&v_i.clone(), &Py::str("a"))?, Py::Float(1e-14))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_834(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t849: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_gradient(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t849 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t850 = v_r.clone(); Py::Bool({ let t851 = v_r.clone(); if py::eq(&t850, &t851) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_892(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t895: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_green(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t895 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t896 = v_r.clone(); Py::Bool({ let t897 = v_r.clone(); if py::eq(&t896, &t897) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_911(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t923: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_hessian(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t923 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t924 = v_r.clone(); Py::Bool({ let t925 = v_r.clone(); if py::eq(&t924, &t925) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_937(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t944: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_implicit(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t944 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t945 = v_r.clone(); Py::Bool({ let t946 = v_r.clone(); if py::eq(&t945, &t946) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_947(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t952: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_948(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t952 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t953 = v_r.clone(); Py::Bool({ let t954 = v_r.clone(); if py::eq(&t953, &t954) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_948(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_969(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t997: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_inflection(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t997 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t998 = v_r.clone(); Py::Bool({ let t999 = v_r.clone(); if py::eq(&t998, &t999) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_99(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t107: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_100(v_inputs.clone(), v_outputs.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t107 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Key(_))) || ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) || (matches!(e, PyErr::ZeroDivision)) || (matches!(e, PyErr::Overflow)) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok((if ({ let t108 = v_r.clone(); Py::Bool({ let t109 = v_r.clone(); if py::eq(&t108, &t109) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}
