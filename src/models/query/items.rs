use diesel;
use diesel::prelude::*;
use schema::items;
use schema::items::dsl::*;
use chrono::prelude::*;

use database_connection;
use models::record::items::*;

pub fn insert_borrower(title_str: &str, owner_str: &str, borrower_str: &str) -> QueryResult<usize> {
    let native: NaiveDateTime = Local::now().naive_utc();

    let new_post = NewItem {
        title: title_str,
        owner: owner_str,
        borrower: borrower_str,
        registered_date: Some(native),
        due_date: None,
    };

    database_connection::connection(|connection| {
        diesel::insert(&new_post).into(items::table)
            .execute(connection)
    })
}

pub fn insert(title_str: &str, owner_str: &str) -> QueryResult<usize> {
    insert_borrower(title_str, owner_str, "")
}

pub fn search_items(item: &str) -> QueryResult<Vec<Item>> {
    database_connection::connection(|connection| {
        items.filter(title.like(format!("%{}%", item))).load::<Item>(connection)
    })
}

pub fn list_borrow_items(name: &str) -> QueryResult<Vec<Item>> {
    database_connection::connection(|connection| {
        items.filter(borrower.eq(name)).load::<Item>(connection)
    })
}

pub fn delete(item: &str) -> QueryResult<usize> {
    database_connection::connection((|connection| {
        diesel::delete(items.filter(title.eq(item))).execute(connection)
    }))
}

pub fn borrow_item(item: &str, borrower_str: &str) -> QueryResult<usize> {
    println!("{} use {}", borrower_str, item);
     database_connection::connection((|connection| {
        diesel::update(items.filter(title.eq(item)))
            .set(borrower.eq(borrower_str))
            .execute(connection)
    }))
}

pub fn return_item(item: &str, borrower_str: &str) -> QueryResult<usize> {
    println!("{} release {}", borrower_str, item);
    database_connection::connection((|connection| {
        diesel::update(items.filter(title.eq(item)).filter(borrower.eq(borrower_str)))
            .set(borrower.eq(""))
            .execute(connection)
    }))
}

