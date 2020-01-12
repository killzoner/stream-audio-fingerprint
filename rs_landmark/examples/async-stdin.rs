use rs_landmark::stdin;
use async_std::{task};

const DEBUG: bool = true;

fn main() {
    task::block_on(stdin::stdin_stream(DEBUG))
}