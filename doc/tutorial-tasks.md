% Rust Tasks and Communication Tutorial

# Introduction

Rust provides safe concurrency primarily through a combination
of lightweight, memory-isolated tasks and message passing.
This tutorial will describe the concurrency model in Rust and how it
relates to the Rust type system, and introduce
the fundamental library abstractions for constructing concurrent programs.

Rust tasks are not the same as traditional threads: rather,
they are considered _green threads_, lightweight units of execution that the Rust
runtime schedules cooperatively onto a small number of operating system threads.
On a multi-core system Rust tasks will be scheduled in parallel by default.
Because tasks are significantly
cheaper to create than traditional threads, Rust can create hundreds of
thousands of concurrent tasks on a typical 32-bit system.
In general, all Rust code executes inside a task, including the `main` function.

In order to make efficient use of memory Rust tasks have dynamically sized stacks.
A task begins its life with a small
amount of stack space (currently in the low thousands of bytes, depending on
platform), and acquires more stack as needed.
Unlike in languages such as C, a Rust task cannot accidentally write to
memory beyond the end of the stack, causing crashes or worse.

Tasks provide failure isolation and recovery. When a fatal error occurs in Rust
code as a result of an explicit call to `fail!()`, an assertion failure, or
another invalid operation, the runtime system destroys the entire
task. Unlike in languages such as Java and C++, there is no way to `catch` an
exception. Instead, tasks may monitor each other for failure.

Tasks use Rust's type system to provide strong memory safety guarantees. In
particular, the type system guarantees that tasks cannot share mutable state
with each other. Tasks communicate with each other by transferring _owned_
data through the global _exchange heap_.

## A note about the libraries

While Rust's type system provides the building blocks needed for safe
and efficient tasks, all of the task functionality itself is implemented
in the core and standard libraries, which are still under development
and do not always present a consistent or complete interface.

For your reference, these are the standard modules involved in Rust
concurrency at this writing.

* [`core::task`] - All code relating to tasks and task scheduling
* [`core::comm`] - The message passing interface
* [`core::pipes`] - The underlying messaging infrastructure
* [`std::comm`] - Additional messaging types based on `core::pipes`
* [`std::sync`] - More exotic synchronization tools, including locks
* [`std::arc`] - The ARC (atomically reference counted) type,
  for safely sharing immutable data

[`core::task`]: core/task.html
[`core::comm`]: core/comm.html
[`core::pipes`]: core/pipes.html
[`std::comm`]: std/comm.html
[`std::sync`]: std/sync.html
[`std::arc`]: std/arc.html

# Your first concurrent Rust program

Let's start by constructing a real program to solve a simple computation.
We're going to pass some numbers to a few tasks, perform computations on them,
and have each of them pass their results to yet another task to format them as a string.
Finally we'll print the string in the main task.

Specifically:

1) The main task spawns 3 tasks.
2) Task 1 multiplies `10 * 2`, sends result to task 3
4) Task 2 adds `30 + 40`, sends result to task 3
5) Task 3 receives the three results, sums and formats them as a string, then sends them to main
6) The main task receives the string from task 3 and prints it to stdout

I like getting the big picture to start with, so I'll show you the solution,
then we'll walk through it step by step.

~~~
fn main() {
    let (out1, in1): (Port<int>, Chan<int>) = stream();
    let (out2, in2): (Port<int>, Chan<int>) = stream();
    let (out_result, in_result): (Port<~str>, Chan<~str>) = stream();

    do spawn {
        in1.send(10 * 2);
    }

    do spawn {
        in2.send(30 + 40);
    }

    do spawn {
        let x = out1.recv();
        let y = out2.recv();
        let result_str = fmt!("%d + %d = %d", x, y, x + y);
        in_result.send(result_str);
    }

    let result_str = out_result.recv();
    println(result_str);
}
~~~

This is just a simple program contained entirely within the `main` function.
Notice that we don't import anything with `use` statements - everything
needed for creating simple concurrent programs is imported by default.

## Setting up the pipes

Rust tasks communicate by passing *messages* over *pipes*.
Messages are simply Rust values.
Pipes are types in the core library that send values in a single direction,
from a sending endpoint to a receiving endpoint.
Pipes come in several forms but the most common is the `Port` and `Chan` (i.e. 'channel')
pair, created by calling `stream`. 

~~~
let (out1, in1): (Port<int>, Chan<int>) = stream();
~~~

