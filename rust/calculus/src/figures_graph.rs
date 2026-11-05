//! Graphs of one function with what the model computed on them: tangents, secants, limits, extrema, derivatives.

use super::*;
use ax_engine::figkit::{self as k, num, stack};

/// The function's graph over [lo, hi] in a plot with room for the extra heights.
fn graph_plot(pts: &[(f64, f64)], lo: f64, hi: f64, extra: &[f64], header: &str) -> Option<DPlot> {
    let (y0, y1) = yrange(pts, extra)?;
    Some(DPlot::new((lo, hi), (y0, y1), "x", "y", header))
}

fn derivative_vals(s: &Solved, f: &str, xs: &[f64], key: &str, out: &str, n: Option<i64>) -> Vec<(f64, f64)> {
    xs.iter().filter_map(|x| {
        let mut set = vec![("f", tx(f)), ("a", Input::Float(*x))];
        if let Some(n) = n {
            set.push(("n", Input::Int(n)));
        }
        let a = s.solve_other(key, &set)?;
        num_of(&a, out).map(|v| (*x, v))
    }).collect()
}

pub fn arc_length(s: &Solved) -> Option<Figure> {
    let (f, a, b, l) = (s.txt("f")?, nv(s, "a")?, nv(s, "b")?, s.out("arc_length")?);
    if !(b > a) {
        return None;
    }
    let pts = fcurve(s, &f, a, b, 41);
    if pts.len() < 10 {
        return None;
    }
    let (pa, pb) = (*pts.first()?, *pts.last()?);
    if pa.0 != a || pb.0 != b {
        return None;
    }
    let chord = ((pb.0 - pa.0).powi(2) + (pb.1 - pa.1).powi(2)).sqrt();
    let xs: Vec<f64> = pts.iter().map(|p| p.0).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1).collect();
    let mut p = DPlot::equal(&xs, &ys, "x", "y", &format!("arc length = {}", num(l)));
    p.dashed(&[pa, pb], 2);
    p.curve(&pts, 0);
    p.dot(pa.0, pa.1, 1, 4.0);
    p.dot(pb.0, pb.1, 1, 4.0);
    p.key(&format!("y = {}, arc {}", clipx(&f, 16), num(l)), 0);
    p.key(&format!("chord {}", num(chord)), 2);
    Some(s.chart("graph", "Arc length against the chord",
                 &format!("The arc of y = {f} from x = {} to x = {}, drawn to scale (curve values computed by the model): \
                           arc length {} against the straight chord between the same end points, {} (from the two end \
                           values f({}) = {} and f({}) = {}).", num(a), num(b), num(l), num(chord), num(a), num(pa.1),
                          num(b), num(pb.1)), &p.body()))
}

pub fn secant(s: &Solved) -> Option<Figure> {
    let (f, a, b) = (s.txt("f")?, nv(s, "a")?, nv(s, "b")?);
    let (m, fa, fb) = (s.out("average_rate")?, s.out("fa")?, s.out("fb")?);
    let (lo, hi) = widen(a, b, 0.3);
    let pts = fcurve(s, &f, lo, hi, 41);
    let line: Vec<(f64, f64)> = [lo, hi].iter().map(|x| (*x, fa + m * (x - a))).collect();
    let mut p = graph_plot(&pts, lo, hi, &[fa, fb], &format!("average rate = {}", num(m)))?;
    p.curve(&pts, 0);
    p.dashed(&line, 2);
    p.seg((a, fa), (b, fa), 3, true);
    p.seg((b, fa), (b, fb), 3, true);
    p.mark(a, fa, &format!("f({}) = {}", num(a), num(fa)));
    p.mark(b, fb, &format!("f({}) = {}", num(b), num(fb)));
    p.key(&format!("y = {}", clipx(&f, 20)), 0);
    p.key("secant", 2);
    Some(s.chart("graph", "Average rate of change as a secant slope",
                 &format!("The secant of y = {f} over [{}, {}] (curve values computed by the model): f({}) = {}, f({}) = \
                           {}, average rate of change Δy/Δx = {}. Marked: the two end points.", num(a), num(b), num(a),
                          num(fa), num(b), num(fb), num(m)), &p.body()))
}

