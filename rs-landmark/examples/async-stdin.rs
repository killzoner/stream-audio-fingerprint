use std::io::{self, BufRead};
//use std::thread;

use futures::{Future, Sink, Stream};
use futures::stream::BoxStream;
//use futures::channel::mpsc::channel;
use async_std::{fs::File, /*io,*/ prelude::*, task};
use async_std::sync::channel as asyncChannel;

use futures::{
    SinkExt // for rx/tx send & receive
};

async fn stdin() -> impl Stream<Item = String> {
    let (mut tx, rx) = asyncChannel(1);
    let reader_task = task::spawn(async {
        let input = io::stdin();
        for line in input.lock().lines() {
            match line {
                Ok(s) => tx.send(s).await,
                Err(_) => break
            }
        }
    });
    rx
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