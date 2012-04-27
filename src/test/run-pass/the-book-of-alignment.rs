// xfail-pretty

#[doc = "

Hey Y'all,

Welcome to The Book of Alignment.

The Ancient and Profound Book of Alignment is not a specification,
nor is it validation of the implementation. It is simply documentation
of the way things are.

"];

fn say_it_like_it_is() {

    assert what_the::<example_type> ([

        ( x86,    all_os, size, pref_align, min_align ),
        ( x86_64, all_os, size, pref_align, min_align ),
        ( arm,    all_os, size, pref_align, min_align )

    ]);

    assert what_the::<int>   ([ (x86     , all_os, 4, 4, 4),
                                (x86_64  , all_os, 8, 8, 8),
                                (arm     , all_os, X, X, X) ]);
    assert what_the::<uint>  ([ (x86     , all_os, 4, 4, 4),
                                (x86_64  , all_os, 8, 8, 8),
                                (arm     , all_os, X, X, X) ]);

    assert what_the::<i8>    ([ (all_arch, all_os, 1, 1, 1) ]);
    assert what_the::<i16>   ([ (all_arch, all_os, 2, 2, 2) ]);
    assert what_the::<i32>   ([ (all_arch, all_os, 4, 4, 4) ]);
    assert what_the::<i64>   ([ (x86     , all_os, 8, 8, 4),
                                (x86_64  , all_os, 8, 8, 8),
                                (arm     , all_os, X, X, X) ]);

    assert what_the::<u8>    ([ (all_arch, all_os, 1, 1, 1) ]);
    assert what_the::<u16>   ([ (all_arch, all_os, 2, 2, 2) ]);
    assert what_the::<u32>   ([ (all_arch, all_os, 4, 4, 4) ]);
    assert what_the::<u64>   ([ (x86     , all_os, 8, 8, 4),
                                (x86_64  , all_os, 8, 8, 8),
                                (arm     , all_os, X, X, X) ]);


    assert what_the::<float> ([ (x86     , all_os, 8, 8, 4),
                                (x86_64  , all_os, 8, 8, 8),
                                (arm     , all_os, X, X, X) ]);
    assert what_the::<f32>   ([ (all_arch, all_os, 4, 4, 4) ]);
    assert what_the::<f64>   ([ (x86     , all_os, 8, 8, 4),
                                (x86_64  , all_os, 8, 8, 8),
                                (arm     , all_os, X, X, X) ]);

    assert what_the::<()>    ([ (all_arch, all_os, 1, 1, 1) ]);
    assert what_the::<bool>  ([ (all_arch, all_os, 1, 1, 1) ]);
    assert what_the::<char>  ([ (all_arch, all_os, 4, 4, 4) ]);

    assert pointery::<str>();
    assert pointery::<@()>();
    assert pointery::<~()>();
    assert pointery::<[()]>();
    assert pointery::<*()>();
    //assert pointery::<&()>();

    assert what_the::<{a: u8}> ([
        (all_arch, all_os, 1, 8, 1) ]);
    assert what_the::<{a: u8, b: u8}> ([
        (all_arch, all_os, 2, 8, 1) ]);
    assert what_the::<{a: u8, b: u8, c: u8}> ([
        (all_arch, all_os, 3, 8, 1) ]);
    assert what_the::<{a: u8, b: u8, c: u8, d: u8}> ([
        (all_arch, all_os, 4, 8, 1) ]);

    assert what_the::<{a: u16}> ([
        (all_arch, all_os, 2, 8, 1) ]);
    assert what_the::<{a: u16, b: u8}> ([
        (all_arch, all_os, 4, 8, 1) ]);
    assert what_the::<{a: u16, b: u8, c: u16}> ([
        (all_arch, all_os, 6, 8, 2) ]);

    assert what_the::<{a: u8, b: u64}> ([
        (x86,    all_os, 12, 8, 4),
        (x86_64, all_os, 16, 8, 8),
        (arm,    all_os, X, X, X)
    ]);

    assert what_the::<{a: u8, b: u32, c: (f64, u8, f32)}> ([
        (x86,    all_os, 24, 8, 4),
        (x86_64, all_os, 24, 8, 8),
        (arm,    all_os, X, X, X)
    ]);

    assert what_the::<{a: uint, b: (u64, u32, u8)}> ([
        (x86,    all_os, 20, 8, 4),
        (x86_64, all_os, 24, 8, 8),
        (arm,    all_os, X, X, X)
    ]);

    // FIXME: Some of these min aligns of 1 seem wrong

    enum yup { thats_right(u64, u32, u8) }
    assert what_the::<yup> ([
        (x86,    all_os, 16, 8, 4),
        (x86_64, all_os, 16, 8, 8),
        (arm,    all_os, X, X, X)
    ]);

    enum mup { breh, feh(u16, u32, u8), meh(float) }
    assert what_the::<mup> ([
        (x86,    all_os, 16, 8, 4),
        (x86_64, all_os, 24, 8, 8),
        (arm,    all_os, X, X, X)
    ]);

    enum dup { thats_righty }
    assert what_the::<dup> ([
        (x86,    all_os, 4, 8, 1), // FIXME: this 1 is clearly wrong
        (x86_64, all_os, 8, 8, 8),
        (arm,    all_os, X, X, X)
    ]);

    enum tup { }
    assert what_the::<tup> ([
        (x86,    all_os, 4, 8, 1), // FIXME: this 1 is clearly wrong
        (x86_64, all_os, 8, 8, 8),
        (arm,    all_os, X, X, X)
    ]);

    enum blup = int;
    assert what_the::<blup> ([
        (x86,    all_os, 4, 8, 1), // FIXME: this 1 is clearly wrong
        (x86_64, all_os, 8, 8, 8),
        (arm,    all_os, X, X, X)
    ]);

}


