"""Calculus procedures: deterministic algorithms with typed inputs and several outputs (domain code, loaded by the
pack; enabled only once tools/certify_procedures.py has certified them).

Expressions arrive as text ("x^3 - 2x", "sin(2x)*exp(-x)") and are read by an ALLOWLISTED parser (Python's ast,
walked node by node; nothing is ever evaluated as code): numbers, the variables a capability names, pi and e,
+ - * / ^ and the functions sin cos tan sec csc cot asin acos atan sinh cosh tanh exp log ln sqrt.

The shipped computations use sympy (exact differentiation, limits, sums, polynomial roots) and mpmath (high-precision
quadrature and special functions), both runtime dependencies of the package. Every procedure carries an INDEPENDENT
reference computation used only by certification (scipy.integrate, scipy.special, numpy, or mpmath's numerical
differentiation / limits / sums against a sympy symbolic run) and a deliberately wrong variant that certification
shows the checks reject.

Conventions (fixed so that run and reference agree exactly): angles in radians; a stated decimal is the exact rational
it spells (0.1 = 1/10) wherever a result is exact (limits, polynomial roots, series); critical, inflection and
mean-value points are distinct real points in ascending order (critical points on [a, b] exclude the ends); a
direction vector is normalised to unit length; Fourier series a_0/2 + sum a_k cos(k pi x/L) + b_k sin(k pi x/L) on
[-L, L] with a_0 = (1/L) * integral of f; Green's circulation is counter-clockwise; flux through a box is outward;
regularized incomplete gamma P(a, x) = gamma(a, x)/Gamma(a). Multiple integrals are product Gauss-Legendre rules of
increasing order accepted only when two orders agree to 1e-12 of the integral of |f| (refused otherwise).
"""
from __future__ import annotations

import ast
import functools
import math
import re
from fractions import Fraction

import mpmath as mp
import sympy as sp

# ------------------------------------------------------------------------------------------- expression reading
X, Y, Z, T = sp.symbols("x y z t", real=True)
U, V = sp.symbols("u v", real=True)
R_, TH = sp.symbols("r theta", real=True)
NN = sp.Symbol("n", integer=True, positive=True)
_SYMS = {"x": X, "y": Y, "z": Z, "t": T, "u": U, "v": V, "r": R_, "theta": TH, "n": NN}

_FN = {"sin": sp.sin, "cos": sp.cos, "tan": sp.tan, "sec": sp.sec, "csc": sp.csc, "cot": sp.cot,
       "asin": sp.asin, "acos": sp.acos, "atan": sp.atan, "arcsin": sp.asin, "arccos": sp.acos, "arctan": sp.atan,
       "sinh": sp.sinh, "cosh": sp.cosh, "tanh": sp.tanh, "exp": sp.exp, "log": sp.log, "ln": sp.log,
       "sqrt": sp.sqrt}
_CONST = {"pi": sp.pi, "e": sp.E}
_MAX_LEN, _MAX_NODES = 300, 160


def _prep(text: str, names) -> str:
    """Written mathematics -> Python arithmetic: ^ -> **, implicit products (2x, 3(x+1), x(x+1), xy), sin^2(x)."""
    s = text.strip().replace("^", "**").replace("−", "-").replace("·", "*").replace("×", "*").replace("π", "pi")
    s = s.replace("²", "**2").replace("³", "**3").replace("θ", "theta").replace("{", "(").replace("}", ")")
    s = s.replace("[", "(").replace("]", ")")
    # sin**2(x) -> (sin(x))**2
    for _ in range(4):
        m = re.search(r"\b(" + "|".join(sorted(_FN, key=len, reverse=True)) + r")\s*\*\*\s*(\d+)\s*\(", s)
        if not m:
            break
        depth, j = 0, m.end() - 1
        for j in range(m.end() - 1, len(s)):
            depth += {"(": 1, ")": -1}.get(s[j], 0)
            if depth == 0:
                break
        if depth != 0:
            break
        s = s[:m.start()] + "(" + m.group(1) + s[m.end() - 1:j + 1] + ")**" + m.group(2) + s[j + 1:]

    def word(m):
        w, after = m.group(0), s[m.end():m.end() + 1]
        if w in _FN or w in _CONST or w in names:
            if w in names or w in _CONST:
                return w + ("*" if after == "(" else "")
            return w
        if all(c in names or c in _CONST for c in w):          # xy -> x*y
            parts = list(w)
            return "*".join(parts) + ("*" if after == "(" else "")
        return w
    s = re.sub(r"[A-Za-z_]+", word, s)
    s = re.sub(r"(\d)\s*(?![eE][-+]?\d)([A-Za-z(])", r"\1*\2", s)   # 2x, 3(x+1), 2sin(x); not 1e-3
    s = re.sub(r"\)\s*([A-Za-z0-9(])", r")*\1", s)                   # (x+1)(x-1), (x+1)x
    return s


@functools.lru_cache(maxsize=4096)
def _parse_cached(text: str, names: tuple):
    return _parse_expr(text, names)


def parse_expr(text: str, names=("x",)) -> sp.Expr:
    """An allowlisted expression over the named variables; ValueError for anything else (cached: sympy
    expressions are immutable)."""
    if not isinstance(text, str):
        raise ValueError("the expression must be text")
    return _parse_cached(text, tuple(names))


def _parse_expr(text: str, names=("x",)) -> sp.Expr:
    if not isinstance(text, str) or not text.strip() or len(text) > _MAX_LEN:
        raise ValueError("the expression is missing or too long")
    names = tuple(names)
    src = _prep(text, names)
    try:
        tree = ast.parse(src, mode="eval")
    except SyntaxError:
        raise ValueError(f"cannot read the expression {text!r}") from None
    if sum(1 for _ in ast.walk(tree)) > _MAX_NODES:
        raise ValueError("the expression is too complex")

    def walk(n):
        if isinstance(n, ast.Expression):
            return walk(n.body)
        if isinstance(n, ast.Constant) and type(n.value) in (int, float):
            return sp.Integer(n.value) if isinstance(n.value, int) else sp.Rational(repr(n.value))
        if isinstance(n, ast.Name):
            if n.id in names:
                return _SYMS[n.id]
            if n.id in _CONST:
                return _CONST[n.id]
            raise ValueError(f"unknown name {n.id!r} (this capability uses {', '.join(names)})")
        if isinstance(n, ast.UnaryOp) and isinstance(n.op, (ast.USub, ast.UAdd)):
            v = walk(n.operand)
            return -v if isinstance(n.op, ast.USub) else v
        if isinstance(n, ast.BinOp) and isinstance(n.op, (ast.Add, ast.Sub, ast.Mult, ast.Div, ast.Pow)):
            a, b = walk(n.left), walk(n.right)
            if isinstance(n.op, ast.Add):
                return a + b
            if isinstance(n.op, ast.Sub):
                return a - b
            if isinstance(n.op, ast.Mult):
                return a * b
            if isinstance(n.op, ast.Div):
                if b == 0:
                    raise ValueError("division by zero in the expression")
                return a / b
            if b.is_number and abs(b) > 64:
                raise ValueError("exponents are limited to 64 in size")
            return a ** b
        if (isinstance(n, ast.Call) and isinstance(n.func, ast.Name) and n.func.id in _FN
                and len(n.args) == 1 and not n.keywords):
            return _FN[n.func.id](walk(n.args[0]))
        raise ValueError(f"unsupported syntax in {text!r}")

    e = walk(tree)
    if e.has(sp.zoo, sp.nan, sp.oo, -sp.oo):
        raise ValueError("the expression is undefined")
    return e


def number(text) -> float:
    """A stated number, possibly written with pi, e, fractions or roots ('pi/2', '3/4', 'sqrt(2)')."""
    if isinstance(text, (int, float)) and not isinstance(text, bool):
        return float(text)
    e = parse_expr(str(text), ())
    v = complex(sp.N(e, 30))
    if v.imag or not math.isfinite(v.real):
        raise ValueError(f"{text!r} is not a finite real number")
    return v.real


# ------------------------------------------------------------------------------------------- evaluation
_MPF = {"sin": mp.sin, "cos": mp.cos, "tan": mp.tan, "sec": mp.sec, "csc": mp.csc, "cot": mp.cot, "asin": mp.asin,
        "acos": mp.acos, "atan": mp.atan, "sinh": mp.sinh, "cosh": mp.cosh, "tanh": mp.tanh, "exp": mp.exp,
        "log": mp.log, "Abs": mp.fabs}
_FLF = {"sin": math.sin, "cos": math.cos, "tan": math.tan, "sec": lambda a: 1 / math.cos(a),
        "csc": lambda a: 1 / math.sin(a), "cot": lambda a: math.cos(a) / math.sin(a), "asin": math.asin,
        "acos": math.acos, "atan": math.atan, "sinh": math.sinh, "cosh": math.cosh, "tanh": math.tanh,
        "exp": math.exp, "log": math.log, "Abs": abs}


def _compile(e, names, use_mp=True):
    """A callable f(*values) for a sympy expression built from the allowlisted pieces (no code is generated)."""
    idx = {(_SYMS[s] if isinstance(s, str) else s): i for i, s in enumerate(names)}
    fns = _MPF if use_mp else _FLF
    num = (lambda q: mp.mpf(q.p) / q.q) if use_mp else (lambda q: q.p / q.q)

    def build(e):
        if e.is_Symbol:
            if e not in idx:
                raise ValueError(f"unexpected variable {e}")
            i = idx[e]
            return lambda v: v[i]
        if e.is_Rational:
            c = num(e)
            return lambda v: c
        if e is sp.pi:
            c = mp.pi if use_mp else math.pi
            return lambda v: (+c)
        if e is sp.E:
            c = mp.e if use_mp else math.e
            return lambda v: (+c)
        if e.is_Float:
            c = mp.mpf(e) if use_mp else float(e)
            return lambda v: c
        if e.is_Add:
            fs = [build(a) for a in e.args]
            return lambda v: sum((f(v) for f in fs[1:]), fs[0](v))
        if e.is_Mul:
            fs = [build(a) for a in e.args]

            def mul(v):
                out = fs[0](v)
                for f in fs[1:]:
                    out = out * f(v)
                return out
            return mul
        if e.is_Pow:
            b = build(e.base)
            if e.exp.is_Integer:
                k = int(e.exp)
                return lambda v: b(v) ** k
            if e.exp == sp.Rational(1, 2):
                return (lambda v: mp.sqrt(b(v))) if use_mp else (lambda v: math.sqrt(b(v)))
            ex = build(e.exp)
            if use_mp:
                return lambda v: mp.power(b(v), ex(v))

            def pw(v):
                r = b(v) ** ex(v)
                if isinstance(r, complex):
                    raise ValueError("not a real number")
                return r
            return pw
        if e.func.__name__ in fns and len(e.args) == 1:
            f, a = fns[e.func.__name__], build(e.args[0])
            return lambda v: f(a(v))
        raise ValueError(f"cannot evaluate {e.func.__name__}")

    g = build(sp.sympify(e))

    def call(*vals):
        try:
            r = g(vals)
        except (ZeroDivisionError, OverflowError, ValueError) as exc:
            raise ValueError(f"the expression is not defined there ({type(exc).__name__})") from None
        if use_mp and isinstance(r, mp.mpc):
            if abs(r.imag) > mp.mpf(10) ** (-(mp.mp.dps - 5)) * max(1, abs(r.real)):
                raise ValueError("the expression is not real there")
            r = r.real
        return r
    return call


def _fin(v, what="the result") -> float:
    v = float(v)
    if not math.isfinite(v):
        raise ValueError(f"{what} is not finite")
    return v


def at(e, names, *vals, dps=30) -> float:
    """The value of a sympy expression at a point, to `dps` digits, as a float (ValueError if undefined)."""
    if sp.sympify(e).has(sp.zoo, sp.nan, sp.oo, -sp.oo):
        raise ValueError("the expression is undefined there")
    with mp.workdps(dps):
        return _fin(_compile(e, names)(*[mp.mpf(v) for v in vals]))


def fx(text, names=("x",)):
    """(sympy expression, mp callable) of a text expression."""
    e = parse_expr(text, names)
    return e, _compile(e, names)


def quad(f, a, b, dps=20, points=None):
    """mpmath tanh-sinh quadrature of a callable on [a, b] with an error check (ValueError if it does not settle)."""
    with mp.workdps(dps):
        pts = [mp.mpf(a)] + [mp.mpf(p) for p in (points or [])] + [mp.mpf(b)]
        v, err = mp.quad(f, pts, error=True)
        if err > mp.mpf(10) ** -15 * max(1, abs(v)):
            pts = list(mp.linspace(pts[0], pts[-1], 17)) if not points else pts
            v, err = mp.quad(f, pts, error=True, maxdegree=10)
        if err > mp.mpf(10) ** -12 * max(1, abs(v)):
            raise ValueError("the integral did not converge numerically")
        return _fin(v, "the integral")


@functools.lru_cache(maxsize=1024)
def continuous_on(e, var, a, b):
    """ValueError unless the expression is real and continuous on the whole closed interval [a, b]."""
    from sympy.calculus.util import continuous_domain
    lo, hi = (a, b) if a <= b else (b, a)
    iv = sp.Interval(sp.Rational(repr(float(lo))), sp.Rational(repr(float(hi))))
    try:
        dom = continuous_domain(e, var, iv)
    except Exception:                                            # undecided: check on a fine grid instead
        g = _compile(e, (var,), use_mp=False)
        for k in range(401):
            g(lo + (hi - lo) * k / 400)
        return
    if dom != iv:
        raise ValueError(f"the function is not continuous (or not real) on the whole interval [{lo:g}, {hi:g}]")


def rel_err(got, want, slack=0.0) -> float:
    """A verifier residual: relative difference beyond `slack` (0 for a correct output)."""
    try:
        d = abs(complex(got) - complex(want)) / max(1.0, abs(complex(want)))
    except (TypeError, ValueError):
        return 1.0
    if d != d:
        return 1.0
    return max(0.0, d - slack)


def worst(*residuals) -> float:
    return max(residuals) if residuals else 0.0


def show(e) -> str:
    """A readable text form of a sympy result (x**2 -> x^2)."""
    return sp.sstr(e).replace("**", "^")


def poly_of(text, var="x"):
    """A polynomial with rational coefficients (sympy Poly), or ValueError."""
    e = parse_expr(text, (var,))
    s = _SYMS[var]
    if not e.is_polynomial(s):
        raise ValueError("this capability needs a polynomial")
    p = sp.Poly(sp.expand(e), s)
    if not all(c.is_rational for c in p.all_coeffs()):
        raise ValueError("the polynomial needs rational coefficients")
    return p


def real_roots_in(p, a, b, open_=True):
    """Real roots of a polynomial (sympy Poly) inside (a, b), exact, sorted, as sympy numbers."""
    if p.is_zero:
        raise ValueError("the polynomial is identically zero")
    if p.degree() < 1:
        return []
    A, B = sp.Rational(repr(float(a))), sp.Rational(repr(float(b)))
    out = []
    for r in sp.real_roots(p):
        if (A < r < B) if open_ else (A <= r <= B):
            out.append(r)
    return sorted(set(out), key=lambda r: float(r))


# ------------------------------------------------------------------------------------------- reference helpers
def ref_lam(text, names=("x",), module="mpmath"):
    """Build-time only: sympy's own lambdify of the parsed expression (an evaluator independent of _compile)."""
    e = parse_expr(text, names)
    return sp.lambdify([_SYMS[n] for n in names], e, modules=module)


def ref_quad(f, a, b, **kw):
    """Build-time only: scipy's adaptive Gauss-Kronrod quadrature (QUADPACK)."""
    from scipy.integrate import quad as squad
    v, _err = squad(f, a, b, epsabs=1e-13, epsrel=1e-13, limit=500, **kw)
    return float(v)


def ref_cdiff(text, names=("x",)):
    """Build-time only: the complex-step derivative f'(x) = Im f(x + ih)/h of the parsed expression, evaluated by
    sympy's lambdify over cmath (exact to rounding for analytic functions; no symbolic differentiation)."""
    g = ref_lam(text, names, module="cmath")
    return lambda x: complex(g(complex(x, 1e-30))).imag / 1e-30


def ref_diff(f, x0, n=1):
    """Build-time only: mpmath's numerical differentiation (finite differences at high precision)."""
    with mp.workdps(40):
        return float(mp.diff(f, mp.mpf(x0), n))


# ------------------------------------------------------------------------------------------- question reading
_PREFIX = re.compile(r"^(?:please\s+|now\s+|then\s+)?(?:(?:can you|could you)\s+)?(?:find|compute|calculate|evaluate|"
                     r"determine|what is|what's|what are|give|get|state|work out|estimate|approximate|show|obtain|"
                     r"use [a-z' ]+? to (?:find|estimate|approximate|compute|calculate|evaluate))\s+(?:me\s+)?")


def _clean(q: str) -> str:
    s = q.strip().lower()
    for a, b in (("−", "-"), ("²", "^2"), ("³", "^3"), ("π", "pi"), ("√", "sqrt"), ("∞", "infinity"),
                 ("θ", "theta"), ("’", "'"), ("‘", "'"), ("“", '"'), ("”", '"'), ("∫", "integral "), ("→", "->")):
        s = s.replace(a, b)
    s = re.sub(r"\s+", " ", s).strip()
    s = re.sub(r"[\s.?!]+$", "", s)
    for _ in range(3):
        s2 = _PREFIX.sub("", s).strip()
        s2 = re.sub(r"^the\s+", "", s2)
        if s2 == s:
            break
        s = s2
    return s


_SLOT = re.compile(r"\{(\w+)(?::(\w))?\}")
_NUMTOK = r"[-+]?(?:[0-9.]+|pi|e|sqrt\([0-9./]+\))(?:\s*[*/]?\s*(?:[0-9.]+|pi|sqrt\([0-9./]+\)))*"


def _pattern(p: str):
    """A question pattern: {f} an expression, {a:n} a number, {p:v} a vector '(1, 2)' or '1, 2', {k:i} a whole
    number, {c:w} a word; a space matches any spacing; '=' may have spaces around it."""
    types, out, pos = {}, [], 0
    for m in _SLOT.finditer(p):
        lit = p[pos:m.start()]
        out.append(_lit(lit))
        name, t = m.group(1), m.group(2) or "f"
        types[name] = t
        out.append({"f": r"(?P<%s>.+?)", "n": r"(?P<%s>" + _NUMTOK + r")",
                    "v": r"\(?\s*(?P<%s>[-+]?[0-9.]+(?:\s*,\s*[-+]?[0-9.]+){1,2})\s*\)?",
                    "i": r"(?P<%s>\d+)", "w": r"(?P<%s>[a-z]+)"}[t] % name)
        pos = m.end()
    out.append(_lit(p[pos:]))
    return re.compile("".join(out)), types


def _lit(text: str) -> str:
    """Literal text of a pattern; <...> holds raw regex (optional words, alternatives)."""
    out = []
    for i, part in enumerate(re.split(r"<([^<>]*)>", text)):
        if i % 2:
            out.append("(?:" + part + ")")
            continue
        s = re.escape(part).replace("\\ ", " ")
        s = re.sub(r" *= *", lambda m: r"\s*=\s*", s)
        s = re.sub(r" +", lambda m: r"\s+", s)
        out.append(s)
    return "".join(out)


_RULES: list = []
_EXPR_LEAD = re.compile(r"^(?:the\s+)?(?:function\s+|curve\s+|graph of\s+|expression\s+|polynomial\s+)?"
                        r"(?:[a-z]\s*\(\s*[a-z]+(?:\s*,\s*[a-z]+)*\s*\)|[a-z])\s*=\s*(?=.)")


def _expr_text(s: str) -> str:
    s = s.strip().strip(",;").strip()
    s2 = _EXPR_LEAD.sub("", s)
    if s2 != s and not re.match(r"^[\s=]", s2):
        s = s2
    return s.strip()


def rule(key, *patterns, fixed=None, post=None, order=50):
    """Question patterns of a capability. Patterns are tried by `order` (more specific forms first), then in the
    order written; the first that reads the whole question decides, so two capabilities never both claim it."""
    for p in patterns:
        rx, types = _pattern(p)
        _RULES.append((order, len(_RULES), key, rx, types, dict(fixed or {}), post))
    _RULES.sort(key=lambda r: (r[0], r[1]))


@functools.lru_cache(maxsize=1024)
def _route(q: str):
    s = _clean(q)
    for _o, _n, key, rx, types, fixed, post in _RULES:
        m = rx.fullmatch(s)
        if not m:
            continue
        d, ok = dict(fixed), True
        for name, val in m.groupdict().items():
            if val is None:
                continue
            t = types.get(name, "w")
            try:
                if t == "n":
                    d[name] = number(val)
                elif t == "i":
                    d[name] = int(val)
                elif t == "v":
                    d[name] = [number(x) for x in val.split(",")]
                elif t == "w":
                    d[name] = val
                else:
                    d[name] = _expr_text(val)
                    if not d[name]:
                        ok = False
            except (ValueError, ZeroDivisionError):
                ok = False
        if not ok:
            continue
        if post is not None:
            d = post(d)
            if d is None:
                continue
        return key, tuple(sorted(d.items(), key=lambda kv: kv[0]))
    return None


def parser(key):
    def parse(question: str):
        if not isinstance(question, str):
            return None
        hit = _route(question)
        if not hit or hit[0] != key:
            return None
        return {k: (list(v) if isinstance(v, list) else v) for k, v in hit[1]}
    return parse


# ------------------------------------------------------------------------------------------- spec helpers
PROCEDURES: dict = {}


def T_(sym, meaning, required=True, max_length=_MAX_LEN):
    return {"symbol": sym, "type": "text", "meaning": meaning, "required": required, "max_length": max_length}


def N_(sym, meaning, required=True, **kw):
    return {"symbol": sym, "kind": "number", "meaning": meaning, "required": required, **kw}


