use std::io::{self, BufRead};
use std::thread;

use futures::{Future, Sink, Stream};
use futures::stream::BoxStream;
use futures::channel::mpsc::channel;

use futures::{
    SinkExt // for rx/tx send & receive
};

fn stdin() -> impl Stream<String, io::Error> {
    let (mut tx, rx) = channel(1);
    thread::spawn(move || {
        let input = io::stdin();
        for line in input.lock().lines() {
            match tx.send(line).wait() {
                Ok(s) => tx = s,
                Err(_) => break,
            }
        }
    });
    rx.then(|e| e.unwrap())
}

fn main() {
    stdin()
        .for_each(|string| {
            println!("{}", string);
            Ok(())
        })
        .wait()
        .unwrap();
}