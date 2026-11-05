//! Solids of revolution, series as stems, convergence of numerical methods, Taylor polynomials, Newton's method,
//! a map of the plane, motion along a line, a polar region and tangent-plane slices.

use super::*;
use ax_engine::figkit::{self as k, num, stack};

fn ellipse(p: &mut DPlot, cx: f64, cy: f64, rx_px: f64, ry_px: f64, class: &str) {
    let (x, y) = p.px(cx, cy);
    if x.is_finite() && y.is_finite() && rx_px > 0.3 && ry_px > 0.3 {
        p.raw(&format!("<ellipse class=\"{class}\" cx=\"{x:.1}\" cy=\"{y:.1}\" rx=\"{rx_px:.1}\" ry=\"{ry_px:.1}\"/>"));
    }
}

/// The height in pixels of a length r on the vertical axis.
fn ry(p: &DPlot, r: f64) -> f64 {
    (p.px(0.0, 0.0).1 - p.px(0.0, r).1).abs()
}

/// A solid made by turning |f| about the x-axis: outline, shading and a few cross-sections.
fn about_x(s: &Solved, f: &str, a: f64, b: f64, header: &str) -> Option<(DPlot, Vec<(f64, f64)>)> {
    let pts = fcurve(s, f, a, b, 41);
    if pts.len() < 10 {
        return None;
    }
    let top: Vec<(f64, f64)> = pts.iter().map(|q| (q.0, q.1.abs())).collect();
    let bot: Vec<(f64, f64)> = pts.iter().map(|q| (q.0, -q.1.abs())).collect();
    let m = top.iter().map(|q| q.1).fold(0.0, f64::max);
    if !(m > 0.0) {
        return None;
    }
    let (xl, xh) = widen(a, b, 0.15);
    let mut p = DPlot::new((xl, xh), (-m, m), "x (axis of revolution)", "y", header);
    let mut poly = top.clone();
    poly.extend(bot.iter().rev().copied());
    p.fill(&poly, 0);
    p.hline(0.0);
    Some((p, top))
}

pub fn disk_solid(s: &Solved) -> Option<Figure> {
    let (f, a, b, v) = (s.txt("f")?, nv(s, "a")?, nv(s, "b")?, s.out("volume")?);
    if !(b > a) {
        return None;
    }
    let (mut p, top) = about_x(s, &f, a, b, &format!("volume = {}", num(v)))?;
    let bot: Vec<(f64, f64)> = top.iter().map(|q| (q.0, -q.1)).collect();
    for i in [10usize, 20, 30, 40] {
        let (x, r) = top[i.min(top.len() - 1)];
        let h = ry(&p, r);
        ellipse(&mut p, x, 0.0, (h * 0.22).max(2.0), h, "axp-c3");
    }
    p.curve(&top, 0);
    p.curve(&bot, 0);
    p.key(&format!("solid swept by y = {}", clipx(&f, 14)), 0);
    p.key("disks", 2);
    Some(s.chart("solid", "A solid of revolution and its disks",
                 &format!("The solid made by turning y = {f} ({} ≤ x ≤ {}) about the x-axis, with circular cross-sections \
                           (disks) drawn at four points (profile and radii computed by the model); the volume by disks, the \
                           integral of π f(x)² from {} to {}, is {}.", num(a), num(b), num(a), num(b), num(v)), &p.body()))
}

pub fn washer_solid(s: &Solved) -> Option<Figure> {
    let (f, g, a, b, v) = (s.txt("f")?, s.txt("g")?, nv(s, "a")?, nv(s, "b")?, s.out("volume")?);
    if !(b > a) {
        return None;
    }
    let (mut p, top) = about_x(s, &f, a, b, &format!("volume = {}", num(v)))?;
    let inner = fcurve(s, &g, a, b, 41);
    if inner.len() != top.len() {
        return None;
    }
    let itop: Vec<(f64, f64)> = inner.iter().map(|q| (q.0, q.1.abs())).collect();
    let mut hole = itop.clone();
    hole.extend(itop.iter().rev().map(|q| (q.0, -q.1)));
    p.fill(&hole, 3);
    let mid = 20usize;
    let (x, ro) = top[mid];
    let ri = itop[mid].1;
    let (ho, hi) = (ry(&p, ro), ry(&p, ri));
    ellipse(&mut p, x, 0.0, (ho * 0.22).max(2.0), ho, "axp-c3");
    ellipse(&mut p, x, 0.0, (hi * 0.22).max(2.0), hi, "axp-c3");
    p.curve(&top, 0);
    p.curve(&top.iter().map(|q| (q.0, -q.1)).collect::<Vec<_>>(), 0);
    p.curve(&itop, 1);
    p.curve(&itop.iter().map(|q| (q.0, -q.1)).collect::<Vec<_>>(), 1);
    p.key(&format!("outer y = {}", clipx(&f, 12)), 0);
    p.key(&format!("inner y = {}", clipx(&g, 12)), 1);
    p.key(&format!("washer at x = {}", num(x)), 2);
    Some(s.chart("solid", "Washer method: a solid with a hole",
                 &format!("The solid made by turning the region between y = {f} (outer) and y = {g} (inner) about the \
                           x-axis for {} ≤ x ≤ {} (profiles computed by the model), with the washer at x = {} sketched \
                           (radii {} and {}): volume {} from the model.", num(a), num(b), num(x), num(ro), num(ri), num(v)),
                 &p.body()))
}