def I_(sym, meaning, lo=None, hi=None, required=True):
    d = {"symbol": sym, "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
    return d


def V_(sym, meaning, lo=2, hi=3, required=True):
    return {"symbol": sym, "type": "vector", "meaning": meaning, "required": required, "min_length": lo, "max_length": hi}


def C_(sym, meaning, choices, required=True):
    return {"symbol": sym, "type": "choice", "meaning": meaning, "choices": list(choices), "required": required}


def O_(sym, meaning, kind="number"):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


def _safe_verify(fn):
    """A verifier that cannot check an output (a missing value, an undefined point) rejects it."""
    def verify(inputs, outputs):
        try:
            r = float(fn(inputs, outputs))
        except (KeyError, TypeError, ValueError, ZeroDivisionError, OverflowError):
            return 1.0
        return r if r == r else 1.0
    return verify


def proc(key, *, name, topic, purpose, inputs, outputs, primary, conditions, method, latex, run, verify, reference,
         wrong, random_inputs, templates, examples, questions, keywords=()):
    PROCEDURES[key] = {"name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs,
                       "primary": primary, "conditions": conditions, "method": method, "latex": latex, "run": run,
                       "verify": _safe_verify(verify), "parse": parser(key), "reference": reference, "wrong": wrong,
                       "random_inputs": random_inputs, "templates": templates, "examples": examples,
                       "questions": questions, "keywords": list(keywords)}


# ------------------------------------------------------------------------------------------- random expressions
def _c(rng, lo=-5, hi=5, nonzero=True):
    while True:
        v = rng.randint(lo, hi)
        if v or not nonzero:
            return v


def _term(coef, body):
    """'3*x^2', '-x', '2' written as a person would."""
    if body == "":
        return str(coef)
    if coef == 1:
        return body
    if coef == -1:
        return "-" + body
    return f"{coef}*{body}"


def _join(terms):
    s = " + ".join(t for t in terms if t)
    return s.replace("+ -", "- ") or "0"


def rand_poly(rng, deg=None, lo=-5, hi=5):
    deg = deg if deg is not None else rng.randint(2, 4)
    terms = []
    for k in range(deg, -1, -1):
        c = _c(rng, lo, hi, nonzero=(k == deg))
        if c == 0:
            continue
        terms.append(_term(c, "" if k == 0 else ("x" if k == 1 else f"x^{k}")))
    return _join(terms)


def rand_smooth(rng, var="x"):
    """A smooth function defined on the whole real line, from textbook families."""
    a, b, c = _c(rng, -4, 4), _c(rng, 1, 3), _c(rng, -3, 3)
    fam = rng.randrange(12)
    s = [rand_poly(rng),
         f"{a}*sin({b}*x) + {c}*x",
         f"{a}*cos({b}*x) + x^2",
         f"{a}*exp({rng.choice([-1, 1]) * b}*x/2)",
         f"x^2*exp(-x) + {c}",
         f"x*sin(x) + {a}",
         f"exp(-x^2) + {a}*x",
         f"{a}/(x^2 + {b})",
         f"atan({b}*x) + {c}*x^2",
         f"sqrt(x^2 + {b}) + {c}*x",
         f"{a}*x^3 + {c}*cos(x)",
         f"log(x^2 + {b}) - {c}*x"][fam]
    s = s.replace("+ -", "- ").replace("- -", "+ ").replace("--", "+")
    return s.replace("x", var) if var != "x" else s


def rand_point(rng, lo=-3, hi=3):
    return rng.choice([float(rng.randint(lo, hi)), round(rng.uniform(lo, hi), 2)])


def rand_interval(rng, lo=-3, hi=3, minw=0.5):
    while True:
        a, b = sorted([rand_point(rng, lo, hi), rand_point(rng, lo, hi)])
        if b - a >= minw:
            return a, b


# =========================================================================================== 1. derivatives
_AT = "<(?:at|when|where|for)> <(?:the point )?>"
_IFX = "<(?:if|for|when|where|given|with)> <(?:the function )?>"


def _mp_diff(text, a, n=1, names=("x",)):
    """mpmath's numerical derivative of the parsed expression (the verifiers' independent route)."""
    with mp.workdps(30):
        f = _compile(parse_expr(text, names), names)
        return mp.diff(f, mp.mpf(a), n)


def derivative_at_point(f, a):
    e = parse_expr(f)
    at(e, (X,), a)                                       # f itself must be defined at a
    d = sp.diff(e, X)
    return {"derivative": at(d, (X,), a), "derivative_expression": show(d)}


proc("derivative_at_point", name="Derivative at a point", topic="derivatives",
     purpose="The derivative f'(a) of a function f(x) at x = a (the slope of its graph there).",
     inputs=[T_("f", "the function f(x)"), N_("a", "the point x = a")],
     outputs=[O_("derivative", "f'(a)"), O_("derivative_expression", "f'(x) as an expression", "text")],
     primary="derivative", conditions=["f is defined and differentiable at x = a"],
     method="exact symbolic differentiation (sum, product, quotient and chain rules), then evaluation at x = a",
     latex=r"f'(a) = \lim_{h\to 0}\frac{f(a+h)-f(a)}{h}",
     run=derivative_at_point,
     verify=lambda i, o: rel_err(o["derivative"], _mp_diff(i["f"], i["a"]), 1e-12),
     reference=lambda f, a: {"derivative": ref_diff(ref_lam(f), a)},
     wrong=lambda f, a: {"derivative": at(parse_expr(f), (X,), a + 1) - at(parse_expr(f), (X,), a)},
     random_inputs=lambda rng: {"f": rand_smooth(rng), "a": rand_point(rng)},
     templates=["Find the derivative of f(x) = x^3 - 2x at x = 2.", "What is f'(0) if f(x) = sin(3x) + x^2?",
                "Differentiate x^2*exp(x) and evaluate at x = 1."],
     examples=[{"inputs": {"f": "x^3 - 2x", "a": 2}, "outputs": {"derivative": 10.0}},
               {"inputs": {"f": "sin(3x) + x^2", "a": 0}, "outputs": {"derivative": 3.0}},
               {"inputs": {"f": "x^2*exp(x)", "a": 1}, "outputs": {"derivative": 3 * math.e}},
               {"inputs": {"f": "log(x)", "a": 4}, "outputs": {"derivative": 0.25}}],
     questions=[{"q": "Find the derivative of f(x) = x^3 - 2x at x = 2.", "outputs": {"derivative": 10.0}},
                {"q": "What is f'(0) if f(x) = sin(3x) + x^2?", "outputs": {"derivative": 3.0}},
                {"q": "Differentiate x^2*exp(x) and evaluate at x = 1.", "outputs": {"derivative": 3 * math.e}},
                {"q": "What is the slope of the curve y = sqrt(x) at x = 9?", "outputs": {"derivative": 1 / 6}},
                {"q": "Find the instantaneous rate of change of x^2 + 3x at x = 1.", "outputs": {"derivative": 5.0}},
                {"q": "Find the derivative of log(x) at x = -1.", "outcome": "INVALID_INPUT"}],
     keywords=["derivative", "slope", "rate of change"])
rule("derivative_at_point",
     "<(?:first )?>derivative of {f} " + _AT + "x = {a:n}",
     "f'({a:n}) " + _IFX + "f(x) = {f}",
     "y'({a:n}) " + _IFX + "y = {f}",
     "differentiate {f} and evaluate <(?:it |the derivative )?>at x = {a:n}",
     "d/dx <(?:of )?>{f} at x = {a:n}",
     "slope of <(?:the )?(?:curve |graph |graph of |function )?>{f} at x = {a:n}",
     "<(?:instantaneous )?>rate of change of {f} at x = {a:n}", order=60)


def second_derivative_at_point(f, a):
    e = parse_expr(f)
    at(e, (X,), a)
    d2 = sp.diff(e, X, 2)
    return {"second_derivative": at(d2, (X,), a), "second_derivative_expression": show(d2)}


proc("second_derivative_at_point", name="Second derivative at a point", topic="derivatives",
     purpose="The second derivative f''(a) of f(x) at x = a (concavity: positive means concave up).",
     inputs=[T_("f", "the function f(x)"), N_("a", "the point x = a")],
     outputs=[O_("second_derivative", "f''(a)"), O_("second_derivative_expression", "f''(x)", "text")],
     primary="second_derivative", conditions=["f is twice differentiable at x = a"],
     method="exact symbolic differentiation twice, then evaluation at x = a", latex=r"f''(a) = \frac{d^2 f}{dx^2}\Big|_{x=a}",
     run=second_derivative_at_point,
     verify=lambda i, o: rel_err(o["second_derivative"], _mp_diff(i["f"], i["a"], 2), 1e-11),
     reference=lambda f, a: {"second_derivative": ref_diff(ref_lam(f), a, 2)},
     wrong=lambda f, a: {"second_derivative": derivative_at_point(f, a)["derivative"]},
     random_inputs=lambda rng: {"f": rand_smooth(rng), "a": rand_point(rng)},
     templates=["Find the second derivative of x^4 - 3x^2 at x = 1.", "What is f''(0) if f(x) = cos(2x)?"],
     examples=[{"inputs": {"f": "x^4 - 3x^2", "a": 1}, "outputs": {"second_derivative": 6.0}},
               {"inputs": {"f": "cos(2x)", "a": 0}, "outputs": {"second_derivative": -4.0}},
               {"inputs": {"f": "exp(2x)", "a": 0}, "outputs": {"second_derivative": 4.0}}],
     questions=[{"q": "Find the second derivative of x^4 - 3x^2 at x = 1.", "outputs": {"second_derivative": 6.0}},
                {"q": "What is f''(0) if f(x) = cos(2x)?", "outputs": {"second_derivative": -4.0}},
                {"q": "Find the second derivative of sqrt(x) at x = -4.", "outcome": "INVALID_INPUT"}],
     keywords=["second derivative", "concavity"])
rule("second_derivative_at_point",
     "<(?:second|2nd) >derivative of {f} " + _AT + "x = {a:n}",
     "f''({a:n}) " + _IFX + "f(x) = {f}",
     "y''({a:n}) " + _IFX + "y = {f}", order=20)

_ORD = {"third": 3, "fourth": 4, "fifth": 5, "sixth": 6, "seventh": 7, "eighth": 8}


def nth_derivative_at_point(f, n, a):
    n = int(n)
    if not 1 <= n <= 12:
        raise ValueError("the order n must be between 1 and 12")
    e = parse_expr(f)
    at(e, (X,), a)
    dn = sp.diff(e, X, n)
    return {"nth_derivative": at(dn, (X,), a), "nth_derivative_expression": show(dn)}


proc("nth_derivative_at_point", name="Higher derivative at a point", topic="derivatives",
     purpose="The n-th derivative f^(n)(a) of f(x) at x = a.",
     inputs=[T_("f", "the function f(x)"), I_("n", "the order of the derivative", 1, 12), N_("a", "the point x = a")],
     outputs=[O_("nth_derivative", "f^(n)(a)"), O_("nth_derivative_expression", "f^(n)(x)", "text")],
     primary="nth_derivative", conditions=["1 <= n <= 12", "f is n times differentiable at x = a"],
     method="exact symbolic differentiation n times, then evaluation at x = a",
     latex=r"f^{(n)}(a) = \frac{d^n f}{dx^n}\Big|_{x=a}",
     run=nth_derivative_at_point,
     verify=lambda i, o: rel_err(o["nth_derivative"], _mp_diff(i["f"], i["a"], int(i["n"])), 1e-9),
     reference=lambda f, n, a: {"nth_derivative": ref_diff(ref_lam(f), a, int(n))},
     wrong=lambda f, n, a: {"nth_derivative": nth_derivative_at_point(f, n, a)["nth_derivative"] / math.factorial(int(n))},
     random_inputs=lambda rng: {"f": rng.choice([rand_smooth(rng), rand_poly(rng, 5), f"sin({_c(rng, 1, 3)}*x)",
                                                 f"exp({_c(rng, -2, 2)}*x)*x"]),
                                "n": rng.randint(2, 6), "a": rand_point(rng, -2, 2)},
     templates=["Find the 4th derivative of sin(2x) at x = 1.", "Find the third derivative of x^5 at x = 2."],
     examples=[{"inputs": {"f": "sin(2x)", "n": 4, "a": 1}, "outputs": {"nth_derivative": 16 * math.sin(2)}},
               {"inputs": {"f": "x^5", "n": 3, "a": 2}, "outputs": {"nth_derivative": 240.0}},
               {"inputs": {"f": "exp(3x)", "n": 5, "a": 0}, "outputs": {"nth_derivative": 243.0}}],
     questions=[{"q": "Find the 4th derivative of sin(2x) at x = 1.", "outputs": {"nth_derivative": 16 * math.sin(2)}},
                {"q": "Find the third derivative of x^5 at x = 2.", "outputs": {"nth_derivative": 240.0}},
                {"q": "What is the 5th derivative of exp(3x) at x = 0?", "outputs": {"nth_derivative": 243.0}}],
     keywords=["third derivative", "higher derivative"])
rule("nth_derivative_at_point",
     "{n:i}<(?:st|nd|rd|th)> derivative of {f} " + _AT + "x = {a:n}",
     "{n:w} derivative of {f} " + _AT + "x = {a:n}",
     post=lambda d: (dict(d, n=_ORD[d["n"]]) if isinstance(d["n"], str) and d["n"] in _ORD else
                     (d if isinstance(d["n"], int) and d["n"] != 2 else None)), order=10)


def tangent_line(f, a):
    e = parse_expr(f)
    y0 = at(e, (X,), a)
    m = at(sp.diff(e, X), (X,), a)
    b = y0 - m * a
    return {"slope": m, "intercept": b, "y0": y0, "equation": f"y = {m:.12g}*x + {b:.12g}".replace("+ -", "- ")}


proc("tangent_line", name="Tangent line", topic="derivatives / tangent and normal lines",
     purpose="The tangent line y = m*x + b to y = f(x) at x = a: slope m = f'(a), intercept b = f(a) - a*f'(a).",
     inputs=[T_("f", "the function f(x)"), N_("a", "the point of tangency x = a")],
     outputs=[O_("slope", "slope m = f'(a)"), O_("intercept", "y-intercept b"), O_("y0", "f(a)"),
              O_("equation", "the line as y = m*x + b", "text")],
     primary="slope", conditions=["f is differentiable at x = a"],
     method="point-slope form through (a, f(a)) with the exact derivative", latex=r"y = f(a) + f'(a)\,(x - a)",
     run=tangent_line,
     verify=lambda i, o: worst(rel_err(o["slope"], _mp_diff(i["f"], i["a"]), 1e-12),
                               rel_err(o["intercept"] + o["slope"] * i["a"], at(parse_expr(i["f"]), (X,), i["a"]), 1e-12)),
     reference=lambda f, a: (lambda g: (lambda m: {"slope": m, "intercept": float(g(a)) - m * a})(ref_diff(g, a)))(ref_lam(f)),
     wrong=lambda f, a: (lambda r: dict(r, intercept=r["y0"] - r["slope"]))(tangent_line(f, a)),
     random_inputs=lambda rng: {"f": rand_smooth(rng), "a": rng.choice([-3.0, -2.0, -1.0, 2.0, 3.0, 0.5, 1.5])},
     templates=["Find the equation of the tangent line to y = x^2 + 1 at x = 2.",
                "Find the tangent line to f(x) = sin(x) at x = 0."],
     examples=[{"inputs": {"f": "x^2 + 1", "a": 2}, "outputs": {"slope": 4.0, "intercept": -3.0}},
               {"inputs": {"f": "sin(x)", "a": 0}, "outputs": {"slope": 1.0, "intercept": 0.0}},
               {"inputs": {"f": "exp(x)", "a": 1}, "outputs": {"slope": math.e, "intercept": 0.0}}],
     questions=[{"q": "Find the equation of the tangent line to y = x^2 + 1 at x = 2.", "outputs": {"slope": 4.0, "intercept": -3.0}},
                {"q": "Find the tangent line to f(x) = sin(x) at x = 0.", "outputs": {"slope": 1.0, "intercept": 0.0}},
                {"q": "What is the slope of the tangent line to y = x^3 at x = -1?", "outputs": {"slope": 3.0, "intercept": 2.0}}],
     keywords=["tangent line"])
rule("tangent_line",
     "<(?:the )?(?:equation of )?(?:the )?(?:slope of )?(?:the )?>tangent <(?:line )?>to <(?:the )?(?:curve |graph of |graph )?>{f} at <(?:the point where )?>x = {a:n}",
     "<(?:the )?(?:equation of )?(?:the )?>tangent <(?:line )?>of {f} at x = {a:n}", order=20)


def normal_line(f, a):
    t = tangent_line(f, a)
    if abs(t["slope"]) < 1e-12:
        raise ValueError("the tangent is horizontal there, so the normal line is vertical (x = a) and has no slope")
    m = -1.0 / t["slope"]
    return {"slope": m, "intercept": t["y0"] - m * a, "y0": t["y0"]}


proc("normal_line", name="Normal line", topic="derivatives / tangent and normal lines",
     purpose="The normal line (perpendicular to the tangent) to y = f(x) at x = a: slope -1/f'(a).",
     inputs=[T_("f", "the function f(x)"), N_("a", "the point x = a")],
     outputs=[O_("slope", "slope -1/f'(a)"), O_("intercept", "y-intercept"), O_("y0", "f(a)")],
     primary="slope", conditions=["f'(a) != 0 (otherwise the normal is vertical)"],
     method="perpendicular slope -1/m through (a, f(a))", latex=r"y = f(a) - \frac{1}{f'(a)}\,(x - a)",
     run=normal_line,
     verify=lambda i, o: worst(rel_err(o["slope"] * float(_mp_diff(i["f"], i["a"])), -1.0, 1e-11),
                               rel_err(o["intercept"] + o["slope"] * i["a"], at(parse_expr(i["f"]), (X,), i["a"]), 1e-12)),
     reference=lambda f, a: (lambda g: (lambda m: {"slope": m, "intercept": float(g(a)) - m * a})(-1 / ref_diff(g, a)))(ref_lam(f)),
     wrong=lambda f, a: (lambda r: {"slope": -r["slope"], "intercept": r["y0"] + r["slope"] * a})(normal_line(f, a)),
     random_inputs=lambda rng: {"f": rand_smooth(rng), "a": rng.choice([-3.0, -2.0, -1.0, 1.0, 2.0, 3.0, 0.5])},
     templates=["Find the normal line to y = x^2 at x = 1.", "Find the equation of the normal line to f(x) = exp(x) at x = 0."],
     examples=[{"inputs": {"f": "x^2", "a": 1}, "outputs": {"slope": -0.5, "intercept": 1.5}},
               {"inputs": {"f": "exp(x)", "a": 0}, "outputs": {"slope": -1.0, "intercept": 1.0}}],
     questions=[{"q": "Find the normal line to y = x^2 at x = 1.", "outputs": {"slope": -0.5, "intercept": 1.5}},
                {"q": "Find the equation of the normal line to f(x) = exp(x) at x = 0.", "outputs": {"slope": -1.0, "intercept": 1.0}},
                {"q": "Find the normal line to y = x^2 at x = 0.", "outcome": "INVALID_INPUT"}],
     keywords=["normal line"])
rule("normal_line",
     "<(?:the )?(?:equation of )?(?:the )?>normal <(?:line )?>to <(?:the )?(?:curve |graph of )?>{f} at <(?:the point where )?>x = {a:n}", order=20)


def average_rate_of_change(f, a, b):
    if a == b:
        raise ValueError("the two points must differ")
    e = parse_expr(f)
    fa, fb = at(e, (X,), a), at(e, (X,), b)
    return {"average_rate": (fb - fa) / (b - a), "fa": fa, "fb": fb}


proc("average_rate_of_change", name="Average rate of change", topic="derivatives / rates of change",
     purpose="The average rate of change (f(b) - f(a))/(b - a) of f over [a, b]: the slope of the secant line.",
     inputs=[T_("f", "the function f(x)"), N_("a", "start of the interval"), N_("b", "end of the interval")],
     outputs=[O_("average_rate", "(f(b) - f(a))/(b - a)"), O_("fa", "f(a)"), O_("fb", "f(b)")],
     primary="average_rate", conditions=["a != b", "f defined at a and b"],
     method="difference quotient of the two function values", latex=r"\frac{\Delta f}{\Delta x} = \frac{f(b)-f(a)}{b-a}",
     run=average_rate_of_change,
     verify=lambda i, o: rel_err(o["average_rate"] * (i["b"] - i["a"]), o["fb"] - o["fa"], 1e-13),
     reference=lambda f, a, b: (lambda d: {"average_rate": ref_quad(sp.lambdify(X, d, "math"), a, b) / (b - a)})(
         sp.diff(parse_expr(f), X)),
     wrong=lambda f, a, b: (lambda r: {"average_rate": -r["average_rate"]})(average_rate_of_change(f, a, b)),
     random_inputs=lambda rng: (lambda ab: {"f": rand_smooth(rng), "a": ab[0], "b": ab[1]})(rand_interval(rng)),
     templates=["Find the average rate of change of f(x) = x^2 from x = 1 to x = 3.",
                "What is the average rate of change of sin(x) on [0, pi/2]?"],
     examples=[{"inputs": {"f": "x^2", "a": 1, "b": 3}, "outputs": {"average_rate": 4.0}},
               {"inputs": {"f": "sin(x)", "a": 0, "b": math.pi / 2}, "outputs": {"average_rate": 2 / math.pi}}],
     questions=[{"q": "Find the average rate of change of f(x) = x^2 from x = 1 to x = 3.", "outputs": {"average_rate": 4.0}},
                {"q": "What is the average rate of change of sin(x) on [0, pi/2]?", "outputs": {"average_rate": 2 / math.pi}},
                {"q": "Find the slope of the secant line of x^3 between x = 0 and x = 2.", "outputs": {"average_rate": 4.0}},
                {"q": "Find the average rate of change of x^2 from x = 2 to x = 2.", "outcome": "INVALID_INPUT"}],
     keywords=["average rate of change", "secant"])
rule("average_rate_of_change",
     "average rate of change of {f} <(?:from|between)> x = {a:n} <(?:to|and)> x = {b:n}",
     "average rate of change of {f} <(?:on|over) \\[?>{a:n}, {b:n}<\\]?>",
     "slope of the secant <(?:line )?>of {f} <(?:from|between)> x = {a:n} <(?:to|and)> x = {b:n}")


def velocity_acceleration(s, t0):
    e = parse_expr(s, ("t",))
    at(e, (T,), t0)
    v, acc = sp.diff(e, T), sp.diff(e, T, 2)
    vel = at(v, (T,), t0)
    return {"velocity": vel, "acceleration": at(acc, (T,), t0), "speed": abs(vel), "position": at(e, (T,), t0)}


proc("velocity_acceleration", name="Velocity and acceleration from position", topic="derivatives / motion",
     purpose="Velocity v = s'(t) and acceleration a = s''(t) at time t0 from a position function s(t).",
     inputs=[T_("s", "the position s(t)"), N_("t0", "the time t0")],
     outputs=[O_("velocity", "v(t0) = s'(t0)"), O_("acceleration", "a(t0) = s''(t0)"), O_("speed", "|v(t0)|"),
              O_("position", "s(t0)")],
     primary="velocity", conditions=["s twice differentiable at t0"],
     method="exact differentiation of the position once and twice", latex=r"v = \frac{ds}{dt},\quad a = \frac{d^2 s}{dt^2}",
     run=velocity_acceleration,
     verify=lambda i, o: worst(rel_err(o["velocity"], _mp_diff(i["s"], i["t0"], 1, ("t",)), 1e-12),
                               rel_err(o["acceleration"], _mp_diff(i["s"], i["t0"], 2, ("t",)), 1e-11)),
     reference=lambda s, t0: (lambda g: {"velocity": ref_diff(g, t0), "acceleration": ref_diff(g, t0, 2)})(ref_lam(s, ("t",))),
     wrong=lambda s, t0: (lambda e: {"velocity": at(e, (T,), t0) / t0, "acceleration": at(e, (T,), t0) / t0 ** 2})(parse_expr(s, ("t",))),
     random_inputs=lambda rng: {"s": rand_smooth(rng, "t"), "t0": float(rng.randint(1, 5))},
     templates=["A particle has position s(t) = t^3 - 6t^2 + 9t. Find its velocity and acceleration at t = 2.",
                "Find the velocity of s(t) = 5t^2 + 2t at t = 3."],
     examples=[{"inputs": {"s": "t^3 - 6t^2 + 9t", "t0": 2}, "outputs": {"velocity": -3.0, "acceleration": 0.0}},
               {"inputs": {"s": "5t^2 + 2t", "t0": 3}, "outputs": {"velocity": 32.0, "acceleration": 10.0}}],
     questions=[{"q": "A particle has position s(t) = t^3 - 6t^2 + 9t. Find its velocity and acceleration at t = 2.",
                 "outputs": {"velocity": -3.0, "acceleration": 0.0}},
                {"q": "Find the velocity of s(t) = 5t^2 + 2t at t = 3.", "outputs": {"velocity": 32.0}},
                {"q": "The position of an object is s(t) = 4sin(t). Find its velocity at t = 0.", "outputs": {"velocity": 4.0}}],
     keywords=["velocity", "acceleration", "position"])
rule("velocity_acceleration",
     "<.*?>position<[a-z ,]*> s(t) = {f}<\\.?,? (?:find|what is|what are|compute|determine|give) (?:its |the )?>"
     "<(?:instantaneous )?>velocity <(?:and (?:its |the )?acceleration )?>at t = {t0:n}",
     "<(?:the )?(?:instantaneous )?>velocity <(?:and (?:the )?acceleration )?>of <(?:a particle with |an object with )?>"
     "<(?:position )?>s(t) = {f} at t = {t0:n}",
     post=lambda d: {"s": d["f"], "t0": d["t0"]})


def linear_approximation(f, a, x1):
    e = parse_expr(f)
    fa, m = at(e, (X,), a), at(sp.diff(e, X), (X,), a)
    approx = fa + m * (x1 - a)
    actual = at(e, (X,), x1)
    return {"approximation": approx, "actual": actual, "error": actual - approx, "slope": m}


proc("linear_approximation", name="Linear approximation", topic="derivatives / linearization",
     purpose="Estimate f(x1) by the tangent line at x = a: L(x1) = f(a) + f'(a)(x1 - a); with the actual value and error.",
     inputs=[T_("f", "the function f(x)"), N_("a", "the base point a"), N_("x1", "the point to estimate")],
     outputs=[O_("approximation", "L(x1)"), O_("actual", "f(x1)"), O_("error", "f(x1) - L(x1)"), O_("slope", "f'(a)")],
     primary="approximation", conditions=["f differentiable at a"],
     method="linearization (tangent line) at the base point", latex=r"L(x) = f(a) + f'(a)\,(x-a)",
     run=linear_approximation,
     verify=lambda i, o: worst(rel_err(o["slope"], _mp_diff(i["f"], i["a"]), 1e-12),
                               rel_err(o["approximation"], at(parse_expr(i["f"]), (X,), i["a"]) + o["slope"] * (i["x1"] - i["a"]), 1e-13)),
     reference=lambda f, a, x1: (lambda g: {"approximation": float(g(a)) + ref_diff(g, a) * (x1 - a), "actual": float(g(x1))})(ref_lam(f)),
     wrong=lambda f, a, x1: (lambda e: {"approximation": at(e, (X,), a) + at(sp.diff(e, X), (X,), x1) * (x1 - a),
                                        "actual": at(e, (X,), x1)})(parse_expr(f)),
     random_inputs=lambda rng: (lambda a: {"f": rand_smooth(rng), "a": a, "x1": a + rng.choice([-0.3, -0.1, 0.1, 0.2, 0.5])})(
         float(rng.randint(-2, 3))),
     templates=["Use the linear approximation of f(x) = sqrt(x) at x = 4 to estimate f(4.1).",
                "Find the linearization of exp(x) at x = 0 and evaluate it at x = 0.1."],
     examples=[{"inputs": {"f": "sqrt(x)", "a": 4, "x1": 4.1}, "outputs": {"approximation": 2.025}},
               {"inputs": {"f": "exp(x)", "a": 0, "x1": 0.1}, "outputs": {"approximation": 1.1, "actual": math.exp(0.1)}}],
     questions=[{"q": "Use the linear approximation of f(x) = sqrt(x) at x = 4 to estimate f(4.1).", "outputs": {"approximation": 2.025}},
                {"q": "Find the linearization of exp(x) at x = 0 and evaluate it at x = 0.1.", "outputs": {"approximation": 1.1}},
                {"q": "Use the tangent line approximation of x^3 at x = 2 to estimate f(2.1).", "outputs": {"approximation": 9.2}}],
     keywords=["linear approximation", "linearization"])
rule("linear_approximation",
     "<(?:use )?(?:the )?>linear <(?:approximation|approximations|linearization)> <(?:of|to)> {f} at x = {a:n} to <(?:estimate|approximate|find)> f({x1:n})",
     "<(?:use )?(?:the )?>tangent line <(?:approximation|approximations)> <(?:of|to)> {f} at x = {a:n} to <(?:estimate|approximate|find)> f({x1:n})",
     "linearization of {f} at x = {a:n} and evaluate it at x = {x1:n}",
     "linearization of {f} at x = {a:n}, evaluated at x = {x1:n}")


def curvature(f, a):
    e = parse_expr(f)
    at(e, (X,), a)
    d1, d2 = at(sp.diff(e, X), (X,), a), at(sp.diff(e, X, 2), (X,), a)
    k = abs(d2) / (1 + d1 * d1) ** 1.5
    out = {"curvature": k}
    if k > 0:
        out["radius_of_curvature"] = 1 / k
    return out


proc("curvature", name="Curvature of a graph", topic="derivatives / curvature",
     purpose="The curvature kappa = |f''| / (1 + f'^2)^(3/2) of y = f(x) at x = a, and the radius of curvature 1/kappa.",
     inputs=[T_("f", "the function f(x)"), N_("a", "the point x = a")],
     outputs=[O_("curvature", "kappa at x = a"), O_("radius_of_curvature", "1/kappa (absent where kappa = 0)")],
     primary="curvature", conditions=["f twice differentiable at a"],
     method="curvature formula of a graph from the exact first and second derivatives",
     latex=r"\kappa = \frac{|f''(a)|}{\left(1 + f'(a)^2\right)^{3/2}}",
     run=curvature,
     verify=lambda i, o: rel_err(o["curvature"], abs(_mp_diff(i["f"], i["a"], 2)) / (1 + _mp_diff(i["f"], i["a"]) ** 2) ** 1.5, 1e-11),
     reference=lambda f, a: (lambda g: (lambda d1, d2: {"curvature": abs(d2) / (1 + d1 * d1) ** 1.5})(ref_diff(g, a), ref_diff(g, a, 2)))(ref_lam(f)),
     wrong=lambda f, a: {"curvature": abs(at(sp.diff(parse_expr(f), X, 2), (X,), a))},
     random_inputs=lambda rng: {"f": rand_smooth(rng), "a": rand_point(rng)},
     templates=["Find the curvature of y = x^2 at x = 0.", "What is the curvature of f(x) = sin(x) at x = pi/2?"],
     examples=[{"inputs": {"f": "x^2", "a": 0}, "outputs": {"curvature": 2.0, "radius_of_curvature": 0.5}},
               {"inputs": {"f": "sin(x)", "a": math.pi / 2}, "outputs": {"curvature": 1.0}},
               {"inputs": {"f": "x^2", "a": 1}, "outputs": {"curvature": 2 / 5 ** 1.5}}],
     questions=[{"q": "Find the curvature of y = x^2 at x = 0.", "outputs": {"curvature": 2.0}},
                {"q": "What is the curvature of f(x) = sin(x) at x = pi/2?", "outputs": {"curvature": 1.0}},
                {"q": "Find the radius of curvature of y = x^2 at x = 0.", "outputs": {"radius_of_curvature": 0.5}}],
     keywords=["curvature", "radius of curvature"])
rule("curvature",
     "<(?:radius of )?>curvature of <(?:the )?(?:curve |graph of |graph )?>{f} at x = {a:n}", order=30)


# =========================================================================================== 2. integrals
_FROM = "<(?:from|between)> <(?:x = )?>{a:n} <(?:to|and)> <(?:x = )?>{b:n}"
_ON = "<(?:on|over) (?:the interval )?\\[?>{a:n}, {b:n}<\\]?>"


def _gl(F, a, b, panels=8, dps=20):
    """Gauss-Legendre quadrature of an mp callable on equal panels (the verifiers' independent rule)."""
    with mp.workdps(dps):
        knots = mp.linspace(mp.mpf(a), mp.mpf(b), panels + 1)
        return mp.quad(F, knots, method="gauss-legendre")


def _gl_check(text, a, b, names=("x",), transform=None):
    """Verifier route: Gauss-Legendre quadrature of the parsed integrand (or of transform(f) if given)."""
    f = _compile(parse_expr(text, names), names)
    g = (lambda x: transform(f, x)) if transform else f
    return _gl(g, a, b)


def definite_integral(f, a, b):
    e = parse_expr(f)
    if a == b:
        return {"integral": 0.0}
    continuous_on(e, X, a, b)
    return {"integral": quad(_compile(e, (X,)), a, b)}


proc("definite_integral", name="Definite integral", topic="integrals",
     purpose="The definite integral of f(x) from x = a to x = b (net signed area).",
     inputs=[T_("f", "the integrand f(x)"), N_("a", "lower limit"), N_("b", "upper limit")],
     outputs=[O_("integral", "the value of the integral")],
     primary="integral", conditions=["f real and continuous on the closed interval between a and b"],
     method="high-precision tanh-sinh quadrature (30 digits) after an exact continuity check on [a, b]",
     latex=r"\int_a^b f(x)\,dx",
     run=definite_integral,
     verify=lambda i, o: rel_err(o["integral"], _gl_check(i["f"], i["a"], i["b"]), 1e-10),
     reference=lambda f, a, b: {"integral": ref_quad(ref_lam(f, module="math"), a, b)},
     wrong=lambda f, a, b: {"integral": -definite_integral(f, a, b)["integral"]},
     random_inputs=lambda rng: (lambda ab: {"f": rand_smooth(rng), "a": ab[0], "b": ab[1]})(rand_interval(rng)),
     templates=["Evaluate the integral of x^2 from 0 to 3.", "Find the definite integral of sin(x) from 0 to pi.",
                "Integrate exp(2x) from x = 0 to x = 1."],
     examples=[{"inputs": {"f": "x^2", "a": 0, "b": 3}, "outputs": {"integral": 9.0}},
               {"inputs": {"f": "sin(x)", "a": 0, "b": math.pi}, "outputs": {"integral": 2.0}},
               {"inputs": {"f": "exp(2x)", "a": 0, "b": 1}, "outputs": {"integral": (math.e ** 2 - 1) / 2}},
               {"inputs": {"f": "1/x", "a": 1, "b": math.e}, "outputs": {"integral": 1.0}}],
     questions=[{"q": "Evaluate the integral of x^2 from 0 to 3.", "outputs": {"integral": 9.0}},
                {"q": "Find the definite integral of sin(x) from 0 to pi.", "outputs": {"integral": 2.0}},
                {"q": "Integrate exp(2x) from x = 0 to x = 1.", "outputs": {"integral": (math.e ** 2 - 1) / 2}},
                {"q": "Compute the integral of 3x^2 + 2x dx from 1 to 2.", "outputs": {"integral": 10.0}},
                {"q": "Evaluate the integral of 1/x from -1 to 1.", "outcome": "INVALID_INPUT"}],
     keywords=["integral", "integrate"])
rule("definite_integral",
     "<(?:definite )?>integral of {f}<(?: dx)?> " + _FROM,
     "<(?:definite )?>integral of {f}<(?: dx)?> " + _ON,
     "integrate {f}<(?: dx)?> " + _FROM,
     "integrate {f}<(?: dx)?> " + _ON, order=60,
     post=lambda d: None if re.search(r"\bdy\b|\bdt\b|\bdx\s*dy|\bdy\s*dx", d["f"]) else d)


def average_value(f, a, b):
    if not a < b:
        raise ValueError("the interval needs a < b")
    i = definite_integral(f, a, b)["integral"]
    return {"average_value": i / (b - a), "integral": i}


proc("average_value", name="Average value of a function", topic="integrals / applications",
     purpose="The average value (1/(b - a)) * integral of f from a to b of a function on [a, b].",
     inputs=[T_("f", "the function f(x)"), N_("a", "start of the interval"), N_("b", "end of the interval")],
     outputs=[O_("average_value", "the average value of f on [a, b]"), O_("integral", "the integral of f over [a, b]")],
     primary="average_value", conditions=["a < b", "f continuous on [a, b]"],
     method="definite integral by high-precision quadrature, divided by the interval length",
     latex=r"f_{\text{avg}} = \frac{1}{b-a}\int_a^b f(x)\,dx",
     run=average_value,
     verify=lambda i, o: rel_err(o["average_value"], _gl_check(i["f"], i["a"], i["b"]) / (i["b"] - i["a"]), 1e-10),
     reference=lambda f, a, b: {"average_value": ref_quad(ref_lam(f, module="math"), a, b) / (b - a)},
     wrong=lambda f, a, b: (lambda r: {"average_value": r["integral"], "integral": r["integral"]})(average_value(f, a, b)),
     random_inputs=lambda rng: (lambda ab: {"f": rand_smooth(rng), "a": ab[0], "b": ab[0] + rng.choice([2, 3, 4, 0.5])})(
         rand_interval(rng)),
     templates=["Find the average value of f(x) = x^2 on [0, 3].", "What is the average value of sin(x) from 0 to pi?"],
     examples=[{"inputs": {"f": "x^2", "a": 0, "b": 3}, "outputs": {"average_value": 3.0}},
               {"inputs": {"f": "sin(x)", "a": 0, "b": math.pi}, "outputs": {"average_value": 2 / math.pi}}],
     questions=[{"q": "Find the average value of f(x) = x^2 on [0, 3].", "outputs": {"average_value": 3.0}},
                {"q": "What is the average value of sin(x) from 0 to pi?", "outputs": {"average_value": 2 / math.pi}},
                {"q": "Find the average value of x^2 on [3, 0].", "outcome": "INVALID_INPUT"}],
     keywords=["average value"])
rule("average_value", "average value of {f} " + _ON, "average value of {f} " + _FROM, order=40)


def _nonneg_on(p, a, b):
    """True if the polynomial p >= 0 on [a, b] (exact: signs at the ends and between consecutive real roots)."""
    pts = [sp.Rational(repr(float(a)))] + real_roots_in(p, a, b) + [sp.Rational(repr(float(b)))]
    tests = [pts[0], pts[-1]] + [(pts[k] + pts[k + 1]) / 2 for k in range(len(pts) - 1)]
    return all(p.eval(sp.nsimplify(t)) >= 0 if t.is_Rational else float(sp.N(p.as_expr().subs(p.gen, t), 30)) >= 0
               for t in tests)


def _poly_abs_integral(h, a, b):
    """Exact integral of |h| over [a, b] for a polynomial h: split at its real roots."""
    H = h.integrate()
    pts = [sp.Rational(repr(float(a)))] + real_roots_in(h, a, b) + [sp.Rational(repr(float(b)))]
    tot = 0
    for k in range(len(pts) - 1):
        tot += sp.Abs(H.as_expr().subs(h.gen, pts[k + 1]) - H.as_expr().subs(h.gen, pts[k]))
    return float(sp.N(tot, 30)), [float(sp.N(r, 30)) for r in pts[1:-1]]


def _verify_abs_area(f, g, a, b):
    """Integral of |f - g| by Gauss-Legendre between the real roots that mpmath's polyroots finds (verifier)."""
    co = [mp.mpf(c.p) / c.q for c in (poly_of(f) - poly_of(g)).all_coeffs()]
    try:
        with mp.workdps(30):
            rs = mp.polyroots(co, maxsteps=400, extraprec=120) if len(co) > 1 else []
        cuts = sorted(float(mp.re(r)) for r in rs if abs(mp.im(r)) < 1e-20 and a < float(mp.re(r)) < b)
    except Exception:                                  # polyroots did not converge: exact real roots instead
        cuts = [float(r) for r in real_roots_in(poly_of(f) - poly_of(g), a, b)]
    knots = [a] + cuts + [b]
    h = f"({f}) - ({g})"
    return sum(abs(_gl_check(h, lo, hi)) for lo, hi in zip(knots[:-1], knots[1:]))


def area_between_curves(f, g, a, b):
    if not a < b:
        raise ValueError("the interval needs a < b")
    h = poly_of(f) - poly_of(g)
    area, cross = _poly_abs_integral(h, a, b)
    net = float(sp.N(h.integrate().as_expr().subs(X, sp.Rational(repr(float(b)))) -
                     h.integrate().as_expr().subs(X, sp.Rational(repr(float(a)))), 30))
    return {"area": area, "net_integral": net, "crossings": cross}


def _ref_abs_area(f, g, a, b):
    import numpy as np
    hf = ref_lam(f"({f}) - ({g})", module="math")
    co = [float(c) for c in (poly_of(f) - poly_of(g)).all_coeffs()]
    pts = sorted(r.real for r in np.roots(co) if abs(r.imag) < 1e-9 and a < r.real < b) if len(co) > 1 else []
    return ref_quad(lambda x: abs(hf(x)), a, b, points=pts or None)


proc("area_between_curves", name="Area between two curves", topic="integrals / area",
     purpose="The area between the graphs of two polynomials y = f(x) and y = g(x) over [a, b], counting every piece "
             "as positive (the curves may cross).",
     inputs=[T_("f", "the first curve f(x) (a polynomial)"), T_("g", "the second curve g(x) (a polynomial)"),
             N_("a", "left end"), N_("b", "right end")],
     outputs=[O_("area", "the area between the curves"), O_("net_integral", "integral of f - g (signed)"),
              O_("crossings", "x where the curves cross inside (a, b)", "vector")],
     primary="area", conditions=["a < b", "f and g polynomials with rational coefficients"],
     method="exact integration of |f - g|, split at the exact real roots of f - g",
     latex=r"A = \int_a^b \left|f(x) - g(x)\right|\,dx",
     run=area_between_curves,
     verify=lambda i, o: rel_err(o["area"], _verify_abs_area(i["f"], i["g"], i["a"], i["b"]), 1e-10),
     reference=lambda f, g, a, b: {"area": _ref_abs_area(f, g, a, b)},
     wrong=lambda f, g, a, b: (lambda r: {"area": abs(r["net_integral"]), "net_integral": r["net_integral"],
                                          "crossings": []})(area_between_curves(f, g, a, b)),
     random_inputs=lambda rng: _rand_crossing(rng),
     templates=["Find the area between y = x^2 and y = x from x = 0 to x = 2.",
                "Find the area between f(x) = 4 - x^2 and g(x) = x + 2 on [-2, 1]."],
     examples=[{"inputs": {"f": "x^2", "g": "x", "a": 0, "b": 2}, "outputs": {"area": 1.0}},
               {"inputs": {"f": "4 - x^2", "g": "x + 2", "a": -2, "b": 1}, "outputs": {"area": 4.5}},
               {"inputs": {"f": "x^3", "g": "0", "a": -1, "b": 1}, "outputs": {"area": 0.5, "net_integral": 0.0}}],
     questions=[{"q": "Find the area between y = x^2 and y = x from x = 0 to x = 2.", "outputs": {"area": 1.0}},
                {"q": "Find the area between f(x) = 4 - x^2 and g(x) = x + 2 on [-2, 1].", "outputs": {"area": 4.5}},
                {"q": "Find the area between y = sin(x) and y = 0 from x = 0 to x = pi.", "outcome": "INVALID_INPUT"}],
     keywords=["area between"])
rule("area_between_curves",
     "area between <(?:the curves |the graphs (?:of )?)?>{f} and {g} " + _FROM,
     "area between <(?:the curves |the graphs (?:of )?)?>{f} and {g} " + _ON, order=30,
     post=lambda d: dict(d, f=_expr_text(d["f"]), g=_expr_text(d["g"])))


def _rand_crossing(rng):
    r1, r2 = sorted(rng.sample(range(-3, 4), 2))
    k = rng.choice([-3, -2, -1, 1, 2, 3])
    g = rand_poly(rng, rng.choice([0, 1, 2]))
    h = sp.expand(k * (X - r1) * (X - r2) * (rng.choice([1, X - rng.randint(-3, 3)])))
    f = show(sp.expand(parse_expr(g) + h))
    a = r1 - rng.choice([0, 0.5, 1])
    b = r2 + rng.choice([0, 0.5, 1, 2]) if rng.random() < 0.8 else r1 + 0.5
    return {"f": f, "g": g, "a": float(a), "b": float(b)}


def area_enclosed_curves(f, g):
    h = poly_of(f) - poly_of(g)
    if h.is_zero:
        raise ValueError("the curves coincide")
    roots = sorted(set(sp.real_roots(h)), key=lambda r: float(r))
    if len(roots) < 2:
        raise ValueError("the curves meet in fewer than two points, so they enclose no bounded region")
    H = h.integrate().as_expr()
    tot = sum(sp.Abs(H.subs(X, roots[k + 1]) - H.subs(X, roots[k])) for k in range(len(roots) - 1))
    return {"area": float(sp.N(tot, 30)), "x_left": float(sp.N(roots[0], 30)), "x_right": float(sp.N(roots[-1], 30)),
            "intersections": [float(sp.N(r, 30)) for r in roots]}


def _ref_enclosed(f, g):
    import numpy as np
    co = [float(c) for c in (poly_of(f) - poly_of(g)).all_coeffs()]
    rs = sorted(r.real for r in np.roots(co) if abs(r.imag) < 1e-7)
    hf = ref_lam(f"({f}) - ({g})", module="math")
    lo, hi = rs[0], rs[-1]
    return {"area": ref_quad(lambda x: abs(hf(x)), lo, hi, points=rs[1:-1] or None), "x_left": lo, "x_right": hi}


proc("area_enclosed_curves", name="Area enclosed by two curves", topic="integrals / area",
     purpose="The area of the region enclosed by two polynomial curves y = f(x) and y = g(x), between their first and "
             "last intersection.",
     inputs=[T_("f", "the first curve f(x) (a polynomial)"), T_("g", "the second curve g(x) (a polynomial)")],
     outputs=[O_("area", "the enclosed area"), O_("x_left", "first intersection"), O_("x_right", "last intersection"),
              O_("intersections", "all intersection x-values", "vector")],
     primary="area", conditions=["f and g polynomials with rational coefficients meeting in at least two points"],
     method="exact real roots of f - g, then exact integration of |f - g| between consecutive roots",
     latex=r"A = \int_{x_1}^{x_n} \left|f(x) - g(x)\right|\,dx",
     run=area_enclosed_curves,
     verify=lambda i, o: worst(
         rel_err(o["area"], sum(abs(_gl_check(f"({i['f']}) - ({i['g']})", lo, hi))
                                for lo, hi in zip(o["intersections"][:-1], o["intersections"][1:])), 1e-10),
         max(abs(at(parse_expr(f"({i['f']}) - ({i['g']})"), (X,), r)) for r in o["intersections"]) / 1e3),
     reference=lambda f, g: _ref_enclosed(f, g),
     wrong=lambda f, g: (lambda r: dict(r, area=definite_integral(f, r["x_left"], r["x_right"])["integral"]))(area_enclosed_curves(f, g)),
     random_inputs=lambda rng: (lambda rs, k, g: {"f": show(sp.expand(parse_expr(g) + k * sp.prod([X - r for r in rs]))), "g": g})(
         sorted(rng.sample(range(-3, 4), rng.choice([2, 2, 3]))), rng.choice([-2, -1, 1, 2]), rand_poly(rng, rng.choice([0, 1, 2]))),
     templates=["Find the area enclosed by y = x^2 and y = x.", "Find the area of the region bounded by y = 4 - x^2 and y = 0."],
     examples=[{"inputs": {"f": "x^2", "g": "x"}, "outputs": {"area": 1 / 6, "x_left": 0.0, "x_right": 1.0}},
               {"inputs": {"f": "4 - x^2", "g": "0"}, "outputs": {"area": 32 / 3}},
               {"inputs": {"f": "x^3", "g": "x"}, "outputs": {"area": 0.5}}],
     questions=[{"q": "Find the area enclosed by y = x^2 and y = x.", "outputs": {"area": 1 / 6}},
                {"q": "Find the area of the region bounded by y = 4 - x^2 and y = 0.", "outputs": {"area": 32 / 3}},
                {"q": "Find the area enclosed by the curves y = x^3 and y = x.", "outputs": {"area": 0.5}},
                {"q": "Find the area enclosed by y = x^2 + 1 and y = 0.", "outcome": "INVALID_INPUT"}],
     keywords=["area enclosed", "region bounded"])
rule("area_enclosed_curves",
     "area <(?:enclosed|bounded) by (?:the curves |the graphs (?:of )?)?>{f} and {g}",
     "area of the region <(?:enclosed|bounded) by (?:the curves |the graphs (?:of )?)?>{f} and {g}", order=35,
     post=lambda d: None if re.search(r"\b(from|between|on|over)\b", d["g"]) else dict(d, f=_expr_text(d["f"]), g=_expr_text(d["g"])))


def disk_volume(f, a, b):
    if not a < b:
        raise ValueError("the interval needs a < b")
    e = parse_expr(f)
    continuous_on(e, X, a, b)
    return {"volume": float(mp.pi) * quad(_compile(e ** 2, (X,)), a, b)}


proc("disk_volume", name="Volume of revolution (disk method)", topic="integrals / volumes of revolution",
     purpose="The volume of the solid formed by rotating y = f(x), a <= x <= b, about the x-axis.",
     inputs=[T_("f", "the curve f(x)"), N_("a", "left end"), N_("b", "right end")],
     outputs=[O_("volume", "the volume of the solid")],
     primary="volume", conditions=["a < b", "f continuous on [a, b]"],
     method="disk method: pi times the integral of f^2, by high-precision quadrature", latex=r"V = \pi\int_a^b f(x)^2\,dx",
     run=disk_volume,
     verify=lambda i, o: rel_err(o["volume"], mp.pi * _gl_check(i["f"], i["a"], i["b"], transform=lambda F, x: F(x) ** 2), 1e-10),
     reference=lambda f, a, b: (lambda g: {"volume": math.pi * ref_quad(lambda x: g(x) ** 2, a, b)})(ref_lam(f, module="math")),
     wrong=lambda f, a, b: {"volume": math.pi * definite_integral(f, a, b)["integral"]},
     random_inputs=lambda rng: (lambda ab: {"f": rand_smooth(rng), "a": ab[0], "b": ab[1]})(rand_interval(rng)),
     templates=["Find the volume when y = x^2 from x = 0 to x = 1 is rotated about the x-axis.",
                "Find the volume of revolution of y = sqrt(x) about the x-axis on [0, 4]."],
     examples=[{"inputs": {"f": "x^2", "a": 0, "b": 1}, "outputs": {"volume": math.pi / 5}},
               {"inputs": {"f": "sqrt(x)", "a": 0, "b": 4}, "outputs": {"volume": 8 * math.pi}},
               {"inputs": {"f": "sqrt(4 - x^2)", "a": -2, "b": 2}, "outputs": {"volume": 32 * math.pi / 3}}],
     questions=[{"q": "Find the volume when y = x^2 from x = 0 to x = 1 is rotated about the x-axis.", "outputs": {"volume": math.pi / 5}},
                {"q": "Find the volume of revolution of y = sqrt(x) about the x-axis on [0, 4].", "outputs": {"volume": 8 * math.pi}},
                {"q": "Find the volume of the solid obtained by rotating y = sqrt(4 - x^2) about the x-axis from x = -2 to x = 2.",
                 "outputs": {"volume": 32 * math.pi / 3}}],
     keywords=["volume of revolution", "disk method"])
rule("disk_volume",
     "volume <(?:of the solid )?(?:obtained |formed |generated )?>when {f} " + _FROM + " is <(?:rotated|revolved)> about the x-axis",
     "volume of <(?:the solid of )?>revolution of {f} about the x-axis " + _ON,
     "volume of <(?:the solid of )?>revolution of {f} about the x-axis " + _FROM,
     "volume of the solid <(?:obtained|formed|generated)> by <(?:rotating|revolving)> {f} about the x-axis " + _FROM,
     "volume of the solid <(?:obtained|formed|generated)> by <(?:rotating|revolving)> {f} about the x-axis " + _ON,
     "volume <(?:by|using) the disk method> <(?:of|for)> {f} " + _FROM, order=40)


def washer_volume(f, g, a, b):
    if not a < b:
        raise ValueError("the interval needs a < b")
    pf, pg = poly_of(f), poly_of(g)
    if not _nonneg_on(pg, a, b) or not _nonneg_on(pf - pg, a, b):
        raise ValueError("the washer method needs outer radius f >= inner radius g >= 0 on [a, b]")
    W = (pf ** 2 - pg ** 2).integrate().as_expr()
    A, B = sp.Rational(repr(float(a))), sp.Rational(repr(float(b)))
    return {"volume": float(sp.N(sp.pi * (W.subs(X, B) - W.subs(X, A)), 30))}


proc("washer_volume", name="Volume of revolution (washer method)", topic="integrals / volumes of revolution",
     purpose="The volume swept when the region between an outer curve y = f(x) and an inner curve y = g(x) (polynomials, "
             "f >= g >= 0) on [a, b] is rotated about the x-axis.",
     inputs=[T_("f", "outer radius f(x) (a polynomial)"), T_("g", "inner radius g(x) (a polynomial)"),
             N_("a", "left end"), N_("b", "right end")],
     outputs=[O_("volume", "the volume of the solid")],
     primary="volume", conditions=["a < b", "f >= g >= 0 on [a, b] (checked exactly)", "polynomials"],
     method="washer method, integrated exactly", latex=r"V = \pi\int_a^b \left(f(x)^2 - g(x)^2\right)dx",
     run=washer_volume,
     verify=lambda i, o: rel_err(o["volume"], mp.pi * _gl_check(f"({i['f']})^2 - ({i['g']})^2", i["a"], i["b"]), 1e-10),
     reference=lambda f, g, a, b: (lambda F, G: {"volume": math.pi * ref_quad(lambda x: F(x) ** 2 - G(x) ** 2, a, b)})(
         ref_lam(f, module="math"), ref_lam(g, module="math")),
     wrong=lambda f, g, a, b: {"volume": float(mp.pi * _gl_check(f"(({f}) - ({g}))^2", a, b))},
     random_inputs=lambda rng: (lambda c, k, a: {"f": f"{c + k} + x^2" if rng.random() < 0.5 else f"{c + k + 4} - x^2",
                                                 "g": f"{c}" if rng.random() < 0.5 else f"{c} + x/2", "a": a, "b": a + rng.choice([0.5, 1, 1.5])})(
         rng.randint(1, 3), rng.randint(1, 3), float(rng.choice([0, 0.5, 1]))),
     templates=["Find the volume when the region between y = x + 2 and y = 1 on [0, 1] is rotated about the x-axis using washers.",
                "Use the washer method for outer radius 4 - x^2 and inner radius 1 on [0, 1]."],
     examples=[{"inputs": {"f": "x + 2", "g": "1", "a": 0, "b": 1}, "outputs": {"volume": 16 * math.pi / 3}},
               {"inputs": {"f": "4 - x^2", "g": "1", "a": 0, "b": 1}, "outputs": {"volume": math.pi * (16 - 8 / 3 + 1 / 5 - 1)}}],
     questions=[{"q": "Find the volume when the region between y = x + 2 and y = 1 on [0, 1] is rotated about the x-axis using washers.",
                 "outputs": {"volume": 16 * math.pi / 3}},
                {"q": "Use the washer method for outer radius 4 - x^2 and inner radius 1 on [0, 1].",
                 "outputs": {"volume": math.pi * (16 - 8 / 3 + 1 / 5 - 1)}},
                {"q": "Use the washer method for outer radius 1 and inner radius x^2 + 2 on [0, 1].", "outcome": "INVALID_INPUT"}],
     keywords=["washer method"])
rule("washer_volume",
     "volume when the region between {f} and {g} " + _ON + " is <(?:rotated|revolved)> about the x-axis using <(?:washers|the washer method)>",
     "volume when the region between {f} and {g} " + _FROM + " is <(?:rotated|revolved)> about the x-axis using <(?:washers|the washer method)>",
     "<(?:use )?(?:the )?>washer method <(?:for|with)> outer radius {f} and inner radius {g} " + _ON,
     "<(?:use )?(?:the )?>washer method <(?:for|with)> outer radius {f} and inner radius {g} " + _FROM, order=30,
     post=lambda d: dict(d, f=_expr_text(d["f"]), g=_expr_text(d["g"])))


def shell_volume(f, a, b):
    if not 0 <= a < b:
        raise ValueError("the shell method about the y-axis needs 0 <= a < b")
    p = poly_of(f)
    if not _nonneg_on(p, a, b):
        raise ValueError("the shell method needs f(x) >= 0 on [a, b]")
    W = (sp.Poly(X, X) * p).integrate().as_expr()
    A, B = sp.Rational(repr(float(a))), sp.Rational(repr(float(b)))
    return {"volume": float(sp.N(2 * sp.pi * (W.subs(X, B) - W.subs(X, A)), 30))}


proc("shell_volume", name="Volume of revolution (shell method)", topic="integrals / volumes of revolution",
     purpose="The volume swept when the region under y = f(x) >= 0 (a polynomial), a <= x <= b with a >= 0, is "
             "rotated about the y-axis.",
     inputs=[T_("f", "the curve f(x) (a polynomial)"), N_("a", "left end (>= 0)"), N_("b", "right end")],
     outputs=[O_("volume", "the volume of the solid")],
     primary="volume", conditions=["0 <= a < b", "f(x) >= 0 on [a, b] (checked exactly)"],
     method="cylindrical shells, integrated exactly", latex=r"V = 2\pi\int_a^b x\,f(x)\,dx",
     run=shell_volume,
     verify=lambda i, o: rel_err(o["volume"], 2 * mp.pi * _gl_check(f"x*({i['f']})", i["a"], i["b"]), 1e-10),
     reference=lambda f, a, b: (lambda F: {"volume": 2 * math.pi * ref_quad(lambda x: x * F(x), a, b)})(ref_lam(f, module="math")),
     wrong=lambda f, a, b: {"volume": float(2 * mp.pi * _gl_check(f, a, b))},
     random_inputs=lambda rng: (lambda a: {"f": rng.choice([f"{rng.randint(1, 5)} + x^2", f"{rng.randint(5, 9)} - x^2",
                                                            f"x^3 + {rng.randint(1, 3)}", f"{rng.randint(2, 4)}*x"]),
                                           "a": a, "b": a + rng.choice([1, 1.5, 2])})(float(rng.choice([0, 0.5, 1]))),
     templates=["Use the shell method to find the volume when y = x^2 on [0, 2] is rotated about the y-axis.",
                "Find the volume of the solid formed by rotating the region under y = 4 - x^2 from x = 0 to x = 2 about the y-axis."],
     examples=[{"inputs": {"f": "x^2", "a": 0, "b": 2}, "outputs": {"volume": 8 * math.pi}},
               {"inputs": {"f": "4 - x^2", "a": 0, "b": 2}, "outputs": {"volume": 8 * math.pi}}],
     questions=[{"q": "Use the shell method to find the volume when y = x^2 on [0, 2] is rotated about the y-axis.", "outputs": {"volume": 8 * math.pi}},
                {"q": "Find the volume of the solid formed by rotating the region under y = 4 - x^2 from x = 0 to x = 2 about the y-axis.",
                 "outputs": {"volume": 8 * math.pi}},
                {"q": "Use the shell method to find the volume when y = x - 3 on [0, 2] is rotated about the y-axis.",
                 "outcome": "INVALID_INPUT"}],
     keywords=["shell method", "about the y-axis"])
rule("shell_volume",
     "<(?:(?:use )?(?:the )?(?:shell method|cylindrical shells|shells) to )?>volume when {f} " + _ON + " is <(?:rotated|revolved)> about the y-axis",
     "<(?:(?:use )?(?:the )?(?:shell method|cylindrical shells|shells) to )?>volume when {f} " + _FROM + " is <(?:rotated|revolved)> about the y-axis",
     "volume of the solid <(?:obtained|formed|generated)> by <(?:rotating|revolving)> the region under {f} " + _FROM + " about the y-axis",
     "volume of the solid <(?:obtained|formed|generated)> by <(?:rotating|revolving)> the region under {f} " + _ON + " about the y-axis",
     order=30)


def arc_length(f, a, b):
    if not a < b:
        raise ValueError("the interval needs a < b")
    e = parse_expr(f)
    d = sp.diff(e, X)
    continuous_on(d, X, a, b)
    return {"arc_length": quad(_compile(sp.sqrt(1 + d ** 2), (X,)), a, b)}


def _ref_arc(f, a, b):
    d = ref_cdiff(f)
    return ref_quad(lambda x: math.sqrt(1 + d(x) ** 2), a, b)


proc("arc_length", name="Arc length of a graph", topic="integrals / arc length",
     purpose="The length of the curve y = f(x) from x = a to x = b.",
     inputs=[T_("f", "the curve f(x)"), N_("a", "left end"), N_("b", "right end")],
     outputs=[O_("arc_length", "the length of the curve")],
     primary="arc_length", conditions=["a < b", "f' continuous on [a, b]"],
     method="arc-length integral of sqrt(1 + f'(x)^2) with the exact derivative, by high-precision quadrature",
     latex=r"L = \int_a^b \sqrt{1 + f'(x)^2}\,dx",
     run=arc_length,
     verify=lambda i, o: rel_err(o["arc_length"], _gl(_compile(sp.sqrt(1 + sp.diff(parse_expr(i["f"]), X) ** 2), (X,)), i["a"], i["b"]), 1e-10),
     reference=lambda f, a, b: {"arc_length": _ref_arc(f, a, b)},
     wrong=lambda f, a, b: {"arc_length": quad(_compile(1 + sp.diff(parse_expr(f), X) ** 2, (X,)), a, b)},
     random_inputs=lambda rng: (lambda ab: {"f": rand_smooth(rng), "a": ab[0], "b": ab[1]})(rand_interval(rng, -2, 2)),
     templates=["Find the arc length of y = x^2 from x = 0 to x = 1.", "What is the length of the curve y = cosh(x) on [0, 1]?"],
     examples=[{"inputs": {"f": "cosh(x)", "a": 0, "b": 1}, "outputs": {"arc_length": math.sinh(1)}},
               {"inputs": {"f": "2x + 1", "a": 0, "b": 3}, "outputs": {"arc_length": 3 * math.sqrt(5)}},
               {"inputs": {"f": "x^2", "a": 0, "b": 1}, "outputs": {"arc_length": math.sqrt(5) / 2 + math.asinh(2) / 4}}],
     questions=[{"q": "Find the arc length of y = x^2 from x = 0 to x = 1.", "outputs": {"arc_length": math.sqrt(5) / 2 + math.asinh(2) / 4}},
                {"q": "What is the length of the curve y = cosh(x) on [0, 1]?", "outputs": {"arc_length": math.sinh(1)}},
                {"q": "Find the arc length of y = sqrt(x) from x = -1 to x = 1.", "outcome": "INVALID_INPUT"}],
     keywords=["arc length", "length of the curve"])
rule("arc_length",
     "arc length of <(?:the curve |the graph of )?>{f} " + _FROM, "arc length of <(?:the curve |the graph of )?>{f} " + _ON,
     "length of the curve {f} " + _FROM, "length of the curve {f} " + _ON, order=40,
     post=lambda d: None if re.match(r"^(r|x)\s*\(", d["f"]) or "theta" in d["f"] or "," in d["f"] else d)


def _sign_changes(g, a, b, n=1000):
    """Points where a float function changes sign on [a, b], located by bisection (verifiers only)."""
    xs = [a + (b - a) * k / n for k in range(n + 1)]
    ys = [g(x) for x in xs]
    out = []
    for k in range(n):
        lo, hi, flo = xs[k], xs[k + 1], ys[k]
        if flo == 0 and k:
            out.append(lo)
        elif flo * ys[k + 1] < 0:
            for _ in range(80):
                mid = (lo + hi) / 2
                fm = g(mid)
                if fm == 0:
                    lo = hi = mid
                    break
                if (fm < 0) == (flo < 0):
                    lo, flo = mid, fm
                else:
                    hi = mid
            out.append((lo + hi) / 2)
    return out


def _verify_surface(f, a, b):
    """Gauss-Legendre of |f| * sqrt(1 + f'^2) on panels split where f changes sign (an independent rule)."""
    e = parse_expr(f)
    F, D = _compile(e, (X,)), _compile(sp.diff(e, X), (X,))
    cuts = _sign_changes(_compile(e, (X,), use_mp=False), a, b)
    with mp.workdps(20):
        knots = [mp.mpf(a)] + [mp.mpf(c) for c in cuts] + [mp.mpf(b)]
        return 2 * mp.pi * mp.quad(lambda x: abs(F(x)) * mp.sqrt(1 + D(x) ** 2), knots, method="gauss-legendre")


def surface_area_revolution(f, a, b):
    if not a < b:
        raise ValueError("the interval needs a < b")
    e = parse_expr(f)
    d = sp.diff(e, X)
    continuous_on(d, X, a, b)
    g = _compile(e, (X,), use_mp=False)
    cuts, n = [], 400
    xs = [a + (b - a) * k / n for k in range(n + 1)]
    ys = [g(x) for x in xs]
    for k in range(n):
        if ys[k] == 0 and 0 < k:
            cuts.append(xs[k])
        elif ys[k] * ys[k + 1] < 0:
            with mp.workdps(30):
                cuts.append(float(mp.findroot(_compile(e, (X,)), (xs[k], xs[k + 1]), solver="anderson")))
    F = _compile(sp.Abs(e) * sp.sqrt(1 + d ** 2), (X,))
    return {"surface_area": 2 * float(mp.pi) * quad(F, a, b, points=cuts)}


proc("surface_area_revolution", name="Surface area of revolution", topic="integrals / surface area",
     purpose="The area of the surface formed by rotating y = f(x), a <= x <= b, about the x-axis.",
     inputs=[T_("f", "the curve f(x)"), N_("a", "left end"), N_("b", "right end")],
     outputs=[O_("surface_area", "the area of the surface of revolution")],
     primary="surface_area", conditions=["a < b", "f' continuous on [a, b]"],
     method="2*pi times the integral of |f| * sqrt(1 + f'^2) (split where f changes sign), high-precision quadrature",
     latex=r"S = 2\pi\int_a^b |f(x)|\sqrt{1 + f'(x)^2}\,dx",
     run=surface_area_revolution,
     verify=lambda i, o: rel_err(o["surface_area"], _verify_surface(i["f"], i["a"], i["b"]), 1e-8),
     reference=lambda f, a, b: (lambda g, d: {"surface_area": 2 * math.pi * ref_quad(
         lambda x: abs(g(x)) * math.sqrt(1 + d(x) ** 2), a, b)})(ref_lam(f, module="math"), ref_cdiff(f)),
     wrong=lambda f, a, b: {"surface_area": 2 * math.pi * float(_gl_check(f, a, b, transform=lambda F, x: abs(F(x))))},
     random_inputs=lambda rng: (lambda ab: {"f": rng.choice([rand_smooth(rng), f"x^2 + {rng.randint(1, 3)}", "sqrt(x + 4)",
                                                             f"exp(x/{rng.randint(1, 3)})"]), "a": ab[0], "b": ab[1]})(rand_interval(rng, -2, 2)),
     templates=["Find the surface area when y = x from x = 0 to x = 1 is rotated about the x-axis.",
                "Find the area of the surface of revolution of y = sqrt(4 - x^2) about the x-axis on [-1, 1]."],
     examples=[{"inputs": {"f": "x", "a": 0, "b": 1}, "outputs": {"surface_area": math.pi * math.sqrt(2)}},
               {"inputs": {"f": "sqrt(4 - x^2)", "a": -1, "b": 1}, "outputs": {"surface_area": 8 * math.pi}}],
     questions=[{"q": "Find the surface area when y = x from x = 0 to x = 1 is rotated about the x-axis.",
                 "outputs": {"surface_area": math.pi * math.sqrt(2)}},
                {"q": "Find the area of the surface of revolution of y = sqrt(4 - x^2) about the x-axis on [-1, 1].",
                 "outputs": {"surface_area": 8 * math.pi}}],
     keywords=["surface area", "surface of revolution"])
rule("surface_area_revolution",
     "surface area when {f} " + _FROM + " is <(?:rotated|revolved)> about the x-axis",
     "surface area when {f} " + _ON + " is <(?:rotated|revolved)> about the x-axis",
     "<(?:the )?>area of the surface of revolution of {f} about the x-axis " + _ON,
     "<(?:the )?>area of the surface of revolution of {f} about the x-axis " + _FROM, order=30)


def centroid_region(f, a, b):
    if not a < b:
        raise ValueError("the interval needs a < b")
    p = poly_of(f)
    if not _nonneg_on(p, a, b):
        raise ValueError("the region under the curve needs f(x) >= 0 on [a, b]")
    A_, B_ = sp.Rational(repr(float(a))), sp.Rational(repr(float(b)))
    integ = lambda q: (lambda Q: Q.subs(X, B_) - Q.subs(X, A_))(q.integrate().as_expr())   # noqa: E731
    area = integ(p)
    if area == 0:
        raise ValueError("the region has zero area")
    xb, yb = integ(sp.Poly(X, X) * p) / area, integ(p ** 2) / (2 * area)
    return {"area": float(area), "x_bar": float(xb), "y_bar": float(yb)}


def _ref_centroid(f, a, b):
    F = ref_lam(f, module="math")
    A = ref_quad(F, a, b)
    return {"area": A, "x_bar": ref_quad(lambda x: x * F(x), a, b) / A, "y_bar": ref_quad(lambda x: F(x) ** 2, a, b) / (2 * A)}


proc("centroid_region", name="Centroid of a region under a curve", topic="integrals / centroids",
     purpose="The area and centroid (x_bar, y_bar) of the region between y = f(x) >= 0 (a polynomial) and the x-axis "
             "for a <= x <= b.",
     inputs=[T_("f", "the curve f(x) (a polynomial)"), N_("a", "left end"), N_("b", "right end")],
     outputs=[O_("area", "area of the region"), O_("x_bar", "x-coordinate of the centroid"),
              O_("y_bar", "y-coordinate of the centroid")],
     primary="x_bar", conditions=["a < b", "f(x) >= 0 on [a, b] (checked exactly)"],
     method="exact moments of the region: x_bar = (1/A) * integral of x f, y_bar = (1/2A) * integral of f^2",
     latex=r"\bar x = \frac{1}{A}\int_a^b x f(x)\,dx,\quad \bar y = \frac{1}{2A}\int_a^b f(x)^2\,dx",
     run=centroid_region,
     verify=lambda i, o: worst(rel_err(o["area"], _gl_check(i["f"], i["a"], i["b"]), 1e-10),
                               rel_err(o["x_bar"] * o["area"], _gl_check(f"x*({i['f']})", i["a"], i["b"]), 1e-10),
                               rel_err(2 * o["y_bar"] * o["area"], _gl_check(f"({i['f']})^2", i["a"], i["b"]), 1e-10)),
     reference=lambda f, a, b: _ref_centroid(f, a, b),
     wrong=lambda f, a, b: (lambda r: {"area": r["area"], "x_bar": (a + b) / 2, "y_bar": 2 * r["y_bar"]})(centroid_region(f, a, b)),
     random_inputs=lambda rng: (lambda a: {"f": rng.choice([f"{rng.randint(1, 4)} + x^2", f"{rng.randint(9, 12)} - x^2",
                                                            f"x^3 + {rng.randint(1, 3)}", f"{rng.randint(1, 3)}*x + 1"]),
                                           "a": a, "b": a + rng.choice([1, 2, 3])})(float(rng.choice([0, 0.5, 1]))),
     templates=["Find the centroid of the region under y = x^2 from x = 0 to x = 1.",
                "Find the centroid of the region bounded by y = 4 - x^2 and the x-axis on [-2, 2]."],
     examples=[{"inputs": {"f": "x^2", "a": 0, "b": 1}, "outputs": {"area": 1 / 3, "x_bar": 0.75, "y_bar": 0.3}},
               {"inputs": {"f": "4 - x^2", "a": -2, "b": 2}, "outputs": {"area": 32 / 3, "x_bar": 0.0, "y_bar": 1.6}}],
     questions=[{"q": "Find the centroid of the region under y = x^2 from x = 0 to x = 1.", "outputs": {"x_bar": 0.75, "y_bar": 0.3}},
                {"q": "Find the centroid of the region bounded by y = 4 - x^2 and the x-axis on [-2, 2].", "outputs": {"y_bar": 1.6}}],
     keywords=["centroid"])
rule("centroid_region",
     "centroid of the region under {f} " + _FROM, "centroid of the region under {f} " + _ON,
     "centroid of the region bounded by {f} and the x-axis " + _ON,
     "centroid of the region bounded by {f} and the x-axis " + _FROM, order=30)


def improper_integral(f, a):
    e = parse_expr(f)
    from sympy.calculus.util import continuous_domain
    lo = sp.Rational(repr(float(a)))
    if continuous_domain(e, X, sp.Interval(lo, sp.oo)) != sp.Interval(lo, sp.oo):
        raise ValueError("the integrand is not continuous on the whole interval [a, infinity)")
    tail = sp.limit(X ** sp.Rational(101, 100) * e, X, sp.oo)
    if not (tail.is_number and tail.is_finite):
        raise ValueError("convergence cannot be established (the integrand does not decay faster than 1/x^1.01)")
    return {"integral": quad(_compile(e, (X,)), a, mp.inf)}


proc("improper_integral", name="Improper integral to infinity", topic="integrals / improper integrals",
     purpose="The improper integral of f(x) from x = a to infinity, when it converges.",
     inputs=[T_("f", "the integrand f(x)"), N_("a", "lower limit")],
     outputs=[O_("integral", "the value of the improper integral")],
     primary="integral", conditions=["f continuous on [a, infinity)",
                                     "x^1.01 * f(x) has a finite limit (comparison test), so the integral converges absolutely"],
     method="convergence by a limit comparison with 1/x^1.01 (exact limit), value by tanh-sinh quadrature on [a, infinity)",
     latex=r"\int_a^\infty f(x)\,dx = \lim_{R\to\infty}\int_a^R f(x)\,dx",
     run=improper_integral,
     verify=lambda i, o: rel_err(o["integral"], (lambda F: mp.quad(F, [i["a"], i["a"] + 1, i["a"] + 10, mp.inf]))(
         _compile(parse_expr(i["f"]), (X,))), 1e-9),
     reference=lambda f, a: {"integral": ref_quad(ref_lam(f, module="math"), a, float("inf"))},
     wrong=lambda f, a: {"integral": quad(_compile(parse_expr(f), (X,)), a, a + 3)},
     random_inputs=lambda rng: {"f": rng.choice([f"exp(-{rng.randint(1, 3)}*x)", f"x*exp(-x)", f"1/(x^2 + {rng.randint(1, 4)})",
                                                 f"{rng.randint(1, 5)}/x^{rng.randint(2, 4)}", "exp(-x^2)", f"x^2*exp(-x/{rng.randint(1, 2)})",
                                                 f"exp(-x)*sin({rng.randint(1, 3)}*x)", f"1/(x*(x + {rng.randint(1, 3)}))"]),
                                "a": float(rng.randint(1, 3))},
     templates=["Evaluate the improper integral of exp(-x) from 0 to infinity.", "Find the integral of 1/x^2 from 1 to infinity."],
     examples=[{"inputs": {"f": "exp(-x)", "a": 0}, "outputs": {"integral": 1.0}},
               {"inputs": {"f": "1/x^2", "a": 1}, "outputs": {"integral": 1.0}},
               {"inputs": {"f": "1/(x^2 + 1)", "a": 0}, "outputs": {"integral": math.pi / 2}},
               {"inputs": {"f": "exp(-x^2)", "a": 0}, "outputs": {"integral": math.sqrt(math.pi) / 2}}],
     questions=[{"q": "Evaluate the improper integral of exp(-x) from 0 to infinity.", "outputs": {"integral": 1.0}},
                {"q": "Find the integral of 1/x^2 from 1 to infinity.", "outputs": {"integral": 1.0}},
                {"q": "Find the integral of 1/x from 1 to infinity.", "outcome": "INVALID_INPUT"},
                {"q": "Evaluate the integral of sin(x) from 0 to infinity.", "outcome": "INVALID_INPUT"}],
     keywords=["improper integral", "to infinity"])
rule("improper_integral",
     "<(?:improper )?>integral of {f}<(?: dx)?> from <(?:x = )?>{a:n} to <(?:\\+)?(?:infinity|inf|oo)>", order=30)


# =========================================================================================== 3. numerical methods
def _memo(fn):
    """Cache a run on its (hashable) inputs; return a fresh copy each time (callers may modify the dict)."""
    cache = {}

    def wrapped(*args, **kw):
        key = (args, tuple(sorted(kw.items())))
        try:
            hash(key)
        except TypeError:
            return fn(*args, **kw)
        if key not in cache:
            if len(cache) > 2048:
                cache.clear()
            try:
                cache[key] = ("ok", fn(*args, **kw))
            except ValueError as exc:
                cache[key] = ("err", str(exc))
        st, v = cache[key]
        if st == "err":
            raise ValueError(v)
        return {k: (list(x) if isinstance(x, list) else x) for k, x in v.items()}
    wrapped.__name__ = fn.__name__
    return wrapped


def _grid(f, a, b, n):
    if int(n) != n or not 1 <= n <= 100000:
        raise ValueError("n must be a whole number of subintervals between 1 and 100000")
    if not a < b:
        raise ValueError("the interval needs a < b")
    return parse_expr(f), (b - a) / int(n), int(n)


def _values(e, pts):
    g = _compile(e, (X,))
    with mp.workdps(20):
        return [g(mp.mpf(p)) for p in pts]


def _exact(e, a, b):
    continuous_on(e, X, a, b)
    return quad(_compile(e, (X,)), a, b)


@_memo
def left_riemann_sum(f, a, b, n):
    e, h, n = _grid(f, a, b, n)
    with mp.workdps(20):
        return {"approximation": float(h * mp.fsum(_values(e, [a + k * h for k in range(n)])))}


@_memo
def right_riemann_sum(f, a, b, n):
    e, h, n = _grid(f, a, b, n)
    with mp.workdps(20):
        return {"approximation": float(h * mp.fsum(_values(e, [a + k * h for k in range(1, n + 1)])))}


def _float_sum(f, pts, w):
    g = _compile(parse_expr(f), (X,), use_mp=False)
    return math.fsum(wi * g(p) for wi, p in zip(w, pts))


def _ref_trap(f, a, b, n):
    import numpy as np
    from scipy.integrate import trapezoid
    g = ref_lam(f, module="numpy")
    xs = np.linspace(a, b, int(n) + 1)
    ys = np.asarray(g(xs), dtype=float) * np.ones_like(xs)
    return float(trapezoid(ys, xs)), (b - a) / int(n), float(ys[0]), float(ys[-1])


def _ref_left_right(f, a, b, n, side):
    t, h, ya, yb = _ref_trap(f, a, b, n)
    return {"approximation": t - h * (yb - ya) / 2 if side == "left" else t + h * (yb - ya) / 2}


_RIEMANN_IN = [T_("f", "the function f(x)"), N_("a", "left end"), N_("b", "right end"),
               I_("n", "number of equal subintervals", 1, 100000)]

proc("left_riemann_sum", name="Left Riemann sum", topic="integrals / Riemann sums",
     purpose="The left-endpoint Riemann sum of f on [a, b] with n equal subintervals.",
     inputs=_RIEMANN_IN, outputs=[O_("approximation", "the left Riemann sum")],
     primary="approximation", conditions=["a < b", "1 <= n <= 100000", "f defined at the sample points"],
     method="h times the sum of f at the left endpoints x_k = a + k h, k = 0..n-1 (h = (b - a)/n)",
     latex=r"L_n = h\sum_{k=0}^{n-1} f(a + k h),\quad h = \frac{b-a}{n}",
     run=left_riemann_sum,
     verify=lambda i, o: rel_err(o["approximation"], _float_sum(i["f"], [i["a"] + k * (i["b"] - i["a"]) / i["n"] for k in range(i["n"])],
                                                            [(i["b"] - i["a"]) / i["n"]] * i["n"]), 1e-11),
     reference=lambda f, a, b, n: _ref_left_right(f, a, b, n, "left"),
     wrong=lambda f, a, b, n: right_riemann_sum(f, a, b, n),
     random_inputs=lambda rng: (lambda ab: {"f": rand_smooth(rng), "a": ab[0], "b": ab[1], "n": rng.choice([2, 4, 5, 8, 10, 20])})(rand_interval(rng)),
     templates=["Find the left Riemann sum of x^2 on [0, 2] with n = 4.",
                "Approximate the integral of x^3 from 0 to 1 using a left Riemann sum with 4 subintervals."],
     examples=[{"inputs": {"f": "x^2", "a": 0, "b": 2, "n": 4}, "outputs": {"approximation": 1.75}},
               {"inputs": {"f": "x^3", "a": 0, "b": 1, "n": 4}, "outputs": {"approximation": 0.140625}}],
     questions=[{"q": "Find the left Riemann sum of x^2 on [0, 2] with n = 4.", "outputs": {"approximation": 1.75}},
                {"q": "Approximate the integral of x^3 from 0 to 1 using a left Riemann sum with 4 subintervals.", "outputs": {"approximation": 0.140625}},
                {"q": "Find the left Riemann sum of x^2 on [0, 2] with n = 0.", "outcome": "INVALID_INPUT"}],
     keywords=["left riemann sum", "left endpoint"])
proc("right_riemann_sum", name="Right Riemann sum", topic="integrals / Riemann sums",
     purpose="The right-endpoint Riemann sum of f on [a, b] with n equal subintervals.",
     inputs=_RIEMANN_IN, outputs=[O_("approximation", "the right Riemann sum")],
     primary="approximation", conditions=["a < b", "1 <= n <= 100000", "f defined at the sample points"],
     method="h times the sum of f at the right endpoints x_k = a + k h, k = 1..n",
     latex=r"R_n = h\sum_{k=1}^{n} f(a + k h),\quad h = \frac{b-a}{n}",
     run=right_riemann_sum,
     verify=lambda i, o: rel_err(o["approximation"], _float_sum(i["f"], [i["a"] + k * (i["b"] - i["a"]) / i["n"] for k in range(1, i["n"] + 1)],
                                                            [(i["b"] - i["a"]) / i["n"]] * i["n"]), 1e-11),
     reference=lambda f, a, b, n: _ref_left_right(f, a, b, n, "right"),
     wrong=lambda f, a, b, n: left_riemann_sum(f, a, b, n),
     random_inputs=lambda rng: (lambda ab: {"f": rand_smooth(rng), "a": ab[0], "b": ab[1], "n": rng.choice([2, 4, 5, 8, 10, 20])})(rand_interval(rng)),
     templates=["Find the right Riemann sum of x^2 on [0, 2] with n = 4.",
                "Approximate the integral of x^3 from 0 to 1 using a right Riemann sum with 4 subintervals."],
     examples=[{"inputs": {"f": "x^2", "a": 0, "b": 2, "n": 4}, "outputs": {"approximation": 3.75}},
               {"inputs": {"f": "x^3", "a": 0, "b": 1, "n": 4}, "outputs": {"approximation": 0.390625}}],
     questions=[{"q": "Find the right Riemann sum of x^2 on [0, 2] with n = 4.", "outputs": {"approximation": 3.75}},
                {"q": "Approximate the integral of x^3 from 0 to 1 using a right Riemann sum with 4 subintervals.", "outputs": {"approximation": 0.390625}}],
     keywords=["right riemann sum", "right endpoint"])
for _side in ("left", "right"):
    rule(f"{_side}_riemann_sum",
         f"{_side} <(?:riemann|endpoint) >sum<(?: approximation)?> of {{f}} " + _ON + " with <(?:n = )?>{n:i}< subintervals| rectangles|>",
         f"{_side} <(?:riemann|endpoint) >sum<(?: approximation)?> of {{f}} " + _FROM + " with <(?:n = )?>{n:i}< subintervals| rectangles|>",
         f"<(?:the )?>integral of {{f}} " + _FROM + f" using <(?:a |the )?>{_side} <(?:riemann |endpoint )?>sum with <(?:n = )?>{{n:i}}< subintervals| rectangles|>",
         f"<(?:the )?>integral of {{f}} " + _ON + f" using <(?:a |the )?>{_side} <(?:riemann |endpoint )?>sum with <(?:n = )?>{{n:i}}< subintervals| rectangles|>",
         order=25)


def _rule_n(e, a, b, n, kind):
    h = (b - a) / n
    if kind == "trapezoid":
        pts, w = [a + k * h for k in range(n + 1)], [h / 2] + [h] * (n - 1) + [h / 2]
    elif kind == "simpson":
        pts = [a + k * h for k in range(n + 1)]
        w = [h / 3 * (1 if k in (0, n) else (4 if k % 2 else 2)) for k in range(n + 1)]
    else:
        pts, w = [a + (k + 0.5) * h for k in range(n)], [h] * n
    return pts, w


def _numrule(kind):
    @_memo
    def run(f, a, b, n):
        e, h, n = _grid(f, a, b, n)
        if kind == "simpson" and n % 2:
            raise ValueError("Simpson's rule needs an even number of subintervals")
        pts, w = _rule_n(e, a, b, n, kind)
        with mp.workdps(20):
            approx = float(mp.fsum(wi * v for wi, v in zip(w, _values(e, pts))))
        exact = _exact(e, a, b)
        return {"approximation": approx, "exact": exact, "error": approx - exact}
    run.__name__ = kind + "_rule"
    return run


def _verify_rule(kind):
    def verify(i, o):
        e = parse_expr(i["f"])
        pts, w = _rule_n(e, i["a"], i["b"], int(i["n"]), kind)
        return worst(rel_err(o["approximation"], _float_sum(i["f"], pts, w), 1e-11),
                     rel_err(o["exact"], _gl_check(i["f"], i["a"], i["b"]), 1e-10),
                     abs(o["error"] - (o["approximation"] - o["exact"])))
    return verify


def _ref_simpson(f, a, b, n):
    import numpy as np
    from scipy.integrate import simpson
    g = ref_lam(f, module="numpy")
    xs = np.linspace(a, b, int(n) + 1)
    return float(simpson(np.asarray(g(xs), dtype=float) * np.ones_like(xs), x=xs))


trapezoid_rule, simpson_rule, midpoint_rule = _numrule("trapezoid"), _numrule("simpson"), _numrule("midpoint")
_EX = lambda f, a, b: ref_quad(ref_lam(f, module="math"), a, b)   # noqa: E731

proc("trapezoid_rule", name="Trapezoidal rule", topic="numerical integration",
     purpose="The composite trapezoidal approximation of the integral of f on [a, b] with n subintervals, with the exact "
             "value and the error (approximation - exact).",
     inputs=_RIEMANN_IN, outputs=[O_("approximation", "trapezoidal approximation T_n"), O_("exact", "the integral"),
                                  O_("error", "T_n - exact")],
     primary="approximation", conditions=["a < b", "1 <= n <= 100000", "f continuous on [a, b]"],
     method="composite trapezoidal rule; exact value by high-precision quadrature",
     latex=r"T_n = \frac{h}{2}\left[f(x_0) + 2\sum_{k=1}^{n-1} f(x_k) + f(x_n)\right]",
     run=trapezoid_rule, verify=_verify_rule("trapezoid"),
     reference=lambda f, a, b, n: (lambda t, ex: {"approximation": t, "exact": ex})(_ref_trap(f, a, b, n)[0], _EX(f, a, b)),
     wrong=lambda f, a, b, n: (lambda r: dict(r, approximation=r["approximation"] + (b - a) / n * (
         at(parse_expr(f), (X,), a) + at(parse_expr(f), (X,), b)) / 2))(trapezoid_rule(f, a, b, n)),
     random_inputs=lambda rng: (lambda ab: {"f": rand_smooth(rng), "a": ab[0], "b": ab[1], "n": rng.choice([2, 4, 6, 8, 10, 16])})(rand_interval(rng)),
     templates=["Approximate the integral of x^2 from 0 to 2 using the trapezoidal rule with n = 4.",
                "Use the trapezoid rule with 4 subintervals to estimate the integral of 1/x from 1 to 2."],
     examples=[{"inputs": {"f": "x^2", "a": 0, "b": 2, "n": 4}, "outputs": {"approximation": 2.75, "exact": 8 / 3, "error": 2.75 - 8 / 3}},
               {"inputs": {"f": "1/x", "a": 1, "b": 2, "n": 4}, "outputs": {"approximation": 1171 / 1680, "exact": math.log(2)}}],
     questions=[{"q": "Approximate the integral of x^2 from 0 to 2 using the trapezoidal rule with n = 4.", "outputs": {"approximation": 2.75}},
                {"q": "Use the trapezoid rule with 4 subintervals to estimate the integral of 1/x from 1 to 2.", "outputs": {"approximation": 1171 / 1680}},
                {"q": "Use the trapezoidal rule with 4 subintervals to estimate the integral of 1/x from -1 to 1.", "outcome": "INVALID_INPUT"}],
     keywords=["trapezoidal rule", "trapezoid rule"])
proc("simpson_rule", name="Simpson's rule", topic="numerical integration",
     purpose="The composite Simpson approximation of the integral of f on [a, b] with an even number n of subintervals, "
             "with the exact value and the error.",
     inputs=_RIEMANN_IN, outputs=[O_("approximation", "Simpson approximation S_n"), O_("exact", "the integral"),
                                  O_("error", "S_n - exact")],
     primary="approximation", conditions=["a < b", "n even, 2 <= n <= 100000", "f continuous on [a, b]"],
     method="composite Simpson's 1/3 rule (weights 1, 4, 2, ..., 4, 1); exact value by high-precision quadrature",
     latex=r"S_n = \frac{h}{3}\left[f(x_0) + 4\sum_{\text{odd } k} f(x_k) + 2\sum_{\text{even } k} f(x_k) + f(x_n)\right]",
     run=simpson_rule, verify=_verify_rule("simpson"),
     reference=lambda f, a, b, n: (lambda s, ex: {"approximation": s, "exact": ex})(_ref_simpson(f, a, b, n), _EX(f, a, b)),
     wrong=lambda f, a, b, n: (lambda e, h: (lambda s: {"approximation": s, "exact": 0.0, "error": 0.0})(
         float(mp.fsum(h / 3 * (1 if k in (0, n) else (2 if k % 2 else 4)) * v for k, v in enumerate(_values(e, [a + k * h for k in range(n + 1)]))))))(
         parse_expr(f), (b - a) / n),
     random_inputs=lambda rng: (lambda ab: {"f": rand_smooth(rng), "a": ab[0], "b": ab[1], "n": rng.choice([2, 4, 6, 8, 10, 16])})(rand_interval(rng)),
     templates=["Use Simpson's rule with n = 4 to approximate the integral of exp(x) from 0 to 1.",
                "Approximate the integral of x^4 from 0 to 2 using Simpson's rule with 4 subintervals."],
     examples=[{"inputs": {"f": "x^4", "a": 0, "b": 2, "n": 4}, "outputs": {"approximation": 6.416666666666667, "exact": 6.4}},
               {"inputs": {"f": "x^3", "a": 0, "b": 2, "n": 2}, "outputs": {"approximation": 4.0, "error": 0.0}}],
     questions=[{"q": "Approximate the integral of x^4 from 0 to 2 using Simpson's rule with 4 subintervals.", "outputs": {"approximation": 6.416666666666667}},
                {"q": "Use Simpson's rule with n = 2 to approximate the integral of x^3 from 0 to 2.", "outputs": {"approximation": 4.0}},
                {"q": "Use Simpson's rule with n = 3 to approximate the integral of x^3 from 0 to 2.", "outcome": "INVALID_INPUT"}],
     keywords=["simpson's rule", "simpson"])
proc("midpoint_rule", name="Midpoint rule", topic="numerical integration",
     purpose="The composite midpoint approximation of the integral of f on [a, b] with n subintervals, with the exact value "
             "and the error.",
     inputs=_RIEMANN_IN, outputs=[O_("approximation", "midpoint approximation M_n"), O_("exact", "the integral"),
                                  O_("error", "M_n - exact")],
     primary="approximation", conditions=["a < b", "1 <= n <= 100000", "f continuous on [a, b]"],
     method="composite midpoint rule; exact value by high-precision quadrature",
     latex=r"M_n = h\sum_{k=0}^{n-1} f\left(a + (k + \tfrac12) h\right)",
     run=midpoint_rule, verify=_verify_rule("midpoint"),
     reference=lambda f, a, b, n: (lambda m, ex: {"approximation": m, "exact": ex})(
         2 * _ref_trap(f, a, b, 2 * n)[0] - _ref_trap(f, a, b, n)[0], _EX(f, a, b)),
     wrong=lambda f, a, b, n: dict(left_riemann_sum(f, a, b, n), exact=0.0, error=0.0),
     random_inputs=lambda rng: (lambda ab: {"f": rand_smooth(rng), "a": ab[0], "b": ab[1], "n": rng.choice([2, 4, 5, 8, 10, 16])})(rand_interval(rng)),
     templates=["Approximate the integral of x^2 from 0 to 2 using the midpoint rule with n = 4.",
                "Use the midpoint rule with 2 subintervals to estimate the integral of x^3 from 0 to 2."],
     examples=[{"inputs": {"f": "x^2", "a": 0, "b": 2, "n": 4}, "outputs": {"approximation": 2.625}},
               {"inputs": {"f": "x^3", "a": 0, "b": 2, "n": 2}, "outputs": {"approximation": 3.5, "exact": 4.0}}],
     questions=[{"q": "Approximate the integral of x^2 from 0 to 2 using the midpoint rule with n = 4.", "outputs": {"approximation": 2.625}},
                {"q": "Use the midpoint rule with 2 subintervals to estimate the integral of x^3 from 0 to 2.", "outputs": {"approximation": 3.5}}],
     keywords=["midpoint rule"])
for _k, _w in (("trapezoid_rule", "(?:the )?(?:trapezoidal|trapezoid) rule"), ("simpson_rule", "(?:simpson's|simpsons|simpson) rule"),
               ("midpoint_rule", "(?:the )?midpoint rule")):
    rule(_k,
         "<(?:the )?>integral of {f}<(?: dx)?> " + _FROM + " using <" + _w + "> with <(?:n = )?>{n:i}< subintervals| intervals| strips|>",
         "<(?:the )?>integral of {f}<(?: dx)?> " + _ON + " using <" + _w + "> with <(?:n = )?>{n:i}< subintervals| intervals| strips|>",
         "<(?:use )?" + _w + "> with <(?:n = )?>{n:i}< subintervals| intervals| strips|> to <(?:estimate|approximate|evaluate)> <(?:the )?>integral of {f}<(?: dx)?> " + _FROM,
         "<(?:use )?" + _w + "> with <(?:n = )?>{n:i}< subintervals| intervals| strips|> to <(?:estimate|approximate|evaluate)> <(?:the )?>integral of {f}<(?: dx)?> " + _ON,
         order=25)


@functools.lru_cache(maxsize=64)
def _gauss_nodes(n):
    """Gauss-Legendre nodes and weights on [-1, 1]: Newton's method on the Legendre recurrence (30 digits)."""
    with mp.workdps(40):
        out = []
        for i in range(1, n + 1):
            x = mp.cos(mp.pi * (i - mp.mpf(1) / 4) / (n + mp.mpf(1) / 2))
            for _ in range(100):
                p0, p1 = mp.mpf(1), x
                for k in range(2, n + 1):
                    p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
                dp = n * (x * p1 - p0) / (x * x - 1) if n > 1 else mp.mpf(1)
                if n == 1:
                    p1, dp = x, mp.mpf(1)
                dx = p1 / dp
                x -= dx
                if abs(dx) < mp.mpf(10) ** -35:
                    break
            p0, p1 = mp.mpf(1), x
            for k in range(2, n + 1):
                p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
            dp = n * (x * p1 - p0) / (x * x - 1) if n > 1 else mp.mpf(1)
            out.append((x, 2 / ((1 - x * x) * dp * dp)))
        return tuple(sorted(out))


@_memo
def gauss_legendre(f, a, b, n):
    n = int(n)
    if not 1 <= n <= 64:
        raise ValueError("the number of nodes must be between 1 and 64")
    if not a < b:
        raise ValueError("the interval needs a < b")
    e = parse_expr(f)
    g = _compile(e, (X,))
    with mp.workdps(30):
        c, r = (mp.mpf(a) + b) / 2, (mp.mpf(b) - a) / 2
        approx = float(r * mp.fsum(w * g(c + r * x) for x, w in _gauss_nodes(n)))
    exact = _exact(e, a, b)
    return {"approximation": approx, "exact": exact, "error": approx - exact}


def _ref_gauss(f, a, b, n):
    import numpy as np
    x, w = np.polynomial.legendre.leggauss(int(n))
    g = ref_lam(f, module="numpy")
    c, r = (a + b) / 2, (b - a) / 2
    approx = float(r * np.sum(w * (np.asarray(g(c + r * x), dtype=float) * np.ones_like(x))))
    return {"approximation": approx, "exact": _EX(f, a, b)}     # the error (a small difference) is checked by verify


proc("gauss_legendre", name="Gauss-Legendre quadrature", topic="numerical integration",
     purpose="The n-point Gauss-Legendre approximation of the integral of f on [a, b] (exact for polynomials of degree "
             "<= 2n - 1), with the exact value and the error.",
     inputs=[T_("f", "the function f(x)"), N_("a", "left end"), N_("b", "right end"), I_("n", "number of nodes", 1, 64)],
     outputs=[O_("approximation", "n-point Gauss-Legendre value"), O_("exact", "the integral"), O_("error", "approximation - exact")],
     primary="approximation", conditions=["a < b", "1 <= n <= 64", "f continuous on [a, b]"],
     method="nodes = roots of the Legendre polynomial P_n (Newton's method on the three-term recurrence), weights "
            "2/((1 - x^2) P_n'(x)^2), mapped to [a, b]",
     latex=r"\int_a^b f\,dx \approx \frac{b-a}{2}\sum_{i=1}^n w_i\, f\!\left(\frac{a+b}{2} + \frac{b-a}{2}x_i\right)",
     run=gauss_legendre,
     verify=lambda i, o: worst(rel_err(o["exact"], _gl_check(i["f"], i["a"], i["b"]), 1e-10),
                               rel_err(o["approximation"], _float_sum(i["f"], [(i["a"] + i["b"]) / 2 + (i["b"] - i["a"]) / 2 * float(x) for x, _w in _gauss_nodes(int(i["n"]))],
                                                                      [(i["b"] - i["a"]) / 2 * float(w) for _x, w in _gauss_nodes(int(i["n"]))]), 1e-11),
                               abs(o["error"] - (o["approximation"] - o["exact"]))),
     reference=lambda f, a, b, n: _ref_gauss(f, a, b, n),
     wrong=lambda f, a, b, n: (lambda r: dict(r, approximation=(b - a) / int(n) * float(mp.fsum(
         _values(parse_expr(f), [(a + b) / 2 + (b - a) / 2 * x for x, _w in _gauss_nodes(int(n))])))))(gauss_legendre(f, a, b, n)),
     random_inputs=lambda rng: (lambda ab: {"f": rand_smooth(rng), "a": ab[0], "b": ab[1], "n": rng.randint(2, 8)})(rand_interval(rng)),
     templates=["Use 2-point Gauss-Legendre quadrature to approximate the integral of x^3 + x^2 from 0 to 2.",
                "Approximate the integral of exp(x) from 0 to 1 using Gaussian quadrature with 3 points."],
     examples=[{"inputs": {"f": "x^3 + x^2", "a": 0, "b": 2, "n": 2}, "outputs": {"approximation": 4 + 8 / 3, "error": 0.0}},
               {"inputs": {"f": "x^4", "a": -1, "b": 1, "n": 2}, "outputs": {"approximation": 2 / 9, "exact": 0.4}}],
     questions=[{"q": "Use 2-point Gauss-Legendre quadrature to approximate the integral of x^3 + x^2 from 0 to 2.", "outputs": {"approximation": 4 + 8 / 3}},
                {"q": "Use 2-point Gaussian quadrature to approximate the integral of x^4 from -1 to 1.", "outputs": {"approximation": 2 / 9}}],
     keywords=["gauss-legendre", "gaussian quadrature"])
rule("gauss_legendre",
     "<(?:use )?>{n:i}<-| >point <(?:gauss-legendre|gauss legendre|gaussian|gauss)> quadrature to <(?:approximate|estimate|evaluate)> <(?:the )?>integral of {f}<(?: dx)?> " + _FROM,
     "<(?:the )?>integral of {f}<(?: dx)?> " + _FROM + " using <(?:gauss-legendre|gauss legendre|gaussian|gauss)> quadrature with {n:i} <(?:points|nodes)>",
     order=25)


@_memo
def numerical_derivative(f, a, h, method):
    if not h > 0:
        raise ValueError("the step h must be positive")
    e = parse_expr(f)
    g = _compile(e, (X,))
    with mp.workdps(30):
        A, H = mp.mpf(a), mp.mpf(h)
        if method == "forward":
            d = (g(A + H) - g(A)) / H
        elif method == "backward":
            d = (g(A) - g(A - H)) / H
        else:
            d = (g(A + H) - g(A - H)) / (2 * H)
    exact = at(sp.diff(e, X), (X,), a)
    return {"approximation": float(d), "exact": exact, "error": float(d) - exact}


def _ref_numdiff(f, a, h, method):
    import numpy as np
    g = ref_lam(f, module="numpy")
    if method == "central":
        d = float(np.gradient(np.array([g(a - h), g(a), g(a + h)], dtype=float), h)[1])
    elif method == "forward":
        d = float(np.gradient(np.array([g(a), g(a + h)], dtype=float), h)[0])
    else:
        d = float(np.gradient(np.array([g(a - h), g(a)], dtype=float), h)[-1])
    ex = ref_diff(ref_lam(f), a)
    return {"approximation": d, "exact": ex}


proc("numerical_derivative", name="Finite-difference derivative", topic="numerical differentiation",
     purpose="A forward, backward or central difference approximation of f'(a) with step h, with the exact derivative "
             "and the error.",
     inputs=[T_("f", "the function f(x)"), N_("a", "the point x = a"), N_("h", "the step size h > 0"),
             C_("method", "forward, backward or central difference", ["forward", "backward", "central"])],
     outputs=[O_("approximation", "the difference quotient"), O_("exact", "the exact f'(a)"), O_("error", "approximation - exact")],
     primary="approximation", conditions=["h > 0", "f defined at the points used"],
     method="difference quotients (f(a+h)-f(a))/h, (f(a)-f(a-h))/h, (f(a+h)-f(a-h))/(2h) at 30 digits; exact derivative symbolically",
     latex=r"f'(a) \approx \frac{f(a+h) - f(a-h)}{2h}",
     run=numerical_derivative,
     verify=lambda i, o: worst(rel_err(o["exact"], _mp_diff(i["f"], i["a"]), 1e-12),
                               abs(o["error"] - (o["approximation"] - o["exact"])),
                               rel_err(o["approximation"], (lambda g, a, h: {"forward": (g(a + h) - g(a)) / h, "backward": (g(a) - g(a - h)) / h,
                                                                             "central": (g(a + h) - g(a - h)) / (2 * h)}[i["method"]])(
                                   _compile(parse_expr(i["f"]), (X,), use_mp=False), i["a"], i["h"]), 1e-9)),
     reference=lambda f, a, h, method: _ref_numdiff(f, a, h, method),
     wrong=lambda f, a, h, method: (lambda r: dict(r, approximation=2 * r["approximation"] if method == "central" else
                                                   numerical_derivative(f, a, h, "central")["approximation"] * 2))(numerical_derivative(f, a, h, method)),
     random_inputs=lambda rng: {"f": rand_smooth(rng), "a": rand_point(rng), "h": rng.choice([0.1, 0.05, 0.2, 0.01]),
                                "method": rng.choice(["forward", "backward", "central"])},
     templates=["Estimate the derivative of x^2 at x = 1 using a central difference with h = 0.1.",
                "Use a forward difference with h = 0.5 to approximate the derivative of x^3 at x = 1."],
     examples=[{"inputs": {"f": "x^2", "a": 1, "h": 0.1, "method": "central"}, "outputs": {"approximation": 2.0, "exact": 2.0}},
               {"inputs": {"f": "x^3", "a": 1, "h": 0.5, "method": "forward"}, "outputs": {"approximation": 4.75, "error": 1.75}},
               {"inputs": {"f": "x^3", "a": 1, "h": 0.5, "method": "backward"}, "outputs": {"approximation": 1.75}}],
     questions=[{"q": "Estimate the derivative of x^2 at x = 1 using a central difference with h = 0.1.", "outputs": {"approximation": 2.0}},
                {"q": "Use a forward difference with h = 0.5 to approximate the derivative of x^3 at x = 1.", "outputs": {"approximation": 4.75}},
                {"q": "Use a backward difference with h = 0.5 to approximate the derivative of x^3 at x = 1.", "outputs": {"approximation": 1.75}},
                {"q": "Use a central difference with h = -0.1 to approximate the derivative of x^3 at x = 1.", "outcome": "INVALID_INPUT"}],
     keywords=["finite difference", "central difference", "forward difference"])
rule("numerical_derivative",
     "derivative of {f} at x = {a:n} using a {method:w}< difference| difference quotient| finite difference> with h = {h:n}",
     "<(?:use )?(?:a )?>{method:w}< difference| difference quotient| finite difference> with h = {h:n} to <(?:approximate|estimate|find)> <(?:the )?>derivative of {f} at x = {a:n}",
     order=15, post=lambda d: d if d["method"] in ("forward", "backward", "central") else None)


@_memo
def newton_method(f, x0, n):
    n = int(n)
    if not 1 <= n <= 50:
        raise ValueError("the number of iterations must be between 1 and 50")
    e = parse_expr(f)
    g, dg = _compile(e, (X,)), _compile(sp.diff(e, X), (X,))
    with mp.workdps(30):
        x, its = mp.mpf(x0), []
        for _ in range(n):
            d = dg(x)
            if d == 0:
                raise ValueError("f'(x) = 0 at an iterate, so Newton's method cannot continue")
            x = x - g(x) / d
            its.append(float(x))
        fx_ = float(g(x))
    return {"x_n": float(x), "f_x_n": fx_, "iterates": its}


def _verify_newton(f, x0, o):
    """Each iterate re-derived from the previous one with mpmath's numerical derivative."""
    g = _compile(parse_expr(f), (X,))
    its = o["iterates"]
    with mp.workdps(30):
        steps = [rel_err(b, float(mp.mpf(a) - g(mp.mpf(a)) / mp.diff(g, mp.mpf(a))), 1e-10)
                 for a, b in zip([x0] + its[:-1], its)]
    return max(steps + [rel_err(its[-1], o["x_n"])])


def _ref_newton(f, x0, n):
    g = ref_lam(f)
    with mp.workdps(30):
        x = mp.mpf(x0)
        for _ in range(int(n)):
            x = x - g(x) / mp.diff(g, x)
        return {"x_n": float(x)}


proc("newton_method", name="Newton's method", topic="numerical methods / root finding",
     purpose="n iterations of Newton's method x_{k+1} = x_k - f(x_k)/f'(x_k) for a root of f, from x_0.",
     inputs=[T_("f", "the function f(x)"), N_("x0", "the starting value x_0"), I_("n", "number of iterations", 1, 50)],
     outputs=[O_("x_n", "the n-th iterate"), O_("f_x_n", "f(x_n)"), O_("iterates", "x_1 .. x_n", "vector")],
     primary="x_n", conditions=["f'(x_k) != 0 at every iterate", "1 <= n <= 50"],
     method="Newton-Raphson with the exact derivative, at 30 digits", latex=r"x_{k+1} = x_k - \frac{f(x_k)}{f'(x_k)}",
     run=newton_method,
     verify=lambda i, o: _verify_newton(i["f"], i["x0"], o),
     reference=lambda f, x0, n: _ref_newton(f, x0, n),
     wrong=lambda f, x0, n: (lambda e: (lambda g, dg: (lambda xs: {"x_n": xs[-1], "f_x_n": 0.0, "iterates": xs})(
         functools.reduce(lambda acc, _k: acc + [acc[-1] + float(g(acc[-1]) / dg(acc[-1]))], range(int(n)), [float(x0)])[1:]))(
         _compile(e, (X,)), _compile(sp.diff(e, X), (X,))))(parse_expr(f)),
     random_inputs=lambda rng: (lambda r: {"f": rng.choice([f"x^2 - {r * r + rng.randint(1, 3)}", f"x^3 - {rng.randint(2, 20)}",
                                                            f"cos(x) - x/{rng.randint(1, 3)}", f"exp(x) - {rng.randint(2, 6)}",
                                                            f"x^3 - {rng.randint(1, 4)}*x - {rng.randint(1, 5)}"]),
                                           "x0": float(r + rng.choice([0.5, 1, 1.5, 2])), "n": rng.randint(1, 5)})(rng.randint(1, 3)),
     templates=["Apply Newton's method to f(x) = x^2 - 2 starting at x0 = 1 for 3 iterations.",
                "Use 2 iterations of Newton's method on x^3 - 2x - 5 from x0 = 2."],
     examples=[{"inputs": {"f": "x^2 - 2", "x0": 1, "n": 3}, "outputs": {"x_n": 577 / 408, "iterates": [1.5, 17 / 12, 577 / 408]}},
               {"inputs": {"f": "x^3 - 2x - 5", "x0": 2, "n": 1}, "outputs": {"x_n": 2.1}}],
     questions=[{"q": "Apply Newton's method to f(x) = x^2 - 2 starting at x0 = 1 for 3 iterations.", "outputs": {"x_n": 577 / 408}},
                {"q": "Use 1 iteration of Newton's method on x^3 - 2x - 5 from x0 = 2.", "outputs": {"x_n": 2.1}},
                {"q": "Apply Newton's method to f(x) = x^2 - 2 starting at x0 = 0 for 3 iterations.", "outcome": "INVALID_INPUT"}],
     keywords=["newton's method", "newton-raphson"])
rule("newton_method",
     "<(?:apply|use) >newton<'?>s method <(?:to|on|for)> {f} starting <(?:at|from|with)> <(?:x0|x_0|x)> = {x0:n} for {n:i} <(?:iterations|iteration|steps|step)>",
     "<(?:use |apply )?>{n:i} <(?:iterations|iteration|steps|step)> of newton<'?>s method <(?:to|on|for)> {f} <(?:from|starting at|with)> <(?:x0|x_0|x)> = {x0:n}",
     order=15)


# =========================================================================================== shared helpers (part 2)
_XY, _XYZ = ("x", "y"), ("x", "y", "z")
_KS = sp.Symbol("k", integer=True)


def _rat(v):
    """A stated decimal as the exact rational it was written as (0.1 -> 1/10)."""
    return sp.Rational(repr(float(v)))


def _mpq(q):
    return mp.mpf(q.p) / q.q


def _pfloat(v) -> float:
    return _fin(sp.N(v, 30))


def _same(a, b) -> float:
    return 0.0 if a == b else 1.0


def _vec_err(got, want, slack=1e-12) -> float:
    got, want = list(got), list(want)
    if len(got) != len(want):
        return 1.0
    return worst(*[rel_err(g, w, slack) for g, w in zip(got, want)])


def _limval(L) -> float:
    """A sympy limit or sum as a finite real number, or ValueError (infinite, oscillating, undetermined)."""
    if not isinstance(L, sp.Expr) or L.free_symbols or L.has(sp.Limit) or L.has(sp.Sum) or L.has(sp.AccumBounds):
        raise ValueError("the limit does not exist or could not be determined")
    if L.has(sp.oo, -sp.oo, sp.zoo, sp.nan):
        raise ValueError("the limit is infinite (it does not exist as a finite number)")
    v = complex(sp.N(L, 30))
    if abs(v.imag) > 1e-20 * max(1.0, abs(v.real)) or not math.isfinite(v.real):
        raise ValueError("the limit is not a finite real number")
    return v.real


def _sympy_limit(e, var, point, d=None):
    try:
        return sp.limit(e, var, point) if d is None else sp.limit(e, var, point, d)
    except ValueError as exc:
        raise ValueError(str(exc).split(".")[0] or "the limit does not exist") from None
    except (NotImplementedError, TypeError, RecursionError, ZeroDivisionError, AttributeError):
        raise ValueError("the limit could not be determined") from None


def _vars(p):
    if len(p) == 2:
        return _XY
    if len(p) == 3:
        return _XYZ
    raise ValueError("the point needs 2 or 3 coordinates")


def _eqn(text, names=_XY):
    if not isinstance(text, str) or text.count("=") != 1:
        raise ValueError("write the curve as one equation, e.g. x^2 + y^2 = 25")
    left, right = text.split("=")
    return parse_expr(left.strip(), names) - parse_expr(right.strip(), names)


def _eqn_text(text):
    left, right = text.split("=")
    return f"({left.strip()}) - ({right.strip()})"


@functools.lru_cache(maxsize=512)
def _lam_cached(text, names, module):
    return ref_lam(text, names, module=module)


def ref_cpartial(text, names, point, i):
    """Build-time only: a complex-step partial derivative (no symbolic differentiation), in 50-digit complex
    arithmetic so that an exactly-zero derivative comes out at rounding level 1e-45, not 1e-15."""
    g = _lam_cached(text, tuple(names), "mpmath")
    with mp.workdps(50):
        h = mp.mpf(10) ** -40
        z = [mp.mpc(mp.mpf(v)) for v in point]
        z[i] += mp.mpc(0, h)
        return float(mp.im(g(*z)) / h)


def ref_mpartial(text, names, point, orders, dps=40):
    """Build-time only: mpmath's numerical (finite-difference, high precision) partial derivative."""
    g = ref_lam(text, names)
    with mp.workdps(dps):
        return float(mp.diff(g, tuple(mp.mpf(v) for v in point), tuple(orders)))


def _mp_partial(text, names, point, orders):
    with mp.workdps(30):
        g = _compile(parse_expr(text, names), names)
        return mp.diff(g, tuple(mp.mpf(v) for v in point), tuple(orders))


_GLF: dict = {}


def _glf(n):
    if n not in _GLF:
        _GLF[n] = [(float(x), float(w)) for x, w in _gauss_nodes(n)]
    return _GLF[n]


def _gl1(g, a, b, n):
    c, r = (a + b) / 2, (b - a) / 2
    return r * math.fsum(w * g(c + r * x) for x, w in _glf(n))


def _gl2(g, a, b, lo, hi, n):
    return _gl1(lambda x: _gl1(lambda y: g(x, y), lo(x), hi(x), n), a, b, n)


def _gl3(g, bx, n):
    (a, b), (c, d), (e, f) = bx
    return _gl1(lambda x: _gl1(lambda y: _gl1(lambda z: g(x, y, z), e, f, n), c, d, n), a, b, n)


def _settled(fn, absfn, ns):
    """fn(n) at increasing Gauss-Legendre orders; accepted once two successive orders agree to 1e-12 of the integral
    of |g| (ValueError otherwise: a singular or strongly oscillating integrand is refused, not approximated)."""
    S = max(absfn(ns[0]), 1e-300)
    prev = fn(ns[0])
    for n in ns[1:]:
        v = fn(n)
        if abs(v - prev) <= 1e-12 * S:
            return v, S
        prev = v
    raise ValueError("the integral did not settle numerically (singular or strongly oscillating integrand)")


def _scaled_err(got, want, S, slack=1e-10):
    """Residual of an integral against a recomputation, relative to its size (or a small part of the integral of
    |integrand| when the integral itself nearly cancels)."""
    d = abs(float(got) - float(want)) / max(abs(float(want)), 1e-3 * S, 1e-300)
    return max(0.0, d - slack)


def _pcoef(f):
    return [float(c) for c in poly_of(f).all_coeffs()]


def _np_real_roots(c, lo=-math.inf, hi=math.inf):
    """Build-time only: real roots of a float polynomial by numpy's companion-matrix eigenvalues."""
    import numpy as np
    c = list(c)
    while c and c[0] == 0:
        c.pop(0)
    if len(c) < 2:
        return []
    rs = sorted(float(r.real) for r in np.roots(c) if abs(r.imag) < 1e-7 and lo < r.real < hi)
    out = []
    for r in rs:
        if not out or abs(r - out[-1]) > 1e-7:
            out.append(r)
    return out


def _peval(p, v) -> float:
    return float(p.eval(_rat(v)))


def _root_res(q, c) -> float:
    """|q(c)| relative to the size of q's terms at c (0 for a root, up to rounding of c)."""
    co = q.all_coeffs()
    deg = len(co) - 1
    scale = sum(abs(float(k)) * max(1.0, abs(c)) ** (deg - j) for j, k in enumerate(co)) or 1.0
    return max(0.0, abs(float(q.eval(_rat(c)))) / scale - 1e-12)


def _n_roots_open(q, a=None, b=None) -> int:
    """Number of DISTINCT real roots of a polynomial (in the open interval (a, b) when given), by Sturm counting."""
    if q.is_zero:
        return -1
    s = sp.Poly(sp.sqf_part(q.as_expr()), X)
    if s.degree() < 1:
        return 0
    if a is None:
        return s.count_roots()
    A, B = _rat(a), _rat(b)
    return s.count_roots(A, B) - int(s.eval(A) == 0) - int(s.eval(B) == 0)


def _distinct_real_roots(q):
    return sorted(set(sp.real_roots(q)), key=lambda r: float(r))


def _side_signs(q, xs):
    """Exact sign of q just left and just right of each of the sorted points xs (at rational points between them)."""
    fl = [float(v) for v in xs]
    cuts = [fl[0] - 1] + [(fl[k] + fl[k + 1]) / 2 for k in range(len(fl) - 1)] + [fl[-1] + 1]
    s = [int(sp.sign(q.eval(_rat(c)))) for c in cuts]
    return [(s[k], s[k + 1]) for k in range(len(fl))]


def _poly_from_roots(rng, roots, times=1):
    """A polynomial whose derivative (times=1) or second derivative (times=2) is k * prod(x - r)."""
    k = rng.choice([1, -1, 2, -2, 3])
    p = sp.expand(k * sp.prod([X - r for r in roots]))
    for _ in range(times):
        p = sp.integrate(p, X)
    p = p + (rng.randint(-3, 3) * X if times == 2 else 0) + rng.randint(-5, 5)
    return show(sp.expand(p))


def _sgn(text):
    return text.replace("+ -", "- ").replace("- -", "+ ").replace("--", "+")


def _sh(var, v):
    return f"({var} - {v})" if v >= 0 else f"({var} + {-v})"


def rand2(rng, three=False):
    a, b = _c(rng, -3, 3), _c(rng, 1, 3)
    fams = ([f"x*y*z + {a}*x^2", f"x^2 + {a}*y^2 + {b}*z^2 + x*z", f"exp(x/{b})*sin(y)*z", f"{a}*x*y^2 + z^3 - {b}*y*z",
             f"sin(x + {b}*y)*cos(z)", f"x*exp(-z^2) + {a}*y*z"] if three else
            [f"{a}*x^2*y + {b}*y^3", f"sin({b}*x)*cos(y)", f"x^2 + {a}*x*y + {b}*y^2", f"log(x^2 + y^2 + {b})",
             f"x*exp(-y^2) + {a}*y", f"sqrt(x^2 + y^2 + {b})", f"exp({a}*x/4)*sin(y) + x", f"{a}*x^3 - {b}*x*y + y^2"])
    return _sgn(rng.choice(fams))


def rand_pt(rng, k):
    return [rand_point(rng, -2, 2) for _ in range(k)]


def _tex_vec(v):
    return "(" + ", ".join(f"{x:g}" for x in v) + ")"


# =========================================================================================== 9. limits
def _verify_limit_pt(i, o):
    with mp.workdps(50):
        g = _compile(parse_expr(i["f"]), (X,))
        A, h = _mpq(_rat(i["a"])), mp.mpf(10) ** -12
        v = (g(A + h) + g(A - h)) / 2
    return rel_err(o["limit"], float(v), 1e-10)


def _ref_limit_pt(f, a):
    g = ref_lam(f)
    with mp.workdps(30):
        A, s = _mpq(_rat(a)), mp.mpf("1e-3")
        right = mp.limit(lambda t: g(A + s * t), 0)
        left = mp.limit(lambda t: g(A - s * t), 0)
        return {"limit": float((right + left) / 2)}


@_memo
def limit_at_point(f, a):
    return {"limit": _limval(_sympy_limit(parse_expr(f), X, _rat(a), "+-"))}


def _wrong_limit_pt(f, a):
    try:
        return {"limit": at(parse_expr(f), (X,), a + 0.5)}
    except ValueError:
        return {"limit": limit_at_point(f, a)["limit"] + 1}


def _rand_limit_pt(rng):
    k, m = _c(rng, -4, 4), rng.randint(1, 4)
    kp = abs(k)
    fam = rng.randrange(9)
    if fam == 0:
        return {"f": f"(x^2 - {k * k})/(x - {k})" if k > 0 else f"(x^2 - {k * k})/(x + {-k})", "a": float(k)}
    if fam == 1:
        return {"f": f"sin({k}*x)/x", "a": 0.0}
    if fam == 2:
        return {"f": f"(exp({k}*x) - 1)/x", "a": 0.0}
    if fam == 3:
        return {"f": f"(1 - cos({k}*x))/x^2", "a": 0.0}
    if fam == 4:
        return {"f": f"(x^3 - {kp ** 3})/(x - {kp})", "a": float(kp)}
    if fam == 5:
        return {"f": f"(sqrt(x + {m * m}) - {m})/x", "a": 0.0}
    if fam == 6:
        return {"f": f"tan({k}*x)/sin(x)", "a": 0.0}
    if fam == 7:
        return {"f": f"log(1 + {kp}*x)/x", "a": 0.0}
    return {"f": rand_smooth(rng), "a": rand_point(rng)}


proc("limit_at_point", name="Limit at a point", topic="limits",
     purpose="The two-sided limit of f(x) as x approaches a (including removable 0/0 forms such as sin(x)/x).",
     inputs=[T_("f", "the function f(x)"), N_("a", "the point x = a")],
     outputs=[O_("limit", "the limit of f(x) as x -> a")],
     primary="limit", conditions=["the left and right limits exist, are finite and agree"],
     method="exact symbolic limit (series expansion / Gruntz algorithm) from both sides; refused if the one-sided limits "
            "differ or are infinite",
     latex=r"\lim_{x\to a} f(x)",
     run=limit_at_point, verify=_verify_limit_pt,
     reference=_ref_limit_pt, wrong=_wrong_limit_pt, random_inputs=_rand_limit_pt,
     templates=["Find the limit of (x^2 - 4)/(x - 2) as x approaches 2.", "What is the limit of sin(3x)/x as x -> 0?"],
     examples=[{"inputs": {"f": "(x^2 - 4)/(x - 2)", "a": 2}, "outputs": {"limit": 4.0}},
               {"inputs": {"f": "sin(3x)/x", "a": 0}, "outputs": {"limit": 3.0}},
               {"inputs": {"f": "(1 - cos(x))/x^2", "a": 0}, "outputs": {"limit": 0.5}},
               {"inputs": {"f": "(exp(2x) - 1)/x", "a": 0}, "outputs": {"limit": 2.0}}],
     questions=[{"q": "Find the limit of (x^2 - 4)/(x - 2) as x approaches 2.", "outputs": {"limit": 4.0}},
                {"q": "What is the limit of sin(3x)/x as x -> 0?", "outputs": {"limit": 3.0}},
                {"q": "Evaluate the limit as x approaches 0 of (1 - cos(x))/x^2.", "outputs": {"limit": 0.5}},
                {"q": "Find the limit of 1/x as x approaches 0.", "outcome": "INVALID_INPUT"},
                {"q": "Find the limit of 1/x^2 as x approaches 0.", "outcome": "INVALID_INPUT"}],
     keywords=["limit", "l'hopital"])
_TO = "<(?:approaches|tends to|goes to|-\\x3e)>"     # \x3e is '>', which cannot appear inside <...>
rule("limit_at_point", "limit of {f} as x " + _TO + " {a:n}", "limit as x " + _TO + " {a:n} of {f}", order=40)


@_memo
def limit_at_infinity(f):
    return {"limit": _limval(_sympy_limit(parse_expr(f), X, sp.oo))}


def _verify_limit_inf(i, o):
    with mp.workdps(60):
        v = _compile(parse_expr(i["f"]), (X,))(mp.mpf(10) ** 20)
    return rel_err(o["limit"], float(v), 1e-10)


def _ref_limit_inf(f):
    g = ref_lam(f)
    with mp.workdps(30):
        return {"limit": float(mp.limit(g, mp.inf))}


def _rand_limit_inf(rng):
    a, b, c, d, e, k = _c(rng, -5, 5), _c(rng, -5, 5), _c(rng, -5, 5), rng.randint(1, 5), rng.randint(1, 5), _c(rng, -3, 3)
    return {"f": _sgn(rng.choice([f"({a}*x^2 + {b}*x + {c})/({d}*x^2 + {e})", f"({a}*x^3 + {b})/({d}*x^3 + {e}*x)",
                                  f"(1 + {abs(k)}/x)^x", f"sqrt(x^2 + {k}*x) - x", f"x*sin({k}/x)",
                                  f"({a}*x + {b})/sqrt({d}*x^2 + 1)", f"atan(x) + {c}/x"]))}


proc("limit_at_infinity", name="Limit at infinity", topic="limits",
     purpose="The limit of f(x) as x grows without bound (horizontal asymptote, end behaviour).",
     inputs=[T_("f", "the function f(x)")],
     outputs=[O_("limit", "the limit of f(x) as x -> infinity")],
     primary="limit", conditions=["the limit exists and is finite"],
     method="exact symbolic limit at +infinity (Gruntz algorithm); refused if infinite or oscillating",
     latex=r"\lim_{x\to\infty} f(x)",
     run=limit_at_infinity, verify=_verify_limit_inf, reference=_ref_limit_inf,
     wrong=lambda f: {"limit": at(parse_expr(f), (X,), 100.0)}, random_inputs=_rand_limit_inf,
     templates=["Find the limit of (3x^2 + 1)/(x^2 - 5) as x approaches infinity.",
                "What is the limit of (1 + 2/x)^x as x -> infinity?"],
     examples=[{"inputs": {"f": "(3x^2 + 1)/(x^2 - 5)"}, "outputs": {"limit": 3.0}},
               {"inputs": {"f": "(1 + 2/x)^x"}, "outputs": {"limit": math.e ** 2}},
               {"inputs": {"f": "sqrt(x^2 + 4x) - x"}, "outputs": {"limit": 2.0}}],
     questions=[{"q": "Find the limit of (3x^2 + 1)/(x^2 - 5) as x approaches infinity.", "outputs": {"limit": 3.0}},
                {"q": "What is the limit of (1 + 2/x)^x as x -> infinity?", "outputs": {"limit": math.e ** 2}},
                {"q": "Find the limit as x approaches infinity of sqrt(x^2 + 4x) - x.", "outputs": {"limit": 2.0}},
                {"q": "Find the limit of x^2 as x approaches infinity.", "outcome": "INVALID_INPUT"},
                {"q": "Find the limit of sin(x) as x approaches infinity.", "outcome": "INVALID_INPUT"}],
     keywords=["limit at infinity", "horizontal asymptote", "end behaviour"])
rule("limit_at_infinity", "limit of {f} as x " + _TO + " <(?:\\+|positive )?>infinity",
     "limit as x " + _TO + " <(?:\\+|positive )?>infinity of {f}", order=30)


# =========================================================================================== 10. derivative applications
@_memo
def implicit_derivative(F, p):
    e = _eqn(F)
    x0, y0 = p
    scale = max([1.0] + [abs(at(t, _XY, x0, y0)) for t in sp.Add.make_args(sp.expand(e))])
    if abs(at(e, _XY, x0, y0)) > 1e-9 * scale:
        raise ValueError(f"the point ({x0:g}, {y0:g}) is not on the curve")
    fx, fy = at(sp.diff(e, X), _XY, x0, y0), at(sp.diff(e, Y), _XY, x0, y0)
    if abs(fy) <= 1e-12 * max(1.0, abs(fx)):
        raise ValueError("F_y = 0 at this point: the tangent is vertical (dy/dx is undefined)")
    return {"dy_dx": -fx / fy}


def _verify_implicit(i, o):
    t = _eqn_text(i["F"])
    fx, fy = _mp_partial(t, _XY, i["p"], (1, 0)), _mp_partial(t, _XY, i["p"], (0, 1))
    return rel_err(o["dy_dx"], float(-fx / fy), 1e-11)


def _ref_implicit(F, p):
    t = _eqn_text(F)
    return {"dy_dx": -ref_cpartial(t, _XY, p, 0) / ref_cpartial(t, _XY, p, 1)}


def _rand_implicit(rng):
    a, b = _c(rng, -3, 3), _c(rng, -3, 3)
    x0, y0 = rng.randint(-3, 3), rng.randint(-3, 3)
    G = _sgn(rng.choice([f"x^2 + {a}*x*y + y^3", f"x^3 + y^3 + {a}*x*y", f"x^2*y + {b}*y^2 + x", f"x*y^2 + {a}*x^2 + y",
                         f"sin(x) + y^3 + {a}*y", f"exp(x)*y + y^3 + {b}*x"]))
    c = at(parse_expr(G, _XY), _XY, x0, y0)
    return {"F": f"{G} = {c:.17g}" if not float(c).is_integer() else f"{G} = {int(c)}", "p": [float(x0), float(y0)]}


proc("implicit_derivative", name="Implicit differentiation", topic="derivatives",
     purpose="The slope dy/dx of a curve given implicitly by F(x, y) = c, at a point on the curve.",
     inputs=[T_("F", "the curve as an equation in x and y"), V_("p", "the point (x, y) on the curve", 2, 2)],
     outputs=[O_("dy_dx", "dy/dx at the point")],
     primary="dy_dx", conditions=["the point lies on the curve", "F_y != 0 there (no vertical tangent)"],
     method="dy/dx = -F_x/F_y with exact symbolic partial derivatives of F(x, y) - c",
     latex=r"\frac{dy}{dx} = -\frac{F_x}{F_y}",
     run=implicit_derivative, verify=_verify_implicit, reference=_ref_implicit,
     wrong=lambda F, p: {"dy_dx": -implicit_derivative(F, p)["dy_dx"]}, random_inputs=_rand_implicit,
     templates=["Find dy/dx for x^2 + y^2 = 25 at the point (3, 4).",
                "Use implicit differentiation to find dy/dx for x^3 + y^3 = 9 at (1, 2)."],
     examples=[{"inputs": {"F": "x^2 + y^2 = 25", "p": "3, 4"}, "outputs": {"dy_dx": -0.75}},
               {"inputs": {"F": "x^3 + y^3 = 9", "p": [1, 2]}, "outputs": {"dy_dx": -0.25}}],
     questions=[{"q": "Find dy/dx for x^2 + y^2 = 25 at the point (3, 4).", "outputs": {"dy_dx": -0.75}},
                {"q": "Use implicit differentiation to find dy/dx for x^3 + y^3 = 9 at (1, 2).", "outputs": {"dy_dx": -0.25}},
                {"q": "Find dy/dx for x^2 + y^2 = 25 at the point (1, 1).", "outcome": "INVALID_INPUT"},
                {"q": "Find dy/dx for x^2 + y^2 = 25 at the point (5, 0).", "outcome": "INVALID_INPUT"}],
     keywords=["implicit differentiation", "dy/dx"])
rule("implicit_derivative", "dy/dx <(?:for|of|if|when|given|on)> <(?:the curve )?>{F} at <(?:the point )?>{p:v}",
     "slope of the curve {F} at <(?:the point )?>{p:v}", order=40,
     post=lambda d: d if d["F"].count("=") == 1 and "y" in d["F"] else None)


@_memo
def extrema_on_interval(f, a, b):
    if not a < b:
        raise ValueError("the interval needs a < b")
    p = poly_of(f)
    crit = [_pfloat(r) for r in real_roots_in(p.diff(X), a, b)] if p.degree() >= 2 else []
    vals = [_peval(p, v) for v in [a, b] + crit]
    return {"absolute_max": max(vals), "absolute_min": min(vals), "critical_points": crit}


def _verify_extrema(i, o):
    p, a, b, cs = poly_of(i["f"]), i["a"], i["b"], list(o["critical_points"])
    dp = p.diff(X)
    n = _n_roots_open(dp, a, b) if p.degree() >= 2 else 0
    if len(cs) != n or cs != sorted(cs) or any(not a < c < b for c in cs):
        return 1.0
    vals = [_peval(p, v) for v in [a, b] + cs]
    return worst(*[_root_res(dp, c) for c in cs], rel_err(o["absolute_max"], max(vals), 1e-12),
                 rel_err(o["absolute_min"], min(vals), 1e-12))


def _ref_extrema(f, a, b):
    import numpy as np
    c = _pcoef(f)
    crit = _np_real_roots(np.polyder(c), a, b) if len(c) > 2 else []
    vals = [float(np.polyval(c, v)) for v in [a, b] + crit]
    return {"absolute_max": max(vals), "absolute_min": min(vals), "critical_points": crit}


def _rand_extrema(rng):
    roots = sorted(rng.sample(range(-3, 4), rng.choice([1, 2, 2, 3])))
    a = roots[0] - rng.choice([0.5, 1.5])
    b = roots[-1] + rng.choice([0.5, 1.5]) - (rng.choice([0, 0, 1]) if len(roots) > 1 else 0)
    return {"f": _poly_from_roots(rng, roots), "a": a, "b": b}


proc("extrema_on_interval", name="Absolute extrema on a closed interval", topic="derivatives / optimization",
     purpose="The absolute maximum and minimum values of a polynomial on [a, b] by the closed-interval method, with the "
             "critical points inside (a, b).",
     inputs=[T_("f", "the polynomial f(x)"), N_("a", "left end"), N_("b", "right end")],
     outputs=[O_("absolute_max", "the largest value of f on [a, b]"), O_("absolute_min", "the smallest value"),
              O_("critical_points", "critical points in (a, b), ascending", "vector")],
     primary="absolute_max", conditions=["a < b", "f is a polynomial with rational coefficients"],
     method="exact real roots of f' in (a, b) (Sturm isolation), then f compared at the critical points and both ends",
     latex=r"\max_{[a,b]} f = \max\{f(a), f(b), f(c) : f'(c) = 0\}",
     run=extrema_on_interval, verify=_verify_extrema, reference=_ref_extrema,
     wrong=lambda f, a, b: (lambda p: {"absolute_max": max(_peval(p, a), _peval(p, b)),
                                       "absolute_min": min(_peval(p, a), _peval(p, b)), "critical_points": []})(poly_of(f)),
     random_inputs=_rand_extrema,
     templates=["Find the absolute maximum and minimum of x^3 - 3x on [0, 2].",
                "Find the absolute extrema of x^2 - 4x + 1 on the interval [-1, 3]."],
     examples=[{"inputs": {"f": "x^3 - 3x", "a": 0, "b": 2}, "outputs": {"absolute_max": 2.0, "absolute_min": -2.0, "critical_points": [1.0]}},
               {"inputs": {"f": "x^2 - 4x + 1", "a": -1, "b": 3}, "outputs": {"absolute_max": 6.0, "absolute_min": -3.0}}],
     questions=[{"q": "Find the absolute maximum and minimum of x^3 - 3x on [0, 2].", "outputs": {"absolute_max": 2.0, "absolute_min": -2.0}},
                {"q": "Find the absolute extrema of x^2 - 4x + 1 on the interval [-1, 3].", "outputs": {"absolute_max": 6.0}},
                {"q": "Find the absolute extrema of x^2 on the interval [3, 1].", "outcome": "INVALID_INPUT"},
                {"q": "Find the absolute extrema of sin(x) on the interval [0, 3].", "outcome": "INVALID_INPUT"}],
     keywords=["absolute maximum", "absolute minimum", "closed interval method", "extreme values"])
rule("extrema_on_interval",
     "absolute <(?:maximum and minimum|minimum and maximum|max and min|extrema|extreme values)> <(?:values )?>of {f} on "
     "<(?:the (?:closed )?interval )?>[{a:n}, {b:n}]", order=30)


@_memo
def critical_points_classify(f):
    p = poly_of(f)
    if p.degree() < 2:
        raise ValueError("needs a polynomial of degree 2 or more (a line has no isolated critical points)")
    dp = p.diff(X)
    rs = _distinct_real_roots(dp)
    mx, mn = [], []
    for r, (left, right) in zip(rs, _side_signs(dp, rs) if rs else []):
        if left > 0 > right:
            mx.append(_pfloat(r))
        elif left < 0 < right:
            mn.append(_pfloat(r))
    return {"critical_points": [_pfloat(r) for r in rs], "local_maxima": mx, "local_minima": mn}


def _verify_classify(i, o):
    p = poly_of(i["f"])
    dp = p.diff(X)
    cs = list(o["critical_points"])
    if len(cs) != _n_roots_open(dp) or cs != sorted(cs):
        return 1.0
    mx, mn = [], []
    for c, (left, right) in zip(cs, _side_signs(dp, cs) if cs else []):
        (mx if left > 0 > right else mn if left < 0 < right else []).append(c)
    return worst(*[_root_res(dp, c) for c in cs], _vec_err(o["local_maxima"], mx, 0), _vec_err(o["local_minima"], mn, 0))


def _ref_classify(f):
    import numpy as np
    c = _pcoef(f)
    cs = _np_real_roots(np.polyder(c))
    d2 = np.polyder(c, 2)
    return {"critical_points": cs, "local_maxima": [r for r in cs if np.polyval(d2, r) < 0],
            "local_minima": [r for r in cs if np.polyval(d2, r) > 0]}


proc("critical_points_classify", name="Critical points and their classification", topic="derivatives / optimization",
     purpose="All critical points of a polynomial f(x) and which are local maxima or minima (first-derivative test).",
     inputs=[T_("f", "the polynomial f(x)")],
     outputs=[O_("critical_points", "real solutions of f'(x) = 0, ascending", "vector"),
              O_("local_maxima", "critical points that are local maxima", "vector"),
              O_("local_minima", "critical points that are local minima", "vector")],
     primary="critical_points", conditions=["f is a polynomial of degree >= 2 with rational coefficients"],
     method="exact real roots of f' (Sturm isolation); the sign of f' on each side decides max / min / neither",
     latex=r"f'(c) = 0,\ f' \text{ changes } + \to - \Rightarrow \text{local max}",
     run=critical_points_classify, verify=_verify_classify, reference=_ref_classify,
     wrong=lambda f: (lambda r: dict(r, local_maxima=r["local_minima"], local_minima=r["local_maxima"]))(critical_points_classify(f)),
     random_inputs=lambda rng: {"f": _poly_from_roots(rng, sorted(rng.sample(range(-3, 4), rng.choice([1, 2, 2, 3]))))},
     templates=["Find and classify the critical points of x^3 - 3x.", "Find the local extrema of x^4 - 2x^2."],
     examples=[{"inputs": {"f": "x^3 - 3x"}, "outputs": {"critical_points": [-1.0, 1.0], "local_maxima": [-1.0], "local_minima": [1.0]}},
               {"inputs": {"f": "x^4 - 2x^2"}, "outputs": {"critical_points": [-1.0, 0.0, 1.0], "local_maxima": [0.0], "local_minima": [-1.0, 1.0]}},
               {"inputs": {"f": "x^3"}, "outputs": {"critical_points": [0.0], "local_maxima": [], "local_minima": []}}],
     questions=[{"q": "Find and classify the critical points of x^3 - 3x.", "outputs": {"local_maxima": [-1.0], "local_minima": [1.0]}},
                {"q": "Find the local extrema of x^4 - 2x^2.", "outputs": {"local_minima": [-1.0, 1.0]}},
                {"q": "Classify the critical points of 2x + 1.", "outcome": "INVALID_INPUT"}],
     keywords=["critical points", "local maximum", "local minimum", "first derivative test"])
rule("critical_points_classify", "<(?:and )?>classify the critical points of {f}", "local <(?:extrema|maxima and minima)> of {f}",
     "critical points of {f} and classify them", order=30)


@_memo
def inflection_points(f):
    p = poly_of(f)
    if p.degree() < 2:
        raise ValueError("a polynomial of degree < 2 has no concavity changes")
    d2 = p.diff(X).diff(X)
    if d2.is_zero or d2.degree() < 1:
        return {"inflection_points": []}
    rs = _distinct_real_roots(d2)
    return {"inflection_points": [_pfloat(r) for r, (left, right) in zip(rs, _side_signs(d2, rs) if rs else [])
                                  if left * right < 0]}


def _verify_inflection(i, o):
    p = poly_of(i["f"])
    d2 = p.diff(X).diff(X)
    pts = list(o["inflection_points"])
    if d2.is_zero or d2.degree() < 1:
        return 0.0 if not pts else 1.0
    odd = sp.prod([fac for fac, m in sp.sqf_list(d2.as_expr())[1] if m % 2 == 1])
    q = sp.Poly(odd, X)
    if len(pts) != (_n_roots_open(q) if q.degree() >= 1 else 0) or pts != sorted(pts):
        return 1.0
    return worst(*[_root_res(q, c) for c in pts])


def _ref_inflection(f):
    import numpy as np
    c = _pcoef(f)
    d2, d3 = np.polyder(c, 2), np.polyder(c, 3)
    return {"inflection_points": [r for r in _np_real_roots(d2) if abs(np.polyval(d3, r)) > 1e-9]}


proc("inflection_points", name="Inflection points", topic="derivatives / concavity",
     purpose="The points where a polynomial changes concavity (f'' changes sign).",
     inputs=[T_("f", "the polynomial f(x)")],
     outputs=[O_("inflection_points", "x-values of the inflection points, ascending", "vector")],
     primary="inflection_points", conditions=["f is a polynomial of degree >= 2 with rational coefficients"],
     method="exact real roots of f''; kept only where f'' changes sign (odd multiplicity)",
     latex=r"f''(c) = 0 \text{ and } f'' \text{ changes sign at } c",
     run=inflection_points, verify=_verify_inflection, reference=_ref_inflection,
     wrong=lambda f: {"inflection_points": [_pfloat(r) for r in _distinct_real_roots(poly_of(f).diff(X))]},
     random_inputs=lambda rng: {"f": _poly_from_roots(rng, sorted(rng.sample(range(-3, 4), rng.choice([1, 1, 2, 3]))), 2)},
     templates=["Find the inflection points of x^3 - 3x^2.", "Find the points of inflection of x^4 - 6x^2."],
     examples=[{"inputs": {"f": "x^3 - 3x^2"}, "outputs": {"inflection_points": [1.0]}},
               {"inputs": {"f": "x^4 - 6x^2"}, "outputs": {"inflection_points": [-1.0, 1.0]}},
               {"inputs": {"f": "x^4"}, "outputs": {"inflection_points": []}}],
     questions=[{"q": "Find the inflection points of x^3 - 3x^2.", "outputs": {"inflection_points": [1.0]}},
                {"q": "Find the points of inflection of x^4 - 6x^2.", "outputs": {"inflection_points": [-1.0, 1.0]}},
                {"q": "Find the inflection points of x^4.", "outputs": {"inflection_points": []}}],
     keywords=["inflection point", "concavity"])
rule("inflection_points", "<(?:the )?>inflection points of {f}", "points of inflection of {f}", order=30)


@_memo
def mean_value_theorem(f, a, b):
    if not a < b:
        raise ValueError("the interval needs a < b")
    p = poly_of(f)
    A, B = _rat(a), _rat(b)
    m = (p.eval(B) - p.eval(A)) / (B - A)
    q = p.diff(X) - m
    if q.is_zero:
        raise ValueError("f is linear: every c in (a, b) satisfies the mean value theorem")
    return {"secant_slope": float(m), "c_values": [_pfloat(c) for c in real_roots_in(q, a, b)]}


def _verify_mvt(i, o):
    p, a, b = poly_of(i["f"]), i["a"], i["b"]
    m = (_peval(p, b) - _peval(p, a)) / (b - a)
    A, B = _rat(a), _rat(b)
    q = p.diff(X) - (p.eval(B) - p.eval(A)) / (B - A)
    cs = list(o["c_values"])
    if len(cs) != _n_roots_open(q, a, b) or cs != sorted(cs) or any(not a < c < b for c in cs):
        return 1.0
    return worst(rel_err(o["secant_slope"], m, 1e-12), *[_root_res(q, c) for c in cs])


def _ref_mvt(f, a, b):
    import numpy as np
    c = _pcoef(f)
    m = (np.polyval(c, b) - np.polyval(c, a)) / (b - a)
    d = list(np.polyder(c))
    d[-1] -= m
    return {"secant_slope": float(m), "c_values": _np_real_roots(d, a, b)}


def _rand_mvt(rng):
    for _ in range(60):
        f = rand_poly(rng, rng.choice([2, 3, 3, 3]))
        a = rng.randint(-3, 1) + rng.choice([0, 0.5])
        b = a + rng.choice([1, 1.5, 2, 3, 4])
        p = poly_of(f)
        A, B = _rat(a), _rat(b)
        q = p.diff(X) - (p.eval(B) - p.eval(A)) / (B - A)
        if q.degree() < 1 or sp.gcd(q, q.diff(X)).degree() > 0 or q.eval(A) == 0 or q.eval(B) == 0:
            continue
        cs = [float(c) for c in real_roots_in(q, a, b)]
        if all(min(c - a, b - c) > 1e-3 for c in cs) and all(y - x > 1e-3 for x, y in zip(cs, cs[1:])):
            return {"f": f, "a": float(a), "b": float(b)}
    return {"f": "x^3", "a": 0.0, "b": 3.0}


proc("mean_value_theorem", name="Mean value theorem", topic="derivatives",
     purpose="The secant slope of a polynomial on [a, b] and every c in (a, b) where f'(c) equals it.",
     inputs=[T_("f", "the polynomial f(x)"), N_("a", "left end"), N_("b", "right end")],
     outputs=[O_("secant_slope", "(f(b) - f(a))/(b - a)"), O_("c_values", "all c in (a, b) with f'(c) = the slope", "vector")],
     primary="secant_slope", conditions=["a < b", "f a non-linear polynomial with rational coefficients"],
     method="exact secant slope, then the exact real roots of f'(x) - slope in (a, b)",
     latex=r"f'(c) = \frac{f(b) - f(a)}{b - a}",
     run=mean_value_theorem, verify=_verify_mvt, reference=_ref_mvt,
     wrong=lambda f, a, b: dict(mean_value_theorem(f, a, b), c_values=[(a + b) / 2]), random_inputs=_rand_mvt,
     templates=["Find the value of c guaranteed by the mean value theorem for x^3 on [0, 3].",
                "Apply the mean value theorem to x^2 on [0, 2]."],
     examples=[{"inputs": {"f": "x^2", "a": 0, "b": 2}, "outputs": {"secant_slope": 2.0, "c_values": [1.0]}},
               {"inputs": {"f": "x^3", "a": 0, "b": 3}, "outputs": {"secant_slope": 9.0, "c_values": [math.sqrt(3)]}}],
     questions=[{"q": "Find the value of c guaranteed by the mean value theorem for x^3 on [0, 3].", "outputs": {"c_values": [math.sqrt(3)]}},
                {"q": "Apply the mean value theorem to x^2 on [0, 2].", "outputs": {"c_values": [1.0]}},
                {"q": "Apply the mean value theorem to 3x + 1 on [0, 2].", "outcome": "INVALID_INPUT"}],
     keywords=["mean value theorem"])
rule("mean_value_theorem",
     "<(?:value|values|number|numbers) of >c <(?:guaranteed by|that satisfies|that satisfy|satisfying|given by)> the mean "
     "value theorem <(?:for|on)> {f} on <(?:the interval )?>[{a:n}, {b:n}]",
     "<(?:apply |use )?(?:the )?>mean value theorem <(?:to|for|on)> {f} on <(?:the interval )?>[{a:n}, {b:n}]", order=30)


@_memo
def ftc_derivative(f, a, g, x0):
    ef, eg = parse_expr(f, ("t",)), parse_expr(g)
    u = at(eg, (X,), x0)
    continuous_on(ef, T, a, u)
    return {"derivative": at(ef, (T,), u) * at(sp.diff(eg, X), (X,), x0)}


def _verify_ftc(i, o):
    with mp.workdps(30):
        F, G = _compile(parse_expr(i["f"], ("t",)), (T,)), _compile(parse_expr(i["g"]), (X,))
        x0 = mp.mpf(i["x0"])
        v = F(G(x0)) * mp.diff(G, x0)
    return rel_err(o["derivative"], float(v), 1e-11)


def _ref_ftc(f, a, g, x0):
    F, G = ref_lam(f, ("t",)), ref_lam(g)
    with mp.workdps(25):
        return {"derivative": float(mp.diff(lambda x: mp.quad(F, [mp.mpf(a), G(x)]), mp.mpf(x0)))}


def _rand_ftc(rng):
    k = rng.randint(1, 3)
    while True:
        d = {"f": rng.choice(["sin(t)", f"t^2 + {k}", "exp(-t^2)", f"cos({k}*t)", "1/(1 + t^2)", "sqrt(1 + t^2)", f"t*exp(t/{k + 1})"]),
             "a": float(rng.randint(-1, 1)), "g": rng.choice(["x^2", f"{k}*x", "sin(x)", "x^3", f"x^2 + {k}", "exp(x/2)"]),
             "x0": rand_point(rng, -1, 1)}
        try:
            if abs(ftc_derivative(**d)["derivative"]) > 0.05:
                return d
        except ValueError:
            pass


proc("ftc_derivative", name="Derivative of an integral (fundamental theorem)", topic="integrals",
     purpose="d/dx of the integral of f(t) from a to g(x), evaluated at x = x0 (first fundamental theorem with the "
             "chain rule).",
     inputs=[T_("f", "the integrand f(t)"), N_("a", "the constant lower limit"), T_("g", "the upper limit g(x)"),
             N_("x0", "the point x = x0")],
     outputs=[O_("derivative", "d/dx of the integral at x0")],
     primary="derivative", conditions=["f continuous between a and g(x0)", "g differentiable at x0"],
     method="f(g(x0)) * g'(x0) with the exact derivative of g",
     latex=r"\frac{d}{dx}\int_a^{g(x)} f(t)\,dt = f(g(x))\,g'(x)",
     run=ftc_derivative, verify=_verify_ftc, reference=_ref_ftc,
     wrong=lambda f, a, g, x0: {"derivative": at(parse_expr(f, ("t",)), (T,), at(parse_expr(g), (X,), x0))},
     random_inputs=_rand_ftc,
     templates=["Find the derivative of the integral from 0 to x^2 of sin(t) dt at x = 1.",
                "What is d/dx of the integral from 1 to 3x of t^2 + 1 dt at x = 2?"],
     examples=[{"inputs": {"f": "sin(t)", "a": 0, "g": "x^2", "x0": 1}, "outputs": {"derivative": 2 * math.sin(1)}},
               {"inputs": {"f": "t^2 + 1", "a": 1, "g": "3x", "x0": 2}, "outputs": {"derivative": 111.0}}],
     questions=[{"q": "Find the derivative of the integral from 0 to x^2 of sin(t) dt at x = 1.", "outputs": {"derivative": 2 * math.sin(1)}},
                {"q": "What is d/dx of the integral from 1 to 3x of t^2 + 1 dt at x = 2?", "outputs": {"derivative": 111.0}},
                {"q": "Find the derivative of the integral from 1 to x of 1/t dt at x = -1.", "outcome": "INVALID_INPUT"}],
     keywords=["fundamental theorem of calculus", "derivative of an integral"])
rule("ftc_derivative",
     "<(?:derivative (?:with respect to x )?of|d/dx of|d/dx) >the integral from {a:n} to {g} of {f} dt at x = {x0:n}", order=10)


# =========================================================================================== 11. sequences and series
def _frac(v):
    return Fraction(repr(float(v)))


def geometric_series_sum(a, r):
    if not abs(r) < 1:
        raise ValueError("an infinite geometric series converges only when |r| < 1")
    return {"sum": float(_frac(a) / (1 - _frac(r)))}


proc("geometric_series_sum", name="Sum of an infinite geometric series", topic="series",
     purpose="The sum a + a r + a r^2 + ... of a convergent geometric series.",
     inputs=[N_("a", "the first term"), N_("r", "the common ratio")],
     outputs=[O_("sum", "the sum of the series")],
     primary="sum", conditions=["|r| < 1"], method="a/(1 - r), exact in the stated decimals",
     latex=r"\sum_{n=0}^{\infty} a r^n = \frac{a}{1-r}",
     run=geometric_series_sum,
     verify=lambda i, o: rel_err(o["sum"] * (1 - i["r"]), i["a"], 1e-14),
     reference=lambda a, r: (lambda np: {"sum": float(np.sum(a * np.power(float(r), np.arange(4000))))})(__import__("numpy")),
     wrong=lambda a, r: {"sum": a / (1 + r)},
     random_inputs=lambda rng: {"a": float(_c(rng, -10, 10)) * rng.choice([1, 0.5]),
                                "r": rng.choice([1, -1]) * round(rng.uniform(0.1, 0.9), 2)},
     templates=["Find the sum of the geometric series with first term 3 and ratio 0.5.",
                "What is the sum of the infinite geometric series with first term 1 and common ratio -1/3?"],
     examples=[{"inputs": {"a": 3, "r": 0.5}, "outputs": {"sum": 6.0}}, {"inputs": {"a": 1, "r": -0.25}, "outputs": {"sum": 0.8}}],
     questions=[{"q": "Find the sum of the geometric series with first term 3 and ratio 0.5.", "outputs": {"sum": 6.0}},
                {"q": "What is the sum of the infinite geometric series with first term 1 and common ratio -1/3?", "outputs": {"sum": 0.75}},
                {"q": "Find the sum of the geometric series with first term 3 and ratio 2.", "outcome": "INVALID_INPUT"}],
     keywords=["geometric series"])
rule("geometric_series_sum",
     "sum of <(?:the |an )?(?:infinite )?>geometric series with first term {a:n} and <(?:common )?>ratio {r:n}", order=30)


def geometric_partial_sum(a, r, n):
    n = int(n)
    if not 1 <= n <= 500:
        raise ValueError("n must be between 1 and 500")
    A, R = _frac(a), _frac(r)
    return {"sum": float(n * A if R == 1 else A * (1 - R ** n) / (1 - R))}


proc("geometric_partial_sum", name="Partial sum of a geometric series", topic="series",
     purpose="The sum of the first n terms a + a r + ... + a r^(n-1).",
     inputs=[N_("a", "the first term"), N_("r", "the common ratio"), I_("n", "the number of terms", 1, 500)],
     outputs=[O_("sum", "the sum of the first n terms")],
     primary="sum", conditions=["1 <= n <= 500"], method="a (1 - r^n)/(1 - r) (n a when r = 1), exact in the stated decimals",
     latex=r"S_n = a\,\frac{1 - r^n}{1 - r}",
     run=geometric_partial_sum,
     verify=lambda i, o: rel_err(o["sum"], math.fsum(i["a"] * i["r"] ** k for k in range(int(i["n"]))), 1e-12),
     reference=lambda a, r, n: (lambda np: {"sum": float(np.sum(a * np.power(float(r), np.arange(int(n)))))})(__import__("numpy")),
     wrong=lambda a, r, n: geometric_partial_sum(a, r, int(n) + 1),
     random_inputs=lambda rng: {"a": float(_c(rng, -10, 10)), "r": rng.choice([1, -1]) * round(rng.uniform(0.1, 0.95), 2) if rng.random() < 0.7
                                else float(rng.choice([2, 3, -2, 1.5])), "n": rng.randint(2, 30)},
     templates=["Find the sum of the first 4 terms of the geometric series with first term 2 and ratio 3."],
     examples=[{"inputs": {"a": 2, "r": 3, "n": 4}, "outputs": {"sum": 80.0}}, {"inputs": {"a": 1, "r": 0.5, "n": 3}, "outputs": {"sum": 1.75}},
               {"inputs": {"a": 5, "r": 1, "n": 6}, "outputs": {"sum": 30.0}}],
     questions=[{"q": "Find the sum of the first 4 terms of the geometric series with first term 2 and ratio 3.", "outputs": {"sum": 80.0}},
                {"q": "What is the sum of the first 3 terms of a geometric sequence with first term 1 and common ratio 0.5?", "outputs": {"sum": 1.75}}],
     keywords=["geometric series", "partial sum"])
rule("geometric_partial_sum",
     "sum of the first {n:i} terms of <(?:the |a )?>geometric <(?:series|sequence|progression)> with first term {a:n} and "
     "<(?:common )?>ratio {r:n}", order=25)


def arithmetic_series_sum(a1, d, n):
    n = int(n)
    if not 1 <= n <= 10 ** 6:
        raise ValueError("n must be between 1 and 1000000")
    A, D = _frac(a1), _frac(d)
    return {"sum": float(Fraction(n, 2) * (2 * A + (n - 1) * D)), "last_term": float(A + (n - 1) * D)}


proc("arithmetic_series_sum", name="Sum of an arithmetic series", topic="series",
     purpose="The sum of the first n terms of an arithmetic sequence, and its last term.",
     inputs=[N_("a1", "the first term"), N_("d", "the common difference"), I_("n", "the number of terms", 1, 10 ** 6)],
     outputs=[O_("sum", "the sum of the first n terms"), O_("last_term", "the n-th term")],
     primary="sum", conditions=["n >= 1"], method="S_n = n (2 a_1 + (n - 1) d)/2, exact in the stated decimals",
     latex=r"S_n = \frac{n}{2}\left(2a_1 + (n-1)d\right)",
     run=arithmetic_series_sum,
     verify=lambda i, o: worst(rel_err(o["last_term"], i["a1"] + (int(i["n"]) - 1) * i["d"], 1e-12),
                               rel_err(o["sum"], int(i["n"]) * (i["a1"] + o["last_term"]) / 2, 1e-12)),
     reference=lambda a1, d, n: (lambda np: (lambda t: {"sum": float(np.sum(t)), "last_term": float(t[-1])})(
         a1 + d * np.arange(int(n), dtype=float)))(__import__("numpy")),
     wrong=lambda a1, d, n: {"sum": int(n) * (2 * a1 + int(n) * d) / 2, "last_term": a1 + int(n) * d},
     random_inputs=lambda rng: {"a1": rng.randint(-20, 20) * rng.choice([1, 0.5]), "d": _c(rng, -6, 6) * rng.choice([1, 0.5]),
                                "n": rng.randint(2, 300)},
     templates=["Find the sum of the first 10 terms of the arithmetic series with first term 2 and common difference 3."],
     examples=[{"inputs": {"a1": 2, "d": 3, "n": 10}, "outputs": {"sum": 155.0, "last_term": 29.0}},
               {"inputs": {"a1": 1, "d": 1, "n": 100}, "outputs": {"sum": 5050.0}}],
     questions=[{"q": "Find the sum of the first 10 terms of the arithmetic series with first term 2 and common difference 3.", "outputs": {"sum": 155.0}},
                {"q": "What is the sum of the first 100 terms of an arithmetic sequence with first term 1 and difference 1?", "outputs": {"sum": 5050.0}}],
     keywords=["arithmetic series", "arithmetic progression"])
rule("arithmetic_series_sum",
     "sum of the first {n:i} terms of <(?:the |an )?>arithmetic <(?:series|sequence|progression)> with first term {a1:n} "
     "and <(?:common )?>difference {d:n}", order=25)


def _series_terms(a):
    return parse_expr(a, ("n",)).subs(NN, _KS)


@_memo
def series_sum(a, k):
    k = int(k)
    if not 0 <= k <= 1000:
        raise ValueError("the starting index must be between 0 and 1000")
    e = _series_terms(a)
    at(e.subs(_KS, k), ())
    try:
        S = sp.summation(e, (_KS, k, sp.oo))
    except (NotImplementedError, ValueError, TypeError, RecursionError, ZeroDivisionError):
        raise ValueError("the sum could not be found in closed form") from None
    if S.has(sp.oo, -sp.oo, sp.zoo):
        raise ValueError("the series diverges")
    if S.has(sp.Sum) or S.has(sp.Piecewise):
        raise ValueError("no closed form was found for this series (or it does not converge)")
    return {"sum": _limval(S)}


@functools.lru_cache(maxsize=256)
def _nsum(a, k, lam=False):
    """A numerical sum of the series: direct summation when the terms decay geometrically, Shanks/Richardson
    extrapolation for alternating terms, Euler-Maclaurin otherwise (mpmath, 25 digits)."""
    g = ref_lam(a, ("n",)) if lam else None
    with mp.workdps(25):
        h = g if g is not None else _compile(parse_expr(a, ("n",)), ("n",))
        k = int(k)
        t = [h(mp.mpf(j)) for j in range(k, k + 4)]
        far = [abs(h(mp.mpf(j))) for j in (k + 60, k + 61)]
        if far[0] and far[1] / far[0] < 0.95 and all(t):                   # geometric decay: sum directly
            total, j = mp.mpf(0), k
            while j < k + 20000:
                term = h(mp.mpf(j))
                total += term
                if j > k + 70 and abs(term) < mp.mpf(10) ** -24 * max(abs(total), mp.mpf(10) ** -30):
                    return total
                j += 1
        alternating = all(t[j] * t[j + 1] < 0 for j in range(3))
        return mp.nsum(lambda n: h(n), [k, mp.inf], method="r+s" if alternating else "e")


def _rand_series(rng):
    c, m, p, q, b = _c(rng, -3, 3), rng.randint(1, 3), rng.randint(1, 3), rng.randint(2, 5), rng.randint(2, 5)
    p = min(p, q - 1)
    return rng.choice([{"a": f"1/n^{rng.choice([2, 4])}", "k": 1}, {"a": f"{c}/(n*(n + {m}))", "k": 1},
                       {"a": f"{c}*({p}/{q})^n", "k": rng.randint(0, 2)}, {"a": f"n/{b}^n", "k": 1},
                       {"a": "1/((2*n - 1)*(2*n + 1))", "k": 1}, {"a": "(-1)^(n + 1)/n", "k": 1},
                       {"a": f"1/{b}^n + 1/n^2", "k": 1}, {"a": f"n^2/{b}^n", "k": 1}])


proc("series_sum", name="Sum of an infinite series", topic="series",
     purpose="The exact sum of a convergent series sum_{n=k}^infinity a_n (telescoping, geometric-type, zeta values, "
             "alternating harmonic, ...).",
     inputs=[T_("a", "the general term a_n, in n"), I_("k", "the starting index", 0, 1000)],
     outputs=[O_("sum", "the sum of the series")],
     primary="sum", conditions=["the series converges", "a closed form exists"],
     method="exact symbolic summation; refused if the series diverges or no closed form is found",
     latex=r"\sum_{n=k}^{\infty} a_n",
     run=series_sum,
     verify=lambda i, o: rel_err(o["sum"], float(_nsum(i["a"], i["k"])), 1e-10),
     reference=lambda a, k: {"sum": float(_nsum(a, k, lam=True))},
     wrong=lambda a, k: {"sum": float(math.fsum(at(_series_terms(a).subs(_KS, j), ()) for j in range(int(k), int(k) + 10)))},
     random_inputs=_rand_series,
     templates=["Find the sum of the series 1/n^2 from n = 1 to infinity.",
                "Find the sum of 1/(n*(n + 1)) from n = 1 to infinity."],
     examples=[{"inputs": {"a": "1/n^2", "k": 1}, "outputs": {"sum": math.pi ** 2 / 6}},
               {"inputs": {"a": "1/(n*(n + 1))", "k": 1}, "outputs": {"sum": 1.0}},
               {"inputs": {"a": "(1/2)^n", "k": 0}, "outputs": {"sum": 2.0}},
               {"inputs": {"a": "(-1)^(n + 1)/n", "k": 1}, "outputs": {"sum": math.log(2)}}],
     questions=[{"q": "Find the sum of the series 1/n^2 from n = 1 to infinity.", "outputs": {"sum": math.pi ** 2 / 6}},
                {"q": "Find the sum of 1/(n*(n + 1)) from n = 1 to infinity.", "outputs": {"sum": 1.0}},
                {"q": "Find the sum of the series 1/n from n = 1 to infinity.", "outcome": "INVALID_INPUT"}],
     keywords=["infinite series", "telescoping series", "sum of a series"])
rule("series_sum", "sum of <(?:the )?(?:infinite )?(?:series )?>{a} from n = {k:i} to <(?:infinity|oo)>", order=35)


@_memo
def ratio_test(a):
    e = parse_expr(a, ("n",))
    if e.subs(NN, 1) == 0 and e.subs(NN, 2) == 0:
        raise ValueError("the terms are zero")
    r = sp.simplify(e.subs(NN, NN + 1) / e)
    L = _limval(_sympy_limit(sp.Abs(r), NN, sp.oo))
    if abs(L - 1) < 1e-12:
        raise ValueError("the ratio limit is 1: the ratio test is inconclusive")
    return {"ratio_limit": L, "verdict": "converges" if L < 1 else "diverges"}


def _verify_ratio(i, o):
    with mp.workdps(40):
        g = _compile(parse_expr(i["a"], ("n",)), ("n",))
        N = mp.mpf(10) ** 12
        L = abs(g(N + 1) / g(N))
    return worst(rel_err(o["ratio_limit"], float(L), 1e-10), _same(o["verdict"], "converges" if o["ratio_limit"] < 1 else "diverges"))


def _ref_ratio(a):
    g = ref_lam(a, ("n",))
    with mp.workdps(30):
        L = float(mp.limit(lambda n: abs(g(n + 1) / g(n)), mp.inf))
    return {"ratio_limit": L, "verdict": "converges" if L < 1 else "diverges"}


def _rand_ratio(rng):
    c = rng.choice(["2", "3", "1/2", "2/3", "3/2", "1/3", "5/4", "4/5"])
    p, b = rng.randint(1, 3), rng.randint(2, 5)
    return {"a": rng.choice([f"({c})^n/n^{p}", f"n^{p}*({c})^n", f"n^{p}/{b}^n", f"({c})^n/(n^2 + 1)", f"{b}^n/(n^{p} + 1)"])}


proc("ratio_test", name="Ratio test", topic="series",
     purpose="The limit L of |a_(n+1)/a_n| and the ratio test's verdict for the series sum a_n (converges if L < 1, "
             "diverges if L > 1).",
     inputs=[T_("a", "the general term a_n, in n")],
     outputs=[O_("ratio_limit", "L = lim |a_(n+1)/a_n|"), O_("verdict", "converges or diverges", "text")],
     primary="ratio_limit", conditions=["L exists, is finite and is not 1 (L = 1 is inconclusive and refused)"],
     method="exact simplification of a_(n+1)/a_n and its symbolic limit as n -> infinity",
     latex=r"L = \lim_{n\to\infty}\left|\frac{a_{n+1}}{a_n}\right|",
     run=ratio_test, verify=_verify_ratio, reference=_ref_ratio,
     wrong=lambda a: (lambda r: {"ratio_limit": 1 / r["ratio_limit"], "verdict": "diverges" if r["verdict"] == "converges" else "converges"})(ratio_test(a)),
     random_inputs=_rand_ratio,
     templates=["Apply the ratio test to the series 2^n/n^3.", "Use the ratio test on n^2/3^n."],
     examples=[{"inputs": {"a": "2^n/n^3"}, "outputs": {"ratio_limit": 2.0, "verdict": "diverges"}},
               {"inputs": {"a": "n^2/3^n"}, "outputs": {"ratio_limit": 1 / 3, "verdict": "converges"}}],
     questions=[{"q": "Apply the ratio test to the series 2^n/n^3.", "outputs": {"verdict": "diverges"}},
                {"q": "Use the ratio test on n^2/3^n.", "outputs": {"ratio_limit": 1 / 3, "verdict": "converges"}},
                {"q": "Apply the ratio test to the series 1/n^2.", "outcome": "INVALID_INPUT"}],
     keywords=["ratio test", "convergence test"])
rule("ratio_test", "<(?:apply |use )?(?:the )?>ratio test <(?:to|for|on)> <(?:the series )?>{a}", order=30)


@_memo
def taylor_polynomial(f, a, n, x1):
    n = int(n)
    if not 0 <= n <= 12:
        raise ValueError("the degree must be between 0 and 12")
    e = parse_expr(f)
    co, d = [], e
    for k in range(n + 1):
        co.append(at(d, (X,), a) / math.factorial(k))
        d = sp.diff(d, X)
    h, val = x1 - a, 0.0
    for c in reversed(co):
        val = val * h + c
    fx1 = at(e, (X,), x1)
    return {"coefficients": co, "value": val, "function_value": fx1, "error": fx1 - val}


def _verify_taylor(i, o):
    co, n, h = list(o["coefficients"]), int(i["n"]), i["x1"] - i["a"]
    if len(co) != n + 1:
        return 1.0
    with mp.workdps(30):
        g = _compile(parse_expr(i["f"]), (X,))
        ref = mp.taylor(g, mp.mpf(i["a"]), n)
        fx1 = g(mp.mpf(i["x1"]))
    val = 0.0
    for c in reversed(co):
        val = val * h + c
    return worst(_vec_err(co, [float(c) for c in ref], 1e-11), rel_err(o["value"], val, 1e-12),
                 rel_err(o["function_value"], float(fx1), 1e-12), abs(o["error"] - (o["function_value"] - o["value"])))


def _ref_taylor(f, a, n, x1):
    g = ref_lam(f)
    with mp.workdps(40):
        co = [float(c) for c in mp.taylor(g, mp.mpf(a), int(n))]
        return {"coefficients": co, "value": float(mp.polyval(co[::-1], mp.mpf(x1) - a)), "function_value": float(g(mp.mpf(x1)))}


proc("taylor_polynomial", name="Taylor polynomial", topic="series / Taylor",
     purpose="The degree-n Taylor polynomial of f about x = a (Maclaurin when a = 0): its coefficients, its value at "
             "x1, f(x1) and the truncation error.",
     inputs=[T_("f", "the function f(x)"), N_("a", "the centre x = a"), I_("n", "the degree", 0, 12), N_("x1", "the evaluation point")],
     outputs=[O_("coefficients", "c_0 .. c_n with c_k = f^(k)(a)/k!", "vector"), O_("value", "P_n(x1)"),
              O_("function_value", "f(x1)"), O_("error", "f(x1) - P_n(x1)")],
     primary="value", conditions=["f is n times differentiable at a and defined at x1", "0 <= n <= 12"],
     method="exact symbolic derivatives at a divided by k!, then Horner evaluation in (x1 - a)",
     latex=r"P_n(x) = \sum_{k=0}^{n}\frac{f^{(k)}(a)}{k!}(x-a)^k",
     run=taylor_polynomial, verify=_verify_taylor, reference=_ref_taylor,
     wrong=lambda f, a, n, x1: (lambda r: (lambda co: dict(r, coefficients=co, value=sum(c * (x1 - a) ** k for k, c in enumerate(co))))(
         [c * math.factorial(k) for k, c in enumerate(r["coefficients"])]))(taylor_polynomial(f, a, n, x1)),
     random_inputs=lambda rng: (lambda a: {"f": rand_smooth(rng), "a": a, "n": rng.randint(2, 6),
                                           "x1": round(a + rng.choice([-1, 1]) * rng.uniform(0.1, 0.6), 2)})(rand_point(rng, -2, 2)),
     templates=["Find the Taylor polynomial of degree 3 of exp(x) about x = 0 and evaluate it at x = 0.5.",
                "Evaluate the degree 4 Maclaurin polynomial of cos(x) at x = 0.3."],
     examples=[{"inputs": {"f": "exp(x)", "a": 0, "n": 3, "x1": 0.5}, "outputs": {"coefficients": [1, 1, 0.5, 1 / 6], "value": 1 + 0.5 + 0.125 + 0.125 / 6}},
               {"inputs": {"f": "cos(x)", "a": 0, "n": 4, "x1": 0.3}, "outputs": {"value": 1 - 0.045 + 0.0081 / 24}},
               {"inputs": {"f": "log(x)", "a": 1, "n": 2, "x1": 1.5}, "outputs": {"coefficients": [0, 1, -0.5], "value": 0.375}}],
     questions=[{"q": "Find the Taylor polynomial of degree 3 of exp(x) about x = 0 and evaluate it at x = 0.5.", "outputs": {"value": 1 + 0.5 + 0.125 + 0.125 / 6}},
                {"q": "Evaluate the degree 4 Maclaurin polynomial of cos(x) at x = 0.3.", "outputs": {"value": 1 - 0.045 + 0.0081 / 24}},
                {"q": "Find the Taylor polynomial of degree 2 of log(x) about x = 0 and evaluate it at x = 0.5.", "outcome": "INVALID_INPUT"}],
     keywords=["taylor polynomial", "maclaurin polynomial", "taylor series"])
rule("taylor_polynomial",
     "taylor polynomial of degree {n:i} <(?:of|for)> {f} <(?:about|at|centered at|centred at|around)> x = {a:n} and "
     "evaluate <(?:it )?>at x = {x1:n}", order=20)
rule("taylor_polynomial", "degree {n:i} maclaurin polynomial of {f} at x = {x1:n}",
     "maclaurin polynomial of degree {n:i} <(?:of|for)> {f} at x = {x1:n}", fixed={"a": 0.0}, order=20)


# =========================================================================================== 12. multivariable calculus
@_memo
def partial_derivative(f, var, p):
    names = _vars(p)
    if var not in names:
        raise ValueError(f"the variable must be one of {', '.join(names)}")
    e = parse_expr(f, names)
    at(e, names, *p)
    return {"partial": at(sp.diff(e, _SYMS[var]), names, *p)}


proc("partial_derivative", name="Partial derivative", topic="multivariable calculus",
     purpose="The partial derivative of f(x, y) or f(x, y, z) with respect to one variable, at a point.",
     inputs=[T_("f", "the function f(x, y) or f(x, y, z)"), C_("var", "the variable", ["x", "y", "z"]),
             V_("p", "the point (x, y) or (x, y, z)")],
     outputs=[O_("partial", "the partial derivative at the point")],
     primary="partial", conditions=["f differentiable at the point"],
     method="exact symbolic partial differentiation, then evaluation at the point",
     latex=r"\frac{\partial f}{\partial x}(p)",
     run=partial_derivative,
     verify=lambda i, o: rel_err(o["partial"], float(_mp_partial(i["f"], _vars(i["p"]), i["p"],
                                                               [int(v == i["var"]) for v in _vars(i["p"])])), 1e-11),
     reference=lambda f, var, p: {"partial": ref_cpartial(f, _vars(p), p, "xyz".index(var))},
     wrong=lambda f, var, p: partial_derivative(f, "y" if var == "x" else "x", p),
     random_inputs=lambda rng: (lambda three: {"f": rand2(rng, three), "var": rng.choice(["x", "y", "z"] if three else ["x", "y"]),
                                               "p": rand_pt(rng, 3 if three else 2)})(rng.random() < 0.35),
     templates=["Find the partial derivative of x^2*y + y^3 with respect to x at (1, 2).",
                "What is the partial derivative of x*y*z with respect to z at the point (1, 2, 3)?"],
     examples=[{"inputs": {"f": "x^2*y + y^3", "var": "x", "p": "1, 2"}, "outputs": {"partial": 4.0}},
               {"inputs": {"f": "x^2*y + y^3", "var": "y", "p": [1, 2]}, "outputs": {"partial": 13.0}},
               {"inputs": {"f": "x*y*z", "var": "z", "p": "1, 2, 3"}, "outputs": {"partial": 2.0}}],
     questions=[{"q": "Find the partial derivative of x^2*y + y^3 with respect to x at (1, 2).", "outputs": {"partial": 4.0}},
                {"q": "What is the partial derivative of x*y*z with respect to z at the point (1, 2, 3)?", "outputs": {"partial": 2.0}},
                {"q": "Find the partial derivative of x^2*y with respect to z at (1, 2).", "outcome": "INVALID_INPUT"}],
     keywords=["partial derivative"])
rule("partial_derivative", "partial derivative of {f} with respect to {var:w} at <(?:the point )?>{p:v}", order=20,
     post=lambda d: d if d["var"] in ("x", "y", "z") else None)


@_memo
def gradient(f, p):
    names = _vars(p)
    e = parse_expr(f, names)
    at(e, names, *p)
    g = [at(sp.diff(e, _SYMS[v]), names, *p) for v in names]
    return {"gradient": g, "magnitude": math.sqrt(math.fsum(c * c for c in g))}


def _verify_gradient(i, o):
    names = _vars(i["p"])
    g = [float(_mp_partial(i["f"], names, i["p"], [int(k == j) for k in range(len(names))])) for j in range(len(names))]
    return worst(_vec_err(o["gradient"], g, 1e-11), rel_err(o["magnitude"], math.sqrt(math.fsum(c * c for c in g)), 1e-11))


def _ref_gradient(f, p):
    import numpy as np
    g = [ref_cpartial(f, _vars(p), p, j) for j in range(len(p))]
    return {"gradient": g, "magnitude": float(np.linalg.norm(g))}


proc("gradient", name="Gradient", topic="multivariable calculus",
     purpose="The gradient vector of f(x, y) or f(x, y, z) at a point and its length (the greatest rate of increase).",
     inputs=[T_("f", "the function f(x, y) or f(x, y, z)"), V_("p", "the point (x, y) or (x, y, z)")],
     outputs=[O_("gradient", "the gradient vector", "vector"), O_("magnitude", "its length")],
     primary="magnitude", conditions=["f differentiable at the point"],
     method="exact symbolic partial derivatives at the point", latex=r"\nabla f = \left(f_x, f_y, f_z\right)",
     run=gradient, verify=_verify_gradient, reference=_ref_gradient,
     wrong=lambda f, p: (lambda r: dict(r, gradient=r["gradient"][::-1] if len(set(r["gradient"])) > 1 else [-c for c in r["gradient"]]))(gradient(f, p)),
     random_inputs=lambda rng: (lambda three: {"f": rand2(rng, three), "p": rand_pt(rng, 3 if three else 2)})(rng.random() < 0.35),
     templates=["Find the gradient of x^2 + y^2 at (1, 2).", "What is the gradient of x*y*z at the point (1, 2, 3)?"],
     examples=[{"inputs": {"f": "x^2 + y^2", "p": "1, 2"}, "outputs": {"gradient": [2.0, 4.0], "magnitude": math.sqrt(20)}},
               {"inputs": {"f": "x*y*z", "p": [1, 2, 3]}, "outputs": {"gradient": [6.0, 3.0, 2.0], "magnitude": 7.0}}],
     questions=[{"q": "Find the gradient of x^2 + y^2 at (1, 2).", "outputs": {"gradient": [2.0, 4.0]}},
                {"q": "What is the gradient of x*y*z at the point (1, 2, 3)?", "outputs": {"gradient": [6.0, 3.0, 2.0], "magnitude": 7.0}},
                {"q": "Find the gradient of log(x) + y at (-1, 2).", "outcome": "INVALID_INPUT"}],
     keywords=["gradient", "grad"])
rule("gradient", "gradient <(?:vector )?>of {f} at <(?:the point )?>{p:v}", order=20)


@_memo
def directional_derivative(f, p, u):
    if len(u) != len(p):
        raise ValueError("the direction needs as many components as the point")
    nu = math.sqrt(math.fsum(c * c for c in u))
    if nu == 0:
        raise ValueError("the direction vector must not be zero")
    g = gradient(f, p)["gradient"]
    return {"directional_derivative": math.fsum(a * b for a, b in zip(g, u)) / nu}


def _verify_dirder(i, o):
    names, u = _vars(i["p"]), i["u"]
    nu = math.sqrt(math.fsum(c * c for c in u))
    g = [float(_mp_partial(i["f"], names, i["p"], [int(k == j) for k in range(len(names))])) for j in range(len(names))]
    return rel_err(o["directional_derivative"], math.fsum(a * b for a, b in zip(g, u)) / nu, 1e-11)


def _ref_dirder(f, p, u):
    g = ref_lam(f, _vars(p))
    with mp.workdps(40):
        nu = mp.sqrt(mp.fsum(mp.mpf(c) ** 2 for c in u))
        return {"directional_derivative": float(mp.diff(lambda t: g(*[mp.mpf(a) + t * b / nu for a, b in zip(p, u)]), 0))}


proc("directional_derivative", name="Directional derivative", topic="multivariable calculus",
     purpose="The rate of change of f at a point in the direction of a vector u (u is normalised to unit length).",
     inputs=[T_("f", "the function f(x, y) or f(x, y, z)"), V_("p", "the point"), V_("u", "the direction vector")],
     outputs=[O_("directional_derivative", "D_u f at the point")],
     primary="directional_derivative", conditions=["u != 0 with as many components as p", "f differentiable at p"],
     method="gradient (exact partial derivatives) dotted with u/|u|", latex=r"D_{\mathbf u} f = \nabla f\cdot\frac{\mathbf u}{|\mathbf u|}",
     run=directional_derivative, verify=_verify_dirder, reference=_ref_dirder,
     wrong=lambda f, p, u: {"directional_derivative": math.fsum(a * b for a, b in zip(gradient(f, p)["gradient"], u))},
     random_inputs=lambda rng: (lambda k: {"f": rand2(rng, k == 3), "p": rand_pt(rng, k),
                                           "u": rng.choice([[3, 4], [1, 2], [-2, 1], [1, 1], [2, -3]] if k == 2 else [[2, -1, 2], [1, 1, 1], [1, 2, 2], [3, 0, 4]])})(
         rng.choice([2, 2, 3])),
     templates=["Find the directional derivative of x^2*y at (1, 2) in the direction (3, 4)."],
     examples=[{"inputs": {"f": "x^2*y", "p": "1, 2", "u": "3, 4"}, "outputs": {"directional_derivative": 3.2}},
               {"inputs": {"f": "x*y*z", "p": [1, 2, 3], "u": [1, 2, 2]}, "outputs": {"directional_derivative": (6 + 6 + 4) / 3}}],
     questions=[{"q": "Find the directional derivative of x^2*y at (1, 2) in the direction (3, 4).", "outputs": {"directional_derivative": 3.2}},
                {"q": "Find the directional derivative of x*y*z at the point (1, 2, 3) in the direction of the vector (1, 2, 2).",
                 "outputs": {"directional_derivative": 16 / 3}},
                {"q": "Find the directional derivative of x^2*y at (1, 2) in the direction (0, 0).", "outcome": "INVALID_INPUT"}],
     keywords=["directional derivative"])
rule("directional_derivative",
     "directional derivative of {f} at <(?:the point )?>{p:v} in the direction <(?:of )?(?:the vector )?>{u:v}", order=15)


@_memo
def tangent_plane(f, p):
    g = gradient(f, p)["gradient"]
    if len(p) != 2:
        raise ValueError("a tangent plane needs a surface z = f(x, y) and a point (x, y)")
    z0 = at(parse_expr(f, _XY), _XY, *p)
    return {"z0": z0, "fx": g[0], "fy": g[1], "c": z0 - g[0] * p[0] - g[1] * p[1]}


def _verify_tplane(i, o):
    p = i["p"]
    fx, fy = float(_mp_partial(i["f"], _XY, p, (1, 0))), float(_mp_partial(i["f"], _XY, p, (0, 1)))
    z0 = at(parse_expr(i["f"], _XY), _XY, *p)
    return worst(rel_err(o["fx"], fx, 1e-11), rel_err(o["fy"], fy, 1e-11), rel_err(o["z0"], z0, 1e-12),
                 rel_err(o["c"], z0 - fx * p[0] - fy * p[1], 1e-11))


def _ref_tplane(f, p):
    fx, fy = ref_cpartial(f, _XY, p, 0), ref_cpartial(f, _XY, p, 1)
    z0 = float(ref_lam(f, _XY, module="math")(*p))
    return {"z0": z0, "fx": fx, "fy": fy, "c": z0 - fx * p[0] - fy * p[1]}


proc("tangent_plane", name="Tangent plane", topic="multivariable calculus",
     purpose="The tangent plane z = f_x x + f_y y + c to the surface z = f(x, y) at (x0, y0).",
     inputs=[T_("f", "the surface z = f(x, y)"), V_("p", "the point (x0, y0)", 2, 2)],
     outputs=[O_("z0", "f(x0, y0)"), O_("fx", "f_x(x0, y0)"), O_("fy", "f_y(x0, y0)"), O_("c", "the constant term c")],
     primary="c", conditions=["f differentiable at (x0, y0)"],
     method="exact partial derivatives; z = z0 + f_x (x - x0) + f_y (y - y0) expanded",
     latex=r"z = f(x_0,y_0) + f_x(x - x_0) + f_y(y - y_0)",
     run=tangent_plane, verify=_verify_tplane, reference=_ref_tplane,
     wrong=lambda f, p: dict(tangent_plane(f, p), c=tangent_plane(f, p)["z0"]),
     random_inputs=lambda rng: {"f": rand2(rng), "p": rand_pt(rng, 2)},
     templates=["Find the tangent plane to z = x^2 + y^2 at (1, 2)."],
     examples=[{"inputs": {"f": "x^2 + y^2", "p": "1, 2"}, "outputs": {"z0": 5.0, "fx": 2.0, "fy": 4.0, "c": -5.0}},
               {"inputs": {"f": "x*y", "p": [2, 3]}, "outputs": {"z0": 6.0, "fx": 3.0, "fy": 2.0, "c": -6.0}}],
     questions=[{"q": "Find the tangent plane to z = x^2 + y^2 at (1, 2).", "outputs": {"fx": 2.0, "fy": 4.0, "c": -5.0}},
                {"q": "Find the equation of the tangent plane to the surface z = x*y at the point (2, 3).", "outputs": {"c": -6.0}}],
     keywords=["tangent plane"])
rule("tangent_plane", "<(?:equation of the )?>tangent plane <(?:to|of)> <(?:the surface |the graph of )?>{f} at <(?:the point )?>{p:v}",
     order=20)


@_memo
def hessian_classification(f, p):
    if len(p) != 2:
        raise ValueError("this test is for f(x, y) at a point (x, y)")
    e = parse_expr(f, _XY)
    at(e, _XY, *p)
    fx, fy = at(sp.diff(e, X), _XY, *p), at(sp.diff(e, Y), _XY, *p)
    fxx, fyy, fxy = (at(sp.diff(e, *v), _XY, *p) for v in ((X, X), (Y, Y), (X, Y)))
    scale = max(1.0, abs(fxx), abs(fyy), abs(fxy))
    if max(abs(fx), abs(fy)) > 1e-10 * scale:
        raise ValueError(f"({p[0]:g}, {p[1]:g}) is not a critical point (the gradient is ({fx:.6g}, {fy:.6g}))")
    D = fxx * fyy - fxy * fxy
    if abs(D) <= 1e-12 * scale * scale:
        raise ValueError("D = 0: the second derivative test is inconclusive")
    kind = "saddle point" if D < 0 else ("local minimum" if fxx > 0 else "local maximum")
    return {"fxx": fxx, "fxy": fxy, "fyy": fyy, "D": D, "classification": kind}


def _verify_hessian(i, o):
    h = [float(_mp_partial(i["f"], _XY, i["p"], v)) for v in ((2, 0), (1, 1), (0, 2))]
    D = h[0] * h[2] - h[1] ** 2
    kind = "saddle point" if o["D"] < 0 else ("local minimum" if o["fxx"] > 0 else "local maximum")
    return worst(rel_err(o["fxx"], h[0], 1e-10), rel_err(o["fxy"], h[1], 1e-10), rel_err(o["fyy"], h[2], 1e-10),
                 rel_err(o["D"], D, 1e-9), _same(o["classification"], kind))


def _ref_hessian(f, p):
    import numpy as np
    h = [ref_mpartial(f, _XY, p, v) for v in ((2, 0), (1, 1), (0, 2))]
    ev = np.linalg.eigvalsh(np.array([[h[0], h[1]], [h[1], h[2]]]))
    kind = "local minimum" if ev.min() > 0 else ("local maximum" if ev.max() < 0 else "saddle point")
    return {"fxx": h[0], "fxy": h[1], "fyy": h[2], "D": h[0] * h[2] - h[1] ** 2, "classification": kind}


def _rand_hessian(rng):
    while True:
        A, B, C = _c(rng, -3, 3), _c(rng, -3, 3), _c(rng, -3, 3)
        if B * B != 4 * A * C:
            break
    p, q = rng.randint(-2, 2), rng.randint(-2, 2)
    xs, ys = _sh("x", p), _sh("y", q)
    extra = rng.choice(["", f" + {xs}^3", f" - {ys}^3", f" + {xs}^2*{ys}", f" + {xs}^4"])
    return {"f": _sgn(f"{A}*{xs}^2 + {B}*{xs}*{ys} + {C}*{ys}^2{extra}"), "p": [float(p), float(q)]}


proc("hessian_classification", name="Second derivative test (two variables)", topic="multivariable calculus / optimization",
     purpose="Classifies a critical point of f(x, y) as a local minimum, local maximum or saddle point from the Hessian.",
     inputs=[T_("f", "the function f(x, y)"), V_("p", "the critical point (x, y)", 2, 2)],
     outputs=[O_("fxx", "f_xx"), O_("fxy", "f_xy"), O_("fyy", "f_yy"), O_("D", "D = f_xx f_yy - f_xy^2"),
              O_("classification", "local minimum, local maximum or saddle point", "text")],
     primary="D", conditions=["the gradient is zero at the point", "D != 0 (D = 0 is inconclusive and refused)"],
     method="exact second partial derivatives at the point; D > 0 and f_xx > 0 minimum, D > 0 and f_xx < 0 maximum, D < 0 saddle",
     latex=r"D = f_{xx}f_{yy} - f_{xy}^2",
     run=hessian_classification, verify=_verify_hessian, reference=_ref_hessian,
     wrong=lambda f, p: (lambda r: (lambda D: dict(r, D=D, classification="saddle point" if D < 0 else (
         "local minimum" if r["fxx"] > 0 else "local maximum")))(r["fxx"] * r["fyy"] + r["fxy"] ** 2))(hessian_classification(f, p)),
     random_inputs=_rand_hessian,
     templates=["Classify the critical point of x^2 + 3y^2 - 2x at (1, 0).",
                "Use the second derivative test for x^2 - y^2 at the point (0, 0)."],
     examples=[{"inputs": {"f": "x^2 + 3y^2 - 2x", "p": "1, 0"}, "outputs": {"D": 12.0, "classification": "local minimum"}},
               {"inputs": {"f": "x^2 - y^2", "p": [0, 0]}, "outputs": {"D": -4.0, "classification": "saddle point"}},
               {"inputs": {"f": "4 - x^2 - y^2 + x*y", "p": [0, 0]}, "outputs": {"D": 3.0, "classification": "local maximum"}}],
     questions=[{"q": "Classify the critical point of x^2 + 3y^2 - 2x at (1, 0).", "outputs": {"classification": "local minimum"}},
                {"q": "Use the second derivative test for x^2 - y^2 at the point (0, 0).", "outputs": {"classification": "saddle point"}},
                {"q": "Classify the critical point of x^2 + y^2 at (1, 0).", "outcome": "INVALID_INPUT"},
                {"q": "Classify the critical point of x^4 + y^4 at (0, 0).", "outcome": "INVALID_INPUT"}],
     keywords=["second derivative test", "hessian", "saddle point"])
rule("hessian_classification", "classify the critical point of {f} at <(?:the point )?>{p:v}",
     "<(?:use |apply )?(?:the )?>second derivative test <(?:for|on|to)> {f} at <(?:the point )?>{p:v}", order=20)


@_memo
def laplacian(f, p):
    names = _vars(p)
    e = parse_expr(f, names)
    at(e, names, *p)
    return {"laplacian": math.fsum(at(sp.diff(e, _SYMS[v], 2), names, *p) for v in names)}


proc("laplacian", name="Laplacian", topic="vector calculus",
     purpose="The Laplacian f_xx + f_yy (+ f_zz) of a function at a point.",
     inputs=[T_("f", "the function f(x, y) or f(x, y, z)"), V_("p", "the point")],
     outputs=[O_("laplacian", "the Laplacian at the point")],
     primary="laplacian", conditions=["f twice differentiable at the point"],
     method="exact second partial derivatives summed", latex=r"\nabla^2 f = f_{xx} + f_{yy} + f_{zz}",
     run=laplacian,
     verify=lambda i, o: rel_err(o["laplacian"], math.fsum(float(_mp_partial(i["f"], _vars(i["p"]), i["p"], [2 * int(k == j) for k in range(len(i["p"]))]))
                                                             for j in range(len(i["p"]))), 1e-10),
     reference=lambda f, p: {"laplacian": math.fsum(ref_mpartial(f, _vars(p), p, [2 * int(k == j) for k in range(len(p))]) for j in range(len(p)))},
     wrong=lambda f, p: {"laplacian": math.fsum(gradient(f, p)["gradient"])},
     random_inputs=lambda rng: (lambda three: {"f": rand2(rng, three), "p": rand_pt(rng, 3 if three else 2)})(rng.random() < 0.35),
     templates=["Find the Laplacian of x^2*y + z^3 at (1, 2, 3)."],
     examples=[{"inputs": {"f": "x^2 + y^2", "p": "1, 1"}, "outputs": {"laplacian": 4.0}},
               {"inputs": {"f": "x^2*y + z^3", "p": [1, 2, 3]}, "outputs": {"laplacian": 22.0}}],
     questions=[{"q": "Find the Laplacian of x^2*y + z^3 at (1, 2, 3).", "outputs": {"laplacian": 22.0}},
                {"q": "Find the Laplacian of x^2 + y^2 at the point (1, 1).", "outputs": {"laplacian": 4.0}}],
     keywords=["laplacian"])
rule("laplacian", "laplacian of {f} at <(?:the point )?>{p:v}", order=20)


def _field(P, Q, R=None):
    names = _XYZ if R is not None else _XY
    return [parse_expr(c, names) for c in ([P, Q, R] if R is not None else [P, Q])], names


@_memo
def divergence(P, Q, R, p):
    if len(p) != 3:
        raise ValueError("the point needs 3 coordinates (x, y, z)")
    F, names = _field(P, Q, R)
    for c in F:
        at(c, names, *p)
    return {"divergence": math.fsum(at(sp.diff(c, s), names, *p) for c, s in zip(F, (X, Y, Z)))}


_FIELD_IN = [T_("P", "the x-component P(x, y, z)"), T_("Q", "the y-component Q(x, y, z)"), T_("R", "the z-component R(x, y, z)"),
             V_("p", "the point (x, y, z)", 3, 3)]


def _rand_field3(rng):
    a, b = _c(rng, -3, 3), _c(rng, 1, 3)
    pool = [f"x*y", f"y*z", f"x*z", f"{a}*x^2", f"y^2 + {a}*z", f"sin(x*y)", f"exp(z/{b})", f"x + {a}*y*z", f"z^2*x", f"cos(y) + x"]
    return {"P": _sgn(rng.choice(pool)), "Q": _sgn(rng.choice(pool)), "R": _sgn(rng.choice(pool)), "p": rand_pt(rng, 3)}


proc("divergence", name="Divergence", topic="vector calculus",
     purpose="The divergence P_x + Q_y + R_z of a vector field F = (P, Q, R) at a point.",
     inputs=_FIELD_IN, outputs=[O_("divergence", "div F at the point")],
     primary="divergence", conditions=["the components are differentiable at the point"],
     method="exact symbolic partial derivatives", latex=r"\nabla\cdot\mathbf F = P_x + Q_y + R_z",
     run=divergence,
     verify=lambda i, o: rel_err(o["divergence"], math.fsum(float(_mp_partial(c, _XYZ, i["p"], [int(k == j) for k in range(3)]))
                                                             for j, c in enumerate((i["P"], i["Q"], i["R"]))), 1e-11),
     reference=lambda P, Q, R, p: {"divergence": math.fsum(ref_cpartial(c, _XYZ, p, j) for j, c in enumerate((P, Q, R)))},
     wrong=lambda P, Q, R, p: {"divergence": math.fsum(at(sp.diff(parse_expr(c, _XYZ), X), _XYZ, *p) for c in (P, Q, R))},
     random_inputs=_rand_field3,
     templates=["Find the divergence of F = (x*y, y*z, x*z) at (1, 2, 3)."],
     examples=[{"inputs": {"P": "x*y", "Q": "y*z", "R": "x*z", "p": "1, 2, 3"}, "outputs": {"divergence": 6.0}},
               {"inputs": {"P": "x^2", "Q": "y^2", "R": "z^2", "p": [1, 1, 1]}, "outputs": {"divergence": 6.0}}],
     questions=[{"q": "Find the divergence of F = (x*y, y*z, x*z) at (1, 2, 3).", "outputs": {"divergence": 6.0}},
                {"q": "Find the divergence of the vector field F = (x^2, y^2, z^2) at the point (1, 1, 1).", "outputs": {"divergence": 6.0}}],
     keywords=["divergence", "div"])
_FEQ = "<(?:the )?(?:vector )?(?:field )?>f = ({P}, {Q}, {R}) at <(?:the point )?>{p:v}"
rule("divergence", "divergence of " + _FEQ, order=20)


@_memo
def curl(P, Q, R, p):
    if len(p) != 3:
        raise ValueError("the point needs 3 coordinates (x, y, z)")
    (eP, eQ, eR), names = _field(P, Q, R)
    for c in (eP, eQ, eR):
        at(c, names, *p)
    d = lambda e, s: at(sp.diff(e, s), names, *p)  # noqa: E731
    return {"curl": [d(eR, Y) - d(eQ, Z), d(eP, Z) - d(eR, X), d(eQ, X) - d(eP, Y)]}


def _curl_from(part):
    return [part(2, 1) - part(1, 2), part(0, 2) - part(2, 0), part(1, 0) - part(0, 1)]


proc("curl", name="Curl", topic="vector calculus",
     purpose="The curl of a vector field F = (P, Q, R) at a point.",
     inputs=_FIELD_IN, outputs=[O_("curl", "curl F at the point", "vector")],
     primary="curl", conditions=["the components are differentiable at the point"],
     method="exact symbolic partial derivatives: (R_y - Q_z, P_z - R_x, Q_x - P_y)",
     latex=r"\nabla\times\mathbf F = (R_y - Q_z,\ P_z - R_x,\ Q_x - P_y)",
     run=curl,
     verify=lambda i, o: _vec_err(o["curl"], _curl_from(lambda c, j: float(_mp_partial((i["P"], i["Q"], i["R"])[c], _XYZ, i["p"],
                                                                                       [int(k == j) for k in range(3)]))), 1e-11),
     reference=lambda P, Q, R, p: {"curl": _curl_from(lambda c, j: ref_cpartial((P, Q, R)[c], _XYZ, p, j))},
     wrong=lambda P, Q, R, p: {"curl": [-v for v in curl(P, Q, R, p)["curl"]]},
     random_inputs=_rand_field3,
     templates=["Find the curl of F = (x*y, y*z, z*x) at (1, 2, 3)."],
     examples=[{"inputs": {"P": "x*y", "Q": "y*z", "R": "z*x", "p": "1, 2, 3"}, "outputs": {"curl": [-2.0, -3.0, -1.0]}},
               {"inputs": {"P": "-y", "Q": "x", "R": "z", "p": [1, 2, 3]}, "outputs": {"curl": [0.0, 0.0, 2.0]}}],
     questions=[{"q": "Find the curl of F = (x*y, y*z, z*x) at (1, 2, 3).", "outputs": {"curl": [-2.0, -3.0, -1.0]}},
                {"q": "Find the curl of the vector field F = (-y, x, z) at the point (1, 2, 3).", "outputs": {"curl": [0.0, 0.0, 2.0]}}],
     keywords=["curl", "rotation of a field"])
rule("curl", "curl of " + _FEQ, order=20)


@_memo
def jacobian_determinant(xu, yv, p):
    names = ("u", "v")
    ex, ey = parse_expr(xu, names), parse_expr(yv, names)
    U_, V_s = _SYMS["u"], _SYMS["v"]
    d = lambda e, s: at(sp.diff(e, s), names, *p)  # noqa: E731
    return {"jacobian": d(ex, U_) * d(ey, V_s) - d(ex, V_s) * d(ey, U_)}


def _ref_jacobian(xu, yv, p):
    import numpy as np
    J = np.array([[ref_cpartial(c, ("u", "v"), p, j) for j in range(2)] for c in (xu, yv)])
    return {"jacobian": float(np.linalg.det(J))}


proc("jacobian_determinant", name="Jacobian determinant", topic="multivariable calculus / change of variables",
     purpose="The Jacobian determinant d(x, y)/d(u, v) of a transformation x = x(u, v), y = y(u, v) at a point.",
     inputs=[T_("xu", "x as a function of u and v"), T_("yv", "y as a function of u and v"), V_("p", "the point (u, v)", 2, 2)],
     outputs=[O_("jacobian", "the Jacobian determinant at the point")],
     primary="jacobian", conditions=["x and y differentiable at the point"],
     method="exact partial derivatives; x_u y_v - x_v y_u",
     latex=r"\frac{\partial(x,y)}{\partial(u,v)} = x_u y_v - x_v y_u",
     run=jacobian_determinant,
     verify=lambda i, o: rel_err(o["jacobian"], float(_mp_partial(i["xu"], ("u", "v"), i["p"], (1, 0)) * _mp_partial(i["yv"], ("u", "v"), i["p"], (0, 1))
                                                       - _mp_partial(i["xu"], ("u", "v"), i["p"], (0, 1)) * _mp_partial(i["yv"], ("u", "v"), i["p"], (1, 0))), 1e-10),
     reference=_ref_jacobian,
     wrong=lambda xu, yv, p: (lambda d: {"jacobian": d(xu, 0) * d(yv, 1) + d(xu, 1) * d(yv, 0)})(
         lambda c, j: at(sp.diff(parse_expr(c, ("u", "v")), _SYMS["uv"[j]]), ("u", "v"), *p)),
     random_inputs=lambda rng: (lambda a, b: rng.choice([{"xu": "u*cos(v)", "yv": "u*sin(v)"}, {"xu": f"u + {a}*v", "yv": f"{b}*u - v"},
                                                         {"xu": "u^2 - v^2", "yv": "2*u*v"}, {"xu": f"u*v + {a}", "yv": f"u^2 + {b}*v"},
                                                         {"xu": "exp(u)*cos(v)", "yv": "exp(u)*sin(v)"}, {"xu": f"u^3 + v", "yv": f"sin(u) + {b}*v^2"}])
                                | {"p": [rand_point(rng, 1, 3), rand_point(rng, -2, 2)]})(_c(rng, 1, 3), _c(rng, 1, 3)),
     templates=["Find the Jacobian of x = u*cos(v), y = u*sin(v) at (2, 0.5)."],
     examples=[{"inputs": {"xu": "u*cos(v)", "yv": "u*sin(v)", "p": "2, 0.5"}, "outputs": {"jacobian": 2.0}},
               {"inputs": {"xu": "u + v", "yv": "u - v", "p": [1, 1]}, "outputs": {"jacobian": -2.0}}],
     questions=[{"q": "Find the Jacobian of x = u*cos(v), y = u*sin(v) at (2, 0.5).", "outputs": {"jacobian": 2.0}},
                {"q": "Find the Jacobian determinant of the transformation x = u + v, y = u - v at the point (1, 1).", "outputs": {"jacobian": -2.0}}],
     keywords=["jacobian", "change of variables"])
rule("jacobian_determinant",
     "jacobian <(?:determinant )?>of <(?:the transformation )?>x = {xu}, y = {yv} at <(?:the point )?>{p:v}", order=20)


# =========================================================================================== 13. multiple integrals
def _fl(e, names):
    return _compile(e, names, use_mp=False)


@_memo
def double_integral_rectangle(f, a, b, c, d):
    if not (a < b and c < d):
        raise ValueError("the rectangle needs a < b and c < d")
    g = _fl(parse_expr(f, _XY), _XY)
    v, _S = _settled(lambda n: _gl2(g, a, b, lambda x: c, lambda x: d, n),
                     lambda n: _gl2(lambda x, y: abs(g(x, y)), a, b, lambda x: c, lambda x: d, n), (24, 40, 64))
    return {"integral": v}


def _verify_dbl(i, o):
    g = _fl(parse_expr(i["f"], _XY), _XY)
    c, d = i["c"], i["d"]
    S = _gl2(lambda x, y: abs(g(x, y)), i["a"], i["b"], lambda x: c, lambda x: d, 32)
    return _scaled_err(o["integral"], _gl2(g, i["a"], i["b"], lambda x: c, lambda x: d, 32), S)


def _ref_dbl(f, a, b, c, d):
    from scipy.integrate import dblquad
    g = ref_lam(f, _XY, module="math")
    return {"integral": float(dblquad(lambda y, x: g(x, y), a, b, c, d, epsabs=1e-13, epsrel=1e-13)[0])}


proc("double_integral_rectangle", name="Double integral over a rectangle", topic="multiple integrals",
     purpose="The double integral of f(x, y) over the rectangle [a, b] x [c, d].",
     inputs=[T_("f", "the integrand f(x, y)"), N_("a", "x from"), N_("b", "x to"), N_("c", "y from"), N_("d", "y to")],
     outputs=[O_("integral", "the value of the double integral")],
     primary="integral", conditions=["a < b, c < d", "f continuous on the rectangle"],
     method="product Gauss-Legendre rules of increasing order (24, 40, 64 nodes per direction) until two agree to 1e-12",
     latex=r"\int_a^b\!\int_c^d f(x,y)\,dy\,dx",
     run=double_integral_rectangle, verify=_verify_dbl, reference=_ref_dbl,
     wrong=lambda f, a, b, c, d: {"integral": (b - a) * (d - c) * at(parse_expr(f, _XY), _XY, (a + b) / 2, (c + d) / 2)},
     random_inputs=lambda rng: (lambda ab, cd: {"f": rand2(rng), "a": ab[0], "b": ab[1], "c": cd[0], "d": cd[1]})(
         rand_interval(rng, -2, 2), rand_interval(rng, -2, 2)),
     templates=["Find the double integral of x*y over [0, 2] x [0, 3].",
                "Evaluate the double integral of x^2 + y over the rectangle [0, 1] x [0, 2]."],
     examples=[{"inputs": {"f": "x*y", "a": 0, "b": 2, "c": 0, "d": 3}, "outputs": {"integral": 9.0}},
               {"inputs": {"f": "x^2 + y", "a": 0, "b": 1, "c": 0, "d": 2}, "outputs": {"integral": 8 / 3}}],
     questions=[{"q": "Find the double integral of x*y over [0, 2] x [0, 3].", "outputs": {"integral": 9.0}},
                {"q": "Evaluate the double integral of x^2 + y over the rectangle [0, 1] x [0, 2].", "outputs": {"integral": 8 / 3}},
                {"q": "Find the double integral of x*y over [2, 0] x [0, 3].", "outcome": "INVALID_INPUT"}],
     keywords=["double integral", "iterated integral"])
_X = "<(?:x|by|times)>"
rule("double_integral_rectangle",
     "double integral of {f} over <(?:the rectangle |the region )?>[{a:n}, {b:n}] " + _X + " [{c:n}, {d:n}]", order=20)


@_memo
def double_integral_region(f, a, b, g1, g2):
    if not a < b:
        raise ValueError("the x-interval needs a < b")
    g = _fl(parse_expr(f, _XY), _XY)
    lo, hi = _fl(parse_expr(g1), ("x",)), _fl(parse_expr(g2), ("x",))
    for x, _w in _glf(64):
        xx = (a + b) / 2 + (b - a) / 2 * x
        if lo(xx) > hi(xx) + 1e-12 * max(1.0, abs(hi(xx))):
            raise ValueError("the lower boundary must lie below the upper boundary on [a, b]")
    v, _S = _settled(lambda n: _gl2(g, a, b, lo, hi, n), lambda n: _gl2(lambda x, y: abs(g(x, y)), a, b, lo, hi, n), (24, 40, 64))
    return {"integral": v}


def _verify_dblr(i, o):
    g = _fl(parse_expr(i["f"], _XY), _XY)
    lo, hi = _fl(parse_expr(i["g1"]), ("x",)), _fl(parse_expr(i["g2"]), ("x",))
    S = _gl2(lambda x, y: abs(g(x, y)), i["a"], i["b"], lo, hi, 32)
    return _scaled_err(o["integral"], _gl2(g, i["a"], i["b"], lo, hi, 32), S)


def _ref_dblr(f, a, b, g1, g2):
    from scipy.integrate import dblquad
    g, lo, hi = ref_lam(f, _XY, module="math"), ref_lam(g1, module="math"), ref_lam(g2, module="math")
    return {"integral": float(dblquad(lambda y, x: g(x, y), a, b, lambda x: lo(x), lambda x: hi(x), epsabs=1e-13, epsrel=1e-13)[0])}


def _rand_region(rng):
    k = rng.randint(1, 3)
    reg = rng.choice([("0", "x", 0.0, 2.0), ("x^2", "2*x", 0.0, 2.0), ("x^2", "x + 2", -1.0, 2.0), ("-1", "cos(x)", -1.0, 1.0),
                      ("0", f"exp(x/{k})", 0.0, 1.0), ("x - 1", "1 - x^2", -1.0, 1.0), ("sin(x)", f"{k} + 1", 0.0, 3.0)])
    return {"f": rand2(rng), "g1": reg[0], "g2": reg[1], "a": reg[2], "b": reg[3]}


proc("double_integral_region", name="Double integral over a region", topic="multiple integrals",
     purpose="The double integral of f(x, y) over the region a <= x <= b, g1(x) <= y <= g2(x).",
     inputs=[T_("f", "the integrand f(x, y)"), N_("a", "x from"), N_("b", "x to"), T_("g1", "the lower curve y = g1(x)"),
             T_("g2", "the upper curve y = g2(x)")],
     outputs=[O_("integral", "the value of the double integral")],
     primary="integral", conditions=["a < b", "g1 <= g2 on [a, b]", "f, g1, g2 continuous"],
     method="iterated Gauss-Legendre rules of increasing order (inner rule mapped to [g1(x), g2(x)]) until two agree",
     latex=r"\int_a^b\!\int_{g_1(x)}^{g_2(x)} f(x,y)\,dy\,dx",
     run=double_integral_region, verify=_verify_dblr, reference=_ref_dblr,
     wrong=lambda f, a, b, g1, g2: double_integral_rectangle(f, a, b, min(_fl(parse_expr(g1), ("x",))(a), _fl(parse_expr(g1), ("x",))(b)),
                                                             max(_fl(parse_expr(g2), ("x",))(a), _fl(parse_expr(g2), ("x",))(b)) + 1),
     random_inputs=_rand_region,
     templates=["Find the double integral of x*y over the region 0 <= x <= 1, x^2 <= y <= x."],
     examples=[{"inputs": {"f": "x*y", "a": 0, "b": 1, "g1": "x^2", "g2": "x"}, "outputs": {"integral": 1 / 24}},
               {"inputs": {"f": "1", "a": 0, "b": 2, "g1": "0", "g2": "x^2"}, "outputs": {"integral": 8 / 3}}],
     questions=[{"q": "Find the double integral of x*y over the region 0 <= x <= 1, x^2 <= y <= x.", "outputs": {"integral": 1 / 24}},
                {"q": "Evaluate the double integral of 1 over 0 <= x <= 2, 0 <= y <= x^2.", "outputs": {"integral": 8 / 3}},
                {"q": "Find the double integral of x*y over the region 0 <= x <= 1, x <= y <= x^2 - 1.", "outcome": "INVALID_INPUT"}],
     keywords=["double integral", "type I region"])
rule("double_integral_region",
     "double integral of {f} over <(?:the region )?>{a:n} <= x <= {b:n}, {g1} <= y <= {g2}", order=15)


@_memo
def triple_integral_box(f, x0, x1, y0, y1, z0, z1):
    if not (x0 < x1 and y0 < y1 and z0 < z1):
        raise ValueError("the box needs x0 < x1, y0 < y1, z0 < z1")
    e = parse_expr(f, _XYZ)
    if e.is_polynomial(X, Y, Z):
        v = sp.integrate(e, (X, _rat(x0), _rat(x1)), (Y, _rat(y0), _rat(y1)), (Z, _rat(z0), _rat(z1)))
        return {"integral": _pfloat(v)}
    g = _fl(e, _XYZ)
    bx = ((x0, x1), (y0, y1), (z0, z1))
    v, _S = _settled(lambda n: _gl3(g, bx, n), lambda n: _gl3(lambda x, y, z: abs(g(x, y, z)), bx, n), (12, 20, 28))
    return {"integral": v}


def _verify_tpl(i, o):
    g = _fl(parse_expr(i["f"], _XYZ), _XYZ)
    bx = ((i["x0"], i["x1"]), (i["y0"], i["y1"]), (i["z0"], i["z1"]))
    return _scaled_err(o["integral"], _gl3(g, bx, 16), _gl3(lambda x, y, z: abs(g(x, y, z)), bx, 16))


def _ref_tpl(f, x0, x1, y0, y1, z0, z1):
    from scipy.integrate import tplquad
    g = ref_lam(f, _XYZ, module="math")
    return {"integral": float(tplquad(lambda z, y, x: g(x, y, z), x0, x1, y0, y1, z0, z1, epsabs=1e-12, epsrel=1e-12)[0])}


def _rand_box(rng):
    out = {"f": rng.choice([rand2(rng, True), _sgn(f"x*y*z + {_c(rng, -3, 3)}*x^2 + z"), "x^2 + y^2 + z^2",
                            _sgn(f"{_c(rng, -3, 3)}*x*y + z^3 - y"), "exp(x + y + z)", f"x*y^2*z^3"])}
    for v in ("x", "y", "z"):
        a, b = rand_interval(rng, -2, 2)
        out[v + "0"], out[v + "1"] = a, b
    return out


proc("triple_integral_box", name="Triple integral over a box", topic="multiple integrals",
     purpose="The triple integral of f(x, y, z) over the box [x0, x1] x [y0, y1] x [z0, z1].",
     inputs=[T_("f", "the integrand f(x, y, z)"), N_("x0", "x from"), N_("x1", "x to"), N_("y0", "y from"), N_("y1", "y to"),
             N_("z0", "z from"), N_("z1", "z to")],
     outputs=[O_("integral", "the value of the triple integral")],
     primary="integral", conditions=["x0 < x1, y0 < y1, z0 < z1", "f continuous on the box"],
     method="exact iterated integration for a polynomial integrand; otherwise product Gauss-Legendre rules of increasing "
            "order until two agree",
     latex=r"\iiint_B f(x,y,z)\,dV",
     run=triple_integral_box, verify=_verify_tpl, reference=_ref_tpl,
     wrong=lambda f, x0, x1, y0, y1, z0, z1: {"integral": (x1 - x0) * (y1 - y0) * (z1 - z0) * at(parse_expr(f, _XYZ), _XYZ, (x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2)},
     random_inputs=_rand_box,
     templates=["Find the triple integral of x*y*z over [0, 1] x [0, 2] x [0, 3]."],
     examples=[{"inputs": {"f": "x*y*z", "x0": 0, "x1": 1, "y0": 0, "y1": 2, "z0": 0, "z1": 3}, "outputs": {"integral": 4.5}},
               {"inputs": {"f": "x^2 + y^2 + z^2", "x0": 0, "x1": 1, "y0": 0, "y1": 1, "z0": 0, "z1": 1}, "outputs": {"integral": 1.0}}],
     questions=[{"q": "Find the triple integral of x*y*z over [0, 1] x [0, 2] x [0, 3].", "outputs": {"integral": 4.5}},
                {"q": "Evaluate the triple integral of x^2 + y^2 + z^2 over the box [0, 1] x [0, 1] x [0, 1].", "outputs": {"integral": 1.0}}],
     keywords=["triple integral"])
_BOX = "[{x0:n}, {x1:n}] " + _X + " [{y0:n}, {y1:n}] " + _X + " [{z0:n}, {z1:n}]"
rule("triple_integral_box", "triple integral of {f} over <(?:the box |the region )?>" + _BOX, order=20)


@_memo
def surface_area_graph(f, a, b, c, d):
    if not (a < b and c < d):
        raise ValueError("the rectangle needs a < b and c < d")
    e = parse_expr(f, _XY)
    g = _fl(sp.sqrt(1 + sp.diff(e, X) ** 2 + sp.diff(e, Y) ** 2), _XY)
    v, _S = _settled(lambda n: _gl2(g, a, b, lambda x: c, lambda x: d, n), lambda n: 1.0, (24, 40, 64))
    return {"surface_area": v}


def _ref_surface_graph(f, a, b, c, d):
    from scipy.integrate import dblquad
    return {"surface_area": float(dblquad(lambda y, x: math.sqrt(1 + ref_cpartial(f, _XY, (x, y), 0) ** 2 + ref_cpartial(f, _XY, (x, y), 1) ** 2),
                                          a, b, c, d, epsabs=1e-12, epsrel=1e-12)[0])}


proc("surface_area_graph", name="Surface area of a graph", topic="multiple integrals",
     purpose="The area of the surface z = f(x, y) over the rectangle [a, b] x [c, d].",
     inputs=[T_("f", "the surface z = f(x, y)"), N_("a", "x from"), N_("b", "x to"), N_("c", "y from"), N_("d", "y to")],
     outputs=[O_("surface_area", "the surface area")],
     primary="surface_area", conditions=["a < b, c < d", "f continuously differentiable on the rectangle"],
     method="exact partial derivatives; Gauss-Legendre product rules of sqrt(1 + f_x^2 + f_y^2) until two orders agree",
     latex=r"A = \iint_R \sqrt{1 + f_x^2 + f_y^2}\,dA",
     run=surface_area_graph,
     verify=lambda i, o: (lambda e: _scaled_err(o["surface_area"], _gl2(_fl(sp.sqrt(1 + sp.diff(e, X) ** 2 + sp.diff(e, Y) ** 2), _XY),
                                                                         i["a"], i["b"], lambda x: i["c"], lambda x: i["d"], 32), 1.0))(parse_expr(i["f"], _XY)),
     reference=_ref_surface_graph,
     wrong=lambda f, a, b, c, d: {"surface_area": _gl2(_fl(1 + sp.diff(parse_expr(f, _XY), X) ** 2 + sp.diff(parse_expr(f, _XY), Y) ** 2, _XY),
                                                        a, b, lambda x: c, lambda x: d, 40)},
     random_inputs=lambda rng: (lambda ab, cd: {"f": rng.choice([rand2(rng), _sgn(f"{_c(rng, -2, 2)}*x + {_c(rng, -2, 2)}*y"),
                                                                 "x^2 + y^2", "x*y", "sin(x) + cos(y)"]),
                                                "a": ab[0], "b": ab[1], "c": cd[0], "d": cd[1]})(rand_interval(rng, -1, 1), rand_interval(rng, -1, 1)),
     templates=["Find the surface area of z = x + y over [0, 1] x [0, 1]."],
     examples=[{"inputs": {"f": "x + y", "a": 0, "b": 1, "c": 0, "d": 1}, "outputs": {"surface_area": math.sqrt(3)}},
               {"inputs": {"f": "3", "a": 0, "b": 2, "c": 0, "d": 2}, "outputs": {"surface_area": 4.0}}],
     questions=[{"q": "Find the surface area of z = x + y over [0, 1] x [0, 1].", "outputs": {"surface_area": math.sqrt(3)}},
                {"q": "Find the area of the surface z = 2x + 2y over the rectangle [0, 1] x [0, 2].", "outputs": {"surface_area": 6.0}}],
     keywords=["surface area", "surface integral"])
rule("surface_area_graph",
     "<(?:surface area|area of the surface)> <(?:of )?>{f} over <(?:the rectangle |the region )?>[{a:n}, {b:n}] " + _X + " [{c:n}, {d:n}]", order=20)


# =========================================================================================== 14. line integrals and theorems
def _path(xt, yt):
    return parse_expr(xt, ("t",)), parse_expr(yt, ("t",))


def _q_t(e, t0, t1):
    if t0 == t1:
        return 0.0
    continuous_on(e, T, t0, t1)
    return quad(_compile(e, (T,)), t0, t1)


@_memo
def line_integral_vector(P, Q, xt, yt, t0, t1):
    ex, ey = _path(xt, yt)
    eP, eQ = (parse_expr(c, _XY).subs({X: ex, Y: ey}, simultaneous=True) for c in (P, Q))
    return {"line_integral": _q_t(eP * sp.diff(ex, T) + eQ * sp.diff(ey, T), t0, t1)}


def _ref_line_vec(P, Q, xt, yt, t0, t1):
    rx, ry = ref_lam(xt, ("t",), module="math"), ref_lam(yt, ("t",), module="math")
    gP, gQ = ref_lam(P, _XY, module="math"), ref_lam(Q, _XY, module="math")
    dx, dy = ref_cdiff(xt, ("t",)), ref_cdiff(yt, ("t",))
    return {"line_integral": ref_quad(lambda t: gP(rx(t), ry(t)) * dx(t) + gQ(rx(t), ry(t)) * dy(t), t0, t1)}


def _verify_line(fn_expr, out_key):
    def verify(i, o):
        with mp.workdps(20):
            v = _gl(_compile(fn_expr(i), (T,)), i["t0"], i["t1"]) if i["t0"] != i["t1"] else 0
        return rel_err(o[out_key], float(v), 1e-10)
    return verify


def _vec_integrand(i):
    ex, ey = _path(i["xt"], i["yt"])
    eP, eQ = (parse_expr(c, _XY).subs({X: ex, Y: ey}, simultaneous=True) for c in (i["P"], i["Q"]))
    return eP * sp.diff(ex, T) + eQ * sp.diff(ey, T)


_PATHS = [("cos(t)", "sin(t)", 0.0, math.pi), ("2*cos(t)", "2*sin(t)", 0.0, math.pi / 2), ("t", "t^2", 0.0, 1.0),
          ("1 + 2*t", "3*t - 1", 0.0, 1.0), ("t^2", "t^3", 0.0, 1.0), ("cos(t)", "sin(t)", 0.0, 2 * math.pi), ("t", "exp(t/2)", -1.0, 1.0)]


def _rand_line_vec(rng):
    a, b = _c(rng, -3, 3), _c(rng, -3, 3)
    P, Q = rng.choice([("y", "x"), ("-y", "x"), (f"x^2 + {a}*y", "x*y"), (f"{a}*y^2", f"x + {b}"), ("exp(x)", "sin(y)"), ("x*y", "y^2 - x")])
    xt, yt, t0, t1 = rng.choice(_PATHS)
    return {"P": _sgn(P), "Q": _sgn(Q), "xt": xt, "yt": yt, "t0": t0, "t1": t1}


proc("line_integral_vector", name="Line integral of a vector field", topic="vector calculus",
     purpose="The line integral (work) of F = (P, Q) along the path x = x(t), y = y(t) from t0 to t1.",
     inputs=[T_("P", "the x-component P(x, y)"), T_("Q", "the y-component Q(x, y)"), T_("xt", "the path x(t)"),
             T_("yt", "the path y(t)"), N_("t0", "start parameter"), N_("t1", "end parameter")],
     outputs=[O_("line_integral", "the integral of F . dr along the path")],
     primary="line_integral", conditions=["the integrand is continuous for t between t0 and t1"],
     method="F(r(t)) . r'(t) built exactly (symbolic r'), then tanh-sinh quadrature at 20 digits",
     latex=r"\int_C \mathbf F\cdot d\mathbf r = \int_{t_0}^{t_1} \left(P\,x'(t) + Q\,y'(t)\right)dt",
     run=line_integral_vector, verify=_verify_line(_vec_integrand, "line_integral"), reference=_ref_line_vec,
     wrong=lambda P, Q, xt, yt, t0, t1: line_integral_vector(Q, P, xt, yt, t0, t1),
     random_inputs=_rand_line_vec,
     templates=["Find the line integral of F = (-y, x) along x = cos(t), y = sin(t) from t = 0 to t = 2pi."],
     examples=[{"inputs": {"P": "-y", "Q": "x", "xt": "cos(t)", "yt": "sin(t)", "t0": 0, "t1": 2 * math.pi}, "outputs": {"line_integral": 2 * math.pi}},
               {"inputs": {"P": "y", "Q": "x", "xt": "t", "yt": "t^2", "t0": 0, "t1": 1}, "outputs": {"line_integral": 1.0}}],
     questions=[{"q": "Find the line integral of F = (-y, x) along x = cos(t), y = sin(t) from t = 0 to t = 2pi.", "outputs": {"line_integral": 2 * math.pi}},
                {"q": "Compute the line integral of the vector field F = (y, x) along the curve x = t, y = t^2 from t = 0 to t = 1.",
                 "outputs": {"line_integral": 1.0}}],
     keywords=["line integral", "work done by a force field"])
_ALONG = " along <(?:the )?(?:path |curve )?>x = {xt}, y = {yt}<,?> from t = {t0:n} to t = {t1:n}"
rule("line_integral_vector", "line integral of <(?:the )?(?:vector )?(?:field )?>f = ({P}, {Q})" + _ALONG,
     "work done by <(?:the )?(?:force )?(?:field )?>f = ({P}, {Q})" + _ALONG, order=15)


def _scal_integrand(i):
    ex, ey = _path(i["xt"], i["yt"])
    ef = parse_expr(i["f"], _XY).subs({X: ex, Y: ey}, simultaneous=True)
    return ef * sp.sqrt(sp.diff(ex, T) ** 2 + sp.diff(ey, T) ** 2)


@_memo
def line_integral_scalar(f, xt, yt, t0, t1):
    if not t0 < t1:
        raise ValueError("the parameter must run forward (t0 < t1) for an integral with respect to arc length")
    ex, ey = _path(xt, yt)
    ef = parse_expr(f, _XY).subs({X: ex, Y: ey}, simultaneous=True)
    return {"line_integral": _q_t(ef * sp.sqrt(sp.diff(ex, T) ** 2 + sp.diff(ey, T) ** 2), t0, t1)}


def _ref_line_scal(f, xt, yt, t0, t1):
    rx, ry = ref_lam(xt, ("t",), module="math"), ref_lam(yt, ("t",), module="math")
    g = ref_lam(f, _XY, module="math")
    dx, dy = ref_cdiff(xt, ("t",)), ref_cdiff(yt, ("t",))
    return {"line_integral": ref_quad(lambda t: g(rx(t), ry(t)) * math.hypot(dx(t), dy(t)), t0, t1)}


proc("line_integral_scalar", name="Line integral with respect to arc length", topic="vector calculus",
     purpose="The integral of f(x, y) ds along the path x = x(t), y = y(t), t0 <= t <= t1 (mass of a wire, average "
             "along a curve).",
     inputs=[T_("f", "the function f(x, y)"), T_("xt", "the path x(t)"), T_("yt", "the path y(t)"), N_("t0", "start parameter"),
             N_("t1", "end parameter")],
     outputs=[O_("line_integral", "the integral of f ds along the path")],
     primary="line_integral", conditions=["t0 < t1", "the integrand continuous on [t0, t1]"],
     method="f(r(t)) |r'(t)| with exact r'(t), then tanh-sinh quadrature",
     latex=r"\int_C f\,ds = \int_{t_0}^{t_1} f(x(t),y(t))\sqrt{x'(t)^2 + y'(t)^2}\,dt",
     run=line_integral_scalar, verify=_verify_line(_scal_integrand, "line_integral"), reference=_ref_line_scal,
     wrong=lambda f, xt, yt, t0, t1: {"line_integral": _q_t(parse_expr(f, _XY).subs(dict(zip((X, Y), _path(xt, yt))), simultaneous=True), t0, t1)},
     random_inputs=lambda rng: (lambda pth: {"f": _sgn(rng.choice(["x^2 + y", "x*y + 2", f"x + y^2 + {rng.randint(1, 3)}", "exp(x) + y",
                                                                     f"{_c(rng, -3, 3)}*x*y^2 + 1"])),
                                             "xt": pth[0], "yt": pth[1], "t0": pth[2], "t1": pth[3]})(rng.choice(_PATHS)),
     templates=["Find the line integral of x + y ds along x = t, y = t from t = 0 to t = 1."],
     examples=[{"inputs": {"f": "x + y", "xt": "t", "yt": "t", "t0": 0, "t1": 1}, "outputs": {"line_integral": math.sqrt(2)}},
               {"inputs": {"f": "1", "xt": "cos(t)", "yt": "sin(t)", "t0": 0, "t1": math.pi}, "outputs": {"line_integral": math.pi}}],
     questions=[{"q": "Find the line integral of x + y ds along x = t, y = t from t = 0 to t = 1.", "outputs": {"line_integral": math.sqrt(2)}},
                {"q": "Find the line integral of x + y ds along x = t, y = t from t = 1 to t = 0.", "outcome": "INVALID_INPUT"}],
     keywords=["line integral", "arc length integral"])
rule("line_integral_scalar", "line integral of {f} ds" + _ALONG, "integral of {f} with respect to arc length" + _ALONG, order=16)


@_memo
def greens_theorem(P, Q, a, b, c, d):
    if not (a < b and c < d):
        raise ValueError("the rectangle needs a < b and c < d")
    eP, eQ = parse_expr(P, _XY), parse_expr(Q, _XY)
    g = _fl(sp.diff(eQ, X) - sp.diff(eP, Y), _XY)
    dbl, S = _settled(lambda n: _gl2(g, a, b, lambda x: c, lambda x: d, n),
                      lambda n: _gl2(lambda x, y: abs(g(x, y)), a, b, lambda x: c, lambda x: d, n), (24, 40, 64))
    side = lambda e, v, lo, hi: 0.0 if not e.free_symbols else quad(_compile(e, (v,)), lo, hi)  # noqa: E731
    const = lambda e, lo, hi: float(e) * (hi - lo) if not e.free_symbols else None  # noqa: E731
    parts = []
    for e, v, lo, hi, s in ((eP.subs(Y, _rat(c)), X, a, b, 1), (eQ.subs(X, _rat(b)), Y, c, d, 1),
                            (eP.subs(Y, _rat(d)), X, a, b, -1), (eQ.subs(X, _rat(a)), Y, c, d, -1)):
        k = const(e, lo, hi)
        parts.append(s * (k if k is not None else side(e, v, lo, hi)))
    circ = math.fsum(parts)
    if abs(circ - dbl) > 1e-8 * max(abs(dbl), 1e-3 * S, 1e-12):
        raise ValueError("the two sides of Green's theorem disagree numerically (is the field smooth on the rectangle?)")
    return {"circulation": circ, "double_integral": dbl}


def _verify_green(i, o):
    eP, eQ = parse_expr(i["P"], _XY), parse_expr(i["Q"], _XY)
    g = _fl(sp.diff(eQ, X) - sp.diff(eP, Y), _XY)
    c, d = i["c"], i["d"]
    S = _gl2(lambda x, y: abs(g(x, y)), i["a"], i["b"], lambda x: c, lambda x: d, 32)
    v = _gl2(g, i["a"], i["b"], lambda x: c, lambda x: d, 32)
    return worst(_scaled_err(o["double_integral"], v, S), _scaled_err(o["circulation"], v, S))


def _ref_green(P, Q, a, b, c, d):
    from scipy.integrate import dblquad
    gP, gQ = ref_lam(P, _XY, module="math"), ref_lam(Q, _XY, module="math")
    circ = (ref_quad(lambda x: gP(x, c), a, b) + ref_quad(lambda y: gQ(b, y), c, d)
            - ref_quad(lambda x: gP(x, d), a, b) - ref_quad(lambda y: gQ(a, y), c, d))
    curl2 = lambda x, y: ref_cpartial(Q, _XY, (x, y), 0) - ref_cpartial(P, _XY, (x, y), 1)  # noqa: E731
    return {"circulation": circ, "double_integral": float(dblquad(lambda y, x: curl2(x, y), a, b, c, d, epsabs=1e-13, epsrel=1e-13)[0])}


def _rand_green(rng):
    a, b = _c(rng, -3, 3), _c(rng, -3, 3)
    P, Q = rng.choice([("-y", "x"), (f"x*y", "x^2"), (f"{a}*y^2 + x", f"x*y + {b}*x"), ("exp(x)*y", "x^2*y"), ("sin(y)", f"{a}*x^3 + y"),
                       (f"x^2*y + {a}*y^3", f"x*y^2 - {b}*x")])
    (a0, b0), (c0, d0) = rand_interval(rng, -2, 2), rand_interval(rng, -2, 2)
    return {"P": _sgn(P), "Q": _sgn(Q), "a": a0, "b": b0, "c": c0, "d": d0}


proc("greens_theorem", name="Green's theorem on a rectangle", topic="vector calculus",
     purpose="Both sides of Green's theorem for F = (P, Q) on the rectangle [a, b] x [c, d]: the counter-clockwise "
             "circulation around the boundary and the double integral of Q_x - P_y.",
     inputs=[T_("P", "the x-component P(x, y)"), T_("Q", "the y-component Q(x, y)"), N_("a", "x from"), N_("b", "x to"),
             N_("c", "y from"), N_("d", "y to")],
     outputs=[O_("circulation", "the boundary line integral (counter-clockwise)"), O_("double_integral", "the integral of Q_x - P_y")],
     primary="circulation", conditions=["a < b, c < d", "P and Q continuously differentiable on the rectangle"],
     method="the four sides by tanh-sinh quadrature; the double integral of the exact Q_x - P_y by Gauss-Legendre product "
            "rules; refused unless the two agree",
     latex=r"\oint_C P\,dx + Q\,dy = \iint_R \left(Q_x - P_y\right)dA",
     run=greens_theorem, verify=_verify_green, reference=_ref_green,
     wrong=lambda P, Q, a, b, c, d: {k: -v for k, v in greens_theorem(P, Q, a, b, c, d).items()},
     random_inputs=_rand_green,
     templates=["Verify Green's theorem for F = (-y, x) on the rectangle [0, 1] x [0, 2]."],
     examples=[{"inputs": {"P": "-y", "Q": "x", "a": 0, "b": 1, "c": 0, "d": 2}, "outputs": {"circulation": 4.0, "double_integral": 4.0}},
               {"inputs": {"P": "x*y", "Q": "x^2", "a": 0, "b": 1, "c": 0, "d": 1}, "outputs": {"circulation": 0.5}}],
     questions=[{"q": "Verify Green's theorem for F = (-y, x) on the rectangle [0, 1] x [0, 2].", "outputs": {"circulation": 4.0}},
                {"q": "Use Green's theorem for F = (x*y, x^2) on [0, 1] x [0, 1].", "outputs": {"double_integral": 0.5}}],
     keywords=["green's theorem", "circulation"])
rule("greens_theorem",
     "<(?:verify |use |apply |check )?>green<'?>s theorem <(?:for|on|with)> <(?:the )?(?:vector )?(?:field )?>f = ({P}, {Q}) on "
     "<(?:the rectangle )?>[{a:n}, {b:n}] " + _X + " [{c:n}, {d:n}]", order=15)


@_memo
def divergence_theorem_box(P, Q, R, x0, x1, y0, y1, z0, z1):
    if not (x0 < x1 and y0 < y1 and z0 < z1):
        raise ValueError("the box needs x0 < x1, y0 < y1, z0 < z1")
    (eP, eQ, eR), _n = _field(P, Q, R)
    div = sp.diff(eP, X) + sp.diff(eQ, Y) + sp.diff(eR, Z)
    if div.is_polynomial(X, Y, Z):
        return {"flux": _pfloat(sp.integrate(div, (X, _rat(x0), _rat(x1)), (Y, _rat(y0), _rat(y1)), (Z, _rat(z0), _rat(z1))))}
    g = _fl(div, _XYZ)
    bx = ((x0, x1), (y0, y1), (z0, z1))
    return {"flux": _settled(lambda n: _gl3(g, bx, n), lambda n: _gl3(lambda x, y, z: abs(g(x, y, z)), bx, n), (12, 20, 28))[0]}


def _verify_divthm(i, o):
    (eP, eQ, eR), _n = _field(i["P"], i["Q"], i["R"])
    g = _fl(sp.diff(eP, X) + sp.diff(eQ, Y) + sp.diff(eR, Z), _XYZ)
    bx = ((i["x0"], i["x1"]), (i["y0"], i["y1"]), (i["z0"], i["z1"]))
    return _scaled_err(o["flux"], _gl3(g, bx, 16), _gl3(lambda x, y, z: abs(g(x, y, z)), bx, 16))


def _ref_divthm(P, Q, R, x0, x1, y0, y1, z0, z1):
    from scipy.integrate import dblquad
    gP, gQ, gR = (ref_lam(c, _XYZ, module="math") for c in (P, Q, R))
    opt = {"epsabs": 1e-13, "epsrel": 1e-13}
    fx = dblquad(lambda z, y: gP(x1, y, z) - gP(x0, y, z), y0, y1, z0, z1, **opt)[0]
    fy = dblquad(lambda z, x: gQ(x, y1, z) - gQ(x, y0, z), x0, x1, z0, z1, **opt)[0]
    fz = dblquad(lambda y, x: gR(x, y, z1) - gR(x, y, z0), x0, x1, y0, y1, **opt)[0]
    return {"flux": float(fx + fy + fz)}


def _rand_divthm(rng):
    a = _c(rng, -3, 3)
    pool = ["x^2", "y^2", "z^2", "x*y", "y*z", "x*z", f"{a}*x", "x*y*z", "z^3", f"x + {a}*y", "sin(y)", "exp(z/2)"]
    out = {"P": _sgn(rng.choice(pool)), "Q": _sgn(rng.choice(pool)), "R": _sgn(rng.choice(pool))}
    for v in ("x", "y", "z"):
        lo, hi = rand_interval(rng, -2, 2)
        out[v + "0"], out[v + "1"] = lo, hi
    return out


proc("divergence_theorem_box", name="Outward flux through a box (divergence theorem)", topic="vector calculus",
     purpose="The outward flux of F = (P, Q, R) through the surface of the box [x0, x1] x [y0, y1] x [z0, z1].",
     inputs=[T_("P", "the x-component"), T_("Q", "the y-component"), T_("R", "the z-component"), N_("x0", "x from"), N_("x1", "x to"),
             N_("y0", "y from"), N_("y1", "y to"), N_("z0", "z from"), N_("z1", "z to")],
     outputs=[O_("flux", "the outward flux")],
     primary="flux", conditions=["the box is non-degenerate", "F continuously differentiable on the box"],
     method="the divergence theorem: the triple integral of the exact divergence (exact for polynomials, otherwise "
            "Gauss-Legendre product rules until two orders agree)",
     latex=r"\oiint_S \mathbf F\cdot\mathbf n\,dS = \iiint_B \nabla\cdot\mathbf F\,dV",
     run=divergence_theorem_box, verify=_verify_divthm, reference=_ref_divthm,
     wrong=lambda P, Q, R, x0, x1, y0, y1, z0, z1: {"flux": -divergence_theorem_box(P, Q, R, x0, x1, y0, y1, z0, z1)["flux"]},
     random_inputs=_rand_divthm,
     templates=["Find the outward flux of F = (x^2, y^2, z^2) through the box [0, 1] x [0, 1] x [0, 1]."],
     examples=[{"inputs": {"P": "x^2", "Q": "y^2", "R": "z^2", "x0": 0, "x1": 1, "y0": 0, "y1": 1, "z0": 0, "z1": 1}, "outputs": {"flux": 3.0}},
               {"inputs": {"P": "x", "Q": "y", "R": "z", "x0": 0, "x1": 1, "y0": 0, "y1": 2, "z0": 0, "z1": 3}, "outputs": {"flux": 18.0}}],
     questions=[{"q": "Find the outward flux of F = (x^2, y^2, z^2) through the box [0, 1] x [0, 1] x [0, 1].", "outputs": {"flux": 3.0}},
                {"q": "Find the flux of the field F = (x, y, z) through the box [0, 1] x [0, 2] x [0, 3].", "outputs": {"flux": 18.0}}],
     keywords=["divergence theorem", "flux", "gauss's theorem"])
rule("divergence_theorem_box",
     "<(?:outward )?>flux of <(?:the )?(?:vector )?(?:field )?>f = ({P}, {Q}, {R}) <(?:through|across|out of)> <(?:the surface of )?(?:the )?>box " + _BOX,
     order=15)


# =========================================================================================== 15. parametric and polar curves
@_memo
def parametric_arc_length(xt, yt, t0, t1):
    if not t0 < t1:
        raise ValueError("the parameter interval needs t0 < t1")
    ex, ey = _path(xt, yt)
    return {"arc_length": _q_t(sp.sqrt(sp.diff(ex, T) ** 2 + sp.diff(ey, T) ** 2), t0, t1)}


proc("parametric_arc_length", name="Arc length of a parametric curve", topic="parametric curves",
     purpose="The length of the curve x = x(t), y = y(t) for t0 <= t <= t1.",
     inputs=[T_("xt", "x(t)"), T_("yt", "y(t)"), N_("t0", "start parameter"), N_("t1", "end parameter")],
     outputs=[O_("arc_length", "the length of the curve")],
     primary="arc_length", conditions=["t0 < t1", "x and y continuously differentiable"],
     method="sqrt(x'(t)^2 + y'(t)^2) with exact derivatives, tanh-sinh quadrature",
     latex=r"L = \int_{t_0}^{t_1}\sqrt{x'(t)^2 + y'(t)^2}\,dt",
     run=parametric_arc_length,
     verify=_verify_line(lambda i: sp.sqrt(sp.diff(_path(i["xt"], i["yt"])[0], T) ** 2 + sp.diff(_path(i["xt"], i["yt"])[1], T) ** 2), "arc_length"),
     reference=lambda xt, yt, t0, t1: (lambda dx, dy: {"arc_length": ref_quad(lambda t: math.hypot(dx(t), dy(t)), t0, t1)})(
         ref_cdiff(xt, ("t",)), ref_cdiff(yt, ("t",))),
     wrong=lambda xt, yt, t0, t1: (lambda ex, ey: {"arc_length": _q_t(sp.diff(ex, T) ** 2 + sp.diff(ey, T) ** 2, t0, t1)})(*_path(xt, yt)),
     random_inputs=lambda rng: rng.choice([{"xt": "cos(t)", "yt": "sin(t)", "t0": 0.0, "t1": round(rng.uniform(1, 6), 2)},
                                           {"xt": "t", "yt": "t^2", "t0": round(rng.uniform(-2, 0), 2), "t1": round(rng.uniform(0.5, 2), 2)},
                                           {"xt": "t^2", "yt": "t^3", "t0": 0.5, "t1": round(rng.uniform(1, 3), 2)},
                                           {"xt": "t - sin(t)", "yt": "1 - cos(t)", "t0": 0.5, "t1": round(rng.uniform(1, 5.5), 2)},
                                           {"xt": f"{rng.randint(1, 4)}*t", "yt": f"{rng.randint(1, 4)}*t + 1", "t0": 0.0, "t1": float(rng.randint(1, 4))},
                                           {"xt": "exp(t)*cos(t)", "yt": "exp(t)*sin(t)", "t0": 0.0, "t1": round(rng.uniform(0.5, 2), 2)}]),
     templates=["Find the arc length of the parametric curve x = cos(t), y = sin(t) from t = 0 to t = pi."],
     examples=[{"inputs": {"xt": "cos(t)", "yt": "sin(t)", "t0": 0, "t1": math.pi}, "outputs": {"arc_length": math.pi}},
               {"inputs": {"xt": "3t", "yt": "4t", "t0": 0, "t1": 2}, "outputs": {"arc_length": 10.0}}],
     questions=[{"q": "Find the arc length of the parametric curve x = cos(t), y = sin(t) from t = 0 to t = pi.", "outputs": {"arc_length": math.pi}},
                {"q": "Find the length of the curve x = 3t, y = 4t from t = 0 to t = 2.", "outputs": {"arc_length": 10.0}}],
     keywords=["arc length", "parametric curve"])
rule("parametric_arc_length",
     "<(?:arc length|length)> of the <(?:parametric )?>curve x = {xt}, y = {yt}<,?> from t = {t0:n} to t = {t1:n}", order=15)


@_memo
def parametric_slope(xt, yt, t0):
    ex, ey = _path(xt, yt)
    dx, dy = sp.diff(ex, T), sp.diff(ey, T)
    vx = at(dx, (T,), t0)
    if abs(vx) <= 1e-12 * max(1.0, abs(at(dy, (T,), t0))):
        raise ValueError("dx/dt = 0 at this parameter: the tangent is vertical (dy/dx undefined)")
    m = dy / dx
    return {"dy_dx": at(dy, (T,), t0) / vx, "d2y_dx2": at(sp.diff(m, T) / dx, (T,), t0)}


def _ref_pslope(xt, yt, t0):
    gx, gy = ref_lam(xt, ("t",)), ref_lam(yt, ("t",))
    with mp.workdps(40):
        m = lambda t: mp.diff(gy, t) / mp.diff(gx, t)  # noqa: E731
        T0 = mp.mpf(t0)
        return {"dy_dx": float(m(T0)), "d2y_dx2": float(mp.diff(m, T0) / mp.diff(gx, T0))}


def _verify_pslope(i, o):
    with mp.workdps(30):
        gx, gy = (_compile(e, (T,)) for e in _path(i["xt"], i["yt"]))
        T0 = mp.mpf(i["t0"])
        m = mp.diff(gy, T0) / mp.diff(gx, T0)
        d2 = mp.diff(lambda t: mp.diff(gy, t) / mp.diff(gx, t), T0) / mp.diff(gx, T0)
    return worst(rel_err(o["dy_dx"], float(m), 1e-11), rel_err(o["d2y_dx2"], float(d2), 1e-9))


proc("parametric_slope", name="Slope and concavity of a parametric curve", topic="parametric curves",
     purpose="dy/dx and d^2y/dx^2 of the curve x = x(t), y = y(t) at a parameter value t0.",
     inputs=[T_("xt", "x(t)"), T_("yt", "y(t)"), N_("t0", "the parameter value")],
     outputs=[O_("dy_dx", "dy/dx at t0"), O_("d2y_dx2", "d^2y/dx^2 at t0")],
     primary="dy_dx", conditions=["dx/dt != 0 at t0"],
     method="dy/dx = y'(t)/x'(t) and d^2y/dx^2 = (d/dt of dy/dx)/x'(t), exact derivatives",
     latex=r"\frac{dy}{dx} = \frac{y'(t)}{x'(t)},\quad \frac{d^2y}{dx^2} = \frac{\frac{d}{dt}\frac{dy}{dx}}{x'(t)}",
     run=parametric_slope, verify=_verify_pslope, reference=_ref_pslope,
     wrong=lambda xt, yt, t0: (lambda r: {"dy_dx": 1 / r["dy_dx"] if r["dy_dx"] else 1.0, "d2y_dx2": r["d2y_dx2"]})(parametric_slope(xt, yt, t0)),
     random_inputs=lambda rng: dict(rng.choice([{"xt": "t^2", "yt": "t^3"}, {"xt": "cos(t)", "yt": "sin(t)"}, {"xt": "t + 1", "yt": "t^2 - 3*t"},
                                                {"xt": "exp(t)", "yt": "t*exp(t)"}, {"xt": "t - sin(t)", "yt": "1 - cos(t)"},
                                                {"xt": "2*cos(t) + t", "yt": "sin(2*t)"}]), t0=round(rng.uniform(0.3, 2.5), 2)),
     templates=["Find dy/dx for x = t^2, y = t^3 at t = 1."],
     examples=[{"inputs": {"xt": "t^2", "yt": "t^3", "t0": 1}, "outputs": {"dy_dx": 1.5, "d2y_dx2": 0.75}},
               {"inputs": {"xt": "cos(t)", "yt": "sin(t)", "t0": math.pi / 4}, "outputs": {"dy_dx": -1.0, "d2y_dx2": -2 * math.sqrt(2)}}],
     questions=[{"q": "Find dy/dx for x = t^2, y = t^3 at t = 1.", "outputs": {"dy_dx": 1.5, "d2y_dx2": 0.75}},
                {"q": "Find dy/dx for x = t^2, y = t^3 at t = 0.", "outcome": "INVALID_INPUT"}],
     keywords=["parametric derivative", "slope of a parametric curve"])
rule("parametric_slope", "<(?:dy/dx|the slope|slope)> <(?:for|of)> <(?:the (?:parametric )?curve )?>x = {xt}, y = {yt} at t = {t0:n}", order=15)


def _ref_polar_area(r, al, be):
    import numpy as np
    g = ref_lam(r, ("theta",), module="numpy")

    def fan(N):                                         # triangles from the origin: an independent (geometric) route
        th = np.linspace(al, be, N + 1)
        rr = np.asarray(g(th), dtype=float) * np.ones_like(th)
        return 0.5 * float(np.sum(rr[:-1] * rr[1:])) * math.sin((be - al) / N)
    R = [fan(256 * 2 ** k) for k in range(5)]
    for j in range(1, 5):
        R = [(4 ** j * R[k + 1] - R[k]) / (4 ** j - 1) for k in range(len(R) - 1)]
    return {"area": R[0]}


@_memo
def polar_area(r, al, be):
    if not al < be:
        raise ValueError("the angle interval needs alpha < beta")
    if be - al > 2 * math.pi + 1e-12:
        raise ValueError("the angle interval may span at most one full turn (2 pi)")
    e = parse_expr(r, ("theta",))
    continuous_on(e, TH, al, be)
    return {"area": quad(_compile(e ** 2 / 2, (TH,)), al, be)}


proc("polar_area", name="Area in polar coordinates", topic="polar curves",
     purpose="The area swept by the polar curve r = r(theta) between theta = alpha and theta = beta.",
     inputs=[T_("r", "the polar curve r(theta)"), N_("al", "alpha, the start angle (radians)"), N_("be", "beta, the end angle (radians)")],
     outputs=[O_("area", "the area")],
     primary="area", conditions=["alpha < beta <= alpha + 2 pi", "r continuous"],
     method="(1/2) integral of r^2 by tanh-sinh quadrature",
     latex=r"A = \frac{1}{2}\int_{\alpha}^{\beta} r(\theta)^2\,d\theta",
     run=polar_area,
     verify=lambda i, o: rel_err(o["area"], _gl_check(f"({i['r']})^2/2", i["al"], i["be"], names=("theta",)), 1e-10),
     reference=_ref_polar_area,
     wrong=lambda r, al, be: {"area": quad(_compile(parse_expr(r, ("theta",)) ** 2, (TH,)), al, be)},
     random_inputs=lambda rng: (lambda a, b, k: {"r": rng.choice([f"{a + b} + {b}*cos(theta)", f"{a}*sin({k}*theta)", f"{a}*exp(theta/{k + 2})",
                                                                 f"{a + b} + {b}*sin(theta)", f"{a} + theta"]),
                                                  "al": 0.0, "be": round(rng.uniform(0.5, 2 * math.pi), 2)})(rng.randint(1, 3), rng.randint(1, 3), rng.randint(1, 4)),
     templates=["Find the area enclosed by the polar curve r = 1 + cos(theta) from theta = 0 to theta = 2pi."],
     examples=[{"inputs": {"r": "1 + cos(theta)", "al": 0, "be": 2 * math.pi}, "outputs": {"area": 1.5 * math.pi}},
               {"inputs": {"r": "2", "al": 0, "be": math.pi / 2}, "outputs": {"area": math.pi}}],
     questions=[{"q": "Find the area enclosed by the polar curve r = 1 + cos(theta) from theta = 0 to theta = 2pi.", "outputs": {"area": 1.5 * math.pi}},
                {"q": "Find the area of the polar region r = 2 from theta = 0 to theta = pi/2.", "outputs": {"area": math.pi}},
                {"q": "Find the area enclosed by the polar curve r = 1 + cos(theta) from theta = 0 to theta = 4pi.", "outcome": "INVALID_INPUT"}],
     keywords=["polar area", "polar coordinates"])
rule("polar_area", "area <(?:enclosed by|inside|of|bounded by|swept by)> the polar <(?:curve |region )?>r = {r} from theta = {al:n} to theta = {be:n}",
     order=15)


# =========================================================================================== 16. special functions
def gamma_function(x):
    try:
        v = math.gamma(x)
    except (ValueError, OverflowError):
        raise ValueError("the gamma function is undefined at 0 and negative whole numbers, and overflows beyond x = 171") from None
    return {"gamma": v}


proc("gamma_function", name="Gamma function", topic="special functions",
     purpose="The value of the gamma function (Gamma(n) = (n-1)! for whole n).",
     inputs=[N_("x", "the argument x")], outputs=[O_("gamma", "Gamma(x)")],
     primary="gamma", conditions=["x is not 0 or a negative whole number", "x < 171.6"],
     method="the standard library's Lanczos-type approximation (double precision)",
     latex=r"\Gamma(x) = \int_0^\infty t^{x-1}e^{-t}\,dt",
     run=gamma_function,
     verify=lambda i, o: rel_err(o["gamma"], float(mp.gamma(i["x"])), 1e-13),
     reference=lambda x: {"gamma": float(__import__("scipy.special", fromlist=["gamma"]).gamma(x))},
     wrong=lambda x: {"gamma": math.gamma(x + 1)},
     random_inputs=lambda rng: {"x": rng.choice([round(rng.uniform(0.05, 12), 2), round(rng.uniform(-4.9, -0.1), 2) + 0.005, float(rng.randint(1, 15))])},
     templates=["Evaluate the gamma function at x = 4.5.", "Find Gamma(5)."],
     examples=[{"inputs": {"x": 5}, "outputs": {"gamma": 24.0}}, {"inputs": {"x": 0.5}, "outputs": {"gamma": math.sqrt(math.pi)}},
               {"inputs": {"x": 4.5}, "outputs": {"gamma": 105 / 16 * math.sqrt(math.pi)}}],
     questions=[{"q": "Evaluate the gamma function at x = 4.5.", "outputs": {"gamma": 105 / 16 * math.sqrt(math.pi)}},
                {"q": "Find Gamma(5).", "outputs": {"gamma": 24.0}}, {"q": "Find Gamma(-2).", "outcome": "INVALID_INPUT"}],
     keywords=["gamma function", "factorial"])
rule("gamma_function", "gamma({x:n})", "gamma function <(?:at|of)> <(?:x = )?>{x:n}", order=20)


def beta_function(a, b):
    if not (a > 0 and b > 0):
        raise ValueError("this capability needs a > 0 and b > 0")
    return {"beta": math.exp(math.lgamma(a) + math.lgamma(b) - math.lgamma(a + b))}


proc("beta_function", name="Beta function", topic="special functions",
     purpose="The beta function B(a, b) for positive a and b.",
     inputs=[N_("a", "the first argument (> 0)"), N_("b", "the second argument (> 0)")], outputs=[O_("beta", "B(a, b)")],
     primary="beta", conditions=["a > 0", "b > 0"], method="exp(ln Gamma(a) + ln Gamma(b) - ln Gamma(a + b))",
     latex=r"B(a,b) = \int_0^1 t^{a-1}(1-t)^{b-1}dt = \frac{\Gamma(a)\Gamma(b)}{\Gamma(a+b)}",
     run=beta_function,
     verify=lambda i, o: rel_err(o["beta"], float(mp.beta(i["a"], i["b"])), 1e-12),
     reference=lambda a, b: {"beta": float(__import__("scipy.special", fromlist=["beta"]).beta(a, b))},
     wrong=lambda a, b: {"beta": math.gamma(a) * math.gamma(b) / math.gamma(a + b + 1)},
     random_inputs=lambda rng: {"a": round(rng.uniform(0.2, 8), 2), "b": round(rng.uniform(0.2, 8), 2)},
     templates=["Evaluate the beta function B(2, 3)."],
     examples=[{"inputs": {"a": 2, "b": 3}, "outputs": {"beta": 1 / 12}}, {"inputs": {"a": 0.5, "b": 0.5}, "outputs": {"beta": math.pi}}],
     questions=[{"q": "Evaluate the beta function B(2, 3).", "outputs": {"beta": 1 / 12}},
                {"q": "Find the beta function B(0.5, 0.5).", "outputs": {"beta": math.pi}},
                {"q": "Evaluate the beta function B(-1, 3).", "outcome": "INVALID_INPUT"}],
     keywords=["beta function"])
rule("beta_function", "beta function <(?:b)?>({a:n}, {b:n})", order=20)


def error_function(x):
    return {"erf": math.erf(x), "erfc": math.erfc(x)}


proc("error_function", name="Error function", topic="special functions",
     purpose="The error function erf(x) and the complementary error function erfc(x) = 1 - erf(x).",
     inputs=[N_("x", "the argument x")], outputs=[O_("erf", "erf(x)"), O_("erfc", "erfc(x)")],
     primary="erf", conditions=["x real"], method="the standard library's erf and erfc (double precision)",
     latex=r"\operatorname{erf}(x) = \frac{2}{\sqrt{\pi}}\int_0^x e^{-t^2}\,dt",
     run=error_function,
     verify=lambda i, o: worst(rel_err(o["erf"], float(mp.erf(i["x"])), 1e-13), rel_err(o["erfc"], float(mp.erfc(i["x"])), 1e-13)),
     reference=lambda x: (lambda s: {"erf": float(s.erf(x)), "erfc": float(s.erfc(x))})(__import__("scipy.special", fromlist=["erf"])),
     wrong=lambda x: {"erf": math.erf(x / math.sqrt(2)), "erfc": math.erfc(x / math.sqrt(2))},
     random_inputs=lambda rng: {"x": round(rng.uniform(-3, 3), 3)},
     templates=["Evaluate erf(0.5).", "Find the error function at x = 1."],
     examples=[{"inputs": {"x": 0}, "outputs": {"erf": 0.0, "erfc": 1.0}}, {"inputs": {"x": 0.5}, "outputs": {"erf": 0.5204998778130465}}],
     questions=[{"q": "Evaluate erf(0.5).", "outputs": {"erf": 0.5204998778130465}}, {"q": "Find the error function at x = 0.", "outputs": {"erf": 0.0}}],
     keywords=["error function", "erf", "erfc"])
rule("error_function", "erf({x:n})", "<(?:the )?>error function <(?:at|of)> <(?:x = )?>{x:n}", order=20)


def _reg_gamma(a, x):
    if x == 0:
        return 0.0, 1.0
    lead = math.exp(-x + a * math.log(x) - math.lgamma(a))
    if x < a + 1:                                         # series for the lower function
        ap, d = a, 1.0 / a
        s = d
        for _ in range(100000):
            ap += 1
            d *= x / ap
            s += d
            if abs(d) < abs(s) * 1e-17:
                break
        P = s * lead
        return P, 1.0 - P
    tiny = 1e-300                                         # continued fraction (modified Lentz) for the upper function
    b = x + 1 - a
    c, d = 1 / tiny, 1 / b
    h = d
    for k in range(1, 100000):
        an = -k * (k - a)
        b += 2
        d = an * d + b
        d = tiny if abs(d) < tiny else d
        c = b + an / c
        c = tiny if abs(c) < tiny else c
        d = 1 / d
        de = d * c
        h *= de
        if abs(de - 1) < 3e-16:
            break
    Q = lead * h
    return 1.0 - Q, Q


def regularized_gamma(a, x):
    if not a > 0 or x < 0:
        raise ValueError("this capability needs a > 0 and x >= 0")
    if a > 1e4 or x > 1e5:
        raise ValueError("a and x are limited to 1e4 and 1e5")
    P, Q = _reg_gamma(a, x)
    return {"P": P, "Q": Q}


proc("regularized_gamma", name="Regularized incomplete gamma functions", topic="special functions",
     purpose="The regularized lower and upper incomplete gamma functions P(a, x) and Q(a, x) = 1 - P(a, x).",
     inputs=[N_("a", "the shape a (> 0)"), N_("x", "the upper limit x (>= 0)")],
     outputs=[O_("P", "P(a, x) = gamma(a, x)/Gamma(a)"), O_("Q", "Q(a, x) = Gamma(a, x)/Gamma(a)")],
     primary="P", conditions=["a > 0", "x >= 0"],
     method="power series for x < a + 1, Lentz continued fraction otherwise (the smaller of P and Q computed directly)",
     latex=r"P(a,x) = \frac{1}{\Gamma(a)}\int_0^x t^{a-1}e^{-t}\,dt",
     run=regularized_gamma,
     verify=lambda i, o: worst(rel_err(o["P"], float(mp.gammainc(i["a"], 0, i["x"], regularized=True)), 1e-11),
                               rel_err(o["Q"], float(mp.gammainc(i["a"], i["x"], mp.inf, regularized=True)), 1e-11)),
     reference=lambda a, x: (lambda s: {"P": float(s.gammainc(a, x)), "Q": float(s.gammaincc(a, x))})(__import__("scipy.special", fromlist=["gammainc"])),
     wrong=lambda a, x: (lambda r: {"P": r["P"] * math.gamma(a), "Q": r["Q"] * math.gamma(a)})(regularized_gamma(a, x)),
     random_inputs=lambda rng: (lambda a: {"a": a, "x": round(a * rng.uniform(0.3, 2.0), 2)})(round(rng.uniform(0.5, 10), 2)),
     templates=["Find the regularized incomplete gamma function P(2, 3)."],
     examples=[{"inputs": {"a": 1, "x": 2}, "outputs": {"P": 1 - math.exp(-2), "Q": math.exp(-2)}},
               {"inputs": {"a": 2, "x": 3}, "outputs": {"P": 1 - 4 * math.exp(-3)}}],
     questions=[{"q": "Find the regularized incomplete gamma function P(2, 3).", "outputs": {"P": 1 - 4 * math.exp(-3)}},
                {"q": "Find the regularized incomplete gamma function P(-1, 3).", "outcome": "INVALID_INPUT"}],
     keywords=["incomplete gamma function", "regularized gamma"])
rule("regularized_gamma", "<(?:the )?(?:regularized )?(?:lower )?>incomplete gamma <(?:function )?>p({a:n}, {x:n})", order=20)


def bessel_j(n, x):
    n = int(n)
    if not 0 <= n <= 50 or abs(x) > 1000:
        raise ValueError("this capability covers orders 0..50 and |x| <= 1000")
    with mp.workdps(30):
        return {"J": _fin(mp.besselj(n, x))}


proc("bessel_j", name="Bessel function of the first kind", topic="special functions",
     purpose="The Bessel function J_n(x) of whole-number order n.",
     inputs=[I_("n", "the order n", 0, 50), N_("x", "the argument x")], outputs=[O_("J", "J_n(x)")],
     primary="J", conditions=["0 <= n <= 50", "|x| <= 1000"], method="hypergeometric series at 30 digits",
     latex=r"J_n(x) = \sum_{m=0}^{\infty}\frac{(-1)^m}{m!\,(m+n)!}\left(\frac{x}{2}\right)^{2m+n}",
     run=bessel_j,
     verify=lambda i, o: rel_err(o["J"], float(mp.quad(lambda t: mp.cos(int(i["n"]) * t - i["x"] * mp.sin(t)), [0, mp.pi]) / mp.pi), 1e-11),
     reference=lambda n, x: {"J": float(__import__("scipy.special", fromlist=["jv"]).jv(int(n), x))},
     wrong=lambda n, x: bessel_j(int(n) + 1, x),
     random_inputs=lambda rng: {"n": rng.randint(0, 5), "x": round(rng.uniform(0.1, 15), 2)},
     templates=["Evaluate the Bessel function J_0(1)."],
     examples=[{"inputs": {"n": 0, "x": 0}, "outputs": {"J": 1.0}}, {"inputs": {"n": 0, "x": 1}, "outputs": {"J": 0.7651976865579666}},
               {"inputs": {"n": 1, "x": 1}, "outputs": {"J": 0.4400505857449335}}],
     questions=[{"q": "Evaluate the Bessel function J_0(1).", "outputs": {"J": 0.7651976865579666}},
                {"q": "Evaluate the Bessel function of the first kind of order 1 at x = 1.", "outputs": {"J": 0.4400505857449335}}],
     keywords=["bessel function"])
rule("bessel_j", "bessel <(?:function )?>j<_?>{n:i}({x:n})", "bessel function of the first kind of order {n:i} at x = {x:n}", order=20)


_B2K = [Fraction(1, 6), Fraction(-1, 30), Fraction(1, 42), Fraction(-1, 30), Fraction(5, 66), Fraction(-691, 2730),
        Fraction(7, 6), Fraction(-3617, 510)]


def _zeta_em(s, N=12):
    """The verifier's route: Euler-Maclaurin summation of sum n^-s (N terms, then the integral and 8 Bernoulli
    corrections), at 30 digits."""
    with mp.workdps(30):
        s = mp.mpf(s)
        v = mp.fsum(mp.mpf(n) ** -s for n in range(1, N)) + mp.mpf(N) ** (1 - s) / (s - 1) + mp.mpf(N) ** -s / 2
        rise = s
        for k, B in enumerate(_B2K, start=1):
            v += mp.mpf(B.numerator) / B.denominator / mp.factorial(2 * k) * rise * mp.mpf(N) ** (-s - 2 * k + 1)
            rise *= (s + 2 * k - 1) * (s + 2 * k)
        return float(v)


def riemann_zeta(s):
    if not s > 1:
        raise ValueError("this capability covers s > 1 (where the defining series converges)")
    if s > 1e4:
        return {"zeta": 1.0}
    with mp.workdps(30):
        return {"zeta": _fin(mp.zeta(s))}


proc("riemann_zeta", name="Riemann zeta function", topic="special functions / series",
     purpose="The Riemann zeta function zeta(s) = sum 1/n^s for s > 1 (the p-series sum).",
     inputs=[N_("s", "the argument s (> 1)")], outputs=[O_("zeta", "zeta(s)")],
     primary="zeta", conditions=["s > 1"], method="Euler-Maclaurin / Borwein evaluation at 30 digits",
     latex=r"\zeta(s) = \sum_{n=1}^{\infty} \frac{1}{n^s}",
     run=riemann_zeta,
     verify=lambda i, o: rel_err(o["zeta"], _zeta_em(i["s"]), 1e-11),
     reference=lambda s: {"zeta": float(__import__("scipy.special", fromlist=["zeta"]).zeta(s))},
     wrong=lambda s: {"zeta": riemann_zeta(s)["zeta"] - 1},
     random_inputs=lambda rng: {"s": rng.choice([round(rng.uniform(1.1, 8), 2), float(rng.randint(2, 10))])},
     templates=["Evaluate the Riemann zeta function at s = 3.", "Find zeta(2)."],
     examples=[{"inputs": {"s": 2}, "outputs": {"zeta": math.pi ** 2 / 6}}, {"inputs": {"s": 4}, "outputs": {"zeta": math.pi ** 4 / 90}}],
     questions=[{"q": "Find zeta(2).", "outputs": {"zeta": math.pi ** 2 / 6}},
                {"q": "Evaluate the Riemann zeta function at s = 4.", "outputs": {"zeta": math.pi ** 4 / 90}},
                {"q": "Find zeta(1).", "outcome": "INVALID_INPUT"}],
     keywords=["riemann zeta", "p-series"])
rule("riemann_zeta", "zeta({s:n})", "<(?:the )?(?:riemann )?>zeta function <(?:at|of)> <(?:s = )?>{s:n}", order=20)


def legendre_polynomial(n, x):
    n = int(n)
    if not 0 <= n <= 200:
        raise ValueError("the degree must be between 0 and 200")
    p0, p1 = 1.0, x
    if n == 0:
        return {"P": 1.0}
    for k in range(1, n):
        p0, p1 = p1, ((2 * k + 1) * x * p1 - k * p0) / (k + 1)
    return {"P": p1}


proc("legendre_polynomial", name="Legendre polynomial", topic="special functions / orthogonal polynomials",
     purpose="The value of the Legendre polynomial P_n(x).",
     inputs=[I_("n", "the degree n", 0, 200), N_("x", "the argument x")], outputs=[O_("P", "P_n(x)")],
     primary="P", conditions=["0 <= n <= 200"], method="Bonnet's three-term recurrence (k+1) P_(k+1) = (2k+1) x P_k - k P_(k-1)",
     latex=r"(k+1)P_{k+1}(x) = (2k+1)\,x\,P_k(x) - k\,P_{k-1}(x)",
     run=legendre_polynomial,
     verify=lambda i, o: rel_err(o["P"], float(mp.legendre(int(i["n"]), i["x"])), 1e-11),
     reference=lambda n, x: {"P": float(__import__("scipy.special", fromlist=["eval_legendre"]).eval_legendre(int(n), x))},
     wrong=lambda n, x: (lambda n: {"P": 1.0 if n == 0 else functools.reduce(lambda pp, k: (pp[1], ((2 * k + 1) * x * pp[1] - k * pp[0]) / k), range(1, n), (1.0, x))[1]})(int(n)),
     random_inputs=lambda rng: {"n": rng.randint(2, 25), "x": round(rng.uniform(-1, 1), 3)},
     templates=["Evaluate the Legendre polynomial P_3(0.5)."],
     examples=[{"inputs": {"n": 2, "x": 0.5}, "outputs": {"P": -0.125}}, {"inputs": {"n": 3, "x": 0.5}, "outputs": {"P": -0.4375}}],
     questions=[{"q": "Evaluate the Legendre polynomial P_3(0.5).", "outputs": {"P": -0.4375}},
                {"q": "Find the Legendre polynomial of degree 2 at x = 0.5.", "outputs": {"P": -0.125}}],
     keywords=["legendre polynomial"])
rule("legendre_polynomial", "<(?:the )?>legendre polynomial p<_?>{n:i}({x:n})", "<(?:the )?>legendre polynomial of degree {n:i} at x = {x:n}", order=20)


def chebyshev_polynomial(n, x):
    n = int(n)
    if not 0 <= n <= 500:
        raise ValueError("the degree must be between 0 and 500")
    t0, t1 = 1.0, x
    if n == 0:
        return {"T": 1.0}
    for _ in range(1, n):
        t0, t1 = t1, 2 * x * t1 - t0
    return {"T": t1}


proc("chebyshev_polynomial", name="Chebyshev polynomial", topic="special functions / orthogonal polynomials",
     purpose="The value of the Chebyshev polynomial of the first kind T_n(x) (T_n(cos t) = cos(n t)).",
     inputs=[I_("n", "the degree n", 0, 500), N_("x", "the argument x")], outputs=[O_("T", "T_n(x)")],
     primary="T", conditions=["0 <= n <= 500"], method="the recurrence T_(k+1) = 2x T_k - T_(k-1)",
     latex=r"T_{k+1}(x) = 2x\,T_k(x) - T_{k-1}(x)",
     run=chebyshev_polynomial,
     verify=lambda i, o: rel_err(o["T"], float(mp.chebyt(int(i["n"]), i["x"])), 1e-11),
     reference=lambda n, x: {"T": float(__import__("scipy.special", fromlist=["eval_chebyt"]).eval_chebyt(int(n), x))},
     wrong=lambda n, x: (lambda n: {"T": 1.0 if n == 0 else functools.reduce(lambda tt, _k: (tt[1], 2 * x * tt[1] - tt[0]), range(1, n), (1.0, 2 * x))[1]})(int(n)),
     random_inputs=lambda rng: {"n": rng.randint(2, 30), "x": round(rng.uniform(-1, 1), 3)},
     templates=["Evaluate the Chebyshev polynomial T_3(0.5)."],
     examples=[{"inputs": {"n": 3, "x": 0.5}, "outputs": {"T": -1.0}}, {"inputs": {"n": 2, "x": 0.3}, "outputs": {"T": -0.82}}],
     questions=[{"q": "Evaluate the Chebyshev polynomial T_3(0.5).", "outputs": {"T": -1.0}},
                {"q": "Find the Chebyshev polynomial of degree 2 at x = 0.3.", "outputs": {"T": -0.82}}],
     keywords=["chebyshev polynomial"])
rule("chebyshev_polynomial", "<(?:the )?>chebyshev polynomial t<_?>{n:i}({x:n})", "<(?:the )?>chebyshev polynomial of degree {n:i} at x = {x:n}", order=20)


# =========================================================================================== 17. Fourier series
@_memo
def fourier_coefficients(f, L, k):
    k = int(k)
    if not L > 0:
        raise ValueError("the half-period L must be positive")
    if not 1 <= k <= 50:
        raise ValueError("the order must be between 1 and 50")
    e = parse_expr(f)
    continuous_on(e, X, -L, L)
    g = _compile(e, (X,))
    pts = [-L + 2 * L * j / (2 * k) for j in range(1, 2 * k)]
    a0 = quad(g, -L, L) / L
    ak = quad(lambda x: g(x) * mp.cos(k * mp.pi * x / L), -L, L, points=pts) / L
    bk = quad(lambda x: g(x) * mp.sin(k * mp.pi * x / L), -L, L, points=pts) / L
    return {"a0": a0, "a_k": ak, "b_k": bk}


@functools.lru_cache(maxsize=256)
def _fourier_gl(f, L, k):
    G = lambda tr: _gl(lambda x: tr(x), -L, L, panels=max(8, 4 * k))  # noqa: E731
    with mp.workdps(20):
        g = _compile(parse_expr(f), (X,))
        return (float(G(g)) / L, float(G(lambda x: g(x) * mp.cos(k * mp.pi * x / L))) / L,
                float(G(lambda x: g(x) * mp.sin(k * mp.pi * x / L))) / L)


def _verify_fourier(i, o):
    a0, ak, bk = _fourier_gl(i["f"], float(i["L"]), int(i["k"]))
    s = max(1.0, abs(a0))
    return worst(rel_err(o["a0"], a0, 1e-10), rel_err(o["a_k"] / s, ak / s, 1e-10), rel_err(o["b_k"] / s, bk / s, 1e-10))


def _ref_fourier(f, L, k):
    g = ref_lam(f, module="math")
    w = int(k) * math.pi / L
    return {"a0": ref_quad(g, -L, L) / L, "a_k": ref_quad(g, -L, L, weight="cos", wvar=w) / L,
            "b_k": ref_quad(g, -L, L, weight="sin", wvar=w) / L}


proc("fourier_coefficients", name="Fourier coefficients", topic="Fourier series",
     purpose="The Fourier coefficients a_0, a_k and b_k of f on [-L, L] (series a_0/2 + sum a_k cos(k pi x/L) + b_k sin(k pi x/L)).",
     inputs=[T_("f", "the function f(x)"), N_("L", "the half-period L (> 0)"), I_("k", "the order k", 1, 50)],
     outputs=[O_("a0", "a_0 = (1/L) integral of f"), O_("a_k", "a_k"), O_("b_k", "b_k")],
     primary="b_k", conditions=["L > 0", "f continuous on [-L, L]", "1 <= k <= 50"],
     method="tanh-sinh quadrature on the k half-wave subintervals at 20 digits",
     latex=r"a_k = \frac{1}{L}\int_{-L}^{L} f(x)\cos\frac{k\pi x}{L}dx,\quad b_k = \frac{1}{L}\int_{-L}^{L} f(x)\sin\frac{k\pi x}{L}dx",
     run=fourier_coefficients, verify=_verify_fourier, reference=_ref_fourier,
     wrong=lambda f, L, k: {key: v / 2 for key, v in fourier_coefficients(f, L, k).items()},
     random_inputs=lambda rng: {"f": rng.choice([rand_poly(rng, rng.randint(1, 3)), "exp(x/2)", _sgn(f"{_c(rng, -3, 3)}*x + sin({rng.randint(1, 3)}*x)"),
                                                 "x*cos(x)", "x^2 + x", "exp(-x^2)"]),
                                "L": rng.choice([1.0, 2.0, math.pi, 0.5]), "k": rng.randint(1, 6)},
     templates=["Find the Fourier coefficients of order 2 of f(x) = x on [-pi, pi]."],
     examples=[{"inputs": {"f": "x", "L": math.pi, "k": 2}, "outputs": {"a0": 0.0, "a_k": 0.0, "b_k": -1.0}},
               {"inputs": {"f": "x^2", "L": math.pi, "k": 1}, "outputs": {"a0": 2 * math.pi ** 2 / 3, "a_k": -4.0, "b_k": 0.0}}],
     questions=[{"q": "Find the Fourier coefficients of order 2 of f(x) = x on [-pi, pi].", "outputs": {"b_k": -1.0}},
                {"q": "Find the Fourier coefficients of order 1 of x^2 on the interval [-pi, pi].", "outputs": {"a_k": -4.0}},
                {"q": "Find the Fourier coefficients of order 1 of 1/x on [-1, 1].", "outcome": "INVALID_INPUT"}],
     keywords=["fourier series", "fourier coefficients"])
rule("fourier_coefficients", "fourier coefficients <(?:of order|for k =|for n =)> {k:i} of {f} on <(?:the interval )?>[{m:n}, {L:n}]",
     order=20, post=lambda d: (lambda m: {k: v for k, v in d.items() if k != "m"} if abs(m + d["L"]) < 1e-12 else None)(d["m"]))


def _newton_fix(rng, d):
    """Newton random cases start on the root's side of the nearest turning point of cos(x) - x/c."""
    if d["f"].startswith("cos"):
        d["x0"] = rng.choice([0.5, 1.0, 1.5])
    return d


PROCEDURES["newton_method"]["random_inputs"] = (lambda old: lambda rng: _newton_fix(rng, old(rng)))(
    PROCEDURES["newton_method"]["random_inputs"])
