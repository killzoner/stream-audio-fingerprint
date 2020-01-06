use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::io::{self, Read};

static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";

fn main() {
    // get current stdin
    let mut buffer = String::new();
    let stdin = match io::stdin().read_line(&mut buffer) {
        Err(why) => panic!("couldn't read stdin: {}", why.description()),
        Ok(process) => process,
    };
    let buffer = buffer.trim();
    println!("{:?}", buffer);

    /*
    let mut r = io::stdin();
    let mut buffer2 = String::new();
    loop {
        let b = r.read(&mut buffer2)();
        if b.is_none() {
            break;
        }
        println!("{}", b.unwrap());
    }*/

    // spawn the ffmpeg command
    let decoder = match Command::new("ffmpeg")
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

    // Spawn the `wc` command
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn wc: {}", why.description()),
        Ok(process) => process,
    };

    // Write a string to the `stdin` of `wc`.
    //
    // `stdin` has type `Option<ChildStdin>`, but since we know this instance
    // must have one, we can directly `unwrap` it.
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why.description()),
        Ok(_) => println!("sent pangram to wc"),
    }

    // Because `stdin` does not live after the above calls, it is `drop`ed,
    // and the pipe is closed.
    //
    // This is very important, otherwise `wc` wouldn't start processing the
    // input we just sent.

    // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why.description()),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}
