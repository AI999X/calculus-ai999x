# Calculus: the desktop app, user guide

**Calculus**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Calculus covers limits, derivatives, integrals, series, Taylor polynomials, multivariable and vector calculus, special functions and numerical differentiation and integration. Its desktop app holds all 82 calculations in 16 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-calculus-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-calculus-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-calculus-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-calculus.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-calculus-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-calculus-1.6.5-linux-x86_64.tar.gz
./ai999x-calculus-1.6.5-linux-x86_64/ai999x-calculus
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Calculus app, opened on Limit at a point with its worked example.](https://www.ai999x.com/?ax_shot=calculus-ai999x/hero.webp)

*The Calculus app, opened on Limit at a point with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Limits › Limit at a point (or type part of its name in the search box). It opens with a worked example filled in: “Find the limit of (x^2 - 4)/(x - 2) as x approaches 2.” The answer: limit = 4.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Limit at a point: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=calculus-ai999x/scene-1.webp)

*Limit at a point: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Find the limit of (x^2 - 4)/(x - 2) as x approaches 2.” The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Limit at a point
limit = 4
  by: Limit at a point  (exact symbolic limit (series expansion / Gruntz algorithm) from both sides; refused if the one-sided limits differ or are infinite)
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Absolute extrema on a closed interval](https://www.ai999x.com/?ax_shot=calculus-ai999x/scene-2.webp)

*Absolute extrema on a closed interval*

![Taylor polynomial](https://www.ai999x.com/?ax_shot=calculus-ai999x/scene-3.webp)

*Taylor polynomial*

![Line integral of a vector field](https://www.ai999x.com/?ax_shot=calculus-ai999x/scene-4.webp)

*Line integral of a vector field*

![Definite integral](https://www.ai999x.com/?ax_shot=calculus-ai999x/scene-5.webp)

*Definite integral*

![Fourier coefficients](https://www.ai999x.com/?ax_shot=calculus-ai999x/scene-6.webp)

*Fourier coefficients*

![Triple integral in cylindrical or spherical coordinates](https://www.ai999x.com/?ax_shot=calculus-ai999x/scene-7.webp)

*Triple integral in cylindrical or spherical coordinates*

![Directional derivative](https://www.ai999x.com/?ax_shot=calculus-ai999x/scene-8.webp)

*Directional derivative*

![Limit at a point: a chart, enlarged.](https://www.ai999x.com/?ax_shot=calculus-ai999x/chart-1.webp)

*Limit at a point: a chart, enlarged.*

![Absolute extrema on a closed interval: a chart, enlarged.](https://www.ai999x.com/?ax_shot=calculus-ai999x/chart-2.webp)

*Absolute extrema on a closed interval: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Calculus app in the dark theme.](https://www.ai999x.com/?ax_shot=calculus-ai999x/hero-dark.webp)

*The Calculus app in the dark theme.*

## 7. Every calculation

82 calculations in 16 topics; each has its own entry on the model's page (https://www.ai999x.com/models/calculus-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Derivatives | Absolute extrema on a closed interval, Average rate of change, Critical points and their classification, Curvature of a graph, Derivative at a point, Higher derivative at a point, Implicit differentiation, Inflection points, Linear approximation, Mean value theorem, Normal line, Second derivative at a point, Tangent line, Velocity and acceleration from position |
| Fourier series | Fourier coefficients |
| Integrals | Antiderivative (indefinite integral), Arc length of a graph, Area between two curves, Area enclosed by two curves, Average value of a function, Centroid of a region under a curve, Definite integral, Derivative of an integral (fundamental theorem), Improper integral to infinity, Left Riemann sum, Right Riemann sum, Root-mean-square value of a function, Surface area of revolution, Volume of revolution (disk method), Volume of revolution (shell method), Volume of revolution (washer method) |
| Limits | Limit at a point, Limit at infinity |
| Multiple integrals | Double integral in polar coordinates, Double integral over a rectangle, Double integral over a region, Surface area of a graph, Triple integral in cylindrical or spherical coordinates, Triple integral over a box |
| Multivariable calculus | Directional derivative, Gradient, Jacobian determinant, Lagrange multipliers: extrema under a constraint, Partial derivative, Second derivative test (two variables), Tangent plane, Total differential and small changes |
| Numerical differentiation | Finite-difference derivative |
| Numerical integration | Gauss-Legendre quadrature, Midpoint rule, Simpson's rule, Trapezoidal rule |
| Numerical methods | Newton's method |
| Parametric curves | Arc length of a parametric curve, Slope and concavity of a parametric curve |
| Polar curves | Area in polar coordinates |
| Series | Partial sum of a geometric series, Radius and interval of convergence of a power series, Ratio test, Sum of an arithmetic series, Sum of an infinite geometric series, Sum of an infinite series, Taylor polynomial |
| Special functions | Bessel function of the first kind, Beta function, Chebyshev polynomial, Error function, Gamma function, Legendre polynomial, Regularized incomplete gamma functions, Riemann zeta function |
| Vector calculus | Conservative vector field and potential function, Curl, Divergence, Green's theorem on a rectangle, Laplacian, Line integral of a vector field, Line integral with respect to arc length, Outward flux through a box (divergence theorem) |
| Vector-valued functions | Arc length of a space curve, Curvature and unit tangent of a space curve |
| Vectors in space | Rectangular, cylindrical and spherical coordinates |

## 8. Other ways to use it

The same engine runs on the command line (`calculus`, its own guide), in Python (`pip install calculus-ai999x`, `import calculus_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/calculus-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/calculus-ai999x/
