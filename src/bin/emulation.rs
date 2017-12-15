extern crate futures;
extern crate tokio_core;

extern crate itembot;

use futures::*;
use futures::sync::mpsc;
use tokio_core::reactor::Core;

use std::io::stdin;
use std::thread;

use itembot::slack_command::SlackCommand;

fn main() {
    let (mut tx, rx): (mpsc::Sender<SlackCommand>, mpsc::Receiver<SlackCommand>) = mpsc::channel::<SlackCommand>(5000);
    let receiver = itembot::stream::set_receiver(rx);

    let th = thread::spawn(move || {
        let mut core = Core::new().unwrap();
        let x = receiver.for_each(|x| {
            println!("eee {}", x);
            Ok(())
        });
        let _ = core.run(x);
    });

    loop {
        println!(">mono SUBCOMMAND");
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let line = line.trim_right(); // Remove the trailing newline
        if line == "exit" {
            break;
        } else if let Some(post) = SlackCommand::create_command("user1", "channel1", line) {
            tx = tx.send(post).wait().unwrap();

        }
    }

    let _ = th.join();
}
