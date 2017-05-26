extern crate tokio_core;
use tokio_core::reactor;

extern crate futures;
use futures::empty;
use futures::future::Future;

use std::thread;
use std::time::Duration;

#[test]
fn default_core() {

    let val = 10;

    let timeout = reactor::Timeout::new(
        Duration::from_millis(10),
        &reactor::default_handle()
    ).unwrap().map(|_| val);

    let result = reactor::with_default_core(|core| core.run(timeout));

    assert_eq!(result.unwrap(), val);
}

#[test]
fn default_core_in_another_thread() {
    thread::spawn(move || {
        let val = 10;

        let timeout = reactor::Timeout::new(
            Duration::from_millis(10),
            &reactor::default_handle()
        ).unwrap().map(|_| val);

        let result = reactor::run_default(timeout);

        assert_eq!(result.unwrap(), val);
    }).join();
}

#[test]
#[should_panic(expected = "could not borrow default core, already borrowed (event loop running?)")]
fn running_core_inside_running_core_panics() {
    reactor::default_handle().spawn(reactor::Timeout::new(
        Duration::from_millis(50),
        &reactor::default_handle()
    ).unwrap().then(|_| reactor::run_default(empty::<(), ()>())));
    reactor::run_default(empty::<(), ()>());
}
