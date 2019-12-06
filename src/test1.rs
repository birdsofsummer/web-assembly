extern crate futures;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;

use futures::{Async, Future, Poll};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, future_to_promise};

/// A future that becomes ready after a tick of the micro task queue.
pub struct NextTick {
    inner: JsFuture,
}

impl NextTick {
    /// Construct a new `NextTick` future.
    pub fn new() -> NextTick {
        // Create a resolved promise that will run its callbacks on the next
        // tick of the micro task queue.
        let promise = js_sys::Promise::resolve(&JsValue::NULL);
        // Convert the promise into a `JsFuture`.
        let inner = JsFuture::from(promise);
        NextTick { inner }
    }
}

impl Future for NextTick {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        // Polling a `NextTick` just forwards to polling if the inner promise is
        // ready.
        match self.inner.poll() {
            Ok(Async::Ready(_)) => Ok(Async::Ready(())),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(_) => unreachable!(
                "We only create NextTick with a resolved inner promise, never \
                 a rejected one, so we can't get an error here"
            ),
        }
    }
}

/// Export a function to JavaScript that does some work in the next tick of the
/// micro task queue!
#[wasm_bindgen]
pub fn schedule_some_work_for_next_tick() -> js_sys::Promise {
    let future = NextTick::new()
        // Do some work...
        .and_then(|_| {
            Ok(42)
        })
        // And then convert the `Item` and `Error` into `JsValue`.
        .map(|result| {
            JsValue::from(result)
        })
        .map_err(|error| {
            let js_error = js_sys::Error::new(&format!("uh oh! {:?}", error));
            JsValue::from(js_error)
        });

    // Convert the `Future<Item = JsValue, Error = JsValue>` into a JavaScript
    // `Promise`!
    future_to_promise(future)
}
