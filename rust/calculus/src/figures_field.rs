//! Maps over two inputs and plane fields: a value shaded over a grid of the model's runs, a function of x and y
//! shaded with gradient or field arrows, paths and regions on it.

use super::*;
use ax_engine::figkit::{self as k, num};

/// The answer shaded over a grid of two of its number inputs (each cell one run of the model).
pub fn two_input(s: &Solved, xsym: &str, ysym: &str, out: &str, log: (bool, bool)) -> Option<Figure> {
    let (x0, y0, v0) = (nv(s, xsym)?, nv(s, ysym)?, s.out(out)?);
    let range = |sym: &str, v: f64, lg: bool| -> Option<(f64, f64)> {
        if lg {
            return (v > 0.0).then(|| (v / 10.0, v * 10.0));
        }
        Some(match (s.key, sym) {
            ("geometric_series_sum", "r") => (-0.95, 0.95),
            ("disk_volume", "a") => {
                let w = (nv(s, "b")? - v).abs().max(0.5);
                (v - w, v + w)
            }
            ("disk_volume", "b") => {
                let w = (v - nv(s, "a")?).abs().max(0.5);
                (v - w / 2.0, v + w / 2.0)
            }
            _ => {
                let h = if v != 0.0 { v.abs() * 0.5 } else { 1.0 };
                (v - h, v + h)
            }
        })
    };
    let (xr, yr) = (range(xsym, x0, log.0)?, range(ysym, y0, log.1)?);
    let xs = k::span(xr.0, xr.1, 13, log.0);
    let ys = k::span(yr.0, yr.1, 9, log.1);
    let vals: Vec<Vec<Option<f64>>> = xs.iter()
        .map(|x| ys.iter().map(|y| s.with(&[(xsym, *x), (ysym, *y)]).and_then(|a| num_of(&a, out))).collect()).collect();
    let n = vals.iter().flatten().flatten().count();
    if n < 20 {
        return None;
    }
    // values over many orders of magnitude are shaded by their logarithm (else one corner holds all the shade)
    let all: Vec<f64> = vals.iter().flatten().flatten().copied().collect();
    let (lo, hi) = all.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)));
    let log_shade = lo > 0.0 && hi / lo > 30.0;
    let shown: Vec<Vec<Option<f64>>> = if log_shade {
        vals.iter().map(|c| c.iter().map(|v| v.map(|x| x.log10())).collect()).collect()
    } else {
        vals.clone()
    };
    let head = if log_shade { format!("{out} = {} (shade: log10 of {out})", num(v0)) } else { format!("{out} = {}", num(v0)) };
    let body = k::field(&xs, &ys, &shown, (x0, y0), xsym, ysym, &head, log);
    let what = match s.key {
        "beta_function" => "Beta function B(a, b)",
        "regularized_gamma" => "Regularized incomplete gamma function P(a, x)",
        "geometric_series_sum" => "Sum of an infinite geometric series",
        _ => "Volume of revolution (disk method)",
    };
    Some(s.chart("field", &format!("{what} over {xsym} and {ysym}"),
                 &format!("{out} over {xsym} from {} to {} and {ysym} from {} to {}{}: each of the 13 × 9 cells is one run \
                           of the model, shaded by {} (darker is larger; from {} to {}); blank cells are inputs the model refuses. \
                           Marked: {xsym} = {}, {ysym} = {}, {out} = {}.", num(xr.0), num(xr.1), num(yr.0), num(yr.1),
                          if log.0 || log.1 { " (logarithmic axes)" } else { "" },
                          if log_shade { format!("the logarithm of its value") } else { "its value".to_string() },
                          num(lo), num(hi), num(x0), num(y0), num(v0)), &body))
}

