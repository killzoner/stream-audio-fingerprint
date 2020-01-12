use async_std::{io, prelude::*, task};
use async_std::io::BufReader;
use futures::channel::mpsc;
use futures::sink::SinkExt;
use futures::{select, FutureExt};

type Sender<T> = mpsc::UnboundedSender<T>;

async fn stdin(mut tx: Sender<String>) -> () {
    let mut lines = BufReader::new(io::stdin()).lines();
    while let Some(Ok(s)) = lines.next().await {
        tx.send(s).await.unwrap()
    }
}

async fn run() {
    let (stdin_sender, stdin_receiver) = mpsc::unbounded::<String>();
    let handle = task::spawn(stdin(stdin_sender));
    let mut stdin_receiver = stdin_receiver.fuse();
    loop {
        select! {
            msg = stdin_receiver.next().fuse() => match msg {
                Some(msg) => println!("{:?}", msg),
                None => break,
            },
        }
    }
    handle.await
}

fn main() {
    task::block_on(run())
}