#![feature(await_macro, async_await, futures_api)]

#[macro_use]
extern crate tokio;

use futures::prelude::*;
use futures::sync::mpsc;
use std::any::Any;
use std::fmt::Debug;
use std::{thread, time};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

#[derive(Default)]
struct Rusty {
    inner: Arc<RwLock<HashMap<String, Box<Any + Send + Sync>>>>
}

impl Rusty {

    pub fn new() -> Rusty {
        Default::default()
    }

    /*
    pub fn get(&self, key: &str) -> Option<&String> {
        let inner = self.inner.read().unwrap();
        inner.get(key)
    }

    pub fn insert(&self, key: String, value: String) {
        self.inner.write().unwrap().insert(key, value);
    }
    */

    pub fn receive<T: Any + Debug + Send + 'static>(&self, address: &'static str) -> mpsc::Receiver<T> {
        let (sender, receiver) = mpsc::channel::<T>(16);
        self.inner.write().expect("receive - should lock write").insert(address.to_owned(), Box::new(sender));
        receiver
    }

    pub fn publish<T: Any + Debug + Send + 'static>(&self, address: &'static str) -> mpsc::Sender<T> {
        let (sender, receiver) = mpsc::channel::<T>(16);

        println!("create new sender");

        let cloned_inner = self.inner.clone();
        tokio::spawn(receiver.for_each(move |msg| {
            println!("Received message: [{:?}]", msg);

            match cloned_inner.read().expect("publish - should lock on read").get(address) {
                Some(_) => println!("Consumer found"),
                None => println!("No consumers found")
            }

            Ok(())
        }));

        sender
    }

}

fn send_hello_msg() {


    tokio::run_async(async {
        let rusty = Rusty::new();

        let publish = rusty.publish("address1");

        await!(publish.clone().send("Hello1".to_owned()));
        await!(publish.clone().send("Hello2".to_owned()));

    });


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

    //#[test]
    fn should_print_hello() {
        say_hello();
    }

    #[test]
    fn should_send_hello_msg() {
        send_hello_msg();
    }

}