pub fn surface_solid(s: &Solved) -> Option<Figure> {
    let (f, a, b, v) = (s.txt("f")?, nv(s, "a")?, nv(s, "b")?, s.out("surface_area")?);
    if !(b > a) {
        return None;
    }
    let (mut p, top) = about_x(s, &f, a, b, &format!("surface area = {}", num(v)))?;
    for i in [8usize, 18, 28, 38] {
        let (x, r) = top[i.min(top.len() - 1)];
        let h = ry(&p, r);
        ellipse(&mut p, x, 0.0, (h * 0.22).max(2.0), h, "axp-c3");
    }
    p.curve(&top, 0);
    p.curve(&top.iter().map(|q| (q.0, -q.1)).collect::<Vec<_>>(), 0);
    p.key(&format!("y = {}", clipx(&f, 20)), 0);
    p.key("traced circles", 2);
    Some(s.chart("solid", "Surface of revolution",
                 &format!("The surface traced by y = {f} for {} ≤ x ≤ {} turned about the x-axis (profile computed by the \
                           model at 41 points, four of the circles its points trace sketched): surface area {} from the \
                           model.", num(a), num(b), num(v)), &p.body()))
}

pub fn shell_solid(s: &Solved) -> Option<Figure> {
    let (f, a, b, v) = (s.txt("f")?, nv(s, "a")?, nv(s, "b")?, s.out("volume")?);
    if !(b > a) || a < 0.0 {
        return None;
    }
    let pts = fcurve(s, &f, a, b, 41);
    if pts.len() != 41 {
        return None;
    }
    let mirror: Vec<(f64, f64)> = pts.iter().map(|q| (-q.0, q.1)).collect();
    let ys: Vec<f64> = pts.iter().map(|q| q.1).chain([0.0]).collect();
    let (y0, y1) = yrange(&pts, &[0.0])?;
    let _ = ys;
    let mut p = DPlot::new((-b * 1.1, b * 1.1), (y0, y1 + (y1 - y0) * 0.1), "x (turned about the y-axis)", "y",
                          &format!("volume = {}", num(v)));
    for side in [&pts, &mirror] {
        let mut poly = vec![(side[0].0, 0.0)];
        poly.extend(side.iter().copied());
        poly.push((side[side.len() - 1].0, 0.0));
        p.fill(&poly, 0);
    }
    for i in [20usize, 34] {
        let (x, h) = pts[i];
        let rx = (p.px(x, 0.0).0 - p.px(0.0, 0.0).0).abs();
        for yy in [0.0, h] {
            ellipse(&mut p, 0.0, yy, rx, (rx * 0.16).max(2.0), "axp-c3");
        }
        p.seg((x, 0.0), (x, h), 2, false);
        p.seg((-x, 0.0), (-x, h), 2, false);
    }
    p.vline(0.0);
    p.curve(&pts, 0);
    p.curve(&mirror, 0);
    p.key(&format!("region under y = {}", clipx(&f, 12)), 0);
    p.key("two shells", 2);
    Some(s.chart("solid", "Shell method about the y-axis",
                 &format!("Cylindrical shells: the region under y = {f} for {} ≤ x ≤ {} turned about the y-axis (curve \
                           values computed by the model), two shells of radius x and height f(x) sketched: volume {} from \
                           the model, the integral of 2πx f(x) dx.", num(a), num(b), num(v)), &p.body()))
}

/// Stems: vertical segments from 0 to each value (colour k, shifted by dx).
fn stems(p: &mut DPlot, pts: &[(f64, f64)], k_: usize, dx: f64) {
    for (x, y) in pts {
        p.seg((x + dx, 0.0), (x + dx, *y), k_, false);
        p.dot(x + dx, *y, k_, 3.5);
    }
}

