//! Areas: integrals as shaded regions, Riemann sums and quadrature rules with their rectangles, regions between
//! curves and their centroids.

use super::*;
use super::graph::fill_sign;
use ax_engine::figkit::{self as k, num, stack};

fn region_between(f: &[(f64, f64)], g: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut poly: Vec<(f64, f64)> = f.to_vec();
    poly.extend(g.iter().rev().copied());
    poly
}

pub fn between(s: &Solved, enclosed: bool) -> Option<Figure> {
    let (f, g) = (s.txt("f")?, s.txt("g")?);
    let area = s.out("area")?;
    let (a, b, marks) = if enclosed {
        (s.out("x_left")?, s.out("x_right")?, s.out_list("intersections").unwrap_or_default())
    } else {
        (nv(s, "a")?, nv(s, "b")?, s.out_list("crossings").unwrap_or_default())
    };
    if !(b > a) {
        return None;
    }
    let (lo, hi) = if enclosed { widen(a, b, 0.2) } else { (a, b) };
    let xs = span(a, b, 41);
    let (fp, gp) = (fvals(s, &f, &xs), fvals(s, &g, &xs));
    if fp.len() != xs.len() || gp.len() != xs.len() {
        return None;
    }
    let (fo, go) = if enclosed { (fcurve(s, &f, lo, hi, 25), fcurve(s, &g, lo, hi, 25)) } else { (fp.clone(), gp.clone()) };
    let all: Vec<(f64, f64)> = fo.iter().chain(go.iter()).copied().collect();
    let (y0, y1) = yrange(&all, &[])?;
    let mut p = DPlot::new((lo, hi), (y0, y1), "x", "y", &format!("area = {}", num(area)));
    p.fill(&region_between(&fp, &gp), 0);
    p.curve(&fo, 0);
    p.curve(&go, 1);
    let at = fvals(s, &f, &marks);
    for q in &at {
        p.mark(q.0, q.1, &format!("x = {}", num(q.0)));
    }
    p.key(&format!("y = {}", clipx(&f, 16)), 0);
    p.key(&format!("y = {}", clipx(&g, 16)), 1);
    let cap = if enclosed {
        format!("The region enclosed by y = {f} and y = {g} (curve values computed by the model): the curves meet at \
                 x = {}, and the enclosed area between x = {} and x = {} is {}. Marked: the intersections.", join(&marks),
                num(a), num(b), num(area))
    } else {
        let net = s.out("net_integral").map(num).unwrap_or_default();
        format!("The region between y = {f} and y = {g} on [{}, {}] (curve values computed by the model): the area, the \
                 integral of |f − g|, is {}, while the net integral of f − g is {}. Marked: the crossing(s) {}.", num(a),
                num(b), num(area), net, if marks.is_empty() { "(none)".into() } else { format!("at x = {}", join(&marks)) })
    };
    Some(s.chart("area", if enclosed { "Area enclosed by two curves" } else { "Area between two curves" }, &cap, &p.body()))
}

pub fn average(s: &Solved) -> Option<Figure> {
    let (f, a, b) = (s.txt("f")?, nv(s, "a")?, nv(s, "b")?);
    let (avg, integral) = (s.out("average_value")?, s.out("integral")?);
    if !(b > a) {
        return None;
    }
    let pts = fcurve(s, &f, a, b, 41);
    if pts.len() < 10 {
        return None;
    }
    let (y0, y1) = yrange(&pts, &[0.0, avg])?;
    let (lo, hi) = widen(a, b, 0.08);
    let mut p = DPlot::new((lo, hi), (y0, y1), "x", "y", &format!("average value = {}", num(avg)));
    fill_sign(&mut p, &pts);
    p.curve(&[(a, 0.0), (a, avg), (b, avg), (b, 0.0)], 1);
    p.curve(&pts, 0);
    p.hline(0.0);
    p.mark((a + b) / 2.0, avg, &format!("average {}", num(avg)));
    p.key(&format!("y = {}", clipx(&f, 20)), 0);
    p.key("rectangle of equal area", 1);
    Some(s.chart("area", "Average value of a function",
                 &format!("y = {f} on [{}, {}] (curve values computed by the model): its integral {} equals the area of a \
                           rectangle of width {} and height the average value {}. Marked: the average value.", num(a),
                          num(b), num(integral), num(b - a), num(avg)), &p.body()))
}

