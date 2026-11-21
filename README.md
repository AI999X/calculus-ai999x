# Calculus (calculus-ai999x)

[![tests](https://github.com/AI999X/calculus-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/calculus-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/calculus-ai999x)](https://pypi.org/project/calculus-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/calculus-ai999x)

Calculus covers limits, derivatives, integrals, series, Taylor polynomials, multivariable and vector calculus, special functions and numerical differentiation and integration: 82 calculations in 16 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Calculus desktop app: Limit at a point, with its worked example and live scene](https://www.ai999x.com/?ax_shot=calculus-ai999x/hero.webp)](https://www.ai999x.com/models/calculus-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-calculus-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-calculus-1.6.7-linux-x86_64.tar.gz) |
| **Command line** `calculus` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-linux-x86_64.tar.gz) |
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

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Calculus on ai999x.com](https://www.ai999x.com/models/calculus-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.7 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/calculus-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.7`; the [GitHub release](https://github.com/AI999X/calculus-ai999x/releases/tag/v1.6.7) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `calculus` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/calculus`) and its data (`rust/models/calculus/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p calculus-ai999x
./target/release/calculus --help
cargo test -p calculus-ai999x
```
