use std::error::Error;
use std::process::{Command, Stdio};
use async_std::{task};
//use async_std::sync::channel;
//use futures::{select, FutureExt, StreamExt, AsyncBufReadExt, AsyncBufRead};

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
        use std::io::prelude::*;
        let mut stream = decoder.stdout.unwrap();
        let mut buffer = Vec::new();

        match stream.read_to_end(&mut buffer) {
            Err(why) => panic!("couldn't read decoder stdout: {}", why.description()),
            Ok(_) => {
                let output = String::from_utf8_lossy(&buffer);
                print!("decoder responded with:\n{:?}", output);
            },
        }
        
    });

    task::block_on(async {
        //wait for stdin and reader to have finished
        reader.await;
    })

}