pub fn critical(s: &Solved) -> Option<Figure> {
    let f = s.txt("f")?;
    let cps = s.out_list("critical_points")?;
    let maxs = s.out_list("local_maxima").unwrap_or_default();
    let mins = s.out_list("local_minima").unwrap_or_default();
    let (l0, h0) = cps.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)));
    let (lo, hi) = if h0 > l0 { widen(l0, h0, 0.6) } else { (l0 - 2.0, l0 + 2.0) };
    let pts = fcurve(s, &f, lo, hi, 41);
    let at = fvals(s, &f, &cps);
    if at.len() != cps.len() {
        return None;
    }
    let mut p = graph_plot(&pts, lo, hi, &at.iter().map(|q| q.1).collect::<Vec<_>>(),
                           &format!("critical points x = {}", clipx(&join(&cps), 30)))?;
    p.curve(&pts, 0);
    let w = (hi - lo) * 0.12;
    for (x, y) in &at {
        p.seg((x - w, *y), (x + w, *y), 3, true);
        let kind = if maxs.iter().any(|m| (m - x).abs() < 1e-9) { "max" } else if mins.iter().any(|m| (m - x).abs() < 1e-9) {
            "min"
        } else {
            "neither"
        };
        p.mark(*x, *y, &format!("{kind} x = {}", num(*x)));
    }
    p.key(&format!("y = {}", clipx(&f, 24)), 0);
    Some(s.chart("graph", "Critical points: local maxima and minima",
                 &format!("The graph of y = {f} (values computed by the model) with its critical points, where f′(x) = 0 \
                           (horizontal tangents dashed): local maxima at x = {}, local minima at x = {}.",
                          if maxs.is_empty() { "none".into() } else { join(&maxs) },
                          if mins.is_empty() { "none".into() } else { join(&mins) }), &p.body()))
}

pub fn curvature(s: &Solved) -> Option<Figure> {
    let (f, a, kappa) = (s.txt("f")?, nv(s, "a")?, s.out("curvature")?);
    let r = s.out("radius_of_curvature")?;
    if !(r > 0.0) || !r.is_finite() {
        return None;
    }
    let d1 = s.solve_other("derivative_at_point", &[("f", tx(&f)), ("a", Input::Float(a))])
        .and_then(|x| num_of(&x, "derivative"))?;
    let d2 = s.solve_other("second_derivative_at_point", &[("f", tx(&f)), ("a", Input::Float(a))])
        .and_then(|x| num_of(&x, "second_derivative"))?;
    let fa = f_at(s, &f, a)?;
    let norm = (1.0 + d1 * d1).sqrt();
    let sg = if d2 >= 0.0 { 1.0 } else { -1.0 };
    let c = (a - sg * r * d1 / norm, fa + sg * r / norm);
    let half = (2.0 * r).max(1.0);
    let pts = fcurve(s, &f, a - half, a + half, 41);
    let circ: Vec<(f64, f64)> = (0..=72).map(|i| {
        let t = i as f64 / 72.0 * std::f64::consts::TAU;
        (c.0 + r * t.cos(), c.1 + r * t.sin())
    }).collect();
    let xs: Vec<f64> = circ.iter().map(|p| p.0).chain([a - half, a + half]).collect();
    let ys: Vec<f64> = circ.iter().map(|p| p.1).chain([fa]).collect();
    let mut p = DPlot::equal(&xs, &ys, "x", "y", &format!("κ = {}, radius {}", num(kappa), num(r)));
    p.curve(&circ, 2);
    p.curve(&pts, 0);
    p.seg((a, fa), c, 3, true);
    p.dot(c.0, c.1, 2, 3.5);
    p.mark(a, fa, &format!("x = {}", num(a)));
    p.key(&format!("y = {}", clipx(&f, 20)), 0);
    p.key("osculating circle", 2);
    Some(s.chart("graph", "Curvature and the osculating circle",
                 &format!("y = {f} (values computed by the model) and its osculating circle at x = {}: curvature κ = {} \
                           gives radius 1/κ = {}; the centre lies along the normal, placed from the slope f′({}) = {} and \
                           the sign of f″({}) = {} (both computed by the model). Drawn to scale.", num(a), num(kappa),
                          num(r), num(a), num(d1), num(a), num(d2)), &p.body()))
}

