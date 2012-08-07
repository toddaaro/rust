/**
 * An atomically reference counted wrapper that can be used to
 * share immutable data between tasks.
 */

import sys::methods;

export Arc, arc, get, clone;

export Exclusive, exclusive, Methods;

#[abi = "cdecl"]
extern mod rustrt {
    #[rust_stack]
    fn rust_atomic_increment(p: &mut libc::intptr_t)
        -> libc::intptr_t;

    #[rust_stack]
    fn rust_atomic_decrement(p: &mut libc::intptr_t)
        -> libc::intptr_t;
}

type ArcData<T> = {
    mut count: libc::intptr_t,
    data: T
};

class ArcDestruct<T> {
  let data: *libc::c_void;
  new(data: *libc::c_void) { self.data = data; }
  drop unsafe {
     let data: ~ArcData<T> = unsafe::reinterpret_cast(self.data);
     let new_count = rustrt::rust_atomic_decrement(&mut data.count);
     assert new_count >= 0;
     if new_count == 0 {
         // drop glue takes over.
     } else {
       unsafe::forget(data);
     }
  }
}

type Arc<T: const send> = ArcDestruct<T>;

/// Create an atomically reference counted wrapper.
fn arc<T: const send>(-data: T) -> Arc<T> {
    let data = ~{mut count: 1, data: data};
    unsafe {
        let ptr = unsafe::transmute(data);
        ArcDestruct(ptr)
    }
}

/**
 * Access the underlying data in an atomically reference counted
 * wrapper.
 */
fn get<T: const send>(rc: &Arc<T>) -> &T {
    unsafe {
        let ptr: ~ArcData<T> = unsafe::reinterpret_cast((*rc).data);
        // Cast us back into the correct region
        let r = unsafe::reinterpret_cast(&ptr.data);
        unsafe::forget(ptr);
        return r;
    }
}

/**
 * Duplicate an atomically reference counted wrapper.
 *
 * The resulting two `arc` objects will point to the same underlying data
 * object. However, one of the `arc` objects can be sent to another task,
 * allowing them to share the underlying data.
 */
fn clone<T: const send>(rc: &Arc<T>) -> Arc<T> {
    unsafe {
        let ptr: ~ArcData<T> = unsafe::reinterpret_cast((*rc).data);
        let new_count = rustrt::rust_atomic_increment(&mut ptr.count);
        assert new_count >= 2;
        unsafe::forget(ptr);
    }
    ArcDestruct((*rc).data)
}

// An arc over mutable data that is protected by a lock.
type ExData<T: send> = {lock: sys::little_lock, mut data: T};
type Exclusive<T: send> = ArcDestruct<ExData<T>>;

fn exclusive<T:send >(-data: T) -> Exclusive<T> {
    let data = ~{mut count: 1, data: {lock: sys::little_lock(),
                                      data: data}};
    unsafe {
        let ptr = unsafe::reinterpret_cast(data);
        unsafe::forget(data);
        ArcDestruct(ptr)
    }
}

impl Methods<T: send> for Exclusive<T> {
    /// Duplicate an exclusive ARC. See arc::clone.
    fn clone() -> Exclusive<T> {
        unsafe {
            // this makes me nervous...
            let ptr: ~ArcData<ExData<T>> =
                  unsafe::reinterpret_cast(self.data);
            let new_count = rustrt::rust_atomic_increment(&mut ptr.count);
            assert new_count > 1;
            unsafe::forget(ptr);
        }
        ArcDestruct(self.data)
    }

    /**
     * Access the underlying mutable data with mutual exclusion from other
     * tasks. The argument closure will be run with the mutex locked; all
     * other tasks wishing to access the data will block until the closure
     * finishes running.
     *
     * Currently, scheduling operations (i.e., yielding, receiving on a pipe,
     * accessing the provided condition variable) are prohibited while inside
     * the exclusive. Supporting that is a work in progress.
     *
     * The reason this function is 'unsafe' is because it is possible to
     * construct a circular reference among multiple ARCs by mutating the
     * underlying data. This creates potential for deadlock, but worse, this
     * will guarantee a memory leak of all involved ARCs. Using exclusive
     * ARCs inside of other ARCs is safe in absence of circular references.
     */
    unsafe fn with<U>(f: fn(x: &mut T) -> U) -> U {
        let ptr: ~ArcData<ExData<T>> =
            unsafe::reinterpret_cast(self.data);
        assert ptr.count > 0;
        let r = {
            let rec: &ExData<T> = &(*ptr).data;
            do rec.lock.lock { f(&mut rec.data) }
        };
        unsafe::forget(ptr);
        r
    }
}

#[cfg(test)]
mod tests {
    import comm::*;
    import future::extensions;

    #[test]
    fn manually_share_arc() {
        let v = ~[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let arc_v = arc::arc(v);

        let p = port();
        let c = chan(p);

        do task::spawn() {
            let p = port();
            c.send(chan(p));

            let arc_v = p.recv();

            let v = *arc::get::<~[int]>(&arc_v);
            assert v[3] == 4;
        };

        let c = p.recv();
        c.send(arc::clone(&arc_v));

        assert (*arc::get(&arc_v))[2] == 3;

        log(info, arc_v);
    }

    #[test]
    #[ignore] // this can probably infinite loop too.
    fn exclusive_arc() {
        let mut futures = ~[];

        let num_tasks = 10u;
        let count = 1000u;

        let total = exclusive(~mut 0u);

        for uint::range(0u, num_tasks) |_i| {
            let total = total.clone();
            futures += ~[future::spawn(|| {
                for uint::range(0u, count) |_i| {
                    do total.with |count| {
                        **count += 1u;
                    }
                }
            })];
        };

        for futures.each |f| { f.get() }

        do total.with |total| {
            assert **total == num_tasks * count
        };
    }
}
