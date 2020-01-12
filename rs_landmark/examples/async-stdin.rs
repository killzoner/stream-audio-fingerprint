use rs_landmark::stdin;
use async_std::{task};
use futures::channel::mpsc;
use futures::{select, FutureExt};
use async_std::{prelude::*};

const DEBUG: bool = true;

fn main() {
    let (sender, receiver) = mpsc::unbounded::<String>();
    task::block_on(async {
        task::spawn(async {
            if DEBUG {
                let mut stdin_receiver = receiver.fuse();
                loop {
                    select! {
                        msg = stdin_receiver.next().fuse() => match msg {
                            Some(msg) => println!("received from stdin : {:?}", msg),
                            None => break,
                        },
                    }
                }
            };
        });
        stdin::stdin_stream(DEBUG, sender).await
    })
}