pub fn derivative(s: &Solved) -> Option<Figure> {
    let (f, a, d) = (s.txt("f")?, nv(s, "a")?, s.out("derivative")?);
    let w = a.abs().max(1.0) * 1.2;
    let (lo, hi) = (a - w, a + w);
    let pts = fcurve(s, &f, lo, hi, 41);
    let fa = f_at(s, &f, a)?;
    let ds = derivative_vals(s, &f, &span(lo, hi, 25), "derivative_at_point", "derivative", None);
    if ds.len() < 5 {
        return None;
    }
    let tan: Vec<(f64, f64)> = [lo, hi].iter().map(|x| (*x, fa + d * (x - a))).collect();
    let mut p1 = graph_plot(&pts, lo, hi, &[fa], &format!("f′({}) = {}", num(a), num(d)))?;
    p1.curve(&pts, 0);
    p1.dashed(&tan, 2);
    p1.mark(a, fa, &format!("({}, {})", num(a), num(fa)));
    p1.key(&format!("y = {}", clipx(&f, 20)), 0);
    p1.key("tangent", 2);
    let (y0, y1) = yrange(&ds, &[d, 0.0])?;
    let mut p2 = DPlot::new((lo, hi), (y0, y1), "x", "f′(x)", "");
    p2.hline(0.0);
    p2.curve(&ds, 1);
    p2.mark(a, d, &format!("f′({}) = {}", num(a), num(d)));
    p2.key("f′(x)", 1);
    let (body, h) = stack(&[p1.body(), p2.body()]);
    Some(s.figure("graph", &format!("A function and its derivative, f′({}) = {}", num(a), num(d)),
                  &format!("Top: y = {f} (values computed by the model) with its tangent at x = {}. Bottom: the \
                            derivative f′(x) computed by the model at 25 points; its value at x = {} is the tangent's \
                            slope. Marked: f′({}) = {}.", num(a), num(a), num(a), num(d)), &body, h))
}

pub fn extrema(s: &Solved) -> Option<Figure> {
    let (f, a, b) = (s.txt("f")?, nv(s, "a")?, nv(s, "b")?);
    let (mx, mn) = (s.out("absolute_max")?, s.out("absolute_min")?);
    let cps = s.out_list("critical_points").unwrap_or_default();
    if !(b > a) {
        return None;
    }
    let (lo, hi) = widen(a, b, 0.2);
    let pts = fcurve(s, &f, lo, hi, 49);
    let inside: Vec<(f64, f64)> = pts.iter().copied().filter(|p| p.0 >= a && p.0 <= b).collect();
    let mut cand: Vec<f64> = vec![a, b];
    cand.extend(cps.iter().copied().filter(|x| *x >= a && *x <= b));
    let at = fvals(s, &f, &{
        let mut c = cand.clone();
        c.sort_by(|x, y| x.partial_cmp(y).unwrap());
        c.dedup();
        c
    });
    let tol = 1e-7 * (mx.abs() + mn.abs() + 1.0);
    let pmax = at.iter().find(|q| (q.1 - mx).abs() <= tol)?;
    let pmin = at.iter().find(|q| (q.1 - mn).abs() <= tol)?;
    let mut p = graph_plot(&pts, lo, hi, &[mx, mn], &format!("max {}, min {}", num(mx), num(mn)))?;
    p.dashed(&pts, 3);
    p.curve(&inside, 0);
    p.vline(a);
    p.vline(b);
    for q in &at {
        p.dot(q.0, q.1, 2, 4.0);
    }
    p.mark(pmax.0, pmax.1, &format!("max {} at x = {}", num(mx), num(pmax.0)));
    p.mark(pmin.0, pmin.1, &format!("min {} at x = {}", num(mn), num(pmin.0)));
    p.key(&format!("y = {} on [{}, {}]", clipx(&f, 16), num(a), num(b)), 0);
    Some(s.chart("graph", "Absolute extrema on a closed interval",
                 &format!("y = {f} on the closed interval [{}, {}] (outside it dashed; curve values computed by the \
                           model). The candidates are the end points and the critical point(s) {}; marked: absolute \
                           maximum {}, absolute minimum {}.", num(a), num(b),
                          if cps.is_empty() { "(none)".into() } else { format!("x = {}", join(&cps)) }, num(mx), num(mn)),
                 &p.body()))
}

