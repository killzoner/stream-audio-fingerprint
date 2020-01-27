use std::error::Error;
use std::process::{Command, Stdio};
use async_std::sync::channel;
use async_std::{task};
use futures::{select, FutureExt, StreamExt, AsyncBufReadExt, AsyncBufRead};

const DEBUG: bool = true;

//cat mp3_sample/sample.mp3  | cargo run --release --stream_ffmpeg_decoder
fn main() {

    // bounded channel for now
    // let (sender, receiver) = channel::<String>(1024);

    // spawn the command
    let cmd = "ffmpeg";
    let decoder = match Command::new(cmd)
        .args(&[
            "-i",
            "pipe:0", 
            "-acodec",
            "pcm_s16le",
            "-ar",
            "22050",
            "-ac",
            "1",
            "-f",
            "wav",
            "-v",
            "fatal",
            "pipe:1"
        ])
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped()) 
        .spawn()
    {
        Err(why) => panic!("couldn't spawn ffmpeg: {}", why.description()),
        Ok(process) => process,
    };
        
    println!("Running {} with id {:?}", cmd, decoder.id());

    let reader = task::spawn_blocking(|| {
        //use async_std::io::BufReader;
        use std::io::{BufReader};
        use std::io::prelude::*;
        let stream = decoder.stdout.unwrap();
        let buf_read = BufReader::new(stream);
        for line in buf_read.bytes() {
            println!("{}", line.unwrap());
        }
    });

    task::block_on(async {
        use async_std::prelude::*;
        //wait for stdin and reader to have finished
        reader.await;
    })

}