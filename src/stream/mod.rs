pub mod item;

use futures::*;
use futures::sync::mpsc;
use tokio_core::reactor::Core;

use std::clone::Clone;
use std::thread;
use std::sync::Arc;

use slack_command::SlackCommand;

pub fn set_receiver(receiver: mpsc::Receiver<SlackCommand>) -> mpsc::Receiver<String> {
    let (tx_return_message, rx_return_message) = mpsc::channel::<String>(5000);
    let (item_tx, item_rx) = mpsc::channel::<Arc<SlackCommand>>(5000);

    let _ = thread::spawn(move || {
        let mut core = Core::new().unwrap();
        let x = receiver.fold(item_tx, |tx, x| {
            let x_arc = Arc::new(x);
            let tx = tx.send(x_arc.clone()).wait().unwrap();
            Ok(tx)
        });
        let _ = core.run(x);
    });

    let _ = thread::spawn(move || {
        let item_fb_rx = item::set_receiver(item_rx);
        let _ = tx_return_message.send_all(item_fb_rx.map_err(|_| { panic!() })).wait();
    });

    rx_return_message
}
