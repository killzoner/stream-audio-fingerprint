use std::error::Error;
use std::process::{Command, Stdio};
use async_std::sync::channel;
use async_std::{task};
use futures::{select, FutureExt, StreamExt, AsyncBufReadExt, AsyncBufRead};

const DEBUG: bool = true;
const DEFAULT_BUF_SIZE: usize = 8 * 1024;

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
        use std::io::{BufReader};
        use std::io::prelude::*;
        let stream = decoder.stdout.unwrap();
        
        /*
        let buf_read = BufReader::new(stream);
        for line in buf_read.lines() {
            println!("{}", line.unwrap());
        }
        */
        
        let mut buf_read = BufReader::new(stream);
        let mut buffer = Vec::with_capacity(DEFAULT_BUF_SIZE);

        /*
         * TODO: match on things below to convert to 
         * relatively valid UTF8 strings and debug
         */
        while let Ok(_) = buf_read.read_until('\n' as u8, &mut buffer) {
            let output = String::from_utf8_lossy(&buffer);
            print!("decoder responded with:\n{:?}", output);
        }

        /*
        match stream.read_line(&mut buffer) {
            Ok(0) => None,
            Ok(_n) => {
                if buf.ends_with("\n") {
                    buf.pop();
                    if buf.ends_with("\r") {
                        buf.pop();
                    }
                }
                Some(Ok(buf))
            }
            Err(e) => Some(Err(e))
        }*/
        
    });

    task::block_on(async {
        use async_std::prelude::*;
        //wait for stdin and reader to have finished
        reader.await;
    })

}