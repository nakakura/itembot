use futures::*;
use futures::sync::mpsc;
use tokio_core::reactor::Core;
use diesel;

use std::thread;
use std::sync::Arc;

use slack_command::SlackCommand;
use models::query::items;

const COMMAND_NAME: &str = "delete";

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
    let result = items::delete(&command.params[0]).map(|len| {
        if len > 0 { "write success".to_string() }
        else { "not found".to_string() }
    });
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