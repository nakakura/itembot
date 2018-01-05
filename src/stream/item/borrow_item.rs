use futures::*;
use futures::sync::mpsc;
use tokio_core::reactor::Core;

use std::thread;
use std::sync::Arc;

use slack_command::SlackCommand;
use models::query::items;
use models::record::items::Item;

const COMMAND_NAME: &str = "borrow";

#[derive(Debug, Clone, PartialOrd, PartialEq)]
enum BorrowStatusEnum {
    NONE,
    MULTIPLE
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct BorrowStatus {
    status: BorrowStatusEnum,
    items: Vec<Item>,
}

impl BorrowStatus {
    pub fn default() -> Self {
        BorrowStatus {
            status: BorrowStatusEnum::NONE,
            items: vec!()
        }
    }

    pub fn specify_index<'a>(&'a self, index: usize) -> Option<&'a Item> {
        if self.status == BorrowStatusEnum::NONE { return None }
        self.items.get(index)
    }
}

pub fn set_receiver(receiver: mpsc::Receiver<Arc<SlackCommand>>) -> mpsc::Receiver<String> {
    let (tx_return_message, rx_return_message) = mpsc::channel::<String>(5000);
    let _ = thread::spawn(move || {
        let mut core = Core::new().unwrap();
        let init_status = BorrowStatus::default();
        let x = receiver.filter(filter).fold((tx_return_message, init_status), access_database);
        let _ = core.run(x);
    });
    rx_return_message
}

fn borrow_item(item: &Item, state: mpsc::Sender<String>, borrower: &str) -> Result<(mpsc::Sender<String>, BorrowStatus), ()> {
    if item.borrower == "" {
        let len = items::borrow_item(&item.title, borrower).map_err(|_| ())?;
        if len == 1 {
            let message = format!("{}を借りたよ", item.title);
            let sender = state.send(message).wait().unwrap();
            Ok((sender, BorrowStatus::default()))
        } else {
            let message = format!("何かしらの理由で借りられなかったよ");
            let sender = state.send(message).wait().unwrap();
            Ok((sender, BorrowStatus::default()))
        }
    } else {
        let message = format!("{}は{}さんが借りているよ", item.title, item.borrower);
        let sender = state.send(message).wait().unwrap();
        Ok((sender, BorrowStatus::default()))
    }
}

fn num_command(state: (mpsc::Sender<String>, BorrowStatus), command: Arc<SlackCommand>) -> Result<(mpsc::Sender<String>, BorrowStatus), ()> {
    let item_opt = state.1.specify_index(command.number.unwrap());
    if let Some(ref item) = item_opt {
        borrow_item(item, state.0, &command.user)
    } else {
        let sender = state.0.send("番号指定がおかしいよ".to_string()).wait().unwrap();
        Ok((sender, state.1.clone()))
    }
}

fn borrow_command(state: (mpsc::Sender<String>, BorrowStatus), command: Arc<SlackCommand>) -> Result<(mpsc::Sender<String>, BorrowStatus), ()> {
    let items = items::search_items(&command.params[0]).map_err(|_| ())?;
    match items.len() {
        1 => {
            borrow_item(&items[0], state.0, &command.user)
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
            Ok((sender, BorrowStatus {
                status: BorrowStatusEnum::MULTIPLE,
                items: items,
            }))
        },
        _ => {
            let sender = state.0.send("そんなものなかったよ".to_string()).wait().unwrap();
            Ok((sender, BorrowStatus::default()))
        },
    }
}

fn access_database(state: (mpsc::Sender<String>, BorrowStatus), command: Arc<SlackCommand>) -> Result<(mpsc::Sender<String>, BorrowStatus), ()> {
    match command.command.as_str() {
        "number" => num_command(state, command),
        COMMAND_NAME => borrow_command(state, command),
        _ => Ok((state.0, BorrowStatus::default())),
    }
}

fn filter(_post: &Arc<SlackCommand>) -> bool {
    true
}

