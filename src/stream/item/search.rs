use futures::*;
use futures::sync::mpsc;
use tokio_core::reactor::Core;

use std::thread;
use std::sync::Arc;

use models::record::items::Item;
use slack_command::SlackCommand;
use models::query::items;

const COMMAND_NAME: &str = "search";

pub fn set_receiver(receiver: mpsc::Receiver<Arc<SlackCommand>>) -> mpsc::Receiver<String> {
    let (tx_return_message, rx_return_message) = mpsc::channel::<String>(5000);
    let _ = thread::spawn(move || {
        let mut core = Core::new().unwrap();
        let x = receiver.filter(filter).fold(tx_return_message, access_database);
        let _ = core.run(x);
    });
    rx_return_message
}

fn access_database(sender: mpsc::Sender<String>, command: Arc<SlackCommand>) -> Result<mpsc::Sender<String>, ()> {
    println!("{:?}", command);
    let result = items::search_items(&command.params[0]);
    let message = create_message(result);
    let sender = sender.send(message).wait().unwrap();
    Ok(sender)
}

fn create_message<E>(result: Result<Vec<Item>, E>) -> String {
    if let Ok(items) = result {
        if items.len() > 0 {
            items.into_iter().fold("似たタイトルのアイテムはこれだけだよ".to_string(), |sum, x| {
                if let Some(borrower) = x.borrower {
                    format!("{}\n{} ({}さんが借りているよ)", sum, x.title, borrower)
                } else {
                    format!("{}\n{}", sum, x.title)
                }
            })
        } else {
            "そんな名前のものはないよ".to_string()
        }
    } else{
        "something happened".to_string()
    }
}

fn filter(post: &Arc<SlackCommand>) -> bool {
    post.plugin == super::PLUGIN_NAME &&
        (post.command == COMMAND_NAME && post.params.len() > 0)
}