`stream` returns a tuple, which is
destructured here into the variables `out1` and `in1`.
For clarity, this declarition is annotated with the type returned by stream,
`(Port<int>, Chan<int>)`, but in practice you may want to leave such
declarations unannotated and rely on type inferrence.
Alternately, the call to `stream` may be passed an explicit type parameter,
as in `stream::<int>`.
This is a slightly less verbose way to provide a type annotation in this scenario.

The 'out' side is a `Port<int>`. A `Port` has a `recv` method which you can use
to try to get a value out of a pipe, waiting until a value is available if the pipe is empty.
`Port` is a generic type that that is parameterized over the type that it can receive.
In this case we have a `Port<int>` so we can receive only `int`s.

The 'in' side is a `Chan<int>`. A `Chan` has a `send` method which you can use
to send a value of the proper type across the pipe.
Again, `Chan` is generic, parameterized over the type that it sends.

The `(out1, in1)` pair will be used to communicate the result from task 1,
and the second `(out2, in2)` pair likewise used for task 2.
The final pipe will be used to send a string containing the result.
This is where Rust's owned types come into play to allow very efficient
communication between tasks while still being completely safe from data races.

Remember that Rust tasks do not share memory.
Allowing parallel threads of execution to have pointers to the same
objects in memory makes it possible to create *race conditions*,
- where two parallel threads try to modify or access the same data at the same time -
and race conditions can result in crashes.
Avoiding race conditions with shared memory is very difficult,
so Rust does not allow shared memory by default.

Owned values are guaranteed to be pointed to be only a single pointer.
When you call `chan.send(~"value")` that value's ownership is transferred
from the sending task, through the pipe, to the receiving task.
This is very effecient because all that is transferred is a pointer,
not a complete copy of the data, but it is also safe from race conditions
because only a single task at a time may point to the sent value.

## Setting up the task

The way to create and run a new task is the `spawn` function.
`spawn` has a very simple type signature: `fn spawn(f:
~fn())`. You give it a function or closure to run and it arranges for that
function to run in its own task.
A closure is like a function, but it gets to grab variables from the denclosing scope.
Because `spawn` accepts only owned closures, `~fn()`, and owned closures
contain only owned data, `spawn` can safely move the entire closure
and all its associated state into an entirely different task for
execution.
In our first task we use this to move a port across tasks:

~~~
#    let (out1, in1): (Port<int>, Chan<int>) = stream();
    do spawn {
        in1.send(10 * 2);
    }
~~~

Though this is just a small block of code there is a lot going on here.
This is a call to the `spawn` function using Rust's `do` notation and the stuff in curly braces is an owned closure.
It could equivalently be written `spawn(|| in1.send(10 * 2))` using the normal
function call and closure syntax.
Our previously-created channel, `in1`, is *captured* in the closure.
Capturing a variable in an owned closure *moves* the value into the closure,
transferring ownership to the spawned task.
As a result, the compiler will not let you use `in1` subsequently in `main` -
it understands that this value now belongs to the other task.



~~~
    let (out1, in1): (Port<int>, Chan<int>) = stream();
    let (out2, in2): (Port<int>, Chan<int>) = stream();
    let (out_result, in_result): (Port<~str>, Chan<~str>) = stream();

    in1.send(10 * 2);
    in2.send(30 + 40);

    do spawn {
        let x = out1.recv();
        let y = out2.recv();
        let result_str = fmt!("%d + %d = %d", x, y, x + y);
        in_result.send(result_str);
    }
~~~


## Communication

Now that we have spawned a new task, it would be nice if we could
communicate with it. Recall that Rust does not have shared mutable
state, so one task may not manipulate variables owned by another task.
Instead we use *pipes*.

A pipe is simply a pair of endpoints: one for sending messages and another for
receiving messages. Pipes are low-level communication building-blocks and so
come in a variety of forms, each one appropriate for a different use case. In
what follows, we cover the most commonly used varieties.

The simplest way to create a pipe is to use the `pipes::stream`
function to create a `(Port, Chan)` pair. In Rust parlance, a *channel*
is a sending endpoint of a pipe, and a *port* is the receiving
endpoint. Consider the following example of calculating two results
concurrently:

~~~~
use core::task::spawn;
use core::comm::{stream, Port, Chan};

let (port, chan): (Port<int>, Chan<int>) = stream();

do spawn || {
    let result = some_expensive_computation();
    chan.send(result);
}

some_other_expensive_computation();
let result = port.recv();
# fn some_expensive_computation() -> int { 42 }
# fn some_other_expensive_computation() {}
~~~~

