import comm::*;
import future::*;

enum state_msg<T: send> {
    as_ch(comm::chan<comm::chan<T>>),
    as_next(comm::chan<T>),
    as_defer(T),
    as_exit
}

enum actor<T: send> {
    actor_({
        data_ch: comm::chan<T>,
        state_ch: comm::chan<state_msg<T>>
    })
}

impl methods<T: send> for actor<T> {
    fn send<O: send>(other: actor<O>, +v: O) {
        comm::send(other.chan(), v)
    }

    fn act(body: fn(msg<T>)) {
        let po = comm::port();

        loop {
            comm::send(self.state_ch, as_next(comm::chan(po)));

            let msg = msg_({
                defer_: @mut false,
                payload_: comm::recv(po)
            });
            body(msg);

            if *msg.defer_ {
                comm::send(self.state_ch, as_defer(msg.payload_))
            } else {
                break;
            }
        }
    }

    fn chan() -> comm::chan<T> {
        alt self {
          actor_({data_ch, _ }) { data_ch }
        }
    }
}

enum msg<T: send> {
    msg_({
        defer_: @mut bool,
        payload_: T
    })
}

impl methods<T: send> for msg<T> {
    fn payload() -> T {
        self.payload_
    }

    fn defer() {
        *self.defer_ = true;
    }
}

impl queue<T: send> for port<T> {
    fn enq(+v: T) { send(chan(self), v) }
    fn deq() -> T { recv(self) }
}

fn become<T: send>(body: fn(actor<T>)) {
    let state_ch = task::spawn_listener {|state_po|
        let data_po = comm::port();

        alt check comm::recv::<state_msg<T>>(state_po) {
          as_ch(ch) { comm::send(ch, comm::chan(data_po)) }
        }

        // FIXME: This needs to be a random draw, not a queue
        let queue = comm::port();
        let deferrals = comm::port();
        let mut awaiting_deferral = false;

        loop {
            alt check comm::recv(state_po) {
              as_next(ch) {
                if awaiting_deferral {
                    // We never deferred the previous message,
                    // so we've changed state and may be interested
                    // in all the old deferrals
                    while peek(deferrals) {
                        queue.enq(deferrals.deq())
                    }
                }

                // Pull in all the new requests
                while peek(data_po) {
                    queue.enq(data_po.deq())
                }
                if peek(queue) {
                    send(ch, queue.deq())
                } else {
                    send(ch, data_po.deq())
                }
                awaiting_deferral = true;
              }
              as_defer(msg) {
                awaiting_deferral = false;
                deferrals.enq(msg)
              }
              as_exit { break; }
            }
        }
    };

    let data_po = comm::port();
    comm::send(state_ch, as_ch(comm::chan(data_po)));
    let data_ch = comm::recv(data_po);

    resource exiter<T: send>(state_ch: comm::chan<state_msg<T>>) {
        send(state_ch, as_exit);
    }

    let _exiter = exiter(state_ch);
    let actor = actor_({
        data_ch: data_ch,
        state_ch: state_ch
    });
    body(actor);
}

fn actor<T: send>(body: fn~(actor<T>)) -> actor<T> {
    // FIXME: I hate writing this pattern to set up
    // communications
    let po = comm::port();
    let ch = comm::chan(po);
    task::spawn {||
        become {|self|
            comm::send(ch, copy(self));
            body(self)
        }
    };

    comm::recv(po)
}

fn exit() {
    rustrt::unsupervise();
    fail "successful exit";
}

fn main() {
    test1();
    test2();
}

fn test2() {
    enum m {
        m1,
        m2,
        m3,
        m4
    }

    become {|sink|
        actor::<()> {|generator|
            generator.send(sink, m1);
            generator.send(sink, m2);
            generator.send(sink, m3);
            generator.send(sink, m4);
        };

        let mut f4 = false;
        let mut f3 = false;
        let mut f2 = false;
        let mut f1 = false;

        #debug("4");
        sink.act {|msg|
            alt msg.payload() {
              m4 { f4 = true; }
              _ { msg.defer() }
            }
        }

        #debug("3");
        sink.act {|msg|
            alt msg.payload() {
              m3 { f3 = true; }
              _ { msg.defer() }
            }
        }

        #debug("2");
        sink.act {|msg|
            alt msg.payload() {
              m2 { f2 = true; }
              _ { msg.defer() }
            }
        }

        #debug("1");
        sink.act {|msg|
            alt msg.payload() {
              m1 { f1 = true; }
              _ { msg.defer() }
            }
        }

        assert f1 && f2 && f3 && f4;
    }
}

fn test1() {

    enum ping_msg {
        msg_ping(actor<pong_msg>),
        msg_exit
    }

    enum pong_msg { msg_pong }

    become {|client|
        let server = actor::<ping_msg> {|server|

            loop {
                server.act {|msg|
                    alt msg.payload() {
                      msg_ping(sender) {
                        server.send(sender, msg_pong)
                      }
                      msg_exit {
                        become {|exiter|
                            exiter.send(client, msg_ping(exiter));
                            exiter.act {|msg|
                                alt msg.payload() {
                                  msg_pong { exit() }
                                }
                            }
                        }
                      }
                    }
                }
            }
        };

        become {|client2|
            client2.send(server, msg_ping(client2));
            client2.act {|msg|
                alt msg.payload() {
                  msg_pong { #info("pong") }
                }
            }
        }

        actor {|client2|
            client2.send(server, msg_ping(client2));
            client2.act {|msg|
                alt msg.payload() {
                  msg_pong { #info("pong") }
                }
            }
        };

        client.send(server, msg_exit);
        client.act {|msg|
            alt msg.payload() {
              msg_ping(sender) {
                client.send(sender, msg_pong);
              }
              _ { msg.defer() }
            }
        }

    }
}

native mod rustrt {
    fn unsupervise();
}