/// A square window round a point, drawn with equal scales; the grid spans what the plot box shows.
fn window(c: (f64, f64), r: f64, header: &str) -> (DPlot, Vec<f64>, Vec<f64>) {
    let p = DPlot::equal(&[c.0 - r, c.0 + r], &[c.1 - r, c.1 + r], "x", "y", header);
    let ((x0, x1), (y0, y1)) = p.ranges();
    let xs = span(x0 + (x1 - x0) / 30.0, x1 - (x1 - x0) / 30.0, 15);
    let ys = span(y0 + (y1 - y0) / 18.0, y1 - (y1 - y0) / 18.0, 9);
    (p, xs, ys)
}

/// Arrows of a plane field, all with one scale (the longest about 0.45 of the grid spacing).
fn arrows(p: &mut DPlot, field: &[((f64, f64), (f64, f64))], spacing: f64, k_: usize) -> f64 {
    let m = field.iter().map(|(_, v)| (v.0 * v.0 + v.1 * v.1).sqrt()).fold(0.0, f64::max);
    if !(m > 0.0) {
        return 0.0;
    }
    let sc = 0.45 * spacing / m;
    for ((x, y), (u, v)) in field {
        p.arrow((x - u * sc / 2.0, y - v * sc / 2.0), (x + u * sc / 2.0, y + v * sc / 2.0), k_);
    }
    sc
}

fn point2(s: &Solved) -> Option<(f64, f64)> {
    let p = s.vec("p")?;
    (p.len() == 2).then(|| (p[0], p[1]))
}

fn radius(c: (f64, f64)) -> f64 {
    (c.0.abs().max(c.1.abs()) * 0.6).max(1.5)
}

/// f(x, y) shaded on a window round c (the model's values); None when too few cells were computed.
fn shaded_f(s: &Solved, f: &str, c: (f64, f64), r: f64, header: &str, signed: bool) -> Option<(DPlot, Vec<f64>, Vec<f64>)> {
    let (mut p, xs, ys) = window(c, r, header);
    let vals = grid(s, f, &xs, &ys);
    if vals.iter().flatten().flatten().count() < 30 {
        return None;
    }
    shade(&mut p, &xs, &ys, &vals, signed)?;
    Some((p, xs, ys))
}

pub fn double_rect(s: &Solved) -> Option<Figure> {
    let (f, a, b, c, d, v) = (s.txt("f")?, nv(s, "a")?, nv(s, "b")?, nv(s, "c")?, nv(s, "d")?, s.out("integral")?);
    if !(b > a && d > c) {
        return None;
    }
    let xs = span(a + (b - a) / 26.0, b - (b - a) / 26.0, 13);
    let ys = span(c + (d - c) / 18.0, d - (d - c) / 18.0, 9);
    let vals = grid(s, &f, &xs, &ys);
    if vals.iter().flatten().flatten().count() < 30 {
        return None;
    }
    let (xl, xh) = widen(a, b, 0.08);
    let (yl, yh) = widen(c, d, 0.08);
    let mut p = DPlot::new((xl, xh), (yl, yh), "x", "y", &format!("∬ f dA = {}", num(v)));
    let (lo, hi) = shade(&mut p, &xs, &ys, &vals, true)?;
    p.outline(&[(a, c), (b, c), (b, d), (a, d)], 3);
    p.key(&format!("f = {}", clipx(&f, 20)), 0);
    Some(s.chart("field", "Double integral over a rectangle",
                 &format!("The integrand f(x, y) = {f} over the rectangle [{}, {}] × [{}, {}], shaded by value (13 × 9 \
                           values computed by the model, accent positive, violet negative; from {} to {}): the double \
                           integral, the signed volume under the surface, is {}.", num(a), num(b), num(c), num(d),
                          num(lo), num(hi), num(v)), &p.body()))
}