pub fn inflection(s: &Solved) -> Option<Figure> {
    let f = s.txt("f")?;
    let ips = s.out_list("inflection_points").unwrap_or_default();
    let (lo, hi) = if ips.is_empty() { (-3.0, 3.0) } else {
        let (l, h) = ips.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)));
        if h > l { widen(l, h, 0.8) } else { (l - 2.5, l + 2.5) }
    };
    let pts = fcurve(s, &f, lo, hi, 41);
    let d2 = derivative_vals(s, &f, &span(lo, hi, 25), "second_derivative_at_point", "second_derivative", None);
    if d2.len() < 5 {
        return None;
    }
    let at = fvals(s, &f, &ips);
    let mut p1 = graph_plot(&pts, lo, hi, &[], &format!("inflection at x = {}",
                                                         if ips.is_empty() { "none".into() } else { clipx(&join(&ips), 24) }))?;
    p1.curve(&pts, 0);
    for q in &at {
        p1.mark(q.0, q.1, &format!("x = {}", num(q.0)));
    }
    p1.key(&format!("y = {}", clipx(&f, 24)), 0);
    let (y0, y1) = yrange(&d2, &[0.0])?;
    let mut p2 = DPlot::new((lo, hi), (y0, y1), "x", "f″(x)", "");
    fill_sign(&mut p2, &d2);
    p2.hline(0.0);
    p2.curve(&d2, 1);
    for x in &ips {
        p2.vline(*x);
    }
    p2.key("f″(x)", 1);
    let (body, h) = stack(&[p1.body(), p2.body()]);
    Some(s.figure("graph", "Inflection points where the concavity changes",
                  &format!("Top: y = {f} (values computed by the model) with its inflection points marked. Bottom: f″(x) \
                            computed by the model at 25 points changes sign there, so the concavity changes. Marked: {}.",
                           if ips.is_empty() { "none".into() } else { format!("x = {}", join(&ips)) }), &body, h))
}

/// The area between a curve and the axis, shaded by sign (accent above, violet below).
pub(crate) fn fill_sign(p: &mut DPlot, pts: &[(f64, f64)]) {
    if pts.len() < 2 {
        return;
    }
    for (k_, sg) in [(0usize, 1.0f64), (2usize, -1.0f64)] {
        let mut poly = vec![(pts[0].0, 0.0)];
        poly.extend(pts.iter().map(|q| (q.0, if q.1 * sg > 0.0 { q.1 } else { 0.0 })));
        poly.push((pts[pts.len() - 1].0, 0.0));
        p.fill(&poly, k_);
    }
}

pub fn limit_inf(s: &Solved) -> Option<Figure> {
    let (f, l) = (s.txt("f")?, s.out("limit")?);
    let xs: Vec<f64> = k::span(3.0, 3e4, 41, true);
    let pts = fvals(s, &f, &xs);
    if pts.len() < 10 {
        return None;
    }
    let (y0, y1) = yrange(&pts[pts.len() / 4..], &[l])?;
    let mut p = DPlot::with_log((3.0, 3e4), (y0, y1), (true, false), "x (logarithmic)", "y", &format!("limit = {}", num(l)));
    p.hline(l);
    p.curve(&pts, 0);
    let last = *pts.last()?;
    p.mark(last.0, last.1, &format!("f({}) = {}", num(last.0), num(last.1)));
    p.key(&format!("y = {}", clipx(&f, 24)), 0);
    p.key(&format!("y = {}", num(l)), 3);
    Some(s.chart("graph", "Limit at infinity: a horizontal asymptote",
                 &format!("y = {f} for x from 3 to 30000 (logarithmic axis; values computed by the model): the values \
                           settle onto the horizontal line y = {}, the limit as x → ∞. Marked: the last value drawn.",
                          num(l)), &p.body()))
}

