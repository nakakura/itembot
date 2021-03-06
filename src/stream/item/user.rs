use futures::*;
use futures::sync::mpsc;
use tokio_core::reactor::Core;

use std::thread;
use std::sync::Arc;

use models::record::items::Item;
use slack_command::SlackCommand;
use models::query::items;
use stream::item::*;

const COMMAND_NAME: &str = "user";

pub fn set_receiver(receiver: mpsc::Receiver<Arc<SlackCommand>>) -> mpsc::Receiver<String> {
    start_item_module_receiver(Params((access_database, filter)), receiver)
}

fn access_database(sender: mpsc::Sender<String>, command: Arc<SlackCommand>) -> Result<mpsc::Sender<String>, ()> {
    let result = items::list_borrow_items(&command.params[0]);
    let message = create_message(result);
    let sender = sender.send(message).wait().unwrap();
    Ok(sender)
}

fn create_message<E>(result: Result<Vec<Item>, E>) -> String {
    if let Ok(items) = result {
        if items.len() > 0 {
             items.into_iter().fold("あなたはこれだけ借りているよ".to_string(), |sum, x| {
                format!("{}\n{}", sum, x.title)
            })
        } else {
            "なにもかりていないよ".to_string()
        }
    } else{
        "something happened".to_string()
    }
}

fn filter(post: &Arc<SlackCommand>) -> bool {
    post.plugin == super::PLUGIN_NAME &&
        (post.command == COMMAND_NAME && post.params.len() > 0)
}