pub fn gradient(s: &Solved) -> Option<Figure> {
    let (f, c) = (s.txt("f")?, point2(s)?);
    let g = s.out_list("gradient")?;
    let mag = s.out("magnitude")?;
    let r = radius(c);
    let (mut p, xs, ys) = shaded_f(s, &f, c, r, &format!("∇f({}, {}) = ({}, {})", num(c.0), num(c.1), num(g[0]), num(g[1])),
                                   true)?;
    let mut field = vec![];
    for x in xs.iter().step_by(3) {
        for y in ys.iter().step_by(2) {
            if let Some(a) = s.solve_other("gradient", &[("f", tx(&f)), ("p", Input::Json(json!([x, y])))]) {
                if let Some(v) = ax_engine::figkit::list_of(&a, "gradient") {
                    field.push(((*x, *y), (v[0], v[1])));
                }
            }
        }
    }
    let sc = arrows(&mut p, &field, (xs[1] - xs[0]) * 3.0, 3);
    if sc > 0.0 {
        p.arrow(c, (c.0 + g[0] * sc, c.1 + g[1] * sc), 1);
    }
    p.mark(c.0, c.1, &format!("|∇f| = {}", num(mag)));
    p.key(&format!("f = {}", clipx(&f, 20)), 0);
    p.key("gradient", 3);
    Some(s.chart("field", "Gradient arrows across the shaded function",
                 &format!("f(x, y) = {f} shaded by value (accent positive, violet negative; f computed by the model on a \
                           15 × 9 grid) and its gradient from the model at {} points (arrows, all scaled by the same \
                           factor): the gradient points uphill, across the level curves. Marked: ∇f({}, {}) = ({}, {}), \
                           magnitude {}.", field.len(), num(c.0), num(c.1), num(g[0]), num(g[1]), num(mag)), &p.body()))
}

pub fn directional(s: &Solved) -> Option<Figure> {
    let (f, c, u) = (s.txt("f")?, point2(s)?, s.vec("u")?);
    let dd = s.out("directional_derivative")?;
    if u.len() != 2 {
        return None;
    }
    let g = s.solve_other("gradient", &[("f", tx(&f)), ("p", Input::Json(json!([c.0, c.1])))])
        .and_then(|a| ax_engine::figkit::list_of(&a, "gradient"))?;
    let r = radius(c);
    let (mut p, _, _) = shaded_f(s, &f, c, r, &format!("D_u f = {}", num(dd)), true)?;
    let gl = (g[0] * g[0] + g[1] * g[1]).sqrt();
    let ul = (u[0] * u[0] + u[1] * u[1]).sqrt();
    if !(ul > 0.0) {
        return None;
    }
    let len = r * 0.7;
    if gl > 0.0 {
        p.arrow(c, (c.0 + g[0] / gl * len, c.1 + g[1] / gl * len), 2);
    }
    p.arrow(c, (c.0 + u[0] / ul * len, c.1 + u[1] / ul * len), 1);
    p.mark(c.0, c.1, &format!("D = {}", num(dd)));
    p.key(&format!("gradient ({}, {})", num(g[0]), num(g[1])), 2);
    p.key(&format!("direction ({}, {})", num(u[0]), num(u[1])), 1);
    Some(s.chart("field", "Directional derivative against the gradient",
                 &format!("f(x, y) = {f} shaded round ({}, {}) (f computed by the model), the gradient there ({}, {}) and \
                           the stated direction ({}, {}) as unit arrows: the directional derivative, the gradient's \
                           component along that direction, is {} (marked).", num(c.0), num(c.1), num(g[0]), num(g[1]),
                          num(u[0]), num(u[1]), num(dd)), &p.body()))
}

