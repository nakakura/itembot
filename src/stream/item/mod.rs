pub mod add;
pub mod user;
pub mod borrow_item;
pub mod return_item;
pub mod delete;
pub mod search;

use futures::*;
use futures::sync::mpsc;
use tokio_core::reactor::Core;

use std::thread;
use std::sync::Arc;

use slack_command::SlackCommand;

pub const PLUGIN_NAME: &str = "item";

pub type AccessDbType = FnMut(mpsc::Sender<String>, Arc<SlackCommand>) -> Result<mpsc::Sender<String>, ()>;

pub struct Params<F>((F, fn(&Arc<SlackCommand>)->bool))
    where F: 'static + Send + FnMut(mpsc::Sender<String>, Arc<SlackCommand>) -> Result<mpsc::Sender<String>, ()>;

pub fn start_item_module_receiver<F>(f: Params<F>, receiver: mpsc::Receiver<Arc<SlackCommand>>) -> mpsc::Receiver<String>
    where F: 'static + Send + FnMut(mpsc::Sender<String>, Arc<SlackCommand>) -> Result<mpsc::Sender<String>, ()> {
    let (tx_return_message, rx_return_message) = mpsc::channel::<String>(5000);
    let Params(f) = f;
    let _ = thread::spawn(move || {
        let mut core = Core::new().unwrap();
        let x = receiver.filter(f.1).fold(tx_return_message, f.0);
        let _ = core.run(x);
    });
    rx_return_message
}

pub fn set_receiver(receiver: mpsc::Receiver<Arc<SlackCommand>>) -> mpsc::Receiver<String> {
    let (tx_return_message, rx_return_message) = mpsc::channel::<String>(5000);
    let (add_tx, add_rx) = mpsc::channel::<Arc<SlackCommand>>(5000);
    let (user_tx, user_rx) = mpsc::channel::<Arc<SlackCommand>>(5000);
    let (borrow_tx, borrow_rx) = mpsc::channel::<Arc<SlackCommand>>(5000);
    let (return_tx, return_rx) = mpsc::channel::<Arc<SlackCommand>>(5000);
    let (delete_tx, delete_rx) = mpsc::channel::<Arc<SlackCommand>>(5000);
    let (search_tx, search_rx) = mpsc::channel::<Arc<SlackCommand>>(5000);

    let _ = thread::spawn(move || {
        let mut core = Core::new().unwrap();
        let senders = vec!(add_tx, user_tx, borrow_tx, return_tx, delete_tx, search_tx);
        let x = receiver.fold(senders, |tx, x| {
            Ok(tx.into_iter().map(|sender| sender.send(x.clone()).wait().unwrap() ).collect())
        });
        let _ = core.run(x);
    });

    let _ = thread::spawn(move || {
        let select = add::set_receiver(add_rx)
            .select(user::set_receiver(user_rx))
            .select(borrow_item::set_receiver(borrow_rx))
            .select(return_item::set_receiver(return_rx))
            .select(search::set_receiver(search_rx))
            .select(delete::set_receiver(delete_rx));
        let _ = tx_return_message.send_all(select.map_err(|_| { panic!() })).wait();
    });

    rx_return_message
}
