/*! DWARF-based unwinding using LLVM's exception infrastructure */

use lib::llvm::ValueRef;
use base::{block, sub_block, block_scope, clean, lpad_block};
use build::{Invoke, Call};

pub struct DwarfUnwindStrategy {
    bogus: int
}

impl DwarfUnwindStrategy {
    static fn new() -> DwarfUnwindStrategy {
        DwarfUnwindStrategy {
            bogus: 0
        }
    }
}

impl DwarfUnwindStrategy: UnwindStrategy {
    fn invoke(bcx: block, llfn: ValueRef, llargs: ~[ValueRef]) -> block {
        invoke(bcx, llfn, llargs)
    }
}

fn invoke(bcx: block, llfn: ValueRef, llargs: ~[ValueRef]) -> block {
    let _icx = bcx.insn_ctxt("invoke_");
    if bcx.unreachable { return bcx; }
    if need_invoke(bcx) {
        log(debug, ~"invoking");
        let normal_bcx = sub_block(bcx, ~"normal return");
        Invoke(bcx, llfn, llargs, normal_bcx.llbb, get_landing_pad(bcx));
        return normal_bcx;
    } else {
        log(debug, ~"calling");
        Call(bcx, llfn, llargs);
        return bcx;
    }
}

fn need_invoke(bcx: block) -> bool {
    if (bcx.ccx().sess.opts.debugging_opts & session::no_landing_pads != 0) {
        return false;
    }

    // Avoid using invoke if we are already inside a landing pad.
    if bcx.is_lpad {
        return false;
    }

    if have_cached_lpad(bcx) {
        return true;
    }

    // Walk the scopes to look for cleanups
    let mut cur = bcx;
    loop {
        match cur.kind {
          block_scope(inf) => {
            for vec::each(inf.cleanups) |cleanup| {
                match *cleanup {
                  clean(_, cleanup_type) | clean_temp(_, _, cleanup_type) => {
                    if cleanup_type == normal_exit_and_unwind {
                        return true;
                    }
                  }
                }
            }
          }
          _ => ()
        }
        cur = match cur.parent {
          Some(next) => next,
          None => return false
        }
    }
}

fn have_cached_lpad(bcx: block) -> bool {
    let mut res = false;
    do in_lpad_scope_cx(bcx) |inf| {
        match inf.landing_pad {
          Some(_) => res = true,
          None => res = false
        }
    }
    return res;
}

fn in_lpad_scope_cx(bcx: block, f: fn(scope_info)) {
    let mut bcx = bcx;
    loop {
        match bcx.kind {
          block_scope(inf) => {
            if inf.cleanups.len() > 0u || bcx.parent.is_none() {
                f(inf); return;
            }
          }
          _ => ()
        }
        bcx = block_parent(bcx);
    }
}

fn get_landing_pad(bcx: block) -> BasicBlockRef {
    let _icx = bcx.insn_ctxt("get_landing_pad");

    let mut cached = None, pad_bcx = bcx; // Guaranteed to be set below
    do in_lpad_scope_cx(bcx) |inf| {
        // If there is a valid landing pad still around, use it
        match copy inf.landing_pad {
          Some(target) => cached = Some(target),
          None => {
            pad_bcx = lpad_block(bcx, ~"unwind");
            inf.landing_pad = Some(pad_bcx.llbb);
          }
        }
    }
    // Can't return from block above
    match cached { Some(b) => return b, None => () }
    // The landing pad return type (the type being propagated). Not sure what
    // this represents but it's determined by the personality function and
    // this is what the EH proposal example uses.
    let llretty = T_struct(~[T_ptr(T_i8()), T_i32()]);
    // The exception handling personality function. This is the C++
    // personality function __gxx_personality_v0, wrapped in our naming
    // convention.
    let personality = bcx.ccx().upcalls.rust_personality;
    // The only landing pad clause will be 'cleanup'
    let llretval = LandingPad(pad_bcx, llretty, personality, 1u);
    // The landing pad block is a cleanup
    SetCleanup(pad_bcx, llretval);

    // Because we may have unwound across a stack boundary, we must call into
    // the runtime to figure out which stack segment we are on and place the
    // stack limit back into the TLS.
    Call(pad_bcx, bcx.ccx().upcalls.reset_stack_limit, ~[]);

    // We store the retval in a function-central alloca, so that calls to
    // Resume can find it.
    match copy bcx.fcx.personality {
      Some(addr) => Store(pad_bcx, llretval, addr),
      None => {
        let addr = alloca(pad_bcx, val_ty(llretval));
        bcx.fcx.personality = Some(addr);
        Store(pad_bcx, llretval, addr);
      }
    }

    // Unwind all parent scopes, and finish with a Resume instr
    cleanup_and_leave(pad_bcx, None, None);
    return pad_bcx.llbb;
}

