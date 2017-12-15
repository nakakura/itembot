pub mod add;
pub mod user;

use futures::*;
use futures::sync::mpsc;
use tokio_core::reactor::Core;

use std::thread;
use std::sync::Arc;

use slack_command::SlackCommand;

pub fn set_receiver(receiver: mpsc::Receiver<Arc<SlackCommand>>) -> mpsc::Receiver<String> {
    let (tx_return_message, rx_return_message) = mpsc::channel::<String>(5000);
    let (add_tx, add_rx) = mpsc::channel::<Arc<SlackCommand>>(5000);
    let (user_tx, user_rx) = mpsc::channel::<Arc<SlackCommand>>(5000);

    let _ = thread::spawn(move || {
        let mut core = Core::new().unwrap();
        let x = receiver.fold((add_tx, user_tx), |tx, x| {
            let tx0 = tx.0.send(x.clone()).wait().unwrap();
            let tx1 = tx.1.send(x.clone()).wait().unwrap();
            Ok((tx0, tx1))
        });
        let _ = core.run(x);
    });

    let _ = thread::spawn(move || {
        let add_fb_rx = add::set_receiver(add_rx);
        let user_fb_rx = user::set_receiver(user_rx);
        let select = add_fb_rx.select(user_fb_rx);
        let _ = tx_return_message.send_all(select.map_err(|_| { panic!() })).wait();
    });

    rx_return_message
}
