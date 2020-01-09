use async_std::{io, prelude::*, task};
use async_std::io::BufReader;
use async_std::sync::channel;
use futures::{FutureExt, select, Stream};

async fn stdin() -> impl Stream<Item = String> {
    let (tx, rx) = channel(1);
    let mut lines = BufReader::new(io::stdin()).lines().fuse();
    loop {
        select! {
            line = lines.next().fuse() => match line {
                Some(Ok(s)) => tx.send(s).await,
                _ => break
            }
        }
    }
    rx
}

fn main() {
    task::block_on( 
        async {
            let mut stdin = stdin().await;
            while let Some(stream) = stdin.next().await {
                let t:String = stream.clone();
                println!("{:?}", t);
            }
        }
    );
}