Let's examine this example in detail. First, the `let` statement creates a
stream for sending and receiving integers (the left-hand side of the `let`,
`(chan, port)`, is an example of a *destructuring let*: the pattern separates
a tuple into its component parts).

~~~~
# use core::comm::{stream, Chan, Port};
let (port, chan): (Port<int>, Chan<int>) = stream();
~~~~

The child task will use the channel to send data to the parent task,
which will wait to receive the data on the port. The next statement
spawns the child task.

~~~~
# use core::task::spawn;
# use core::comm::stream;
# fn some_expensive_computation() -> int { 42 }
# let (port, chan) = stream();
do spawn || {
    let result = some_expensive_computation();
    chan.send(result);
}
~~~~

Notice that the creation of the task closure transfers `chan` to the child
task implicitly: the closure captures `chan` in its environment. Both `Chan`
and `Port` are sendable types and may be captured into tasks or otherwise
transferred between them. In the example, the child task runs an expensive
computation, then sends the result over the captured channel.

Finally, the parent continues with some other expensive
computation, then waits for the child's result to arrive on the
port:

~~~~
# use core::comm::{stream};
# fn some_other_expensive_computation() {}
# let (port, chan) = stream::<int>();
# chan.send(0);
some_other_expensive_computation();
let result = port.recv();
~~~~

The `Port` and `Chan` pair created by `stream` enables efficient communication
between a single sender and a single receiver, but multiple senders cannot use
a single `Chan`, and multiple receivers cannot use a single `Port`.  What if our
example needed to compute multiple results across a number of tasks? The
following program is ill-typed:

~~~ {.xfail-test}
# use core::task::{spawn};
# use core::comm::{stream, Port, Chan};
# fn some_expensive_computation() -> int { 42 }
let (port, chan) = stream();

do spawn {
    chan.send(some_expensive_computation());
}

// ERROR! The previous spawn statement already owns the channel,
// so the compiler will not allow it to be captured again
do spawn {
    chan.send(some_expensive_computation());
}
~~~

Instead we can use a `SharedChan`, a type that allows a single
`Chan` to be shared by multiple senders.

~~~
# use core::task::spawn;
use core::comm::{stream, SharedChan};

let (port, chan) = stream();
let chan = SharedChan(chan);

for uint::range(0, 3) |init_val| {
    // Create a new channel handle to distribute to the child task
    let child_chan = chan.clone();
    do spawn {
        child_chan.send(some_expensive_computation(init_val));
    }
}

let result = port.recv() + port.recv() + port.recv();
# fn some_expensive_computation(_i: uint) -> int { 42 }
~~~

Here we transfer ownership of the channel into a new `SharedChan` value.  Like
`Chan`, `SharedChan` is a non-copyable, owned type (sometimes also referred to
as an *affine* or *linear* type). Unlike with `Chan`, though, the programmer
may duplicate a `SharedChan`, with the `clone()` method.  A cloned
`SharedChan` produces a new handle to the same channel, allowing multiple
tasks to send data to a single port.  Between `spawn`, `stream` and
`SharedChan`, we have enough tools to implement many useful concurrency
patterns.

Note that the above `SharedChan` example is somewhat contrived since
you could also simply use three `stream` pairs, but it serves to
illustrate the point. For reference, written with multiple streams, it
might look like the example below.

~~~
# use core::task::spawn;
# use core::comm::stream;

// Create a vector of ports, one for each child task
let ports = do vec::from_fn(3) |init_val| {
    let (port, chan) = stream();
    do spawn {
        chan.send(some_expensive_computation(init_val));
    }
    port
};

// Wait on each port, accumulating the results
let result = ports.foldl(0, |accum, port| *accum + port.recv() );
# fn some_expensive_computation(_i: uint) -> int { 42 }
~~~

# Handling task failure

Rust has a built-in mechanism for raising exceptions. The `fail!()` macro
(which can also be written with an error string as an argument: `fail!(
~reason)`) and the `fail_unless!` construct (which effectively calls `fail!()`
if a boolean expression is false) are both ways to raise exceptions. When a
task raises an exception the task unwinds its stack---running destructors and
freeing memory along the way---and then exits. Unlike exceptions in C++,
exceptions in Rust are unrecoverable within a single task: once a task fails,
there is no way to "catch" the exception.

All tasks are, by default, _linked_ to each other. That means that the fates
of all tasks are intertwined: if one fails, so do all the others.

~~~
# use core::task::spawn;
# fn do_some_work() { loop { task::yield() } }
# do task::try {
// Create a child task that fails
do spawn { fail!() }