pub fn fourier(s: &Solved) -> Option<Figure> {
    let (f, l, k0) = (s.txt("f")?, nv(s, "L")?, nv(s, "k")?);
    let bk0 = s.out("b_k")?;
    let ak0 = s.out("a_k")?;
    let a0 = s.out("a0")?;
    let kmax = (k0 as i64).clamp(10, 16);
    let mut av = vec![];
    let mut bv = vec![];
    for kk in 1..=kmax {
        if let Some(r) = s.with_input(&[("k", Input::Int(kk))]) {
            if let (Some(x), Some(y)) = (num_of(&r, "a_k"), num_of(&r, "b_k")) {
                av.push((kk as f64, x));
                bv.push((kk as f64, y));
            }
        }
    }
    if av.len() < 5 {
        return None;
    }
    let all: Vec<(f64, f64)> = av.iter().chain(bv.iter()).copied().collect();
    let (y0, y1) = yrange(&all, &[0.0])?;
    let mut p = DPlot::new((0.0, kmax as f64 + 1.0), (y0, y1), "k", "coefficient", &format!("a0 = {}", num(a0)));
    p.hline(0.0);
    stems(&mut p, &av, 1, -0.15);
    stems(&mut p, &bv, 0, 0.15);
    p.mark(k0 + 0.15, bk0, &format!("b{} = {}", num(k0), num(bk0)));
    p.key("a_k (cosine)", 1);
    p.key("b_k (sine)", 0);
    Some(s.chart("stems", &format!("Fourier coefficients of f(x) = {}", clipx(&f, 30)),
                 &format!("Fourier coefficients of f(x) = {f} on [−L, L] with L = {}, for k = 1 … {kmax}, each computed by \
                           the model: a0 = {}; at the requested k = {}, a_k = {} and b_k = {} (marked).", num(l), num(a0),
                          num(k0), num(ak0), num(bk0)), &p.body()))
}

pub fn partial_sums(s: &Solved) -> Option<Figure> {
    let (a, r, n0, sum) = (nv(s, "a")?, nv(s, "r")?, nv(s, "n")?, s.out("sum")?);
    let nmax = (n0 as i64 + 4).clamp(12, 30);
    let pts: Vec<(f64, f64)> = (1..=nmax).filter_map(|n| {
        s.with_input(&[("n", Input::Int(n))]).and_then(|x| num_of(&x, "sum")).map(|v| (n as f64, v))
    }).collect();
    if pts.len() < 5 {
        return None;
    }
    let lim = if r.abs() < 1.0 {
        s.solve_other("geometric_series_sum", &[("a", Input::Float(a)), ("r", Input::Float(r))])
            .and_then(|x| num_of(&x, "sum"))
    } else {
        None
    };
    // sums growing over orders of magnitude (|r| > 1) on a logarithmic axis, stems rising from its foot
    let (lo, hi) = pts.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), q| (a.min(q.1), b.max(q.1)));
    let logy = lo > 0.0 && hi / lo > 1e3;
    let (y0, y1) = if logy { (lo, hi) } else { yrange(&pts, &lim.into_iter().chain([0.0]).collect::<Vec<_>>())? };
    let mut p = DPlot::with_log((0.0, nmax as f64 + 1.0), (y0, y1), (false, logy), "number of terms n",
                               if logy { "partial sum S_n (logarithmic)" } else { "partial sum S_n" },
                               &format!("S_{} = {}", num(n0), num(sum)));
    if !logy {
        p.hline(0.0);
    }
    if let Some(l) = lim {
        p.hline(l);
        p.key(&format!("sum of the series {}", num(l)), 3);
    }
    if logy {
        let foot = p.ranges().1 .0;
        for (x, y) in &pts {
            p.seg((*x, foot), (*x, *y), 0, false);
            p.dot(*x, *y, 0, 3.5);
        }
    } else {
        stems(&mut p, &pts, 0, 0.0);
    }
    p.mark(n0, sum, &format!("S_{} = {}", num(n0), num(sum)));
    p.key("partial sums", 0);
    let tail = match lim {
        Some(l) => format!(" |r| < 1, so they close in on the sum of the series, {} (dashed).", num(l)),
        None => " |r| ≥ 1, so the series has no finite sum.".to_string(),
    };
    Some(s.chart("stems", "Partial sums of a geometric series",
                 &format!("Partial sums S_n of the geometric series with first term {} and ratio {}, for n = 1 … {nmax}, \
                           each computed by the model: S_{} = {} (marked);{tail}", num(a), num(r), num(n0), num(sum)),
                 &p.body()))
}

