use futures::*;
use futures::sync::mpsc;
use tokio_core::reactor::Core;
use diesel;

use std::thread;
use std::sync::Arc;

use slack_command::SlackCommand;
use models::query::items;
use models::record::items::Item;
use stream::item::*;

const COMMAND_NAME: &str = "delete";

#[derive(Debug, Clone, PartialOrd, PartialEq)]
enum DeleteStatusEnum {
    NONE,
    MULTIPLE
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct DeleteStatus {
    pub status: DeleteStatusEnum,
    items: Vec<Item>,
}

impl DeleteStatus {
    pub fn default() -> Self {
        DeleteStatus {
            status: DeleteStatusEnum::NONE,
            items: vec!()
        }
    }

    pub fn specify_index<'a>(&'a self, index: usize) -> Option<&'a Item> {
        if self.status == DeleteStatusEnum::NONE { return None }
        self.items.get(index)
    }
}

pub fn set_receiver(receiver: mpsc::Receiver<Arc<SlackCommand>>) -> mpsc::Receiver<String> {
    start_item_module_receiver(Params((adapter(), filter)), receiver)
}

fn access_database(state: (mpsc::Sender<String>, &DeleteStatus), command: Arc<SlackCommand>) -> Result<(mpsc::Sender<String>, DeleteStatus), ()> {
    match command.command.as_str() {
        "number" => num_command(state, command),
        COMMAND_NAME => delete_command(state, command),
        _ => Ok((state.0, DeleteStatus::default())),
    }
}

fn filter(post: &Arc<SlackCommand>) -> bool {
    true
}

fn adapter() -> impl FnMut(mpsc::Sender<String>, Arc<SlackCommand>) -> Result<mpsc::Sender<String>, ()> {
    let mut state = DeleteStatus::default();
    move |sender, command| {
        let result = access_database((sender, &state), command)?;
        state = result.1;
        Ok(result.0)
    }
}

fn delete_item(item: &Item, state: (mpsc::Sender<String>, &DeleteStatus)) -> Result<(mpsc::Sender<String>, DeleteStatus), ()> {
    if item.borrower == "" {
        let len = items::delete(&item.title).map_err(|_| ())?;
        if len == 1 {
            let message = format!("delete {}", item.title);
            let sender = state.0.send(message).wait().unwrap();
            Ok((sender, DeleteStatus::default()))
        } else {
            let message = format!("何かしらの理由で借りられなかったよ");
            let sender = state.0.send(message).wait().unwrap();
            Ok((sender, DeleteStatus::default()))
        }
    } else {
        let message = format!("{}は{}さんが借りているよ", item.title, item.borrower);
        let sender = state.0.send(message).wait().unwrap();
        Ok((sender, state.1.clone()))
    }
}

fn num_command(state: (mpsc::Sender<String>, &DeleteStatus), command: Arc<SlackCommand>) -> Result<(mpsc::Sender<String>, DeleteStatus), ()> {
    if state.1.status == DeleteStatusEnum::NONE {
        return Ok((state.0, DeleteStatus::default()));
    }

    let item_opt = state.1.specify_index(command.number.unwrap());
    if let Some(ref item) = item_opt {
        delete_item(item, state)
    } else {
        let sender = state.0.send("番号指定がおかしいよ".to_string()).wait().unwrap();
        Ok((sender, state.1.clone()))
    }
}

fn delete_command(state: (mpsc::Sender<String>, &DeleteStatus), command: Arc<SlackCommand>) -> Result<(mpsc::Sender<String>, DeleteStatus), ()> {
    let items = items::search_items(&command.params[0]).map_err(|_| ())?;
    match items.len() {
        1 => {
            delete_item(&items[0], state)
        },
        _x if _x > 1 => {
            let message = items.clone().into_iter().fold("複数ひっかかったよ".to_string(), |sum, x| {
                if x.borrower == "" {
                    format!("{}\n{}", sum, x.title)
                } else {
                    format!("{}\n{} ({}さんが借りているよ)", sum, x.title, x.borrower)
                }
            });
            let sender = state.0.send(message).wait().unwrap();
            Ok((sender, DeleteStatus {
                status: DeleteStatusEnum::MULTIPLE,
                items: items,
            }))
        },
        _ => {
            let sender = state.0.send("そんなものなかったよ".to_string()).wait().unwrap();
            Ok((sender, DeleteStatus::default()))
        },
    }
}