pub fn limit_point(s: &Solved) -> Option<Figure> {
    let (f, a, l) = (s.txt("f")?, nv(s, "a")?, s.out("limit")?);
    let w = a.abs().max(1.0);
    let xs: Vec<f64> = span(a - w, a + w, 42).into_iter().filter(|x| (x - a).abs() > 1e-9 * w).collect();
    let pts = fvals(s, &f, &xs);
    if pts.len() < 10 {
        return None;
    }
    let left: Vec<(f64, f64)> = pts.iter().copied().filter(|q| q.0 < a).collect();
    let right: Vec<(f64, f64)> = pts.iter().copied().filter(|q| q.0 > a).collect();
    let mut p = graph_plot(&pts, a - w, a + w, &[l], &format!("limit at x = {}: {}", num(a), num(l)))?;
    p.hline(l);
    p.vline(a);
    p.curve(&left, 0);
    p.curve(&right, 0);
    let (cx, cy) = p.px(a, l);
    p.raw(&format!("<circle class=\"axp-k\" cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"6\"/>"));
    p.label(a + w * 0.08, l - (p.ranges().1 .1 - p.ranges().1 .0) * 0.08, "start", "axp-mt", &format!("limit {}", num(l)));
    p.key(&format!("y = {}", clipx(&f, 24)), 0);
    Some(s.chart("graph", &format!("Limit at x = {} across a gap", num(a)),
                 &format!("y = {f} near x = {} (values computed by the model at {} points, none at x = {} itself): from \
                           both sides the values approach {}. Marked: the limit at the gap.", num(a), pts.len(), num(a),
                          num(l)), &p.body()))
}

pub fn linear_approx(s: &Solved) -> Option<Figure> {
    let (f, a, x1) = (s.txt("f")?, nv(s, "a")?, nv(s, "x1")?);
    let (ap, act, err, m) = (s.out("approximation")?, s.out("actual")?, s.out("error")?, s.out("slope")?);
    let fa = f_at(s, &f, a)?;
    let w = (x1 - a).abs().max(a.abs() * 0.5).max(1.0) * 1.5;
    let (lo, hi) = (a.min(x1) - w * 0.5, a.max(x1) + w);
    let pts = fcurve(s, &f, lo, hi, 41);
    let tan: Vec<(f64, f64)> = [lo, hi].iter().map(|x| (*x, fa + m * (x - a))).collect();
    let mut p = graph_plot(&pts, lo, hi, &[fa, ap, act], &format!("L({}) = {}, error {}", num(x1), num(ap), num(err)))?;
    p.curve(&pts, 0);
    p.dashed(&tan, 2);
    p.seg((x1, act), (x1, ap), 1, false);
    p.dot(x1, ap, 2, 4.0);
    p.dot(x1, act, 0, 4.0);
    p.mark(a, fa, &format!("base x = {}", num(a)));
    p.key(&format!("y = {}", clipx(&f, 18)), 0);
    p.key(&format!("tangent, slope {}", num(m)), 2);
    Some(s.chart("graph", "Linear approximation and its error away from the base point",
                 &format!("The tangent-line approximation of y = {f} at x = {} (slope {}; curve values computed by the \
                           model). At x = {} it gives {} against the actual value {} (error {}); the gap between line and \
                           curve grows away from the base point. Marked: the base point.", num(a), num(m), num(x1),
                          num(ap), num(act), num(err)), &p.body()))
}

