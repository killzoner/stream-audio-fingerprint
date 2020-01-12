use rs_landmark::stdin;
use async_std::{task};
use futures::channel::mpsc;
use futures::{select, FutureExt};

const DEBUG: bool = true;

fn main() {
    let (sender, receiver) = mpsc::unbounded::<String>();
    task::block_on(async {
        let reader = task::spawn(async {
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
        use async_std::prelude::*;
        //wait for stdin and reader to have finished
        stdin::stdin_stream(DEBUG, sender).join(reader).await;
    })
}