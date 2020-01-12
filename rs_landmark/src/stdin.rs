use async_std::{io, prelude::*, task};
use async_std::io::BufReader;
use futures::channel::mpsc;
use futures::sink::SinkExt;

type Sender<T> = mpsc::UnboundedSender<T>;

async fn stdin(mut tx: Sender<String>, debug: bool) -> () {
    let mut lines = BufReader::new(io::stdin()).lines();
    while let Some(Ok(s)) = lines.next().await {
        if debug {
            println!("from stdin : {:?}", s);
        }
        tx.send(s).await.unwrap()
    }
}

pub async fn stdin_stream(
    debug: bool, 
    sender: Sender<String>) {
    let handle = task::spawn(stdin(sender, debug));
    handle.await
}