pub fn mvt(s: &Solved) -> Option<Figure> {
    let (f, a, b, m) = (s.txt("f")?, nv(s, "a")?, nv(s, "b")?, s.out("secant_slope")?);
    let cs = s.out_list("c_values")?;
    if !(b > a) {
        return None;
    }
    let (lo, hi) = widen(a, b, 0.15);
    let pts = fcurve(s, &f, lo, hi, 41);
    let ends = fvals(s, &f, &[a, b]);
    if ends.len() != 2 {
        return None;
    }
    let at = fvals(s, &f, &cs);
    let mut p = graph_plot(&pts, lo, hi, &[ends[0].1, ends[1].1], &format!("secant slope = {}", num(m)))?;
    p.curve(&pts, 0);
    p.seg(ends[0], ends[1], 3, true);
    let w = (b - a) * 0.22;
    for (c, fc) in &at {
        p.seg((c - w, fc - m * w), (c + w, fc + m * w), 2, false);
        p.mark(*c, *fc, &format!("c = {}", num(*c)));
    }
    p.dot(ends[0].0, ends[0].1, 3, 4.0);
    p.dot(ends[1].0, ends[1].1, 3, 4.0);
    p.key(&format!("y = {}", clipx(&f, 20)), 0);
    p.key("tangent at c", 2);
    p.key("secant", 3);
    Some(s.chart("graph", "Mean value theorem: tangents parallel to the secant",
                 &format!("The mean value theorem for y = {f} on [{}, {}] (curve values computed by the model): the secant \
                           slope is {}, and the tangent at each returned c has the same slope (segments drawn with that \
                           slope through (c, f(c))). Marked: c = {}.", num(a), num(b), num(m), join(&cs)), &p.body()))
}

pub fn normal(s: &Solved) -> Option<Figure> {
    let (f, a) = (s.txt("f")?, nv(s, "a")?);
    let (m, c, y0) = (s.out("slope")?, s.out("intercept")?, s.out("y0")?);
    let t = s.solve_other("tangent_line", &[("f", tx(&f)), ("a", Input::Float(a))])?;
    let mt = num_of(&t, "slope")?;
    let w = a.abs().max(1.0) * 1.5;
    let xs: Vec<f64> = vec![a - w, a + w];
    let ys: Vec<f64> = vec![y0 - w, y0 + w];
    let mut p = DPlot::equal(&xs, &ys, "x", "y", &format!("normal: {}", line_eq(m, c)));
    let ((xl, xh), _) = p.ranges();
    let pts = fcurve(s, &f, xl, xh, 49);
    p.curve(&pts, 0);
    p.curve(&[(xl, m * xl + c), (xh, m * xh + c)], 1);
    p.dashed(&[(xl, y0 + mt * (xl - a)), (xh, y0 + mt * (xh - a))], 3);
    p.mark(a, y0, &format!("({}, {})", num(a), num(y0)));
    p.key(&format!("y = {}", clipx(&f, 18)), 0);
    p.key(&format!("normal, slope {}", num(m)), 1);
    p.key(&format!("tangent, slope {}", num(mt)), 3);
    Some(s.chart("graph", &format!("Normal line to y = {} at x = {}", clipx(&f, 30), num(a)),
                 &format!("The graph of y = {f} (values computed by the model) with its normal line at x = {}, drawn with \
                           equal scales so the right angle shows: normal slope {}, intercept {}; the tangent (slope {}) \
                           dashed. Marked: ({}, {}).", num(a), num(m), num(c), num(mt), num(a), num(y0)), &p.body()))
}

pub fn nth_derivative(s: &Solved) -> Option<Figure> {
    let (f, a, n, d) = (s.txt("f")?, nv(s, "a")?, nv(s, "n")?, s.out("nth_derivative")?);
    let n = n as i64;
    if !(1..=40).contains(&n) {
        return None;
    }
    let w = a.abs().max(1.0) * 1.5;
    let (lo, hi) = (a - w, a + w);
    let xs = span(lo, hi, 25);
    let mut series: Vec<(String, Vec<(f64, f64)>)> = vec![("f".into(), fvals(s, &f, &xs))];
    let orders: Vec<i64> = if n <= 3 { (1..=n).collect() } else { vec![n - 2, n - 1, n] };
    for o in &orders {
        let v = derivative_vals(s, &f, &xs, "nth_derivative_at_point", "nth_derivative", Some(*o));
        if v.len() >= 5 {
            series.push((format!("order {o}"), v));
        }
    }
    if series.len() < 2 || series.last()?.0 != format!("order {n}") {
        return None;
    }
    let all: Vec<(f64, f64)> = series.iter().flat_map(|x| x.1.clone()).collect();
    let mut p = graph_plot(&all, lo, hi, &[d], &format!("order-{n} derivative at {} = {}", num(a), num(d)))?;
    for (i, (name, pts)) in series.iter().enumerate() {
        let kk = if i == 0 { 3 } else { [2, 1, 0][(series.len() - 1 - i).min(2)] };
        p.curve(pts, kk);
        p.key(name, kk);
    }
    p.mark(a, d, &format!("{}", num(d)));
    Some(s.chart("graph", &format!("Derivatives of {} up to order {n}", clipx(&f, 30)),
                 &format!("f(x) = {f} (grey) and its derivatives of order {} on [{}, {}], each computed by the model at 25 \
                           points. Marked: the order-{n} derivative at x = {}, {}.",
                          orders.iter().map(|o| o.to_string()).collect::<Vec<_>>().join(", "), num(lo), num(hi), num(a),
                          num(d)), &p.body()))
}

