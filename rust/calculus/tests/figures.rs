//! Every Calculus capability's own live figures (src/figures.rs): its worked example gives each
//! published figure kind, a changed input changes the drawing, and no drawing holds a non-finite number or text
//! outside its canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use calculus_ai999x::pack;

/// The published figure kinds of every capability (publish/website/models/calculus-ai999x/figures,
/// except the generic "curve"), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("arc_length", &["graph"]),
    ("area_between_curves", &["area"]),
    ("area_enclosed_curves", &["area"]),
    ("average_rate_of_change", &["graph"]),
    ("average_value", &["area"]),
    ("beta_function", &["field"]),
    ("centroid_region", &["region"]),
    ("critical_points_classify", &["graph"]),
    ("curl", &["field"]),
    ("curvature", &["graph"]),
    ("definite_integral", &["area"]),
    ("derivative_at_point", &["graph"]),
    ("directional_derivative", &["field"]),
    ("disk_volume", &["field", "solid"]),
    ("divergence", &["field"]),
    ("double_integral_rectangle", &["field"]),
    ("double_integral_region", &["region"]),
    ("extrema_on_interval", &["graph"]),
    ("fourier_coefficients", &["stems"]),
    ("ftc_derivative", &["area"]),
    ("gauss_legendre", &["convergence"]),
    ("geometric_partial_sum", &["stems"]),
    ("geometric_series_sum", &["field"]),
    ("gradient", &["field"]),
    ("greens_theorem", &["field"]),
    ("hessian_classification", &["field"]),
    ("improper_integral", &["area"]),
    ("inflection_points", &["graph"]),
    ("jacobian_determinant", &["map"]),
    ("laplacian", &["field"]),
    ("left_riemann_sum", &["area", "convergence"]),
    ("limit_at_infinity", &["graph"]),
    ("limit_at_point", &["graph"]),
    ("line_integral_scalar", &["field"]),
    ("line_integral_vector", &["field"]),
    ("linear_approximation", &["graph"]),
    ("mean_value_theorem", &["graph"]),
    ("midpoint_rule", &["area"]),
    ("newton_method", &["iteration"]),
    ("normal_line", &["graph"]),
    ("nth_derivative_at_point", &["graph"]),
    ("numerical_derivative", &["convergence"]),
    ("partial_derivative", &["field"]),
    ("polar_area", &["polar"]),
    ("ratio_test", &["stems"]),
    ("regularized_gamma", &["field"]),
    ("right_riemann_sum", &["area"]),
    ("second_derivative_at_point", &["graph"]),
    ("series_sum", &["stems"]),
    ("shell_volume", &["solid"]),
    ("simpson_rule", &["area"]),
    ("surface_area_revolution", &["solid"]),
    ("tangent_line", &["graph"]),
    ("tangent_plane", &["slices"]),
    ("taylor_polynomial", &["curve-2"]),
    ("trapezoid_rule", &["area"]),
    ("velocity_acceleration", &["motion"]),
    ("washer_volume", &["solid"]),
];

/// Inputs to change for a capability whose inputs are not plain quantities (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("critical_points_classify", "f", "x^3 - 12x"),
    ("inflection_points", "f", "x^3 - 6x^2 + 2"),
    ("limit_at_infinity", "f", "(5x^2 + 2)/(x^2 + 1)"),
    ("limit_at_point", "f", "(x^2 - 9)/(x - 3)"),
    ("area_enclosed_curves", "f", "x^3"),
    ("ratio_test", "a", "3^n/n^2"),
    ("series_sum", "a", "1/n^3"),
    ("polar_area", "r", "2 + sin(theta)"),
    ("fourier_coefficients", "f", "x^2"),
    ("jacobian_determinant", "p", "1.5, 0.8"),
    ("tangent_plane", "p", "2, 1"),
    ("gradient", "p", "2, 1"),
    ("directional_derivative", "u", "1, 1"),
    ("partial_derivative", "p", "2, 1"),
    ("hessian_classification", "f", "2x^2 + 3y^2 - 4x"),
    ("curvature", "a", "1"),
    ("improper_integral", "f", "exp(-2x)"),
    ("laplacian", "f", "x^3 + y^2"),
    ("curl", "p", "2, 1, 1"),
    ("divergence", "p", "2, 1, 1"),
    ("velocity_acceleration", "s", "t^3 - 3t"),
    ("newton_method", "x0", "2"),
];

/// The arguments of a structured example: solve('key', a='1 V', b=8, c="(A·B)'").
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else {
            let s0 = i;
            while i < cs.len() && cs[i] != ',' && cs[i] != ')' {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    // the written example rounds 2π up to 6.28319, more than one turn, which the model refuses; the page's worked
    // example states 2π in full
    for e in out.iter_mut() {
        if key == "polar_area" && e.0 == "be" {
            e.1 = Input::Text("6.283185307179586".into());
        }
    }
    out
}

fn kind_of(f: &Figure) -> &str {
    &f.kind
}

/// Every text inside the canvas, no non-finite number (ax_engine::figkit::check_figure).
fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| kind_of(f) == *kind) {
                Some(f) => {
                    assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    // ("infinite" is a word of some titles: check_figure looks for inf as a number)
                    assert!(!f.svg.contains("NaN"), "{key} {kind}: a non-finite number");
                    check_texts(f);
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        // the model's own figures come first
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// Numbers in a text multiplied by f ("12 V" -> "16.44 V"; "3, 6" -> "4.11, 8.22").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || c == '.' || (c == '-' && cur.is_empty()) || ((c == 'e' || c == 'E') && !cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        // the other example: a listed replacement, else each number input scaled until the model solves it
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        if let Some((_, sym, val)) = OTHER.iter().find(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        for f in [1.37, 0.73] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn an_impossible_input_draws_nothing_wrong() {
    // a point outside the function's domain is refused by the model: no figure at all, never a wrong one
    let p = pack();
    for (key, given) in [("tangent_line", vec![("f", "sqrt(x)"), ("a", "-4")]),
                         ("definite_integral", vec![("f", "1/x"), ("a", "-1"), ("b", "1")]),
                         ("gradient", vec![("f", "log(x*y)"), ("p", "-1, 1")])] {
        let given: Vec<(String, Input)> = given.iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
        let a = ax_engine::solve(p, key, &given, None);
        let figs = figures(p, key, &given, None, None);
        if !a.solved() {
            assert!(figs.is_empty(), "{key}: a refused input drew a figure");
        } else {
            for f in &figs {
                check_texts(f);
            }
        }
    }
}

#[test]
fn own_figures_time() {
    let p = pack();
    for (key, _) in KINDS {
        let given = example(key);
        let t = std::time::Instant::now();
        let _ = figures(p, key, &given, None, None);
        eprintln!("TIME {key}: {} ms", t.elapsed().as_millis());
    }
}
