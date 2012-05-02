#[doc(hidden)];

export chan_from_global_ptr, weaken_task;

import comm::*;
import compare_and_swap = rustrt::rust_compare_and_swap_ptr;

type rust_port_id = uint;

native mod rustrt {
    fn rust_compare_and_swap_ptr(address: *libc::uintptr_t,
                                 oldval: libc::uintptr_t,
                                 newval: libc::uintptr_t) -> bool;
    fn rust_task_weaken(ch: rust_port_id);
    fn rust_task_unweaken(ch: rust_port_id);
}

type global_ptr = *libc::uintptr_t;

#[doc = "
Atomically gets a channel from a pointer to a pointer-sized memory location
or, if no channel exists creates and installs a new channel and sets up a new
task to receive from it.
"]
unsafe fn chan_from_global_ptr<T: send>(
    global: global_ptr,
    builder: fn() -> task::builder,
    f: fn~(port<T>)
) -> comm::chan<T> {

    enum msg {
        proceed,
        abort
    }

    log(debug,"ENTERING chan_from_global_ptr, before is_prob_zero check");
    let is_probably_zero = *global == 0u;
    log(debug,"after is_prob_zero check");
    if is_probably_zero {
        log(debug,"is probably zero...");
        // There's no global channel. We must make it

        listen {|parent_setup_ch|
            let child_setup_ch =
                task::run_listener::<msg>(builder()) {|child_setup_po|

                let po = port();

                parent_setup_ch.send(po.chan());

                // Wait to hear if we are the official instance of
                // this global task
                alt child_setup_po.recv() {
                  proceed { f(po); }
                  abort { }
                }
            };

            log(debug,"before setup recv..");
            // This is the proposed global channel
            let ch = parent_setup_ch.recv();
            // 0 is our sentinal value. It is not a valid channel
            assert unsafe::reinterpret_cast(ch) != 0u;

            // Install the channel
            log(debug,"BEFORE COMPARE AND SWAP");
            let swapped = compare_and_swap(
                global, 0u, unsafe::reinterpret_cast(ch));
            log(debug,#fmt("AFTER .. swapped? %?", swapped));

            if swapped {
                // Success!
                child_setup_ch.send(proceed);
                ch
            } else {
                // Somebody else got in before we did
                child_setup_ch.send(abort);
                unsafe::reinterpret_cast(*global)
            }
        }
    } else {
        log(debug, "global != 0");
        unsafe::reinterpret_cast(*global)
    }
}

#[test]
fn test_from_global_chan1() unsafe {

    // The global channel
    let globchan = 0u;
    let globchanp = ptr::addr_of(globchan);

    // Create the global channel, attached to a new task
    let ch = chan_from_global_ptr::<chan<bool>>(globchanp, task::builder) {|po|
        po.recv().send(true);
        po.recv().send(true);
    };

    // Talk to it
    listen {|ch1|
        ch.send(ch1);
        assert ch1.recv() == true;
    }

    // This one just reuses the previous channel
    let ch = chan_from_global_ptr::<chan<bool>>(globchanp, task::builder) {|po|
        po.recv().send(false);
    };

    // Talk to the original global task
    listen {|ch2|
        ch.send(ch2);
        assert ch2.recv() == true;
    }
}

#[test]
fn test_from_global_chan2() unsafe {

    iter::repeat(100u) {||
        // The global channel
        let globchan = 0u;
        let globchanp = ptr::addr_of(globchan);

        listen {|resultch|

            // Spawn a bunch of tasks that all want to compete to
            // create the global channel
            uint::range(0u, 10u) {|i|
                task::spawn() {||
                    let ch = chan_from_global_ptr::<chan<uint>>(
                        globchanp, task::builder) {|po|

                        uint::range(0u, 10u) {|_j|
                            po.recv().send(copy(i));
                        }
                    };
                    listen {|winnerch|
                        ch.send(winnerch);
                        // We are the winner if our version of the
                        // task was installed
                        let winner = winnerch.recv();
                        resultch.send(winner == i);
                    }
                }
            }
            // There should be only one winner
            let mut winners = 0u;
            uint::range(0u, 10u) {|_i|
                let res = resultch.recv();
                if res { winners += 1u };
            }
            assert winners == 1u;
        }
    }
}

#[doc = "
Convert the current task to a 'weak' task temporarily

As a weak task it will not be counted towards the runtime's set
of live tasks. When there are no more outstanding live (non-weak) tasks
the runtime will send an exit message on the provided channel.

This function is super-unsafe. Do not use.

# Safety notes

* Weak tasks must either die on their own or exit upon receipt of
  the exit message. Failure to do so will cause the runtime to never
  exit
* Tasks must not call `weaken_task` multiple times. This will
  break the kernel's accounting of live tasks.
* Weak tasks must not be supervised. A supervised task keeps
  a reference to its parent, so the parent will not die.
"]
unsafe fn weaken_task(f: fn(port<()>)) unsafe {
    let po = port();
    let ch = po.chan();
    rustrt::rust_task_weaken(unsafe::reinterpret_cast(ch));
    let _unweaken = unweaken(ch);
    f(po);

    resource unweaken(ch: chan<()>) unsafe {
        rustrt::rust_task_unweaken(unsafe::reinterpret_cast(ch));
    }
}

#[test]
fn test_weaken_task_then_unweaken() unsafe {
    task::try {||
        weaken_task {|_po|
        }
    };
}

#[test]
fn test_weaken_task_wait() unsafe {
    let builder = task::builder();
    task::unsupervise(builder);
    task::run(builder) {||
        weaken_task {|po|
            po.recv();
        }
    }
}

#[test]
fn test_weaken_task_stress() unsafe {
    // Create a bunch of weak tasks
    iter::repeat(100u) {||
        task::spawn {||
            weaken_task {|_po|
            }
        }
        let builder = task::builder();
        task::unsupervise(builder);
        task::run(builder) {||
            weaken_task {|po|
                // Wait for it to tell us to die
                po.recv();
            }
        }
    }
}

#[test]
#[ignore(cfg(target_os = "win32"))]
fn test_weaken_task_fail() unsafe {
    let res = task::try {||
        weaken_task {|_po|
            fail;
        }
    };
    assert result::is_failure(res);
}