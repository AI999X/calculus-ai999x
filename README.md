# calculus-ai999x — offline, verified calculus solver with step-by-step answers

[![tests](https://github.com/AI999X/calculus-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/calculus-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/calculus-ai999x)](https://pypi.org/project/calculus-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/calculus-ai999x)

**[Install](https://pypi.org/project/calculus-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/calculus-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/calculus-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.3

Field: **Mathematics & Statistics** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

calculus-ai999x is an offline, deterministic Python solver that answers calculus questions with verified results from 71 certified capabilities, gives 374 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**calculus-ai999x** is a compact, deterministic calculus engine for Python. It reads a question in plain English (or a typed call), picks a certified calculus capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified calculus capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 374 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** limits, derivatives, integrals, series, Taylor polynomials, multivariable and vector calculus, special functions and numerical differentiation and integration.

## Model overview

| Property | Value |
|---|---|
| Package | `calculus-ai999x` 1.0.3 (import `calculus_ai999x`) |
| Type | Deterministic neuro-symbolic calculus engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 71 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 374 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install calculus-ai999x
```

```python
import calculus_ai999x as calculus

calculus.ask("Find the limit of (x^2 - 4)/(x - 2) as x approaches 2.").show()
# -> SOLVED — limit = 4  (limit_at_point)
calculus.ask("Find the absolute maximum and minimum of x^3 - 3x on [0, 2].").show()
# -> SOLVED — absolute_max = 2, absolute_min = -2, critical_points = [1.0]  (extrema_on_interval)
calculus.ask("Find the Taylor polynomial of degree 3 of exp(x) about x = 0 and evaluate it at x = 0.5.").show()
# -> SOLVED — coefficients = [1.0, 1.0, 0.5, 0.16666666666666666], value = 1.64583, function_value = 1.64872, error = 0.00288794  (taylor_polynomial)
calculus.ask("Find the line integral of F = (-y, x) along x = cos(t), y = sin(t) from t = 0 to t = 2pi.").show()
# -> SOLVED — line_integral = 6.28319  (line_integral_vector)
calculus.solve("definite_integral", f="x^2*exp(-x)", a=0, b=2).show()
# -> SOLVED — integral = 0.646647  (definite_integral)
calculus.ask("Who created you?").message
# -> calculus-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `calculus-ai999x "<question>"`, `calculus-ai999x --capabilities`. The outputs above were produced by running this exact code on calculus-ai999x 1.0.3.

## Find the right capability, and draw it

`calculus.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "calculus-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import calculus_ai999x as calculus

calculus.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Worked examples

Each question was put to `calculus-ai999x`; the answer is the one it returned, with the steps it took.

**Limit at a point** · _Limits_

> Find the limit of (x^2 - 4)/(x - 2) as x approaches 2.

→ **limit = 4**

<details><summary>Steps</summary>

- Limit at a point: a = 2, f = (x^2 - 4)/(x - 2) → limit = 4

</details>

**Absolute extrema on a closed interval** · _Derivatives_

> Find the absolute maximum and minimum of x^3 - 3x on [0, 2].

→ **absolute_max = 2, absolute_min = -2, critical_points = [1]**

<details><summary>Steps</summary>

- Absolute extrema on a closed interval: a = 0, b = 2, f = x^3 - 3x → absolute_max = 2, absolute_min = -2, critical_points = [1]

</details>

**Taylor polynomial** · _Series_

> Find the Taylor polynomial of degree 3 of exp(x) about x = 0 and evaluate it at x = 0.5.

→ **coefficients = [1, 1, 0.5, 0.1666666667], value = 1.645833333, function_value = 1.648721271, error = 0.002887937367**

<details><summary>Steps</summary>

- Taylor polynomial: a = 0, f = exp(x), n = 3, x1 = 0.5 → coefficients = [1, 1, 0.5, 0.1666666667], value = 1.645833333, function_value = 1.648721271, error = 0.002887937367

</details>

**Line integral of a vector field** · _Vector calculus_

> Find the line integral of F = (-y, x) along x = cos(t), y = sin(t) from t = 0 to t = 2pi.

→ **line_integral = 6.283185307**

<details><summary>Steps</summary>

- Line integral of a vector field: P = -y, Q = x, t0 = 0, t1 = 6.283185307, xt = cos(t), yt = sin(t) → line_integral = 6.283185307

</details>

**Definite integral** · _Integrals_

`m.solve("definite_integral", f="x^2*exp(-x)", a=0, b=2)`

→ **integral = 0.6466471676**

<details><summary>Steps</summary>

- Definite integral: f = x^2*exp(-x), a = 0, b = 2 → integral = 0.6466471676

</details>

**Fourier coefficients** · _Fourier series_

> Find the Fourier coefficients of order 2 of f(x) = x on [-pi, pi].

→ **a0 = 0, a_k = 0, b_k = -1**

<details><summary>Steps</summary>

- Fourier coefficients: L = 3.141592654, f = x, k = 2 → a0 = 0, a_k = 0, b_k = -1

</details>

## Figures

Every point was computed by `calculus-ai999x`.

**Taylor polynomials closing in on sin x**

![Taylor polynomials closing in on sin x — computed by calculus-ai999x](https://huggingface.co/AI999X/calculus-ai999x/resolve/main/figures/taylor-polynomial-curve.svg)

_Taylor polynomials of sin x about 0 of degree 1, 3, 5 and 7 against sin x itself, each evaluated by the package at 121 points; marked: x = 2.5, degree-7 value = 0.588534 (sin 2.5 = 0.598472, error 0.00993823)._

**Left Riemann sum under y = x²**

![Left Riemann sum under y = x² — computed by calculus-ai999x](https://huggingface.co/AI999X/calculus-ai999x/resolve/main/figures/left-riemann-sum-area.svg)

_The area under y = x² on [0, 2] and the left Riemann sum with 8 rectangles (heights f(xᵢ) and the curve evaluated by the package): left sum 2.1875, right sum 3.1875, exact integral 2.66667._

**Newton's method converging to √2**

![Newton's method converging to √2 — computed by calculus-ai999x](https://huggingface.co/AI999X/calculus-ai999x/resolve/main/figures/newton-method-iteration.svg)

_Newton's method on x² − 2 from x₀ = 3: each tangent (through the curve point computed by the package) meets the axis at the next iterate — 1.83333, 1.46212, 1.415, 1.41421; marked: x₄ = 1.41421, f(x₄) = 6.1568e-07._

**Partial sums of a geometric series converging**

![Partial sums of a geometric series converging — computed by calculus-ai999x](https://huggingface.co/AI999X/calculus-ai999x/resolve/main/figures/geometric-partial-sum-stems.svg)

_Partial sums of the geometric series 1 + ½ + ¼ + …, each computed by the package: S₁ = 1, S₄ = 1.875, S₁₂ = 1.99951; the gap to the sum of the series, 2, halves with every term._

**A solid of revolution and its disks**

![A solid of revolution and its disks — computed by calculus-ai999x](https://huggingface.co/AI999X/calculus-ai999x/resolve/main/figures/disk-volume-solid.svg)

_The solid made by turning y = √x (0 ≤ x ≤ 4) about the x-axis, with circular cross-sections drawn at x = 1, 2, 3, 4 (profile and radii computed by the package); the package's volume by disks, the integral of π(√x)² from 0 to 4, is 25.1327._

## Capabilities

71 certified procedures. A few of them:

- **Procedures:** Absolute extrema on a closed interval; Arc length of a graph; Arc length of a parametric curve; Area between two curves; Area enclosed by two curves; Area in polar coordinates; Average rate of change; Average value of a function; Bessel function of the first kind; Beta function; Centroid of a region under a curve; Chebyshev polynomial; Critical points and their classification; Curl; Curvature of a graph; Definite integral; Derivative at a point; Derivative of an integral (fundamental theorem); Directional derivative; Divergence; Double integral over a rectangle; Double integral over a region; Error function; Finite-difference derivative …

`calculus_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified calculus capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (limits, derivatives, integrals, series, Taylor polynomials, multivariable and vector calculus, special functions and numerical differentiation and integration); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{calculus_ai999x_1_0_3,
  title  = {calculus-ai999x: an offline, verified calculus solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.3},
  url    = {https://huggingface.co/AI999X/calculus-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/calculus-ai999x/discussions or visit https://ai999x.com.

## Support the project

calculus-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/calculus-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/calculus-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/calculus-ai999x/tree/main/onnx).