// This will also fail because the task we spawned failed
do_some_work();
# };
~~~

While it isn't possible for a task to recover from failure, tasks may notify
each other of failure. The simplest way of handling task failure is with the
`try` function, which is similar to `spawn`, but immediately blocks waiting
for the child task to finish. `try` returns a value of type `Result<int,
()>`. `Result` is an `enum` type with two variants: `Ok` and `Err`. In this
case, because the type arguments to `Result` are `int` and `()`, callers can
pattern-match on a result to check whether it's an `Ok` result with an `int`
field (representing a successful result) or an `Err` result (representing
termination with an error).

~~~
# fn some_condition() -> bool { false }
# fn calculate_result() -> int { 0 }
let result: Result<int, ()> = do task::try {
    if some_condition() {
        calculate_result()
    } else {
        fail!(~"oops!");
    }
};
fail_unless!(result.is_err());
~~~

Unlike `spawn`, the function spawned using `try` may return a value,
which `try` will dutifully propagate back to the caller in a [`Result`]
enum. If the child task terminates successfully, `try` will
return an `Ok` result; if the child task fails, `try` will return
an `Error` result.

[`Result`]: core/result.html

> ***Note:*** A failed task does not currently produce a useful error
> value (`try` always returns `Err(())`). In the
> future, it may be possible for tasks to intercept the value passed to
> `fail!()`.

TODO: Need discussion of `future_result` in order to make failure
modes useful.

But not all failure is created equal. In some cases you might need to
abort the entire program (perhaps you're writing an assert which, if
it trips, indicates an unrecoverable logic error); in other cases you
might want to contain the failure at a certain boundary (perhaps a
small piece of input from the outside world, which you happen to be
processing in parallel, is malformed and its processing task can't
proceed). Hence, you will need different _linked failure modes_.

## Failure modes

By default, task failure is _bidirectionally linked_, which means that if
either task fails, it kills the other one.

~~~
# fn sleep_forever() { loop { task::yield() } }
# do task::try {
do task::spawn {
    do task::spawn {
        fail!();  // All three tasks will fail.
    }
    sleep_forever();  // Will get woken up by force, then fail
}
sleep_forever();  // Will get woken up by force, then fail
# };
~~~

If you want parent tasks to be able to kill their children, but do not want a
parent to fail automatically if one of its child task fails, you can call
`task::spawn_supervised` for _unidirectionally linked_ failure. The
function `task::try`, which we saw previously, uses `spawn_supervised`
internally, with additional logic to wait for the child task to finish
before returning. Hence:

~~~
# use core::comm::{stream, Chan, Port};
# use core::task::{spawn, try};
# fn sleep_forever() { loop { task::yield() } }
# do task::try {
let (receiver, sender): (Port<int>, Chan<int>) = stream();
do spawn {  // Bidirectionally linked
    // Wait for the supervised child task to exist.
    let message = receiver.recv();
    // Kill both it and the parent task.
    fail_unless!(message != 42);
}
do try {  // Unidirectionally linked
    sender.send(42);
    sleep_forever();  // Will get woken up by force
}
// Flow never reaches here -- parent task was killed too.
# };
~~~

Supervised failure is useful in any situation where one task manages
multiple fallible child tasks, and the parent task can recover
if any child fails. On the other hand, if the _parent_ (supervisor) fails,
then there is nothing the children can do to recover, so they should
also fail.

Supervised task failure propagates across multiple generations even if
an intermediate generation has already exited:

~~~
# fn sleep_forever() { loop { task::yield() } }
# fn wait_for_a_while() { for 1000.times { task::yield() } }
# do task::try::<int> {
do task::spawn_supervised {
    do task::spawn_supervised {
        sleep_forever();  // Will get woken up by force, then fail
    }
    // Intermediate task immediately exits
}
wait_for_a_while();
fail!();  // Will kill grandchild even if child has already exited
# };
~~~

Finally, tasks can be configured to not propagate failure to each
other at all, using `task::spawn_unlinked` for _isolated failure_.

~~~
# fn random() -> uint { 100 }
# fn sleep_for(i: uint) { for i.times { task::yield() } }
# do task::try::<()> {
let (time1, time2) = (random(), random());
do task::spawn_unlinked {
    sleep_for(time2);  // Won't get forced awake
    fail!();
}
sleep_for(time1);  // Won't get forced awake
fail!();
// It will take MAX(time1,time2) for the program to finish.
# };
~~~