pub fn ratio(s: &Solved) -> Option<Figure> {
    let a = s.txt("a")?;
    let verdict = s.out_text("verdict").unwrap_or_default();
    let lim = s.out("ratio_limit");
    let ax = subst(&a, "n", "x");
    let ns: Vec<f64> = (1..=41).map(|n| n as f64).collect();
    let terms = fvals(s, &ax, &ns);
    let ratios: Vec<(f64, f64)> = terms.windows(2).filter(|w| w[1].0 == w[0].0 + 1.0 && w[0].1 != 0.0)
        .map(|w| (w[0].0, (w[1].1 / w[0].1).abs())).filter(|q| q.1.is_finite()).collect();
    if ratios.len() < 10 {
        return None;
    }
    let (y0, y1) = yrange(&ratios, &lim.into_iter().chain([0.0, 1.0]).collect::<Vec<_>>())?;
    let mut p = DPlot::new((0.0, 41.0), (y0, y1), "n", "|a(n+1) / a(n)|",
                          &format!("L = {}: {verdict}", lim.map(num).unwrap_or_else(|| "∞".into())));
    p.hline(1.0);
    if let Some(l) = lim {
        p.seg((0.0, l), (41.0, l), 1, true);
        p.key(&format!("L = {}", num(l)), 1);
    }
    stems(&mut p, &ratios, 0, 0.0);
    p.key("ratios", 0);
    p.key("1", 3);
    Some(s.chart("stems", "Ratio test: consecutive ratios against 1",
                 &format!("The ratio test for a(n) = {a}: |a(n+1)/a(n)| for n = 1 … {}, the ratios formed from term values \
                           computed by the model (n written as x). They approach L = {}, so the series {verdict}.",
                          ratios.len(), lim.map(num).unwrap_or_else(|| "∞".into())), &p.body()))
}

pub fn tails(s: &Solved) -> Option<Figure> {
    let (a, k0, sum) = (s.txt("a")?, nv(s, "k")?, s.out("sum")?);
    let k0 = k0 as i64;
    let ks: Vec<i64> = (k0..k0 + 12).collect();
    let tails: Vec<(f64, f64)> = ks.iter().filter_map(|kk| {
        s.with_input(&[("k", Input::Int(*kk))]).and_then(|x| num_of(&x, "sum")).map(|v| (*kk as f64, v))
    }).collect();
    let terms = fvals(s, &subst(&a, "n", "x"), &ks.iter().map(|x| *x as f64).collect::<Vec<_>>());
    if tails.len() < 5 {
        return None;
    }
    let all: Vec<(f64, f64)> = tails.iter().chain(terms.iter()).copied().collect();
    let (y0, y1) = yrange(&all, &[0.0])?;
    let mut p = DPlot::new((k0 as f64 - 1.0, (k0 + 12) as f64), (y0, y1), "starting index k", "value",
                          &format!("sum from n = {k0}: {}", num(sum)));
    p.hline(0.0);
    stems(&mut p, &tails, 0, -0.12);
    stems(&mut p, &terms, 1, 0.12);
    p.mark(k0 as f64 - 0.12, sum, &format!("{}", num(sum)));
    p.key("sum from n = k", 0);
    p.key("term a(k)", 1);
    Some(s.chart("stems", "Tails of a convergent series",
                 &format!("The infinite series of a(n) = {a} summed from n = k onward, for k = {k0} … {}, each sum computed \
                           by the model, beside the terms themselves (computed by the model, n written as x): the tails \
                           shrink toward 0. Marked: the sum from n = {k0}, {}.", k0 + 11, num(sum)), &p.body()))
}

/// |error| of a rule for a run of point counts (zeros, an exact result, left out).
fn errs(s: &Solved, key: &str, f: &str, a: f64, b: f64, ns: &[i64]) -> Vec<(f64, f64)> {
    ns.iter().filter_map(|n| {
        let r = s.solve_other(key, &[("f", tx(f)), ("a", Input::Float(a)), ("b", Input::Float(b)), ("n", Input::Int(*n))])?;
        num_of(&r, "error").map(|e| (*n as f64, e.abs()))
    }).filter(|q| q.1 > 0.0).collect()
}