pub fn partial(s: &Solved) -> Option<Figure> {
    let (f, c, var) = (s.txt("f")?, point2(s)?, s.txt("var")?);
    let d = s.out("partial")?;
    let other = if var == "x" { "y" } else if var == "y" { "x" } else { return None };
    let d2 = s.solve_other("partial_derivative", &[("f", tx(&f)), ("var", tx(other)), ("p", Input::Json(json!([c.0, c.1])))])
        .and_then(|a| num_of(&a, "partial"))?;
    let (dx, dy) = if var == "x" { (d, d2) } else { (d2, d) };
    let fv = f2(s, &f, c.0, c.1)?;
    let r = radius(c);
    let (mut p, _, _) = shaded_f(s, &f, c, r, &format!("∂f/∂x = {}, ∂f/∂y = {}", num(dx), num(dy)), true)?;
    let ((x0, x1), (y0, y1)) = p.ranges();
    p.seg((x0, c.1), (x1, c.1), 0, false);
    p.seg((c.0, y0), (c.0, y1), 2, false);
    p.mark(c.0, c.1, &format!("f = {}", num(fv)));
    p.key(&format!("along x: ∂f/∂x = {}", num(dx)), 0);
    p.key(&format!("along y: ∂f/∂y = {}", num(dy)), 2);
    Some(s.chart("field", "Partial derivatives along the coordinate lines",
                 &format!("f(x, y) = {f} shaded round ({}, {}) (f computed by the model): the partial derivatives are the \
                           slopes of f along the two lines through the point, ∂f/∂x = {} and ∂f/∂y = {}, both from the \
                           model. Marked: the point, f = {}.", num(c.0), num(c.1), num(dx), num(dy), num(fv)), &p.body()))
}

pub fn hessian(s: &Solved) -> Option<Figure> {
    let (f, c) = (s.txt("f")?, point2(s)?);
    let (fxx, fxy, fyy, dd) = (s.out("fxx")?, s.out("fxy")?, s.out("fyy")?, s.out("D")?);
    let cls = s.out_text("classification")?;
    let r = radius(c).min(2.0);
    // shaded from its smallest value (light) to its largest (dark): a minimum shows as a light hollow
    let (mut p, _, _) = shaded_f(s, &f, c, r, &format!("D = {}: {cls}", num(dd)), false)?;
    p.mark(c.0, c.1, &cls);
    p.key(&format!("f = {}", clipx(&f, 24)), 0);
    Some(s.chart("field", "Second-derivative test on the shaded function",
                 &format!("f(x, y) = {f} shaded round ({}, {}) (f computed by the model on a 15 × 9 grid, light where it is \
                           smallest, dark where largest): fxx = {}, fyy = {}, fxy = {}, D = fxx·fyy − fxy² = {}, so the point is a {cls}.",
                          num(c.0), num(c.1), num(fxx), num(fyy), num(fxy), num(dd)), &p.body()))
}

pub fn laplacian(s: &Solved) -> Option<Figure> {
    let (f, c, l) = (s.txt("f")?, point2(s)?, s.out("laplacian")?);
    let r = radius(c);
    let (mut p, xs, ys) = window(c, r, &format!("∇²f({}, {}) = {}", num(c.0), num(c.1), num(l)));
    let xs: Vec<f64> = xs.into_iter().step_by(1).collect::<Vec<_>>();
    let xs: Vec<f64> = span(xs[0], xs[xs.len() - 1], 13);
    let vals: Vec<Vec<Option<f64>>> = xs.iter().map(|x| ys.iter().map(|y| {
        s.solve_other("laplacian", &[("f", tx(&f)), ("p", Input::Json(json!([x, y])))]).and_then(|a| num_of(&a, "laplacian"))
    }).collect()).collect();
    if vals.iter().flatten().flatten().count() < 30 {
        return None;
    }
    let (lo, hi) = shade(&mut p, &xs, &ys, &vals, true)?;
    p.mark(c.0, c.1, &format!("∇²f = {}", num(l)));
    p.key(&format!("∇²f from {} to {}", num(lo), num(hi)), 0);
    Some(s.chart("field", "Where the Laplacian is positive and negative",
                 &format!("The Laplacian of f(x, y) = {f} round ({}, {}), computed by the model at {} points: accent where \
                           ∇²f > 0 (f below the average of its surroundings, a hollow), violet where ∇²f < 0 (a hump); \
                           here it ranges from {} to {}. Marked: ∇²f({}, {}) = {}.", num(c.0), num(c.1),
                          vals.iter().flatten().flatten().count(), num(lo), num(hi), num(c.0), num(c.1), num(l)),
                 &p.body()))
}

