# Calculus (calculus-ai999x)

[![tests](https://github.com/AI999X/calculus-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/calculus-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/calculus-ai999x)](https://pypi.org/project/calculus-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/calculus-ai999x)

Calculus covers limits, derivatives, integrals, series, Taylor polynomials, multivariable and vector calculus, special functions and numerical differentiation and integration: 82 calculations in 16 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Calculus desktop app: Limit at a point, with its worked example and live scene](https://www.ai999x.com/?ax_shot=calculus-ai999x/hero.webp)](https://www.ai999x.com/models/calculus-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-calculus-1.6.5-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-calculus-1.6.5-linux-x86_64.tar.gz) |
| **Command line** `calculus` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/calculus-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install calculus-ai999x`: a pure-Python build of the same calculations (the compiled engine is in the app, the command line and the browser) |

## See it

From the Calculus desktop app: every scene and chart is drawn from the computed answer.

[![Calculus desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=calculus-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=calculus-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=calculus-ai999x/clip.mp4)*

| | |
|---|---|
| [![Limit at a point: the live scene in the Calculus desktop app](https://www.ai999x.com/?ax_shot=calculus-ai999x/scene-1.webp)](https://www.ai999x.com/models/calculus-ai999x/#cap-limit_at_point)<br>Limit at a point | [![Absolute extrema on a closed interval: the live scene in the Calculus desktop app](https://www.ai999x.com/?ax_shot=calculus-ai999x/scene-2.webp)](https://www.ai999x.com/models/calculus-ai999x/#cap-extrema_on_interval)<br>Absolute extrema on a closed interval |
| [![Taylor polynomial: the live scene in the Calculus desktop app](https://www.ai999x.com/?ax_shot=calculus-ai999x/scene-3.webp)](https://www.ai999x.com/models/calculus-ai999x/#cap-taylor_polynomial)<br>Taylor polynomial | [![Line integral of a vector field: the live scene in the Calculus desktop app](https://www.ai999x.com/?ax_shot=calculus-ai999x/scene-4.webp)](https://www.ai999x.com/models/calculus-ai999x/#cap-line_integral_vector)<br>Line integral of a vector field |
| [![Definite integral: the live scene in the Calculus desktop app](https://www.ai999x.com/?ax_shot=calculus-ai999x/scene-5.webp)](https://www.ai999x.com/models/calculus-ai999x/#cap-definite_integral)<br>Definite integral | [![Fourier coefficients: the live scene in the Calculus desktop app](https://www.ai999x.com/?ax_shot=calculus-ai999x/scene-6.webp)](https://www.ai999x.com/models/calculus-ai999x/#cap-fourier_coefficients)<br>Fourier coefficients |
| [![Triple integral in cylindrical or spherical coordinates: the live scene in the Calculus desktop app](https://www.ai999x.com/?ax_shot=calculus-ai999x/scene-7.webp)](https://www.ai999x.com/models/calculus-ai999x/#cap-curvilinear_triple_integral)<br>Triple integral in cylindrical or spherical coordinates | [![Directional derivative: the live scene in the Calculus desktop app](https://www.ai999x.com/?ax_shot=calculus-ai999x/scene-8.webp)](https://www.ai999x.com/models/calculus-ai999x/#cap-directional_derivative)<br>Directional derivative |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Limit at a point: a chart in the Calculus desktop app](https://www.ai999x.com/?ax_shot=calculus-ai999x/chart-1.webp)](https://www.ai999x.com/models/calculus-ai999x/#cap-limit_at_point)<br>Limit at a point | [![Absolute extrema on a closed interval: a chart in the Calculus desktop app](https://www.ai999x.com/?ax_shot=calculus-ai999x/chart-2.webp)](https://www.ai999x.com/models/calculus-ai999x/#cap-extrema_on_interval)<br>Absolute extrema on a closed interval | [![Taylor polynomial: a chart in the Calculus desktop app](https://www.ai999x.com/?ax_shot=calculus-ai999x/chart-3.webp)](https://www.ai999x.com/models/calculus-ai999x/#cap-taylor_polynomial)<br>Taylor polynomial | [![Line integral of a vector field: a chart in the Calculus desktop app](https://www.ai999x.com/?ax_shot=calculus-ai999x/chart-4.webp)](https://www.ai999x.com/models/calculus-ai999x/#cap-line_integral_vector)<br>Line integral of a vector field |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import calculus_ai999x as calculus

a = calculus.ask(
    "Find the Fourier coefficients of order 2 of f(x) = x on [-pi, "
    "pi]."
)
print(a.outcome, a.values)
# SOLVED {'a0': 0.0, 'a_k': 0.0, 'b_k': -1.0}

a = calculus.ask("Find the antiderivative of 3x^2 + 2x.")
print(a.outcome, a.values)
# SOLVED {'antiderivative': 'x^3 + x^2 + C'}

a = calculus.ask(
    "Find the limit of (x^2 - 4)/(x - 2) as x approaches 2."
)
print(a.outcome, a.values)
# SOLVED {'limit': 4.0}

a = calculus.ask(
    "Find the double integral of x^2 + y^2 over 0 <= r <= 1, 0 <= "
    "theta <= 2pi."
)
print(a.outcome, a.values)
# SOLVED {'integral': 1.57079632679, 'region_area': 3.14159265359}

a = calculus.ask(
    "Find the directional derivative of x^2*y at (1, 2) in the "
    "direction (3, 4)."
)
print(a.outcome, a.values)
# SOLVED {'directional_derivative': 3.2}

a = calculus.ask(
    "Estimate the derivative of x^2 at x = 1 using a central "
    "difference with h = 0.1."
)
print(a.outcome, a.values)
# SOLVED {'approximation': 2.0, 'exact': 2.0, 'error': 0.0}

a = calculus.ask(
    "Find the arc length of the parametric curve x = cos(t), y = "
    "sin(t) from t = 0 to t = pi."
)
print(a.outcome, a.values)
# SOLVED {'arc_length': 3.141592653589793}

a = calculus.ask(
    "Find the area enclosed by the polar curve r = 1 + cos(theta) from "
    "theta = 0 to theta = 2pi."
)
print(a.outcome, a.values)
# SOLVED {'area': 4.71238898038469}

a = calculus.ask(
    "Find the sum of the first 4 terms of the geometric series with "
    "first term 2 and ratio 3."
)
print(a.outcome, a.values)
# SOLVED {'sum': 80.0}

a = calculus.ask("Evaluate the Bessel function J_0(1).")
print(a.outcome, a.values)
# SOLVED {'J': 0.7651976865579666}

a = calculus.ask("Is F = (2x*y, x^2) conservative?")
print(a.outcome, a.values)
# SOLVED {'conservative': 'yes', 'potential': 'x^2*y + C'}

a = calculus.ask(
    "Find the arc length of r(t) = (cos(t), sin(t), t) from t = 0 to t "
    "= 2pi."
)
print(a.outcome, a.values)
# SOLVED {'arc_length': 8.885765876316732}

a = calculus.ask(
    "Find the average rate of change of f(x) = x^2 from x = 1 to x = "
    "3."
)
print(a.outcome, a.values)
# SOLVED {'average_rate': 4.0, 'fa': 1.0, 'fb': 9.0}

a = calculus.ask("Find the arc length of y = x^2 from x = 0 to x = 1.")
print(a.outcome, a.values)
# SOLVED {'arc_length': 1.4789428575445975}

a = calculus.ask(
    "Find the limit of (3x^2 + 1)/(x^2 - 5) as x approaches infinity."
)
print(a.outcome, a.values)
# SOLVED {'limit': 3.0}
```

## Command line: 15 examples

Each command below was run with `calculus` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ calculus solve extrema_on_interval a=0 b=2 "f=x^3 - 3x"
absolute_max = 2
absolute_min = -2
critical_points = [1.0]

$ calculus solve fourier_coefficients L=3.141592654 f=x k=2
a0 = 0
a_k = 0
b_k = -1

$ calculus solve antiderivative "f=3x^2 + 2x"
antiderivative = x^3 + x^2 + C

$ calculus solve limit_at_point a=2 "f=(x^2 - 4)/(x - 2)"
limit = 4

$ calculus solve polar_double_integral al=0 be=6.283185307 "f=x^2 + y^2" r1=0 r2=1
integral = 1.570796327
region_area = 3.141592654

$ calculus solve directional_derivative "f=x^2*y" "p=[1, 2]" "u=[3, 4]"
directional_derivative = 3.2

$ calculus solve numerical_derivative a=1 "f=x^2" h=0.1 method=central
approximation = 2
exact = 2
error = 0

$ calculus solve gauss_legendre a=0 b=2 "f=x^3 + x^2" n=2
approximation = 6.666666667
exact = 6.666666667
error = 0

$ calculus solve newton_method "f=x^2 - 2" n=3 x0=1
x_n = 1.414215686
f_x_n = 6.007304883e-06
iterates = [1.5,1.4166666666666667,1.4142156862745099]

$ calculus solve parametric_arc_length t0=0 t1=3.141592654 "xt=cos(t)" "yt=sin(t)"
arc_length = 3.141592654

$ calculus solve polar_area al=0 be=6.283185307 "r=1 + cos(theta)"
area = 4.71238898

$ calculus solve geometric_partial_sum a=2 n=4 r=3
sum = 80

$ calculus solve bessel_j n=0 x=1
J = 0.7651976866

$ calculus solve conservative_field "P=2x*y" "Q=x^2"
conservative = yes
potential = y*x^2 + C

$ calculus solve space_curve_arc_length t0=0 t1=6.283185307 "xt=cos(t)" "yt=sin(t)" zt=t
arc_length = 8.885765876
```

## Rust library: 15 examples

The crate `calculus-ai999x` (in `rust/` of the GitHub repository) answers a JSON request with `calculus_ai999x::solve_json`. Each call below was compiled and run on this release; the comment under it is what it printed. The whole program is `rust/calculus/examples/readme.rs`: `cd rust && cargo run --release --example readme -p calculus-ai999x`.

```rust
use serde_json::{json, Value};

fn show(request: Value) {
    let answer: Value = serde_json::from_str(&calculus_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

show(json!({"key": "extrema_on_interval", "inputs": {"a": "0", "b": "2", "f": "x^3 - 3x"}}));
// SOLVED
// absolute_max = 2.0
// absolute_min = -2.0
// critical_points = [1.0]

show(json!({"key": "fourier_coefficients", "inputs": {"L": "3.141592654", "f": "x", "k": "2"}}));
// SOLVED
// a0 = 0.0
// a_k = 0.0
// b_k = -1.0000000001305729

show(json!({"key": "antiderivative", "inputs": {"f": "3x^2 + 2x"}}));
// SOLVED
// antiderivative = "x^3 + x^2 + C"

show(json!({"key": "limit_at_point", "inputs": {"a": "2", "f": "(x^2 - 4)/(x - 2)"}}));
// SOLVED
// limit = 4.0

show(json!({"key": "polar_double_integral", "inputs": {"al": "0", "be": "6.283185307", "f": "x^2 + y^2", "r1": "0", "r2": "1"}}));
// SOLVED
// integral = 1.57079632675
// region_area = 3.1415926535

show(json!({"key": "directional_derivative", "inputs": {"f": "x^2*y", "p": [1, 2], "u": [3, 4]}}));
// SOLVED
// directional_derivative = 3.2

show(json!({"key": "numerical_derivative", "inputs": {"a": "1", "f": "x^2", "h": "0.1", "method": "central"}}));
// SOLVED
// approximation = 2.0
// exact = 2.0
// error = 0.0

show(json!({"key": "gauss_legendre", "inputs": {"a": "0", "b": "2", "f": "x^3 + x^2", "n": "2"}}));
// SOLVED
// approximation = 6.666666666666667
// exact = 6.666666666666667
// error = 0.0

show(json!({"key": "newton_method", "inputs": {"f": "x^2 - 2", "n": "3", "x0": "1"}}));
// SOLVED
// x_n = 1.4142156862745099
// f_x_n = 6.007304882737409e-6
// iterates = [1.5,1.4166666666666667,1.4142156862745099]

show(json!({"key": "parametric_arc_length", "inputs": {"t0": "0", "t1": "3.141592654", "xt": "cos(t)", "yt": "sin(t)"}}));
// SOLVED
// arc_length = 3.141592654

show(json!({"key": "polar_area", "inputs": {"al": "0", "be": "6.283185307", "r": "1 + cos(theta)"}}));
// SOLVED
// area = 4.712388980025517

show(json!({"key": "geometric_partial_sum", "inputs": {"a": "2", "n": "4", "r": "3"}}));
// SOLVED
// sum = 80.0

show(json!({"key": "bessel_j", "inputs": {"n": "0", "x": "1"}}));
// SOLVED
// J = 0.7651976865579666

show(json!({"key": "conservative_field", "inputs": {"P": "2x*y", "Q": "x^2"}}));
// SOLVED
// conservative = "yes"
// potential = "y*x^2 + C"

show(json!({"key": "space_curve_arc_length", "inputs": {"t0": "0", "t1": "6.283185307", "xt": "cos(t)", "yt": "sin(t)", "zt": "t"}}));
// SOLVED
// arc_length = 8.88576587606276
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Calculus on ai999x.com](https://www.ai999x.com/models/calculus-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/calculus-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/calculus-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `calculus` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/calculus`) and its data (`rust/models/calculus/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p calculus-ai999x
./target/release/calculus --help
cargo test -p calculus-ai999x
```