pub fn definite(s: &Solved) -> Option<Figure> {
    let (f, a, b, v) = (s.txt("f")?, nv(s, "a")?, nv(s, "b")?, s.out("integral")?);
    let (l, h) = (a.min(b), a.max(b));
    if !(h > l) {
        return None;
    }
    let pts = fcurve(s, &f, l, h, 49);
    if pts.len() < 10 {
        return None;
    }
    let (y0, y1) = yrange(&pts, &[0.0])?;
    let (lo, hi) = widen(l, h, 0.08);
    let mut p = DPlot::new((lo, hi), (y0, y1), "x", "y", &format!("integral = {}", num(v)));
    fill_sign(&mut p, &pts);
    p.hline(0.0);
    p.curve(&pts, 0);
    let mid = pts[pts.len() / 2];
    p.mark(mid.0, mid.1 / 2.0, &format!("∫ = {}", num(v)));
    p.key(&format!("y = {}", clipx(&f, 20)), 0);
    Some(s.chart("area", "A definite integral as signed area",
                 &format!("The definite integral of y = {f} from {} to {} (curve values computed by the model) is the \
                           signed area: regions above the axis count positive, below negative. Marked: the integral {}.",
                          num(a), num(b), num(v)), &p.body()))
}

pub fn ftc(s: &Solved) -> Option<Figure> {
    let (f, a, g, x0, d) = (s.txt("f")?, nv(s, "a")?, s.txt("g")?, nv(s, "x0")?, s.out("derivative")?);
    let fx = subst(&f, "t", "x");
    let gx = f_at(s, &g, x0)?;
    let (l, h) = (a.min(gx), a.max(gx));
    let w = (h - l).max(0.5);
    let (lo, hi) = (l - 0.25 * w, h + 0.35 * w);
    let pts = fcurve(s, &fx, lo, hi, 41);
    let inside = fcurve(s, &fx, l, h, 25);
    if pts.len() < 10 || inside.len() < 5 {
        return None;
    }
    let (y0, y1) = yrange(&pts, &[0.0])?;
    let mut p = DPlot::new((lo, hi), (y0, y1), "t", "f(t)", &format!("F′({}) = {}", num(x0), num(d)));
    fill_sign(&mut p, &inside);
    p.hline(0.0);
    p.curve(&pts, 0);
    p.vline(gx);
    let fg = f_at(s, &fx, gx)?;
    p.mark(gx, fg, &format!("g({}) = {}", num(x0), num(gx)));
    p.key(&format!("f(t) = {}", clipx(&f, 18)), 0);
    p.key(&format!("from {} to g(x) = {}", num(a), clipx(&g, 10)), 3);
    Some(s.chart("area", "Derivative of an integral with a variable limit",
                 &format!("F(x) = ∫ from {} to g(x) = {g} of f(t) = {f} dt: the shaded area up to the moving limit g({}) = \
                           {} (values computed by the model, t written as x). By the fundamental theorem its rate of \
                           change is f(g(x))·g′(x); the model gives F′({}) = {} (marked).", num(a), num(x0), num(gx),
                          num(x0), num(d)), &p.body()))
}

