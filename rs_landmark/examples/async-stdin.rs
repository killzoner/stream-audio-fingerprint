use async_std::{io, prelude::*, task};
use async_std::io::BufReader;
use futures::channel::mpsc;
use futures::sink::SinkExt;

type Sender<T> = mpsc::UnboundedSender<T>;

async fn stdin(mut tx: Sender<String>) -> () {
    let mut lines = BufReader::new(io::stdin()).lines();
    while let Some(Ok(s)) = lines.next().await {
        println!("{:?}", s);
        tx.send(s).await.unwrap()
    }
}

async fn run() {
    let (stdin_sender, _stdin_receiver) = mpsc::unbounded::<String>();
    let handle = task::spawn(stdin(stdin_sender));
    handle.await
}

fn main() {
    task::block_on(run())
}