pub fn gauss(s: &Solved) -> Option<Figure> {
    let (f, a, b, n0) = (s.txt("f")?, nv(s, "a")?, nv(s, "b")?, nv(s, "n")?);
    let (ap, ex) = (s.out("approximation")?, s.out("exact")?);
    let gn: Vec<i64> = (1..=6).collect();
    let sn: Vec<i64> = vec![2, 4, 6, 8, 10, 12];
    let (mut fe, mut ae, mut be) = (f.clone(), a, b);
    let mut g = errs(s, "gauss_legendre", &fe, ae, be, &gn);
    let mut sm = errs(s, "simpson_rule", &fe, ae, be, &sn);
    // errors at rounding level (a polynomial the rule integrates exactly) show nothing of the convergence
    let rel = |v: &[(f64, f64)]| v.iter().filter(|q| q.1 > 1e-12 * ex.abs().max(1.0)).count();
    let chosen = rel(&g) < 3;
    if chosen {
        (fe, ae, be) = ("exp(x)".to_string(), 0.0, 1.0);
        g = errs(s, "gauss_legendre", &fe, ae, be, &gn);
        sm = errs(s, "simpson_rule", &fe, ae, be, &sn);
    }
    if g.len() < 2 {
        return None;
    }
    let mut series = vec![("Gauss–Legendre".to_string(), g.clone())];
    if sm.len() >= 2 {
        series.push(("Simpson".to_string(), sm));
    }
    let header = if chosen {
        format!("{} on [{}, {}]; request n = {}: {}", clipx(&fe, 12), num(ae), num(be), num(n0), num(ap))
    } else {
        format!("{} on [{}, {}], n = {}: {}", clipx(&fe, 14), num(ae), num(be), num(n0), num(ap))
    };
    let body = k::lines(&series, "n", "|error|", &header, None, "", false);
    let note = if chosen {
        format!("The request's integral of {f} from {} to {} with n = {} is exact or nearly so (approximation {}, exact \
                 {}), so the errors are shown for exp(x) from 0 to 1 instead.", num(a), num(b), num(n0), num(ap), num(ex))
    } else {
        format!("Request: n = {}, approximation {}, exact {}.", num(n0), num(ap), num(ex))
    };
    Some(s.chart("convergence", "Gauss–Legendre accuracy against the number of points",
                 &format!("Error of n-point Gauss–Legendre quadrature for the integral of {fe} from {} to {}, n = 1 … 6, \
                           each run of the model, against Simpson's rule with n panels (errors of exactly 0 left out; the \
                           error axis turns logarithmic over a wide range). {note}", num(ae), num(be)), &body))
}

pub fn fd_convergence(s: &Solved) -> Option<Figure> {
    let (f, a, h0) = (s.txt("f")?, nv(s, "a")?, nv(s, "h")?);
    let (ap, err) = (s.out("approximation")?, s.out("error")?);
    let hs: Vec<f64> = (0..=7).map(|i| 10f64.powi(-i)).collect();
    let run = |fe: &str, ae: f64| -> Vec<(String, Vec<(f64, f64)>)> {
        ["forward", "backward", "central"].iter().map(|m| {
            let pts: Vec<(f64, f64)> = hs.iter().filter_map(|h| {
                let r = s.solve_other("numerical_derivative", &[("f", tx(fe)), ("a", Input::Float(ae)),
                                                                ("h", Input::Float(*h)), ("method", tx(m))])?;
                num_of(&r, "error").map(|e| (*h, e.abs()))
            }).filter(|q| q.1 > 0.0).collect();
            (m.to_string(), pts)
        }).filter(|x| x.1.len() >= 3).collect()
    };
    let mut series = run(&f, a);
    let chosen = series.len() < 2;
    let (fe, ae) = if chosen { ("sin(x)".to_string(), 1.0) } else { (f.clone(), a) };
    if chosen {
        series = run(&fe, ae);
    }
    if series.is_empty() {
        return None;
    }
    let names: Vec<String> = series.iter().map(|x| x.0.clone()).collect();
    let body = k::lines(&series, "step h (logarithmic)", "|error|", &format!("d/dx {} at x = {}", clipx(&fe, 16), num(ae)),
                        None, "", true);
    let missing: Vec<&str> = ["forward", "backward", "central"].into_iter().filter(|m| !names.iter().any(|n| n == m)).collect();
    let note = if missing.is_empty() { String::new() } else {
        format!(" The {} difference is exact here (error 0 at every step), so it has no curve.", missing.join(" and "))
    };
    let req = if chosen {
        format!(" The request ({f} at x = {}, h = {}) has errors of 0, so sin(x) at x = 1 is shown instead; its answer \
                 is {}, error {}.", num(a), num(h0), num(ap), num(err))
    } else {
        format!(" Request: h = {}, approximation {}, error {}.", num(h0), num(ap), num(err))
    };
    Some(s.chart("convergence", "Finite-difference error against the step",
                 &format!("Error of the {} difference approximations of the derivative of {fe} at x = {} for steps h from 1 \
                           down to 1e−7, each run of the model (logarithmic axes): one-sided errors fall like h, the \
                           central one like h², until rounding takes over.{note}{req}", names.join(", "), num(ae)), &body))
}