pub fn improper(s: &Solved) -> Option<Figure> {
    let (f, a, v) = (s.txt("f")?, nv(s, "a")?, s.out("integral")?);
    let w = 8.0 * a.abs().max(1.0);
    let pts = fcurve(s, &f, a, a + w, 41);
    if pts.len() < 10 {
        return None;
    }
    let (y0, y1) = yrange(&pts, &[0.0])?;
    let mut p1 = DPlot::new((a, a + w), (y0, y1), "x", "y", &format!("∫ from {} to ∞ = {}", num(a), num(v)));
    fill_sign(&mut p1, &pts);
    p1.hline(0.0);
    p1.curve(&pts, 0);
    p1.key(&format!("y = {}", clipx(&f, 24)), 0);
    let bs = span(a + w / 16.0, a + w, 14);
    let partial: Vec<(f64, f64)> = bs.iter().filter_map(|b| {
        let r = s.solve_other("definite_integral", &[("f", tx(&f)), ("a", Input::Float(a)), ("b", Input::Float(*b))])?;
        num_of(&r, "integral").map(|x| (*b, x))
    }).collect();
    if partial.len() < 4 {
        return None;
    }
    let (q0, q1) = yrange(&partial, &[v, 0.0])?;
    let mut p2 = DPlot::new((a, a + w), (q0, q1), "upper limit b", "∫ from a to b", "");
    p2.hline(v);
    p2.curve(&partial, 1);
    for q in &partial {
        p2.dot(q.0, q.1, 1, 3.0);
    }
    p2.mark(a + w, v, &format!("limit {}", num(v)));
    p2.key("integral from a to b", 1);
    let (body, h) = stack(&[p1.body(), p2.body()]);
    Some(s.figure("area", "An improper integral as the limit of finite ones",
                  &format!("Top: y = {f} from x = {} (values computed by the model), the area extending without end. \
                            Bottom: the integral from {} to b for b up to {}, each computed by the model, approaching the \
                            improper integral {} (marked).", num(a), num(a), num(a + w), num(v)), &body, h))
}

/// The model's own sum for one rule, the same function and interval (left, right, midpoint, trapezoid, simpson).
fn rule_sum(s: &Solved, rule: &str, f: &str, a: f64, b: f64, n: i64) -> Option<f64> {
    let (key, out) = match rule {
        "left" => ("left_riemann_sum", "approximation"),
        "right" => ("right_riemann_sum", "approximation"),
        "midpoint" => ("midpoint_rule", "approximation"),
        "trapezoid" => ("trapezoid_rule", "approximation"),
        _ => ("simpson_rule", "approximation"),
    };
    let r = s.solve_other(key, &[("f", tx(f)), ("a", Input::Float(a)), ("b", Input::Float(b)), ("n", Input::Int(n))])?;
    num_of(&r, out)
}

