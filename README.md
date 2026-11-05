# calculus-ai999x — offline, verified calculus solver with step-by-step answers

[![tests](https://github.com/AI999X/calculus-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/calculus-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/calculus-ai999x)](https://pypi.org/project/calculus-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/calculus-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/calculus-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/calculus-ai999x/)** · **[Guides](https://huggingface.co/AI999X/calculus-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/calculus-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.1.0

Field: **Mathematics & Statistics** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

calculus-ai999x is an offline, deterministic Python solver that answers calculus questions with verified results from 71 certified capabilities, gives 374 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**calculus-ai999x** is a compact, deterministic calculus engine for Python. It reads a question in plain English (or a typed call), picks a certified calculus capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** calculus-ai999x comes as the AI999X desktop app and as the command-line program `calculus` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Calculus desktop app** 1.4.0 | Windows, Linux — this model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-calculus-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-calculus-1.4.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.4.0 | Windows, Linux — every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz) |
| **Command line** (`calculus`) | Windows, Linux — one program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz) |
| **In your browser** | any device — nothing to install | [Compute on the model page](https://www.ai999x.com/models/calculus-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install calculus-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Calculus** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Calculus among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-calculus-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-calculus-1.4.0-linux-x86_64.tar.gz
./ai999x-calculus-1.4.0-linux-x86_64/ai999x-calculus
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.4.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.4.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
calculus --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.4.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.4.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
calculus --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words** — Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ calculus ask "Find the limit of (x^2 - 4)/(x - 2) as x approaches 2."
Read as: Limit at a point
limit = 4
  by: Limit at a point  (exact symbolic limit (series expansion / Gruntz algorithm) from both sides; refused if the one-sided limits differ or are infinite)
  a = 2
  f = (x^2 - 4)/(x - 2)
  assumes: the left and right limits exist, are finite and agree
```

**What a capability needs** — The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ calculus info limit_at_point
Limit at a point
  exact symbolic limit (series expansion / Gruntz algorithm) from both sides; refused if the one-sided limits differ or are infinite
  in   f        the function f(x) (text) [1]
  in   a        the point x = a [1]
  out  limit    the limit of f(x) as x -> a [1]
```

**Solve** — Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ calculus solve limit_at_point a=2.0 "f=(x^2 - 4)/(x - 2)"
limit = 4
  by: Limit at a point  (exact symbolic limit (series expansion / Gruntz algorithm) from both sides; refused if the one-sided limits differ or are infinite)
  a = 2
  f = (x^2 - 4)/(x - 2)
  assumes: the left and right limits exist, are finite and agree
```

**Many questions at once** — One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | calculus batch

`cases.jsonl`:

```json
{"key": "limit_at_point", "inputs": {"a": 2.0, "f": "(x^2 - 4)/(x - 2)"}}
{"key": "extrema_on_interval", "inputs": {"a": 0.0, "b": 2.0, "f": "x^3 - 3x"}}
```

```console
$ calculus batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":4.0,"unit":"","solved_for":"limit","si":[["a",2.0]],"values":[["limit",4.0,""]],"target":"limit","phrase":"the limit of f(x) as x -> a","assumptions":["the left and right limits exist, are finite and agree"],"name":"Limit at a point","equation":"exact symbolic limit (series expansion / Gruntz algorithm) from both sides; refused if the one-sided limits differ or are infinite"}
{"outcome":"SOLVED","message":"","value":2.0,"unit":"","solved_for":"absolute_max","si":[["a",0.0],["b",2.0]],"values":[["absolute_max",2.0,""],["absolute_min",-2.0,""],["critical_points",[1.0],""]],"target":"absolute_max","phrase":"the largest value of f on [a, b]","assumptions":["a < b","f is a polynomial with rational coefficients"],"name":"Absolute extrema on a closed interval","equation":"exact real roots of f' in (a, b) (Sturm isolation), then f compared at the critical points and both ends"}
```

### In your browser

Open [Calculus on ai999x.com](https://www.ai999x.com/models/calculus-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `calculus_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install calculus-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Absolute extrema on a closed interval

The absolute maximum and minimum values of a polynomial on [a, b] by the closed-interval method, with the critical points inside (a, b).

Command line:

```console
$ calculus solve extrema_on_interval a=0 b=2 "f=x^3 - 3x"
absolute_max = 2
absolute_min = -2
critical_points = [1.0]
  by: Absolute extrema on a closed interval  (exact real roots of f' in (a, b) (Sturm isolation), then f compared at the critical points and both ends)
  a = 0
  b = 2
  f = x^3 - 3x
  assumes: a < b; f is a polynomial with rational coefficients
```

Python:

```python
import calculus_ai999x as m

a = m.solve("extrema_on_interval", a="0", b="2", f="x^3 - 3x")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/calculus-ai999x/#cap-extrema_on_interval) and press **Compute in your browser**.

_What this means:_ This calculation determines the highest and lowest values attained by the polynomial f(x) over the closed interval from a to b. The method identifies all critical points where the derivative vanishes within the open interval, then compares the function value at each such point against the values at the two endpoints. The largest of these comparisons yields the absolute maximum 2, while the smallest gives the absolute minimum -2. This approach relies on the Extreme Value Theorem, which guarantees that a continuous function on a closed bounded interval must attain both its global extrema.

### Average rate of change

The average rate of change (f(b) - f(a))/(b - a) of f over [a, b]: the slope of the secant line.

Command line:

```console
$ calculus solve average_rate_of_change a=1 b=3 "f=x^2"
average_rate = 4
fa = 1
fb = 9
  by: Average rate of change  (difference quotient of the two function values)
  a = 1
  b = 3
  f = x^2
  assumes: a != b; f defined at a and b
```

Python:

```python
import calculus_ai999x as m

a = m.solve("average_rate_of_change", a="1", b="3", f="x^2")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/calculus-ai999x/#cap-average_rate_of_change) and press **Compute in your browser**.

_What this means:_ This calculation determines the average rate of change for the function over the interval defined by the start point 1 and the end point 3. It is computed using the difference quotient, which takes the value of the function at the end point, 9, subtracts the value at the start point, 1, and divides this difference by the width of the interval. The resulting value, 4, represents the slope of the secant line connecting these two points on the graph, indicating the mean speed or change per unit interval.

### Critical points and their classification

All critical points of a polynomial f(x) and which are local maxima or minima (first-derivative test).

Command line:

```console
$ calculus solve critical_points_classify "f=x^3 - 3x"
critical_points = [-1.0,1.0]
local_maxima = [-1.0]
local_minima = [1.0]
  by: Critical points and their classification  (exact real roots of f' (Sturm isolation); the sign of f' on each side decides max / min / neither)
  f = x^3 - 3x
  assumes: f is a polynomial of degree >= 2 with rational coefficients
```

Python:

```python
import calculus_ai999x as m

a = m.solve("critical_points_classify", f="x^3 - 3x")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/calculus-ai999x/#cap-critical_points_classify) and press **Compute in your browser**.

_What this means:_ This calculation determines the locations where the slope of the polynomial x^3 - 3x is zero. Finding the exact real roots of the derivative identifies all critical points listed in ascending order as [-1, 1]. The sign change of the derivative around each point classifies them, resulting in the set of local maxima [-1] and the set of local minima [1].

### Curvature of a graph

The curvature kappa = |f''| / (1 + f'^2)^(3/2) of y = f(x) at x = a, and the radius of curvature 1/kappa.

Command line:

```console
$ calculus solve curvature a=0 "f=x^2"
curvature = 2
radius_of_curvature = 0.5
  by: Curvature of a graph  (curvature formula of a graph from the exact first and second derivatives)
  a = 0
  f = x^2
  assumes: f twice differentiable at a
```

Python:

```python
import calculus_ai999x as m

a = m.solve("curvature", a="0", f="x^2")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/calculus-ai999x/#cap-curvature) and press **Compute in your browser**.

_What this means:_ This calculation determines how sharply the graph of the function bends at the specific point defined by the input x-value. Using the exact first and second derivatives of the function, it computes the curvature value and the corresponding radius of the osculating circle. The result indicates that at this location, the curve has a curvature of 2 and fits within a circle of radius 0.5, providing a precise geometric measure of the local bend.

## Documentation

- **Rust user guide** — the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_rust_guide.md)
- **Python guide** — every capability with a Python example: [PDF](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/calculus-ai999x/blob/main/docs/calculus_ai999x_capabilities.md)
- **Desktop app & command line** — downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page** — compute in your browser, every capability explained: https://www.ai999x.com/models/calculus-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified calculus capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 374 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `calculus` command line for Windows and Linux, and WebAssembly on the model page — no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** limits, derivatives, integrals, series, Taylor polynomials, multivariable and vector calculus, special functions and numerical differentiation and integration.

## Model overview

| Property | Value |
|---|---|
| Package | `calculus-ai999x` 1.1.0 (import `calculus_ai999x`) |
| Type | Deterministic neuro-symbolic calculus engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 71 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 374 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `calculus-ai999x "<question>"`; for the native program use `calculus` (above). The outputs above were produced by running this exact code on calculus-ai999x 1.1.0.

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

## Python worked examples, with their steps

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
@software{calculus_ai999x_1_1_0,
  title  = {calculus-ai999x: an offline, verified calculus solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.1.0},
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