fn point3(s: &Solved) -> Option<(f64, f64, f64)> {
    let p = s.vec("p")?;
    (p.len() == 3).then(|| (p[0], p[1], p[2]))
}

fn field3(s: &Solved) -> Option<(String, String, String)> {
    Some((s.txt("P")?, s.txt("Q")?, s.txt("R")?))
}

pub fn curl(s: &Solved) -> Option<Figure> {
    let ((pp, qq, rr), c) = (field3(s)?, point3(s)?);
    let cv = s.out_list("curl")?;
    let r = radius((c.0, c.1));
    let (mut p, xs, ys) = window((c.0, c.1), r, &format!("curl at the point = ({}, {}, {})", num(cv[0]), num(cv[1]),
                                                          num(cv[2])));
    let xs = span(xs[0], xs[xs.len() - 1], 11);
    let ys = span(ys[0], ys[ys.len() - 1], 7);
    let mut zs: Vec<Vec<Option<f64>>> = vec![];
    let mut field = vec![];
    for x in &xs {
        let mut col = vec![];
        for y in &ys {
            let a = s.solve_other("curl", &[("P", tx(&pp)), ("Q", tx(&qq)), ("R", tx(&rr)),
                                            ("p", Input::Json(json!([x, y, c.2])))])
                .and_then(|a| ax_engine::figkit::list_of(&a, "curl"));
            col.push(a.as_ref().map(|v| v[2]));
            if let Some(v) = a {
                field.push(((*x, *y), (v[0], v[1])));
            }
        }
        zs.push(col);
    }
    if field.len() < 20 {
        return None;
    }
    shade(&mut p, &xs, &ys, &zs, true);
    arrows(&mut p, &field, xs[1] - xs[0], 3);
    p.mark(c.0, c.1, &format!("({}, {}, {})", num(cv[0]), num(cv[1]), num(cv[2])));
    p.key("x and y components", 3);
    p.key("z component shaded", 0);
    Some(s.chart("field", &format!("Curl of a field in the plane z = {}", num(c.2)),
                 &format!("The curl of F = ({pp}, {qq}, {rr}) in the plane z = {}, computed by the model at {} points: \
                           arrows show its x and y components (one scale), shading its z component (accent positive, \
                           violet negative). At ({}, {}, {}) the curl is ({}, {}, {}).", num(c.2), field.len(), num(c.0),
                          num(c.1), num(c.2), num(cv[0]), num(cv[1]), num(cv[2])), &p.body()))
}

pub fn divergence(s: &Solved) -> Option<Figure> {
    let ((pp, qq, rr), c) = (field3(s)?, point3(s)?);
    let dv = s.out("divergence")?;
    let r = radius((c.0, c.1));
    let (mut p, xs, ys) = window((c.0, c.1), r, &format!("div F at the point = {}", num(dv)));
    let xs1 = span(xs[0], xs[xs.len() - 1], 11);
    let ys1 = span(ys[0], ys[ys.len() - 1], 7);
    let vals: Vec<Vec<Option<f64>>> = xs1.iter().map(|x| ys1.iter().map(|y| {
        s.solve_other("divergence", &[("P", tx(&pp)), ("Q", tx(&qq)), ("R", tx(&rr)), ("p", Input::Json(json!([x, y, c.2])))])
            .and_then(|a| num_of(&a, "divergence"))
    }).collect()).collect();
    if vals.iter().flatten().flatten().count() < 20 {
        return None;
    }
    shade(&mut p, &xs1, &ys1, &vals, true)?;
    // the field's own x and y components in that plane (z written in as its value)
    let (pz, qz) = (subst(&pp, "z", &lit(c.2)), subst(&qq, "z", &lit(c.2)));
    let mut field = vec![];
    for x in span(xs[0], xs[xs.len() - 1], 7) {
        for y in span(ys[0], ys[ys.len() - 1], 5) {
            if let (Some(u), Some(v)) = (f2(s, &pz, x, y), f2(s, &qz, x, y)) {
                field.push(((x, y), (u, v)));
            }
        }
    }
    let spacing = (xs[xs.len() - 1] - xs[0]) / 6.0;
    arrows(&mut p, &field, spacing, 3);
    p.mark(c.0, c.1, &format!("div F = {}", num(dv)));
    p.key("source + / sink −", 0);
    p.key("(P, Q) arrows", 3);
    Some(s.chart("field", &format!("Divergence of a field in the plane z = {}", num(c.2)),
                 &format!("The divergence of F = ({pp}, {qq}, {rr}) in the plane z = {}, computed by the model at {} points \
                           (accent: a source, violet: a sink), with the field's x and y components as arrows (computed by \
                           the model, z written in). At ({}, {}, {}): div F = {}.", num(c.2),
                          vals.iter().flatten().flatten().count(), num(c.0), num(c.1), num(c.2), num(dv)), &p.body()))
}

