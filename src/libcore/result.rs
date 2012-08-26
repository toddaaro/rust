//! A type representing either success or failure

import either::Either;

/// The result type
enum Result<T, U> {
    /// Contains the successful result value
    Ok(T),
    /// Contains the error value
    Err(U)
}

/**
 * Get the value out of a successful result
 *
 * # Failure
 *
 * If the result is an error
 */
pure fn get<T: copy, U>(res: Result<T, U>) -> T {
    match res {
      Ok(t) => t,
      Err(the_err) => unchecked {
        fail fmt!("get called on error result: %?", the_err)
      }
    }
}

/**
 * Get a reference to the value out of a successful result
 *
 * # Failure
 *
 * If the result is an error
 */
pure fn get_ref<T, U>(res: &a/Result<T, U>) -> &a/T {
    match *res {
        Ok(ref t) => t,
        Err(ref the_err) => unchecked {
            fail fmt!("get_ref called on error result: %?", the_err)
        }
    }
}

/**
 * Get the value out of an error result
 *
 * # Failure
 *
 * If the result is not an error
 */
pure fn get_err<T, U: copy>(res: Result<T, U>) -> U {
    match res {
      Err(u) => u,
      Ok(_) => fail ~"get_error called on ok result"
    }
}

/// Returns true if the result is `ok`
pure fn is_ok<T, U>(res: Result<T, U>) -> bool {
    match res {
      Ok(_) => true,
      Err(_) => false
    }
}

/// Returns true if the result is `err`
pure fn is_err<T, U>(res: Result<T, U>) -> bool {
    !is_ok(res)
}

/**
 * Convert to the `either` type
 *
 * `ok` result variants are converted to `either::right` variants, `err`
 * result variants are converted to `either::left`.
 */
pure fn to_either<T: copy, U: copy>(res: Result<U, T>) -> Either<T, U> {
    match res {
      Ok(res) => either::Right(res),
      Err(fail_) => either::Left(fail_)
    }
}

/**
 * Call a function based on a previous result
 *
 * If `res` is `ok` then the value is extracted and passed to `op` whereupon
 * `op`s result is returned. if `res` is `err` then it is immediately
 * returned. This function can be used to compose the results of two
 * functions.
 *
 * Example:
 *
 *     let res = chain(read_file(file)) { |buf|
 *         ok(parse_buf(buf))
 *     }
 */
fn chain<T, U: copy, V: copy>(res: Result<T, V>, op: fn(T) -> Result<U, V>)
    -> Result<U, V> {
    match res {
      Ok(t) => op(t),
      Err(e) => Err(e)
    }
}

/**
 * Call a function based on a previous result
 *
 * If `res` is `err` then the value is extracted and passed to `op`
 * whereupon `op`s result is returned. if `res` is `ok` then it is
 * immediately returned.  This function can be used to pass through a
 * successful result while handling an error.
 */
fn chain_err<T: copy, U: copy, V: copy>(
    res: Result<T, V>,
    op: fn(V) -> Result<T, U>)
    -> Result<T, U> {
    match res {
      Ok(t) => Ok(t),
      Err(v) => op(v)
    }
}

/**
 * Call a function based on a previous result
 *
 * If `res` is `ok` then the value is extracted and passed to `op` whereupon
 * `op`s result is returned. if `res` is `err` then it is immediately
 * returned. This function can be used to compose the results of two
 * functions.
 *
 * Example:
 *
 *     iter(read_file(file)) { |buf|
 *         print_buf(buf)
 *     }
 */
fn iter<T, E>(res: Result<T, E>, f: fn(T)) {
    match res {
      Ok(t) => f(t),
      Err(_) => ()
    }
}

/**
 * Call a function based on a previous result
 *
 * If `res` is `err` then the value is extracted and passed to `op` whereupon
 * `op`s result is returned. if `res` is `ok` then it is immediately returned.
 * This function can be used to pass through a successful result while
 * handling an error.
 */
fn iter_err<T, E>(res: Result<T, E>, f: fn(E)) {
    match res {
      Ok(_) => (),
      Err(e) => f(e)
    }
}

/**
 * Call a function based on a previous result
 *
 * If `res` is `ok` then the value is extracted and passed to `op` whereupon
 * `op`s result is wrapped in `ok` and returned. if `res` is `err` then it is
 * immediately returned.  This function can be used to compose the results of
 * two functions.
 *
 * Example:
 *
 *     let res = map(read_file(file)) { |buf|
 *         parse_buf(buf)
 *     }
 */
fn map<T, E: copy, U: copy>(res: Result<T, E>, op: fn(T) -> U)
  -> Result<U, E> {
    match res {
      Ok(t) => Ok(op(t)),
      Err(e) => Err(e)
    }
}

/**
 * Call a function based on a previous result
 *
 * If `res` is `err` then the value is extracted and passed to `op` whereupon
 * `op`s result is wrapped in an `err` and returned. if `res` is `ok` then it
 * is immediately returned.  This function can be used to pass through a
 * successful result while handling an error.
 */
fn map_err<T: copy, E, F: copy>(res: Result<T, E>, op: fn(E) -> F)
  -> Result<T, F> {
    match res {
      Ok(t) => Ok(t),
      Err(e) => Err(op(e))
    }
}

impl<T, E> Result<T, E> {
    fn is_ok() -> bool { is_ok(self) }

    fn is_err() -> bool { is_err(self) }

    fn iter(f: fn(T)) {
        match self {
          Ok(t) => f(t),
          Err(_) => ()
        }
    }

    fn iter_err(f: fn(E)) {
        match self {
          Ok(_) => (),
          Err(e) => f(e)
        }
    }
}

impl<T: copy, E> Result<T, E> {
    fn get() -> T { get(self) }

