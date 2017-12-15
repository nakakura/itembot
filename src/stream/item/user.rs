use futures::*;
use futures::sync::mpsc;
use tokio_core::reactor::Core;

use std::thread;
use std::sync::Arc;

use slack_command::SlackCommand;

pub fn set_receiver(receiver: mpsc::Receiver<Arc<SlackCommand>>) -> mpsc::Receiver<String> {
    let (tx_return_message, rx_return_message) = mpsc::channel::<String>(5000);
    let _ = thread::spawn(move || {
        let mut core = Core::new().unwrap();
        let x = receiver.fold(tx_return_message, |tx, x| {
            let tx = tx.send("moge".to_string()).wait().unwrap();
            Ok(tx)
        });
        let _ = core.run(x);
    });
    rx_return_message
}