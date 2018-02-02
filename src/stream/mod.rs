pub mod item;
pub mod rocky;

use futures::*;
use futures::sync::mpsc;
use tokio_core::reactor::Core;

use std::clone::Clone;
use std::thread;
use std::sync::Arc;

use slack_command::SlackCommand;

pub fn set_receiver(receiver: mpsc::Receiver<Arc<SlackCommand>>) -> mpsc::Receiver<String> {
    let (tx_return_message, rx_return_message) = mpsc::channel::<String>(5000);
    let (item_tx, item_rx) = mpsc::channel::<Arc<SlackCommand>>(5000);
    let (rocky_tx, rocky_rx) = mpsc::channel::<Arc<SlackCommand>>(5000);

    let _ = thread::spawn(move || {
        let mut core = Core::new().unwrap();
        let senders = vec!(item_tx, rocky_tx);
        let x = receiver.fold(senders, |tx, x| {
            Ok(tx.into_iter().map(|sender| sender.send(x.clone()).wait().unwrap() ).collect())
        });
        let _ = core.run(x);
    });

    let _ = thread::spawn(move || {
        let item_fb_rx = item::set_receiver(item_rx)
            .select(rocky::set_receiver(rocky_rx));
        let _ = tx_return_message.send_all(item_fb_rx.map_err(|_| { panic!() })).wait();
    });

    rx_return_message
}
