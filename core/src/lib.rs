#![feature(await_macro, async_await, futures_api)]

#[macro_use]
extern crate tokio;

use futures::prelude::*;
use futures::sync::mpsc;
use std::any::Any;
use std::sync::RwLock;
use std::collections::HashMap;

#[derive(Default)]
struct Map {
    inner: RwLock<HashMap<String, (mpsc::Sender<Sized>, mpsc::Receiver<Sized>)>>
}

impl Map {

    /*
    pub fn get(&self, key: &str) -> Option<&String> {
        let inner = self.inner.read().unwrap();
        inner.get(key)
    }

    pub fn insert(&self, key: String, value: String) {
        self.inner.write().unwrap().insert(key, value);
    }
    */

    pub fn publish(address: &str, msg: &str) {
        let (sender, receiver) = mpsc::channel(16);
    }

}


fn say_hello() {
    // And we are async...
    tokio::run_async(async {
        println!("Hello");
    });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_print_hello() {
        say_hello();
    }

}