#[doc = "

The rest is just details

"]
mod details_beneath_here { }


type spec = (arch, os, size, pref_align, min_align);
enum arch { all_arch, x86, x86_64, arm, print_me_nicely(shape_code) }
enum os { all_os, linux, macos, win32, freebsd }
const X: int = 9;

fn what_the<T>(specs: [spec]) -> bool {

    we_should_have_full_arch_coverage_with_these(specs)

        &&

    specs.map {|spec|

        alt spec {
          (x86,    _, _, _, _) { believe_in_x86::<T>(spec) }
          (x86_64, _, _, _, _) { believe_in_x86_64::<T>(spec) }
          (arm,    _, _, _, _) { believe_in_arm::<T>(spec) }
          _                    { believe_in_arch::<T>(spec) }
        }

    } .all { | step_on_no_pets
             | step_on_no_pets
             | step_on_no_pets
             | step_on_no_pets }
}

fn pointery<T>() -> bool {
    what_the::<T> ([

        (x86,    all_os, 4, 4, 4),
        (x86_64, all_os, 8, 8, 8),
        (arm,    all_os, X, X, X)

    ])
}


fn we_should_have_full_arch_coverage_with_these(specs: [spec]) -> bool {
    if 12u == specs.flat_map {|spec|
        alt spec {
          (all_arch, os, s, p, m) {
            [ (x86, os, s, p, m),
              (x86_64, os, s, p, m),
              (arm, os, s, p, m) ]
          }
          _ {[ spec ]}
        }
    }.flat_map {|spec|
        alt spec {
          (arch, all_os, s, p, m) {
            [ (arch, linux, s, p, m),
              (arch, macos, s, p, m),
              (arch, win32, s, p, m),
              (arch, freebsd, s, p, m) ]
          }
          _ {[ spec ]}
        }
    }.len() {
        true
    } else {
        #error("do not have full coverage with: %?", specs);
        false
    }
}

fn believe_in_arch<T>(spec: spec) -> bool {
    alt spec {
      (_, linux,   _, _, _) { believe_in_linux::<T>(spec) }
      (_, macos,   _, _, _) { believe_in_macos::<T>(spec) }
      (_, win32,   _, _, _) { believe_in_win32::<T>(spec) }
      (_, freebsd, _, _, _) { believe_in_freebsd::<T>(spec) }
      _                     { believe_in_rust::<T>(spec) }
    }
}

fn believe_in_rust<T>(spec: spec) -> bool {

    let (_, _, s, p, m) = spec;

    #debug("%? %? %?", size_is::<T>(s),
           pref_align_is::<T>(p), min_align_is::<T>(m));
    #debug("%? %? %?",
           sys::size_of::<T>(),
           sys::pref_align_of::<T>(),
           sys::min_align_of::<T>());

    if size_is::<T>(s)
        && pref_align_is::<T>(p)
        && min_align_is::<T>(m) {

        #debug("%? looks good", spec); true

    } else {

        #error("size: %u, pref_align: %u, min_align: %u",
               sys::size_of::<T>(),
               sys::pref_align_of::<T>(),
               sys::min_align_of::<T>());

        #error("%? looks bad", spec); false

    }
}