/// Arrows of the plane field (P, Q) over the window (each component computed by the model).
fn pq_arrows(s: &Solved, p: &mut DPlot, pp: &str, qq: &str, nx: usize, ny: usize) -> usize {
    let ((x0, x1), (y0, y1)) = p.ranges();
    let xs = span(x0 + (x1 - x0) / (2.0 * nx as f64), x1 - (x1 - x0) / (2.0 * nx as f64), nx);
    let ys = span(y0 + (y1 - y0) / (2.0 * ny as f64), y1 - (y1 - y0) / (2.0 * ny as f64), ny);
    let mut field = vec![];
    for x in &xs {
        for y in &ys {
            if let (Some(u), Some(v)) = (f2(s, pp, *x, *y), f2(s, qq, *x, *y)) {
                field.push(((*x, *y), (u, v)));
            }
        }
    }
    arrows(p, &field, xs[1] - xs[0], 3);
    field.len()
}

pub fn greens(s: &Solved) -> Option<Figure> {
    let (pp, qq) = (s.txt("P")?, s.txt("Q")?);
    let (a, b, c, d) = (nv(s, "a")?, nv(s, "b")?, nv(s, "c")?, nv(s, "d")?);
    let (circ, dbl) = (s.out("circulation")?, s.out("double_integral")?);
    if !(b > a && d > c) {
        return None;
    }
    let r = ((b - a).max(d - c)) * 0.75;
    let mut p = DPlot::equal(&[(a + b) / 2.0 - r, (a + b) / 2.0 + r], &[(c + d) / 2.0 - r, (c + d) / 2.0 + r], "x", "y",
                            &format!("circulation {} = double integral {}", num(circ), num(dbl)));
    let n = pq_arrows(s, &mut p, &pp, &qq, 9, 7);
    if n < 20 {
        return None;
    }
    let corners = [(a, c), (b, c), (b, d), (a, d)];
    p.fill(&corners, 1);
    p.outline(&corners, 1);
    for i in 0..4 {
        let (u, v) = (corners[i], corners[(i + 1) % 4]);
        p.arrow((u.0 * 0.6 + v.0 * 0.4, u.1 * 0.6 + v.1 * 0.4), (u.0 * 0.4 + v.0 * 0.6, u.1 * 0.4 + v.1 * 0.6), 1);
    }
    p.key(&format!("F = ({}, {})", clipx(&pp, 10), clipx(&qq, 10)), 3);
    p.key("boundary, anticlockwise", 1);
    Some(s.chart("field", "Green's theorem on a rectangle",
                 &format!("The field F = ({pp}, {qq}) (arrows of one scale, components computed by the model at {n} points) \
                           and the rectangle [{}, {}] × [{}, {}] traversed counter-clockwise: Green's theorem equates the \
                           circulation around it, {}, with the double integral of ∂Q/∂x − ∂P/∂y over it, {}, both from the \
                           model.", num(a), num(b), num(c), num(d), num(circ), num(dbl)), &p.body()))
}

