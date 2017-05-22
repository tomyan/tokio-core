extern crate tokio_core;
use tokio_core::reactor;

extern crate futures;
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
