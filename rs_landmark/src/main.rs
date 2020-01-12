use std::error::Error;
use std::process::{Command, Stdio};
use rs_landmark::stdin::{stdin_stream};
use async_std::{task};
use futures::{select, FutureExt, StreamExt};
use async_std::sync::channel;
use std::{
    sync::Arc
};
use std::io::Write;

const DEBUG_STDIN: bool = false;

async fn run() {
    let (stdin_sender, stdin_receiver) = channel::<String>(16);
    // let mut stdin_test = stdin_receiver.clone();
    let stdin_handle = stdin_stream(false, stdin_sender);

    //DEBUG BEGIN
    /*
    let stdin_display = task::spawn(async move {
        if DEBUG_STDIN {
            loop {
                select! {
                    msg = stdin_test.next().fuse() => match msg {
                        Some(msg) => println!("{:?}", msg),
                        None => break,
                    }
                }
            }
        }
    });
    use async_std::prelude::*;
    stdin_handle.join(stdin_display).await;
    */
    //DEBUG END

    // spawn the ffmpeg command
    let mut stdin_to_decoder = stdin_receiver.clone();
    let decoder = task::spawn_blocking( || {
        let process = match Command::new("ffmpeg")
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
                "pipe:1",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(why) => panic!("couldn't spawn ffmpeg: {}", why.description()),
            Ok(process) => process,
        };
        println!("Running ffmpeg with id {:?}", process.id());
        //let ffmpeg_stdin = process.stdin.unwrap();
        process
        /*let mut s = String::new();
        match process.stdout.unwrap().read_to_string(&mut s) {
            Err(why) => panic!("couldn't read wc stdout: {}", why.description()),
            Ok(_) => print!("wc responded with:\n{}", s),
        }*/
    });
    let decoder = decoder.await;
    println!("{:?}", decoder.id());

    task::spawn(async move {
        let decoder_stdin = decoder.stdin.unwrap();
        let decoder_stdin = Arc::new(decoder_stdin);
        //let decoder = &*decoder;
        if DEBUG_STDIN {
            loop {
                select! {
                    msg = stdin_to_decoder.next().fuse() => match msg {
                        Some(msg) => {
                            //let decoder_stdin = &*decoder_stdin;
                            match decoder_stdin.write_all(msg.as_bytes()) {
                                Err(why) => panic!("couldn't write to wc stdin: {}", why.description()),
                                Ok(_) => println!("sent pangram to wc"),
                            }
                        },
                        None => break,
                    }
                }
            }
        }
    });
    /*let mut stdin_process = stdin_receiver.clone();
    let task = task::spawn(async move {
        loop {
            select! {
                msg = stdin_process.next().fuse() => match msg {
                    Some(msg) => {
                        println!("Sending to ffmpeg {:?}", msg);
                    },
                    None => break,
                }
            }
        }
    });
    task.await;*/
}

fn main() {
    task::block_on(run());
}