/// The path (x(t), y(t)) for t over [t0, t1], each coordinate computed by the model (t written as x).
fn path(s: &Solved, t0: f64, t1: f64, n: usize) -> Option<Vec<(f64, f64)>> {
    let (xt, yt) = (subst(&s.txt("xt")?, "t", "x"), subst(&s.txt("yt")?, "t", "x"));
    let ts = span(t0, t1, n);
    let (xs, ys) = (fvals(s, &xt, &ts), fvals(s, &yt, &ts));
    if xs.len() != ts.len() || ys.len() != ts.len() {
        return None;
    }
    Some(xs.iter().zip(&ys).map(|(a, b)| (a.1, b.1)).collect())
}

fn bbox(pts: &[(f64, f64)]) -> ((f64, f64), f64) {
    let (xl, xh) = pts.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), q| (a.min(q.0), b.max(q.0)));
    let (yl, yh) = pts.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), q| (a.min(q.1), b.max(q.1)));
    (((xl + xh) / 2.0, (yl + yh) / 2.0), ((xh - xl).max(yh - yl) * 0.65).max(0.5))
}

pub fn line_scalar(s: &Solved) -> Option<Figure> {
    let (f, t0, t1, v) = (s.txt("f")?, nv(s, "t0")?, nv(s, "t1")?, s.out("line_integral")?);
    let pts = path(s, t0, t1, 26)?;
    let (c, r) = bbox(&pts);
    let (mut p, _, _) = shaded_f(s, &f, c, r, &format!("∫ f ds = {}", num(v)), true)?;
    p.curve(&pts, 1);
    let e = *pts.last()?;
    p.arrow(pts[pts.len() - 2], e, 1);
    p.mark(e.0, e.1, &format!("∫ f ds = {}", num(v)));
    p.key(&format!("f = {}", clipx(&f, 16)), 0);
    p.key("path", 1);
    Some(s.chart("field", "A scalar line integral along a path",
                 &format!("f(x, y) = {f} shaded by value (computed by the model) and the path x = {}, y = {}, t from {} to \
                           {} (points computed by the model with t written as x): the line integral of f with respect to \
                           arc length, {}, sums f along the path (marked at its end).", s.txt("xt")?, s.txt("yt")?,
                          num(t0), num(t1), num(v)), &p.body()))
}

pub fn line_vector(s: &Solved) -> Option<Figure> {
    let (pp, qq, t0, t1, v) = (s.txt("P")?, s.txt("Q")?, nv(s, "t0")?, nv(s, "t1")?, s.out("line_integral")?);
    let pts = path(s, t0, t1, 30)?;
    let (c, r) = bbox(&pts);
    let mut p = DPlot::equal(&[c.0 - r, c.0 + r], &[c.1 - r, c.1 + r], "x", "y", &format!("work = {}", num(v)));
    let n = pq_arrows(s, &mut p, &pp, &qq, 9, 7);
    if n < 20 {
        return None;
    }
    p.curve(&pts, 0);
    let m = pts.len() / 3;
    p.arrow(pts[m - 1], pts[m], 0);
    p.mark(pts[0].0, pts[0].1, "start");
    p.key(&format!("F = ({}, {})", clipx(&pp, 10), clipx(&qq, 10)), 3);
    p.key("path", 0);
    Some(s.chart("field", "Work done by a field along a path",
                 &format!("The field F = ({pp}, {qq}) (arrows, computed by the model) and the path x = {}, y = {} for t from \
                           {} to {} (points computed by the model with t written as x): where the path runs along the \
                           arrows the work is positive; the line integral is {} (marked at the start).", s.txt("xt")?,
                          s.txt("yt")?, num(t0), num(t1), num(v)), &p.body()))
}