fn disbelieve<T>(_spec: spec) -> bool { true }


#[cfg(target_arch = "x86")]
fn believe_in_x86<T>(spec: spec) -> bool { believe_in_arch::<T>(spec) }

#[cfg(target_arch = "x86_64")]
fn believe_in_x86<T>(spec: spec) -> bool { disbelieve::<T>(spec) }

#[cfg(target_arch = "arm")]
fn believe_in_x86<T>(spec: spec) -> bool { disbelieve::<T>(spec) }


#[cfg(target_arch = "x86")]
fn believe_in_x86_64<T>(spec: spec) -> bool { disbelieve::<T>(spec) }

#[cfg(target_arch = "x86_64")]
fn believe_in_x86_64<T>(spec: spec) -> bool { believe_in_arch::<T>(spec) }

#[cfg(target_arch = "arm")]
fn believe_in_x86_64<T>(spec: spec) -> bool { disbelieve::<T>(spec) }


#[cfg(target_arch = "x86")]
fn believe_in_arm<T>(spec: spec) -> bool { disbelieve::<T>(spec) }

#[cfg(target_arch = "x86_64")]
fn believe_in_arm<T>(spec: spec) -> bool { disbelieve::<T>(spec) }

#[cfg(target_arch = "arm")]
fn believe_in_arm<T>(spec: spec) -> bool { believe_in_arch::<T>(spec) }


#[cfg(target_os = "linux")]
fn believe_in_linux<T>(spec: spec) -> bool { believe_in_rust::<T>(spec) }

#[cfg(target_os = "macos")]
fn believe_in_linux<T>(spec: spec) -> bool { disbelieve::<T>(spec) }

#[cfg(target_os = "win32")]
fn believe_in_linux<T>(spec: spec) -> bool { disbelieve::<T>(spec) }

#[cfg(target_os = "freebsd")]
fn believe_in_linux<T>(spec: spec) -> bool { disbelieve::<T>(spec) }


#[cfg(target_os = "linux")]
fn believe_in_macos<T>(spec: spec) -> bool { disbelieve::<T>(spec) }

#[cfg(target_os = "macos")]
fn believe_in_macos<T>(spec: spec) -> bool { believe_in_rust::<T>(spec) }

#[cfg(target_os = "win32")]
fn believe_in_macos<T>(spec: spec) -> bool { disbelieve::<T>(spec) }

#[cfg(target_os = "freebsd")]
fn believe_in_macos<T>(spec: spec) -> bool { disbelieve::<T>(spec) }


#[cfg(target_os = "linux")]
fn believe_in_win32<T>(spec: spec) -> bool { disbelieve::<T>(spec) }

#[cfg(target_os = "macos")]
fn believe_in_win32<T>(spec: spec) -> bool { disbelieve::<T>(spec) }

#[cfg(target_os = "win32")]
fn believe_in_win32<T>(spec: spec) -> bool { believe_in_rust::<T>(spec) }

#[cfg(target_os = "freebsd")]
fn believe_in_win32<T>(spec: spec) -> bool { disbelieve::<T>(spec) }


#[cfg(target_os = "linux")]
fn believe_in_freebsd<T>(spec: spec) -> bool { disbelieve::<T>(spec) }

#[cfg(target_os = "macos")]
fn believe_in_freebsd<T>(spec: spec) -> bool { disbelieve::<T>(spec) }

#[cfg(target_os = "win32")]
fn believe_in_freebsd<T>(spec: spec) -> bool { disbelieve::<T>(spec) }

#[cfg(target_os = "freebsd")]
fn believe_in_freebsd<T>(spec: spec) -> bool { believe_in_rust::<T>(spec) }


// Types used for illustration of the spec tuple
type size = int;
type pref_align = int;
type min_align = int;
type shape_code = int;


// Stuff used in the example
type example_type = bool;
const size: size = 1;
const pref_align: pref_align = 1;
const min_align: min_align = 1;


fn main() {
    #info("testing arch %s", os::arch());
    say_it_like_it_is();
}


fn size_is<T>(a: int) -> bool {
    sys::size_of::<T>() == a as uint
}
fn pref_align_is<T>(a: int) -> bool {
    sys::pref_align_of::<T>() == a as uint
}
fn min_align_is<T>(a: int) ->  bool {
    sys::min_align_of::<T>() == a as uint
}
