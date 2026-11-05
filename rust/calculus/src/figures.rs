//! Calculus' own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model computes:
//! every curve value of a function is a further solve of the model (f(x) from its average rate of change, f(x, y)
//! from its tangent plane, derivatives, sums and integrals from the capabilities that compute them). A figure the
//! model cannot support for these inputs is left out, never drawn wrong (docs/FIGURE_PARITY.md).

use ax_engine::figkit::{num_of, DPlot, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;
use serde_json::json;

#[path = "figures_graph.rs"]
mod graph;
#[path = "figures_area.rs"]
mod area;
#[path = "figures_field.rs"]
mod field;
#[path = "figures_misc.rs"]
mod misc;

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        "arc_length" => vec![graph::arc_length(s)],
        "average_rate_of_change" => vec![graph::secant(s)],
        "critical_points_classify" => vec![graph::critical(s)],
        "curvature" => vec![graph::curvature(s)],
        "derivative_at_point" => vec![graph::derivative(s)],
        "extrema_on_interval" => vec![graph::extrema(s)],
        "inflection_points" => vec![graph::inflection(s)],
        "limit_at_infinity" => vec![graph::limit_inf(s)],
        "limit_at_point" => vec![graph::limit_point(s)],
        "linear_approximation" => vec![graph::linear_approx(s)],
        "mean_value_theorem" => vec![graph::mvt(s)],
        "normal_line" => vec![graph::normal(s)],
        "nth_derivative_at_point" => vec![graph::nth_derivative(s)],
        "second_derivative_at_point" => vec![graph::second_derivative(s)],
        "tangent_line" => vec![graph::tangent(s)],
        "area_between_curves" => vec![area::between(s, false)],
        "area_enclosed_curves" => vec![area::between(s, true)],
        "average_value" => vec![area::average(s)],
        "definite_integral" => vec![area::definite(s)],
        "ftc_derivative" => vec![area::ftc(s)],
        "improper_integral" => vec![area::improper(s)],
        "left_riemann_sum" => vec![area::riemann(s, "left"), area::riemann_convergence(s)],
        "right_riemann_sum" => vec![area::riemann(s, "right")],
        "midpoint_rule" => vec![area::riemann(s, "midpoint")],
        "trapezoid_rule" => vec![area::riemann(s, "trapezoid")],
        "simpson_rule" => vec![area::riemann(s, "simpson")],
        "centroid_region" => vec![area::centroid(s)],
        "double_integral_region" => vec![area::double_region(s)],
        "beta_function" => vec![field::two_input(s, "a", "b", "beta", (true, true))],
        "regularized_gamma" => vec![field::two_input(s, "a", "x", "P", (true, true))],
        "geometric_series_sum" => vec![field::two_input(s, "r", "a", "sum", (false, false))],
        "disk_volume" => vec![field::two_input(s, "b", "a", "volume", (false, false)), misc::disk_solid(s)],
        "double_integral_rectangle" => vec![field::double_rect(s)],
        "gradient" => vec![field::gradient(s)],
        "directional_derivative" => vec![field::directional(s)],
        "partial_derivative" => vec![field::partial(s)],
        "hessian_classification" => vec![field::hessian(s)],
        "laplacian" => vec![field::laplacian(s)],
        "curl" => vec![field::curl(s)],
        "divergence" => vec![field::divergence(s)],
        "greens_theorem" => vec![field::greens(s)],
        "line_integral_scalar" => vec![field::line_scalar(s)],
        "line_integral_vector" => vec![field::line_vector(s)],
        "shell_volume" => vec![misc::shell_solid(s)],
        "washer_volume" => vec![misc::washer_solid(s)],
        "surface_area_revolution" => vec![misc::surface_solid(s)],
        "fourier_coefficients" => vec![misc::fourier(s)],
        "geometric_partial_sum" => vec![misc::partial_sums(s)],
        "ratio_test" => vec![misc::ratio(s)],
        "series_sum" => vec![misc::tails(s)],
        "gauss_legendre" => vec![misc::gauss(s)],
        "numerical_derivative" => vec![misc::fd_convergence(s)],
        "taylor_polynomial" => vec![misc::taylor(s)],
        "newton_method" => vec![misc::newton(s)],
        "jacobian_determinant" => vec![misc::jacobian(s)],
        "velocity_acceleration" => vec![misc::motion(s)],
        "polar_area" => vec![misc::polar(s)],
        "tangent_plane" => vec![misc::slices(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ shared helpers
pub(crate) fn tx(t: &str) -> Input {
    Input::Text(t.to_string())
}

/// A number input as read (a number or a whole number).
pub(crate) fn nv(s: &Solved, sym: &str) -> Option<f64> {
    s.inp(sym).or_else(|| s.vec(sym).and_then(|v| (v.len() == 1).then(|| v[0])))
}

/// The expression with one variable renamed (whole names only: "t" in "sqrt(t)" but not in "sqrt"), e.g. t written
/// as x so that the model's one-variable capabilities evaluate it.
pub(crate) fn subst(e: &str, from: &str, to: &str) -> String {
    let cs: Vec<char> = e.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < cs.len() {
        if cs[i].is_ascii_alphabetic() || cs[i] == '_' {
            let st = i;
            while i < cs.len() && (cs[i].is_ascii_alphanumeric() || cs[i] == '_') {
                i += 1;
            }
            let w: String = cs[st..i].iter().collect();
            if w == from { out.push_str(to) } else { out.push_str(&w) }
        } else {
            out.push(cs[i]);
            i += 1;
        }
    }
    out
}

/// A number written into an expression (in brackets, no exponent).
pub(crate) fn lit(x: f64) -> String {
    format!("({x})")
}

/// f(x) as the model computes it (its average rate of change over [x, x + d] returns f(x)).
pub(crate) fn f_at(s: &Solved, f: &str, x: f64) -> Option<f64> {
    let d = 1e-6 * x.abs().max(1.0);
    let a = s.solve_other("average_rate_of_change", &[("f", tx(f)), ("a", Input::Float(x)), ("b", Input::Float(x + d))])?;
    num_of(&a, "fa")
}

/// f at each x (two values per solve of the model); points the model refuses are left out.
pub(crate) fn fvals(s: &Solved, f: &str, xs: &[f64]) -> Vec<(f64, f64)> {
    let mut out = vec![];
    let mut i = 0;
    while i < xs.len() {
        if i + 1 < xs.len() && xs[i + 1] > xs[i] {
            if let Some(a) = s.solve_other("average_rate_of_change",
                                           &[("f", tx(f)), ("a", Input::Float(xs[i])), ("b", Input::Float(xs[i + 1]))]) {
                if let (Some(fa), Some(fb)) = (num_of(&a, "fa"), num_of(&a, "fb")) {
                    out.push((xs[i], fa));
                    out.push((xs[i + 1], fb));
                    i += 2;
                    continue;
                }
            }
            for j in [i, i + 1] {
                if let Some(v) = f_at(s, f, xs[j]) {
                    out.push((xs[j], v));
                }
            }
            i += 2;
        } else {
            if let Some(v) = f_at(s, f, xs[i]) {
                out.push((xs[i], v));
            }
            i += 1;
        }
    }
    out
}

/// f over [lo, hi] at n points.
pub(crate) fn fcurve(s: &Solved, f: &str, lo: f64, hi: f64, n: usize) -> Vec<(f64, f64)> {
    fvals(s, f, &span(lo, hi, n))
}

pub(crate) fn span(lo: f64, hi: f64, n: usize) -> Vec<f64> {
    ax_engine::figkit::span(lo, hi, n, false)
}

/// f(x, y) as the model computes it (the height z0 of its tangent plane at (x, y)).
pub(crate) fn f2(s: &Solved, f: &str, x: f64, y: f64) -> Option<f64> {
    let a = s.solve_other("tangent_plane", &[("f", tx(f)), ("p", Input::Json(json!([x, y])))])?;
    num_of(&a, "z0")
}

/// f(x, y) on a grid: vals[i][j] at (xs[i], ys[j]).
pub(crate) fn grid(s: &Solved, f: &str, xs: &[f64], ys: &[f64]) -> Vec<Vec<Option<f64>>> {
    xs.iter().map(|x| ys.iter().map(|y| f2(s, f, *x, *y)).collect()).collect()
}

/// A vertical range holding the values (a few wild values left outside when they would flatten the rest).
pub(crate) fn yrange(pts: &[(f64, f64)], extra: &[f64]) -> Option<(f64, f64)> {
    let mut v: Vec<f64> = pts.iter().map(|p| p.1).chain(extra.iter().copied()).filter(|x| x.is_finite()).collect();
    if v.is_empty() {
        return None;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let (lo, hi) = (v[0], v[v.len() - 1]);
    if v.len() >= 12 {
        let q = |f: f64| v[((v.len() - 1) as f64 * f).round() as usize];
        let (ql, qh) = (q(0.05), q(0.95));
        let core = (qh - ql).max(1e-12);
        if hi - lo > 12.0 * core {
            let mut l = ql - core;
            let mut h = qh + core;
            for e in extra.iter().filter(|x| x.is_finite()) {
                l = l.min(*e);
                h = h.max(*e);
            }
            return Some((l, h));
        }
    }
    Some((lo, hi))
}

/// A range [lo, hi] widened by a fraction of its width on each side.
pub(crate) fn widen(lo: f64, hi: f64, f: f64) -> (f64, f64) {
    let w = (hi - lo).abs().max(1e-9);
    (lo.min(hi) - w * f, lo.max(hi) + w * f)
}

/// Cells of a grid shaded by value on a plot (accent positive, violet negative, darker = larger magnitude);
/// vals[i][j] at (xs[i], ys[j]), a refused cell blank. Returns the largest magnitude shaded.
pub(crate) fn shade(p: &mut DPlot, xs: &[f64], ys: &[f64], vals: &[Vec<Option<f64>>], signed: bool) -> Option<(f64, f64)> {
    let all: Vec<f64> = vals.iter().flatten().flatten().copied().filter(|v| v.is_finite()).collect();
    if all.is_empty() || xs.len() < 2 || ys.len() < 2 {
        return None;
    }
    let (lo, hi) = all.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)));
    let m = lo.abs().max(hi.abs()).max(1e-300);
    let edge = |v: &[f64], i: usize| -> (f64, f64) {
        let l = if i == 0 { v[0] - (v[1] - v[0]) / 2.0 } else { (v[i - 1] + v[i]) / 2.0 };
        let r = if i + 1 >= v.len() { v[i] + (v[i] - v[i - 1]) / 2.0 } else { (v[i] + v[i + 1]) / 2.0 };
        (l, r)
    };
    let mut svg = String::new();
    for (i, col) in vals.iter().enumerate().take(xs.len()) {
        for (j, v) in col.iter().enumerate().take(ys.len()) {
            let Some(v) = v.filter(|v| v.is_finite()) else { continue };
            let (l, r) = edge(xs, i);
            let (b, t) = edge(ys, j);
            let (x1, y1) = p.px(l, t);
            let (x2, y2) = p.px(r, b);
            let (class, op) = if signed {
                (if v >= 0.0 { "axp-hp" } else { "axp-hn" }, 0.05 + 0.55 * v.abs() / m)
            } else {
                ("axp-hp", if hi > lo { 0.05 + 0.55 * (v - lo) / (hi - lo) } else { 0.3 })
            };
            svg.push_str(&format!("<rect class=\"{class}\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
                                   fill-opacity=\"{op:.2}\"/>", x1.min(x2), y1.min(y2), (x2 - x1).abs() + 0.6,
                                  (y2 - y1).abs() + 0.6));
        }
    }
    p.raw(&svg);
    Some((lo, hi))
}

/// "f = ..." for headers: the expression clipped to fit.
pub(crate) fn clipx(e: &str, n: usize) -> String {
    let c: Vec<char> = e.chars().collect();
    if c.len() <= n { e.to_string() } else { format!("{}...", c[..n - 3].iter().collect::<String>()) }
}

/// Numbers as a list "1, 2, 3".
pub(crate) fn join(v: &[f64]) -> String {
    v.iter().map(|x| ax_engine::figkit::num(*x)).collect::<Vec<_>>().join(", ")
}
