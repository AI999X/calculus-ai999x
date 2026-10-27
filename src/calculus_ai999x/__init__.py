"""calculus-ai999x — offline calculus answers with their formula, steps and verification.

    import calculus_ai999x as m
    m.ask("Find the limit of (x^2 - 4)/(x - 2) as x approaches 2.").show()          # limit = 4
    m.solve("definite_integral", f="x^2*exp(-x)", a=0, b=2).show()                   # 2 - 10/e^2
    m.capabilities()                   # every certified capability

Covers limits (at a point, at infinity), derivatives (at a point, higher order, implicit, tangent and normal lines,
linear approximation, curvature, velocity and acceleration, critical points, absolute extrema, inflection points,
the mean value theorem), integrals (definite and improper, the fundamental theorem, average value, areas between
curves, volumes by discs, washers and shells, arc length, surface area of revolution, centroids), sequences and
series (arithmetic and geometric sums, closed-form sums of infinite series, the ratio test, Taylor and Maclaurin
polynomials), multivariable and vector calculus (partial derivatives, gradient, directional derivative, tangent
plane, second derivative test, Laplacian, divergence, curl, Jacobians, double and triple integrals, surface area,
line integrals, Green's theorem, flux through a box), parametric and polar curves, special functions (gamma, beta,
error function, incomplete gamma, Bessel J_n, Riemann zeta, Legendre and Chebyshev polynomials), Fourier
coefficients, and numerical methods for any function written as an expression (Riemann sums, trapezoid, midpoint,
Simpson and Gauss-Legendre rules, finite differences, Newton's method).

A function is written as text ("x^3 - 2x", "sin(2x)*exp(-x)") and read by an allowlisted parser, never evaluated as
code. Conventions: angles in radians; the Fourier series is a_0/2 + sum a_k cos(k pi x/L) + b_k sin(k pi x/L) on
[-L, L]; critical, inflection and mean-value points are listed in ascending order; a direction vector is normalised
to unit length. A question that is not determined (a limit that does not exist, a divergent series, an inconclusive
test, a point off the curve) is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.1"
_P = Product("calculus", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
