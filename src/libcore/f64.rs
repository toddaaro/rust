#[doc = "Operations and constants for `f64`"];

export NaN, infinity, neg_infinity;
export acos, asin, atan, atan2, cbrt, ceil, copysign, cos, cosh, floor;
export erf, erfc, exp, expm1, exp2, abs, abs_sub, mul_add, fmax, fmin;
export nextafter, frexp, hypot, ldexp, lgamma, ln, log_radix, ln1p;
export log10, log2, ilog_radix, modf, pow, round, sin, sinh, sqrt, tan;
export tanh, tgamma, trunc;
export j0, j1, jn, y0, y1, yn;
export is_NaN, negated, inverse, add, sub, mul, div, rem;
export lt, le, eq, ne, ge, gt;
export is_positive, is_negative, is_nonpositive, is_nonnegative;
export is_zero, is_infinite, signbit, logarithm, pow_with_uint;
// FIXME: Not working correctly
//export extensions;
export consts;

// Import everything from the math crate
import math::f64::*;

//
// Local Variables:
// mode: rust
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// End:
//
