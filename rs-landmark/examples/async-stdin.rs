use async_std::{io, prelude::*, task};
use async_std::io::BufReader;
use async_std::sync::channel;
use futures::{FutureExt, select, Stream};
use futures::stream::StreamExt; // for `next`

async fn stdin() -> impl Stream<Item = String> {
    let (tx, rx) = channel(1);
    let mut lines = BufReader::new(io::stdin()).lines().fuse();
    loop {
        select! {
            line = lines.next().fuse() => match line {
                Some(line) => {
                    match line {
                        Ok(s) => tx.send(s).await,
                        Err(_) => break
                    }
                }
                None => break,
            }
        }
    }
    rx
}

fn main() {
    /*let future = stdin()
    .for_each(|string| {
        println!("{}", string);
        Ok(())
    })
    task::block_on(
        async {
            future
        }
    )*/
    task::block_on( 
        async {
            let stdin = stdin().await;
            while let Some(stream) = stdin.next().await {
                let t:String = stream.clone();
                println!("{:?}", t);
            }
        }
    );
    /*async {
        let lock: () = input.lock().await;
    };*/
    /*stdin()
        .for_each(|string| {
            println!("{}", string);
            Ok(())
        })
        .wait()
        .unwrap();*/
}