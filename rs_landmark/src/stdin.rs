use async_std::{io, prelude::*, task};
use async_std::io::BufReader;
use futures::channel::mpsc;
use futures::sink::SinkExt;
use futures::{select, FutureExt};

type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

async fn stdin(mut tx: Sender<String>) -> () {
    let mut lines = BufReader::new(io::stdin()).lines();
    while let Some(Ok(s)) = lines.next().await {
        tx.send(s).await.unwrap()
    }
}

pub async fn stdin_stream(
    debug: bool, 
    sender: Sender<String>,
    receiver: Receiver<String>) {
    let handle = task::spawn(stdin(sender));
    if debug {
        let mut stdin_receiver = receiver.fuse();
        loop {
            select! {
                msg = stdin_receiver.next().fuse() => match msg {
                    Some(msg) => println!("{:?}", msg),
                    None => break,
                },
            }
        }
    }
    handle.await
}