pub fn taylor(s: &Solved) -> Option<Figure> {
    let (f, a, n0, x1) = (s.txt("f")?, nv(s, "a")?, nv(s, "n")?, nv(s, "x1")?);
    let v = s.out("value")?;
    let n0 = n0 as i64;
    let r = (1.5 * (x1 - a).abs()).max(2.0);
    let xs = span(a - r, a + r, 25);
    let mut series: Vec<(String, Vec<(f64, f64)>)> = vec![];
    let mut fpts = vec![];
    for (i, n) in [n0, n0 + 1, n0 + 2].iter().enumerate() {
        let mut pts = vec![];
        for x in &xs {
            if let Some(ans) = s.with_input(&[("n", Input::Int(*n)), ("x1", Input::Float(*x))]) {
                if let Some(y) = num_of(&ans, "value") {
                    pts.push((*x, y));
                }
                if i == 0 {
                    if let Some(y) = num_of(&ans, "function_value") {
                        fpts.push((*x, y));
                    }
                }
            }
        }
        if pts.len() >= 5 {
            series.push((format!("n = {n}"), pts));
        }
    }
    if series.is_empty() || fpts.len() < 5 {
        return None;
    }
    series.push((format!("f = {}", clipx(&f, 14)), fpts));
    let body = k::lines(&series, "x1 (evaluation point)", "P_n(x1)", &format!("P_{n0}({}) = {}", num(x1), num(v)),
                        Some((x1, v)), &format!("x1 = {}", num(x1)), false);
    Some(s.chart("curve-2", "Taylor polynomial: P_n(x1) against the evaluation point",
                 &format!("The Taylor polynomial of f = {f} about a = {} evaluated at x1 from {} to {}, one curve for each \
                           degree n = {n0}, {}, {}, beside f itself, 25 points per curve computed by the model; marked: \
                           x1 = {}, value = {}.", num(a), num(a - r), num(a + r), n0 + 1, n0 + 2, num(x1), num(v)), &body))
}

pub fn newton(s: &Solved) -> Option<Figure> {
    let (f, x0, xn) = (s.txt("f")?, nv(s, "x0")?, s.out("x_n")?);
    let fxn = s.out("f_x_n")?;
    let it = s.out_list("iterates")?;
    let mut xs = vec![x0];
    xs.extend(it.iter().take(8).copied());
    let pts: Vec<(f64, f64)> = xs.iter().filter_map(|x| f_at(s, &f, *x).map(|y| (*x, y))).collect();
    if pts.len() != xs.len() {
        return None;
    }
    let (l, h) = xs.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)));
    let (lo, hi) = if h > l { widen(l, h, 0.25) } else { (l - 1.0, l + 1.0) };
    let curve = fcurve(s, &f, lo, hi, 41);
    let (y0, y1) = yrange(&curve, &pts.iter().map(|q| q.1).chain([0.0]).collect::<Vec<_>>())?;
    let mut p = DPlot::new((lo, hi), (y0, y1), "x", "y", &format!("x_n = {}", num(xn)));
    p.hline(0.0);
    p.curve(&curve, 0);
    for w in pts.windows(2) {
        p.seg(w[0], (w[1].0, 0.0), 2, false);
        p.seg((w[1].0, 0.0), w[1], 3, true);
        p.dot(w[1].0, 0.0, 1, 3.5);
    }
    p.dot(pts[0].0, pts[0].1, 2, 4.0);
    p.mark(xn, 0.0, &format!("x_n = {}", num(xn)));
    p.key(&format!("y = {}", clipx(&f, 16)), 0);
    p.key("tangent steps", 2);
    Some(s.chart("iteration", "Newton's method: tangents to the next iterate",
                 &format!("Newton's method on f(x) = {f} from x0 = {}: each tangent (through the curve point computed by the \
                           model) meets the axis at the next iterate: {}; marked: x_n = {}, f(x_n) = {}.", num(x0),
                          join(&it), num(xn), num(fxn)), &p.body()))
}

