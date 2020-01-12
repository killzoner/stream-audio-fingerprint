use std::error::Error;
use std::process::{Command, Stdio};
use rs_landmark::stdin::{stdin_stream};
use async_std::{task};
use futures::{select, FutureExt};
use async_std::sync::channel;

const DEBUG_STDIN: bool = true;

fn main() {
    let (stdin_sender, mut stdin_receiver) = channel::<String>(16);
    let stdin_handle = stdin_stream(false, stdin_sender);
    let stdin_display = task::spawn(async move {
        if DEBUG_STDIN {
            loop {
                select! {
                    msg = stdin_receiver.next().fuse() => match msg {
                        Some(msg) => println!("{:?}", msg),
                        None => break,
                    },
                    complete =>  {
                        println!("end");
                        break
                    }
                }
            }
        }
    });
    use async_std::prelude::*;
    task::block_on(stdin_handle.join(stdin_display));

    // spawn the ffmpeg command
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
        process
    });
    task::block_on(decoder);
}
