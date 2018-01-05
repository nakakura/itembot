use futures::*;
use futures::sync::mpsc;
use tokio_core::reactor::Core;
use diesel;

use std::thread;
use std::sync::Arc;

use slack_command::SlackCommand;
use models::query::items;
use stream::item::*;

const COMMAND_NAME: &str = "return";

pub fn set_receiver(receiver: mpsc::Receiver<Arc<SlackCommand>>) -> mpsc::Receiver<String> {
    start_item_module_receiver(Params((access_database, filter)), receiver)
}

fn access_database(sender: mpsc::Sender<String>, command: Arc<SlackCommand>) -> Result<mpsc::Sender<String>, ()> {
    println!("return");
    let result = items::return_item(&command.params[0], &command.user).map(|x| {
        println!("return - {}", x);
        if x > 0 { "write success".to_string() }
            else { "item not found".to_string() }
    });
    println!("borrow {:?}", result);
    let message = create_message(result);
    let sender = sender.send(message).wait().unwrap();
    Ok(sender)
}

fn filter(post: &Arc<SlackCommand>) -> bool {
    post.plugin == super::PLUGIN_NAME &&
        (post.command == COMMAND_NAME && post.params.len() > 0)
}

fn create_message(result: Result<String, diesel::result::Error>) -> String {
    use diesel::result::Error;
    use diesel::result::DatabaseErrorKind;

    match result {
        Ok(x) => x,
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => "unique violation".to_string(),
        _ => "something happen".to_string()
    }
}