pub fn jacobian(s: &Solved) -> Option<Figure> {
    let (xu, yv, j) = (s.txt("xu")?, s.txt("yv")?, s.out("jacobian")?);
    let pv = s.vec("p")?;
    if pv.len() != 2 {
        return None;
    }
    let (u0, v0) = (pv[0], pv[1]);
    let to_xy = |e: &str| subst(&subst(e, "u", "x"), "v", "y");
    let (fx, fy) = (to_xy(&xu), to_xy(&yv));
    let du = (u0.abs() * 0.5).max(0.5);
    let dv = (v0.abs() * 0.5).max(0.5);
    let us = span(u0 - du, u0 + du, 7);
    let vs = span(v0 - dv, v0 + dv, 7);
    let img = |u: f64, v: f64| -> Option<(f64, f64)> { Some((f2(s, &fx, u, v)?, f2(s, &fy, u, v)?)) };
    let g: Vec<Vec<Option<(f64, f64)>>> = us.iter().map(|u| vs.iter().map(|v| img(*u, *v)).collect()).collect();
    if g.iter().flatten().flatten().count() < 30 {
        return None;
    }
    let h = du.min(dv) / 3.0;
    let cell: Vec<(f64, f64)> = [(u0, v0), (u0 + h, v0), (u0 + h, v0 + h), (u0, v0 + h)].iter()
        .filter_map(|(u, v)| img(*u, *v)).collect();
    let all: Vec<(f64, f64)> = g.iter().flatten().flatten().copied().collect();
    let mut p = DPlot::equal(&all.iter().map(|q| q.0).collect::<Vec<_>>(), &all.iter().map(|q| q.1).collect::<Vec<_>>(), "x",
                            "y", &format!("J at ({}, {}) = {}", num(u0), num(v0), num(j)));
    for row in &g {
        let line: Vec<(f64, f64)> = row.iter().map(|q| q.unwrap_or((f64::NAN, f64::NAN))).collect();
        p.curve(&line, 3);
    }
    for jx in 0..vs.len() {
        let line: Vec<(f64, f64)> = g.iter().map(|row| row[jx].unwrap_or((f64::NAN, f64::NAN))).collect();
        p.curve(&line, 3);
    }
    if cell.len() == 4 {
        p.fill(&cell, 1);
        p.outline(&cell, 1);
    }
    if let Some(c) = img(u0, v0) {
        p.mark(c.0, c.1, &format!("J = {}", num(j)));
    }
    p.key("grid images", 3);
    p.key("small cell", 1);
    Some(s.chart("map", "Jacobian: how a map stretches area",
                 &format!("The map x = {xu}, y = {yv} applied to a 7 × 7 grid round (u, v) = ({}, {}) (images computed by \
                           the model, u and v written as x and y in the expressions): the image of a small cell of side {} \
                           at that point has area about |J| times the cell's, with the Jacobian determinant J = {} \
                           (marked).", num(u0), num(v0), num(h), num(j)), &p.body()))
}

pub fn motion(s: &Solved) -> Option<Figure> {
    let (st, t0) = (s.txt("s")?, nv(s, "t0")?);
    let (v0, a0, x0, sp) = (s.out("velocity")?, s.out("acceleration")?, s.out("position")?, s.out("speed")?);
    let w = 4.5 * (t0.abs() / 2.0).max(1.0);
    let lo = if t0 >= 0.0 { (t0 - w * 0.45).max(0.0) } else { t0 - w * 0.45 };
    let ts = span(lo, lo + w, 31);
    let mut pos = vec![];
    let mut vel = vec![];
    let mut acc = vec![];
    for t in &ts {
        if let Some(r) = s.with(&[("t0", *t)]) {
            if let (Some(x), Some(v), Some(a)) = (num_of(&r, "position"), num_of(&r, "velocity"), num_of(&r, "acceleration")) {
                pos.push((*t, x));
                vel.push((*t, v));
                acc.push((*t, a));
            }
        }
    }
    if pos.len() < 10 {
        return None;
    }
    let (y0, y1) = yrange(&pos, &[x0])?;
    let mut p1 = DPlot::new((lo, lo + w), (y0, y1), "t", "s(t)", &format!("at t = {}: s = {}", num(t0), num(x0)));
    p1.hline(0.0);
    p1.curve(&pos, 0);
    p1.mark(t0, x0, &format!("s = {}", num(x0)));
    p1.key("position", 0);
    let va: Vec<(f64, f64)> = vel.iter().chain(acc.iter()).copied().collect();
    let (q0, q1) = yrange(&va, &[0.0])?;
    let mut p2 = DPlot::new((lo, lo + w), (q0, q1), "t", "v(t), a(t)", &format!("v = {}, a = {}, speed {}", num(v0), num(a0),
                                                                                   num(sp)));
    p2.hline(0.0);
    p2.curve(&vel, 1);
    p2.curve(&acc, 2);
    p2.mark(t0, v0, &format!("v = {}", num(v0)));
    p2.dot(t0, a0, 2, 4.5);
    p2.key("velocity", 1);
    p2.key("acceleration", 2);
    let (body, h) = stack(&[p1.body(), p2.body()]);
    Some(s.figure("motion", "Position, velocity and acceleration",
                  &format!("Position s(t) = {st}, its velocity and its acceleration for t from {} to {}, each computed by \
                            the model at {} times; velocity is zero where s turns. At t = {}: s = {}, v = {}, a = {}, speed \
                            {}.", num(lo), num(lo + w), pos.len(), num(t0), num(x0), num(v0), num(a0), num(sp)), &body, h))
}