    fn map_err<F:copy>(op: fn(E) -> F) -> Result<T,F> {
        match self {
          Ok(t) => Ok(t),
          Err(e) => Err(op(e))
        }
    }
}

impl<T, E: copy> Result<T, E> {
    fn get_err() -> E { get_err(self) }

    fn map<U:copy>(op: fn(T) -> U) -> Result<U,E> {
        match self {
          Ok(t) => Ok(op(t)),
          Err(e) => Err(e)
        }
    }
}

impl<T: copy, E: copy> Result<T, E> {
    fn chain<U:copy>(op: fn(T) -> Result<U,E>) -> Result<U,E> {
        chain(self, op)
    }

    fn chain_err<F:copy>(op: fn(E) -> Result<T,F>) -> Result<T,F> {
        chain_err(self, op)
    }
}

/**
 * Maps each element in the vector `ts` using the operation `op`.  Should an
 * error occur, no further mappings are performed and the error is returned.
 * Should no error occur, a vector containing the result of each map is
 * returned.
 *
 * Here is an example which increments every integer in a vector,
 * checking for overflow:
 *
 *     fn inc_conditionally(x: uint) -> result<uint,str> {
 *         if x == uint::max_value { return err("overflow"); }
 *         else { return ok(x+1u); }
 *     }
 *     map(~[1u, 2u, 3u], inc_conditionally).chain {|incd|
 *         assert incd == ~[2u, 3u, 4u];
 *     }
 */
fn map_vec<T,U:copy,V:copy>(
    ts: &[T], op: fn(T) -> Result<V,U>) -> Result<~[V],U> {

    let mut vs: ~[V] = ~[];
    vec::reserve(vs, vec::len(ts));
    for vec::each(ts) |t| {
        match op(t) {
          Ok(v) => vec::push(vs, v),
          Err(u) => return Err(u)
        }
    }
    return Ok(vs);
}

fn map_opt<T,U:copy,V:copy>(
    o_t: Option<T>, op: fn(T) -> Result<V,U>) -> Result<Option<V>,U> {

    match o_t {
      None => Ok(None),
      Some(t) => match op(t) {
        Ok(v) => Ok(Some(v)),
        Err(e) => Err(e)
      }
    }
}

/**
 * Same as map, but it operates over two parallel vectors.
 *
 * A precondition is used here to ensure that the vectors are the same
 * length.  While we do not often use preconditions in the standard
 * library, a precondition is used here because result::t is generally
 * used in 'careful' code contexts where it is both appropriate and easy
 * to accommodate an error like the vectors being of different lengths.
 */
fn map_vec2<S,T,U:copy,V:copy>(ss: &[S], ts: &[T],
                               op: fn(S,T) -> Result<V,U>) -> Result<~[V],U> {

    assert vec::same_length(ss, ts);
    let n = vec::len(ts);
    let mut vs = ~[];
    vec::reserve(vs, n);
    let mut i = 0u;
    while i < n {
        match op(ss[i],ts[i]) {
          Ok(v) => vec::push(vs, v),
          Err(u) => return Err(u)
        }
        i += 1u;
    }
    return Ok(vs);
}

/**
 * Applies op to the pairwise elements from `ss` and `ts`, aborting on
 * error.  This could be implemented using `map2()` but it is more efficient
 * on its own as no result vector is built.
 */
fn iter_vec2<S,T,U:copy>(ss: &[S], ts: &[T],
                         op: fn(S,T) -> Result<(),U>) -> Result<(),U> {

    assert vec::same_length(ss, ts);
    let n = vec::len(ts);
    let mut i = 0u;
    while i < n {
        match op(ss[i],ts[i]) {
          Ok(()) => (),
          Err(u) => return Err(u)
        }
        i += 1u;
    }
    return Ok(());
}

/// Unwraps a result, assuming it is an `ok(T)`
fn unwrap<T, U>(-res: Result<T, U>) -> T {
    unsafe {
        let addr = match res {
          Ok(x) => ptr::addr_of(x),
          Err(_) => fail ~"error result"
        };
        let liberated_value = unsafe::reinterpret_cast(*addr);
        unsafe::forget(res);
        return liberated_value;
    }
}

#[cfg(test)]
mod tests {
    fn op1() -> result::result<int, ~str> { result::ok(666) }

    fn op2(&&i: int) -> result::result<uint, ~str> {
        result::ok(i as uint + 1u)
    }

    fn op3() -> result::result<int, ~str> { result::err(~"sadface") }

    #[test]
    fn chain_success() {
        assert get(chain(op1(), op2)) == 667u;
    }

    #[test]
    fn chain_failure() {
        assert get_err(chain(op3(), op2)) == ~"sadface";
    }

    #[test]
    fn test_impl_iter() {
        let mut valid = false;
        ok::<~str, ~str>(~"a").iter(|_x| valid = true);
        assert valid;

        err::<~str, ~str>(~"b").iter(|_x| valid = false);
        assert valid;
    }

    #[test]
    fn test_impl_iter_err() {
        let mut valid = true;
        ok::<~str, ~str>(~"a").iter_err(|_x| valid = false);
        assert valid;

        valid = false;
        err::<~str, ~str>(~"b").iter_err(|_x| valid = true);
        assert valid;
    }

    #[test]
    fn test_impl_map() {
        assert ok::<~str, ~str>(~"a").map(|_x| ~"b") == ok(~"b");
        assert err::<~str, ~str>(~"a").map(|_x| ~"b") == err(~"a");
    }

    #[test]
    fn test_impl_map_err() {
        assert ok::<~str, ~str>(~"a").map_err(|_x| ~"b") == ok(~"a");
        assert err::<~str, ~str>(~"a").map_err(|_x| ~"b") == err(~"b");
    }
}