pub fn riemann(s: &Solved, rule: &str) -> Option<Figure> {
    let (f, a, b, n) = (s.txt("f")?, nv(s, "a")?, nv(s, "b")?, nv(s, "n")?);
    let sum = s.out("approximation")?;
    let n = n as usize;
    if !(b > a) || n == 0 || n > 120 {
        return None;
    }
    let h = (b - a) / n as f64;
    let curve = fcurve(s, &f, a, b, 49);
    let nodes: Vec<f64> = match rule {
        "midpoint" => (0..n).map(|i| a + (i as f64 + 0.5) * h).collect(),
        "left" => (0..n).map(|i| a + i as f64 * h).collect(),
        "right" => (1..=n).map(|i| a + i as f64 * h).collect(),
        _ => (0..=n).map(|i| a + i as f64 * h).collect(),
    };
    let fv = fvals(s, &f, &nodes);
    if fv.len() != nodes.len() || curve.len() < 10 {
        return None;
    }
    let exact = s.out("exact").or_else(|| {
        let r = s.solve_other("definite_integral", &[("f", tx(&f)), ("a", Input::Float(a)), ("b", Input::Float(b))])?;
        num_of(&r, "integral")
    });
    let (y0, y1) = yrange(&curve, &fv.iter().map(|q| q.1).chain([0.0]).collect::<Vec<_>>())?;
    let (lo, hi) = widen(a, b, 0.05);
    let name = match rule {
        "left" => "left rectangles",
        "right" => "right rectangles",
        "midpoint" => "midpoint rectangles",
        "trapezoid" => "trapezoids",
        _ => "Simpson parabolas",
    };
    let mut p = DPlot::new((lo, hi), (y0, y1), "x", "y", &format!("{name}: {}", num(sum)));
    match rule {
        "trapezoid" => {
            for w in fv.windows(2) {
                p.fill(&[(w[0].0, 0.0), w[0], w[1], (w[1].0, 0.0)], 2);
                p.seg(w[0], w[1], 2, false);
                p.seg((w[1].0, 0.0), w[1], 3, false);
            }
        }
        "simpson" => {
            // a parabola through each pair of panels (its three node values from the model), panels separated
            for (j, w) in fv.windows(3).enumerate().filter(|(j, _)| j % 2 == 0) {
                let _ = j;
                let par: Vec<(f64, f64)> = span(w[0].0, w[2].0, 17).into_iter().map(|x| {
                    let l0 = (x - w[1].0) * (x - w[2].0) / ((w[0].0 - w[1].0) * (w[0].0 - w[2].0));
                    let l1 = (x - w[0].0) * (x - w[2].0) / ((w[1].0 - w[0].0) * (w[1].0 - w[2].0));
                    let l2 = (x - w[0].0) * (x - w[1].0) / ((w[2].0 - w[0].0) * (w[2].0 - w[1].0));
                    (x, w[0].1 * l0 + w[1].1 * l1 + w[2].1 * l2)
                }).collect();
                let mut poly = vec![(w[0].0, 0.0)];
                poly.extend(par.iter().copied());
                poly.push((w[2].0, 0.0));
                p.fill(&poly, 2);
                p.curve(&par, 2);
                p.seg((w[2].0, 0.0), w[2], 3, true);
                p.seg((w[0].0, 0.0), w[0], 3, true);
            }
        }
        _ => {
            for (i, q) in fv.iter().enumerate() {
                let x0 = match rule {
                    "left" => q.0,
                    "right" => q.0 - h,
                    _ => q.0 - h / 2.0,
                };
                let _ = i;
                p.rect(x0, x0 + h, 0.0, q.1, 2);
            }
        }
    }
    p.hline(0.0);
    p.curve(&curve, 0);
    if fv.len() <= 40 {
        for q in &fv {
            p.dot(q.0, q.1, 1, 3.2);
        }
    }
    p.key(name, 2);
    p.key(&format!("y = {}", clipx(&f, 18)), 0);
    let ex = exact.map(|e| format!(", exact integral {}", num(e))).unwrap_or_default();
    let err = s.out("error").map(|e| format!(", error {}", num(e))).unwrap_or_default();
    let title = match rule {
        "left" => "Left Riemann sum",
        "right" => "Right Riemann sum",
        "midpoint" => "Midpoint rule",
        "trapezoid" => "Trapezoid rule",
        _ => "Simpson's rule",
    };
    Some(s.chart("area", title,
                 &format!("y = {f} on [{}, {}] and the {n} {name} of the rule (node values and curve computed by the \
                           model; dots mark the sample points): sum {}{ex}{err}.", num(a), num(b), num(sum)), &p.body()))
}

pub fn riemann_convergence(s: &Solved) -> Option<Figure> {
    let (f, a, b, n0) = (s.txt("f")?, nv(s, "a")?, nv(s, "b")?, nv(s, "n")?);
    if !(b > a) {
        return None;
    }
    let ns: Vec<i64> = [1, 2, 4, 8, 16, 32, 64].to_vec();
    let mut left = vec![];
    let mut right = vec![];
    for n in &ns {
        if let Some(v) = rule_sum(s, "left", &f, a, b, *n) {
            left.push((*n as f64, v));
        }
        if let Some(v) = rule_sum(s, "right", &f, a, b, *n) {
            right.push((*n as f64, v));
        }
    }
    let exact = {
        let r = s.solve_other("definite_integral", &[("f", tx(&f)), ("a", Input::Float(a)), ("b", Input::Float(b))])?;
        num_of(&r, "integral")?
    };
    if left.len() < 3 || right.len() < 3 {
        return None;
    }
    let mine = s.out("approximation")?;
    let ex: Vec<(f64, f64)> = vec![(1.0, exact), (64.0, exact)];
    let series = vec![("left sum".to_string(), left.clone()), ("right sum".to_string(), right.clone()),
                      (format!("integral {}", num(exact)), ex)];
    let body = k::lines(&series, "Number of rectangles n (logarithmic)", "Sum", &format!("n = {}: left sum {}", num(n0),
                        num(mine)), Some((n0, mine)), &format!("n = {}", num(n0)), true);
    let (l64, r64) = (left.last()?.1, right.last()?.1);
    Some(s.chart("convergence", "Left and right sums closing in on the integral",
                 &format!("Left and right Riemann sums of y = {f} on [{}, {}] for n = 1 … 64 rectangles, each computed by \
                           the model: both close in on the integral {}; at n = 64 they are {} and {}. Marked: the \
                           request's n = {}, left sum {}.", num(a), num(b), num(exact), num(l64), num(r64), num(n0),
                          num(mine)), &body))
}