pub fn polar(s: &Solved) -> Option<Figure> {
    let (r, al, be, area) = (s.txt("r")?, nv(s, "al")?, nv(s, "be")?, s.out("area")?);
    if !(be > al) {
        return None;
    }
    let th = span(al, be, 73);
    let rv = fvals(s, &subst(&r, "theta", "x"), &th);
    if rv.len() < 20 {
        return None;
    }
    let pts: Vec<(f64, f64)> = rv.iter().map(|(t, rr)| (rr * t.cos(), rr * t.sin())).collect();
    let rmax = rv.iter().map(|q| q.1.abs()).fold(0.0, f64::max);
    if !(rmax > 0.0) {
        return None;
    }
    let mut p = DPlot::equal(&[-rmax, rmax], &[-rmax, rmax], "x", "y", &format!("area = {}", num(area)));
    let step = {
        let raw = rmax / 3.0;
        let mag = 10f64.powf(raw.log10().floor());
        (raw / mag).round().max(1.0) * mag
    };
    let mut rr = step;
    while rr <= rmax * 1.05 {
        let ring: Vec<(f64, f64)> = (0..=72).map(|i| {
            let t = i as f64 / 72.0 * std::f64::consts::TAU;
            (rr * t.cos(), rr * t.sin())
        }).collect();
        p.dashed(&ring, 3);
        rr += step;
    }
    for i in 0..8 {
        let t = i as f64 * std::f64::consts::FRAC_PI_4;
        p.seg((0.0, 0.0), (rmax * 1.05 * t.cos(), rmax * 1.05 * t.sin()), 3, true);
    }
    let mut poly = vec![(0.0, 0.0)];
    poly.extend(pts.iter().copied());
    p.fill(&poly, 0);
    p.curve(&pts, 0);
    p.key(&format!("r = {}", clipx(&r, 20)), 0);
    Some(s.chart("polar", "Area enclosed by a polar curve",
                 &format!("The polar curve r = {r} for θ from {} to {} (r computed by the model at {} angles, θ written as x \
                           in the expression) and the region it sweeps: area {}, the integral of r²/2 dθ, from the model. \
                           Dashed rings every {}.", num(al), num(be), rv.len(), num(area), num(step)), &p.body()))
}

pub fn slices(s: &Solved) -> Option<Figure> {
    let f = s.txt("f")?;
    let pv = s.vec("p")?;
    if pv.len() != 2 {
        return None;
    }
    let (x0, y0) = (pv[0], pv[1]);
    let (z0, fx, fy) = (s.out("z0")?, s.out("fx")?, s.out("fy")?);
    let rx = x0.abs().max(1.0) * 2.0;
    let ry_ = y0.abs().max(1.0) * 1.0;
    let ry_ = ry_.max(2.0);
    let along_x: Vec<(f64, f64)> = span(x0 - rx, x0 + rx, 21).into_iter().filter_map(|x| f2(s, &f, x, y0).map(|z| (x, z))).collect();
    let along_y: Vec<(f64, f64)> = span(y0 - ry_, y0 + ry_, 21).into_iter().filter_map(|y| f2(s, &f, x0, y).map(|z| (y, z))).collect();
    if along_x.len() < 8 || along_y.len() < 8 {
        return None;
    }
    let panel = |pts: &[(f64, f64)], c: f64, slope: f64, lab: &str, name: &str| -> Option<String> {
        let (lo, hi) = (pts[0].0, pts[pts.len() - 1].0);
        let tl: Vec<(f64, f64)> = [lo, hi].iter().map(|x| (*x, z0 + slope * (x - c))).collect();
        let (q0, q1) = yrange(pts, &[z0])?;
        let mut p = DPlot::new((lo, hi), (q0, q1), lab, "z", "");
        p.curve(pts, 0);
        p.curve(&tl, 1);
        p.mark(c, z0, &format!("z0 = {}", num(z0)));
        p.key(name, 0);
        p.key(&format!("tangent plane, slope {}", num(slope)), 1);
        Some(p.body())
    };
    let b1 = panel(&along_x, x0, fx, "x", &format!("f along x (y = {})", num(y0)))?;
    let b2 = panel(&along_y, y0, fy, "y", &format!("f along y (x = {})", num(x0)))?;
    let (body, h) = stack(&[b1, b2]);
    let c = s.out("c").map(|c| format!(" (constant term {})", num(c))).unwrap_or_default();
    Some(s.figure("slices", "Tangent plane seen in two slices",
                  &format!("Two slices through the surface z = {f} at ({}, {}) (surface values computed by the model): \
                            along x and along y the tangent plane z = {} + {}(x − {}) + {}(y − {}){c} touches the surface \
                            with slopes fx = {} and fy = {}. Marked: z0 = {}.", num(x0), num(y0), num(z0), num(fx),
                           num(x0), num(fy), num(y0), num(fx), num(fy), num(z0)), &body, h))
}
