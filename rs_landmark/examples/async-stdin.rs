use rs_landmark::stdin;
use async_std::{task};
use futures::channel::mpsc;

const DEBUG: bool = true;

fn main() {
    let chan = mpsc::unbounded::<String>();
    task::block_on(stdin::stdin_stream(DEBUG, chan.0, chan.1))
}