pub fn centroid(s: &Solved) -> Option<Figure> {
    let (f, a, b) = (s.txt("f")?, nv(s, "a")?, nv(s, "b")?);
    let (area, xb, yb) = (s.out("area")?, s.out("x_bar")?, s.out("y_bar")?);
    if !(b > a) {
        return None;
    }
    let pts = fcurve(s, &f, a, b, 41);
    if pts.len() < 10 {
        return None;
    }
    let xs: Vec<f64> = pts.iter().map(|q| q.0).chain([xb]).collect();
    let ys: Vec<f64> = pts.iter().map(|q| q.1).chain([0.0, yb]).collect();
    let mut p = DPlot::equal(&xs, &ys, "x", "y", &format!("area = {}", num(area)));
    fill_sign(&mut p, &pts);
    p.hline(0.0);
    p.curve(&pts, 0);
    p.mark(xb, yb, &format!("centroid ({}, {})", num(xb), num(yb)));
    p.key(&format!("y = {}", clipx(&f, 22)), 0);
    Some(s.chart("region", "Centroid of a region under a curve",
                 &format!("The region under y = {f} between x = {} and x = {}, drawn to scale (curve values computed by the \
                           model): area {}. Marked: its centroid (x̄, ȳ) = ({}, {}).", num(a), num(b), num(area), num(xb),
                          num(yb)), &p.body()))
}

pub fn double_region(s: &Solved) -> Option<Figure> {
    let (f, a, b, g1, g2) = (s.txt("f")?, nv(s, "a")?, nv(s, "b")?, s.txt("g1")?, s.txt("g2")?);
    let v = s.out("integral")?;
    if !(b > a) {
        return None;
    }
    let xs = span(a, b, 33);
    let (lo_c, hi_c) = (fvals(s, &g1, &xs), fvals(s, &g2, &xs));
    if lo_c.len() != xs.len() || hi_c.len() != xs.len() {
        return None;
    }
    let all: Vec<(f64, f64)> = lo_c.iter().chain(hi_c.iter()).copied().collect();
    let (y0, y1) = yrange(&all, &[])?;
    let (xl, xh) = widen(a, b, 0.1);
    let (yl, yh) = widen(y0, y1, 0.1);
    let mut p = DPlot::new((xl, xh), (yl, yh), "x", "y", &format!("∬ f dA = {}", num(v)));
    p.fill(&region_between(&lo_c, &hi_c), 0);
    for i in (2..xs.len() - 1).step_by(4) {
        p.seg(lo_c[i], hi_c[i], 3, true);
        p.arrow(lo_c[i], ((lo_c[i].0 + hi_c[i].0) / 2.0, (lo_c[i].1 + hi_c[i].1) / 2.0), 3);
    }
    p.curve(&lo_c, 1);
    p.curve(&hi_c, 2);
    p.vline(a);
    p.vline(b);
    p.key(&format!("y = {}", clipx(&g1, 14)), 1);
    p.key(&format!("y = {}", clipx(&g2, 14)), 2);
    Some(s.chart("region", "Double integral over a region between curves",
                 &format!("The region {} ≤ x ≤ {}, between y = {g1} and y = {g2} (boundary curves computed by the model), \
                           with vertical strips marking the inner integration in y: the double integral of f = {f} over it \
                           is {}.", num(a), num(b), num(v)), &p.body()))
}
