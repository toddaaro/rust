#[doc = "Operations and constants for `float`"];

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

export to_str_common, to_str_exact, to_str, from_str;

// Import everything from the math crate
import math::float::*;

#[doc = "
Converts a float to a string

# Arguments

* num - The float value
* digits - The number of significant digits
* exact - Whether to enforce the exact number of significant digits
"]
fn to_str_common(num: float, digits: uint, exact: bool) -> str {
    if is_NaN(num) { ret "NaN"; }
    if num == infinity { ret "inf"; }
    if num == neg_infinity { ret "-inf"; }
    let mut (num, accum) = if num < 0.0 { (-num, "-") } else { (num, "") };
    let trunc = num as uint;
    let mut frac = num - (trunc as float);
    accum += uint::str(trunc);

    // FIXME: Can't seem to access consts::epsilon
    let epsilon = 0.000000001f;

    if (frac < epsilon && !exact) || digits == 0u { ret accum; }
    accum += ".";
    let mut i = digits;
    let mut epsilon_prime = 1. / pow_with_uint(10u, i);
    while i > 0u && (frac >= epsilon_prime || exact) {
        frac *= 10.0;
        epsilon_prime *= 10.0;
        let digit = frac as uint;
        accum += uint::str(digit);
        frac -= digit as float;
        i -= 1u;
    }
    ret accum;

}

#[doc = "
Converts a float to a string with exactly the number of
provided significant digits

# Arguments

* num - The float value
* digits - The number of significant digits
"]
fn to_str_exact(num: float, digits: uint) -> str {
    to_str_common(num, digits, true)
}

#[test]
fn test_to_str_exact_do_decimal() {
    let s = to_str_exact(5.0, 4u);
    assert s == "5.0000";
}


#[doc = "
Converts a float to a string with a maximum number of
significant digits

# Arguments

* num - The float value
* digits - The number of significant digits
"]
fn to_str(num: float, digits: uint) -> str {
    to_str_common(num, digits, false)
}

#[doc = "
Convert a string to a float

This function accepts strings such as

* '3.14'
* '+3.14', equivalent to '3.14'
* '-3.14'
* '2.5E10', or equivalently, '2.5e10'
* '2.5E-10'
* '', or, equivalently, '.' (understood as 0)
* '5.'
* '.5', or, equivalently,  '0.5'
* 'inf', '-inf', 'NaN'

Leading and trailing whitespace are ignored.

# Arguments

* num - A string

# Return value

`none` if the string did not represent a valid number.  Otherwise, `some(n)`
where `n` is the floating-point number represented by `[num]`.
"]
fn from_str(num: str) -> option<float> {
   if num == "inf" {
       ret some(infinity);
   } else if num == "-inf" {
       ret some(neg_infinity);
   } else if num == "NaN" {
       ret some(NaN);
   }

   let mut pos = 0u;               //Current byte position in the string.
                                   //Used to walk the string in O(n).
   let len = str::len(num);        //Length of the string, in bytes.

   if len == 0u { ret none; }
   let mut total = 0f;             //Accumulated result
   let mut c     = 'z';            //Latest char.

   //The string must start with one of the following characters.
   alt str::char_at(num, 0u) {
      '-' | '+' | '0' to '9' | '.' {}
      _ { ret none; }
   }

   //Determine if first char is '-'/'+'. Set [pos] and [neg] accordingly.
   let mut neg = false;               //Sign of the result
   alt str::char_at(num, 0u) {
      '-' {
          neg = true;
          pos = 1u;
      }
      '+' {
          pos = 1u;
      }
      _ {}
   }

   //Examine the following chars until '.', 'e', 'E'
   while(pos < len) {
       let char_range = str::char_range_at(num, pos);
       c   = char_range.ch;
       pos = char_range.next;
       alt c {
         '0' to '9' {
           total = total * 10f;
           total += ((c as int) - ('0' as int)) as float;
         }
         '.' | 'e' | 'E' {
           break;
         }
         _ {
           ret none;
         }
       }
   }

   if c == '.' {//Examine decimal part
      let mut decimal = 1f;
      while(pos < len) {
         let char_range = str::char_range_at(num, pos);
         c = char_range.ch;
         pos = char_range.next;
         alt c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6'| '7' | '8' | '9'  {
                 decimal /= 10f;
                 total += (((c as int) - ('0' as int)) as float)*decimal;
             }
             'e' | 'E' {
                 break;
             }
             _ {
                 ret none;
             }
         }
      }
   }

   if (c == 'e') | (c == 'E') {//Examine exponent
      let mut exponent = 0u;
      let mut neg_exponent = false;
      if(pos < len) {
          let char_range = str::char_range_at(num, pos);
          c   = char_range.ch;
          alt c  {
             '+' {
                pos = char_range.next;
             }
             '-' {
                pos = char_range.next;
                neg_exponent = true;
             }
             _ {}
          }
          while(pos < len) {
             let char_range = str::char_range_at(num, pos);
             c = char_range.ch;
             alt c {
                 '0' | '1' | '2' | '3' | '4' | '5' | '6'| '7' | '8' | '9' {
                     exponent *= 10u;
                     exponent += ((c as uint) - ('0' as uint));
                 }
                 _ {
                     break;
                 }
             }
             pos = char_range.next;
          }
          let multiplier = pow_with_uint(10u, exponent);
              //Note: not [int::pow], otherwise, we'll quickly
              //end up with a nice overflow
          if neg_exponent {
             total = total / multiplier;
          } else {
             total = total * multiplier;
          }
      } else {
         ret none;
      }
   }

   if(pos < len) {
     ret none;
   } else {
     if(neg) {
        total *= -1f;
     }
     ret some(total);
   }
}

#[test]
fn test_to_str_inf() {
    assert to_str(infinity, 10u) == "inf";
    assert to_str(-infinity, 10u) == "-inf";
}

//
// Local Variables:
// mode: rust
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// End:
//





