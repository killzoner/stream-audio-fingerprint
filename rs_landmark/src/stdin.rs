use async_std::{io, prelude::*, task};
use async_std::io::BufReader;
use futures::channel::mpsc;
//use futures::{FutureExt, select, Stream};
use futures::sink::SinkExt;

type Sender<T> = mpsc::UnboundedSender<T>;
//type Receiver<T> = mpsc::UnboundedReceiver<T>;
//type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn stdin(mut tx: Sender<String>) -> () {
    let mut lines = BufReader::new(io::stdin()).lines();
    while let Some(Ok(s)) = lines.next().await {
        println!("{:?}", s);
        tx.send(s).await.unwrap()
    }
}

pub async fn stdin_stream() -> () {
    let (stdin_sender, _stdin_receiver) = mpsc::unbounded::<String>();
    let handle = task::spawn(stdin(stdin_sender));
    handle.await
}