pub fn second_derivative(s: &Solved) -> Option<Figure> {
    let (f, a, d) = (s.txt("f")?, nv(s, "a")?, s.out("second_derivative")?);
    let w = a.abs().max(1.0) * 2.2;
    let (lo, hi) = (a - w, a + w);
    let pts = fcurve(s, &f, lo, hi, 41);
    let d2 = derivative_vals(s, &f, &span(lo, hi, 25), "second_derivative_at_point", "second_derivative", None);
    if d2.len() < 5 {
        return None;
    }
    let fa = f_at(s, &f, a)?;
    let mut p1 = graph_plot(&pts, lo, hi, &[fa], &format!("f″({}) = {}", num(a), num(d)))?;
    p1.curve(&pts, 0);
    p1.mark(a, fa, &format!("x = {}", num(a)));
    p1.key(&format!("y = {}", clipx(&f, 24)), 0);
    let (y0, y1) = yrange(&d2, &[0.0, d])?;
    let mut p2 = DPlot::new((lo, hi), (y0, y1), "x", "f″(x)", "");
    fill_sign(&mut p2, &d2);
    p2.hline(0.0);
    p2.curve(&d2, 1);
    p2.mark(a, d, &format!("f″({}) = {}", num(a), num(d)));
    p2.key("f″(x)", 1);
    let (body, h) = stack(&[p1.body(), p2.body()]);
    Some(s.figure("graph", "Second derivative and concavity",
                  &format!("Top: y = {f} on [{}, {}] (values computed by the model). Bottom: the second derivative f″(x) \
                            from the model at 25 points, shaded where it is positive (concave up) and negative (concave \
                            down). Marked: f″({}) = {}.", num(lo), num(hi), num(a), num(d)), &body, h))
}

pub fn tangent(s: &Solved) -> Option<Figure> {
    let (f, a) = (s.txt("f")?, nv(s, "a")?);
    let (m, c, y0) = (s.out("slope")?, s.out("intercept")?, s.out("y0")?);
    let w = a.abs().max(1.0) * 1.5;
    let (lo, hi) = (a - w, a + w);
    let pts = fcurve(s, &f, lo, hi, 41);
    let mut p = graph_plot(&pts, lo, hi, &[y0], &format!("tangent: {}", line_eq(m, c)))?;
    p.curve(&pts, 0);
    p.curve(&[(lo, m * lo + c), (hi, m * hi + c)], 1);
    p.mark(a, y0, &format!("({}, {})", num(a), num(y0)));
    p.key(&format!("y = {}", clipx(&f, 20)), 0);
    p.key(&format!("tangent, slope {}", num(m)), 1);
    Some(s.chart("graph", &format!("Tangent to y = {} at x = {}", clipx(&f, 30), num(a)),
                 &format!("The graph of y = {f} (curve values computed by the model) and its tangent line at x = {}: slope \
                           f′({}) = {}, intercept {}. Marked: the point of tangency ({}, {}).", num(a), num(a), num(m),
                          num(c), num(a), num(y0)), &p.body()))
}

/// "y = m x + c" with the sign of c written once.
pub(crate) fn line_eq(m: f64, c: f64) -> String {
    let c = ax_engine::figkit::snap(c, m.abs().max(1.0));
    if c < 0.0 { format!("y = {}x − {}", num(m), num(-c)) } else { format!("y = {}x + {}", num(m), num(c)) }
}
