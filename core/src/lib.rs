#![feature(await_macro, async_await, futures_api)]

#[macro_use]
extern crate tokio;

use futures::prelude::*;
use futures::sync::mpsc;

//use std::future::

use std::any::Any;
use std::fmt::Debug;
use std::{thread, time};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

#[derive(Default)]
pub struct Rusty {
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

    pub fn consumer<T: Any + Debug + Send + 'static>(&self, address: &'static str) -> mpsc::Receiver<T> {
        let (sender, receiver) = mpsc::channel::<T>(16);
        self.inner.write().expect("receive - should lock write").insert(address.to_owned(), Box::new(sender));
        receiver
    }

    pub fn producer<T: Any + Debug + Send + 'static>(&self, address: &'static str) -> mpsc::Sender<T> {
        let (sender, receiver) = mpsc::channel::<T>(16);

        println!("create new sender");

        let cloned_inner = self.inner.clone();
        tokio::spawn(receiver.for_each(move |msg| {
            println!("Received message: [{:?}]", msg);

            match cloned_inner.read().expect("publish - should lock on read").get(address) {
                Some(consumer) => {
                    println!("Consumer found...");
                    if let Some(c) = consumer.downcast_ref::<mpsc::Sender<T>>() {
                        println!("... and it's the correct consumer!!");
                        //println!("[{:?}]", &c);
                            let clone = c.clone();
                            clone.send(msg).wait().expect("Unable to send");
                            thread::sleep(time::Duration::from_millis(200));
                    } else {
                        println!("...but not the correct consumer...");
                    }
                },
                None => println!("No consumers found")
            }

            Ok(())
        }));

        sender
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_send_hello_msg() {
        tokio::run_async(async {
            let rusty = Rusty::new();

            let publish = rusty.producer("address1");

            await!(publish.clone().send("Hello1".to_owned()));
            await!(publish.clone().send("Hello2".to_owned()));
        });
    }

    #[test]
    fn should_send_receive_msg() {
        tokio::run_async(async {
            let rusty = Rusty::new();

            let consumer = rusty.consumer::<String>("address1");
            tokio::spawn(consumer.for_each(|msg| {
                println!("'address1' consumer received msg: [{}]", &msg);
                Ok(())
            }));

            let producer = rusty.producer("address1");

            await!(producer.clone().send("Hello1".to_owned()));
            await!(producer.clone().send("Hello2".to_owned()));
        });
    }


}