use diesel;
use diesel::prelude::*;
use schema::items;
use schema::items::dsl::*;
use chrono::prelude::*;

use database_connection;
use models::record::items::*;

pub fn insert_borrower(title_str: &str, owner_str: &str, borrower_opt: Option<String>) -> QueryResult<usize> {
    let native: NaiveDateTime = Local::now().naive_utc();

    let new_post = NewItem {
        title: title_str,
        owner: owner_str,
        registered_date: Some(native),
        due_date: None,
        borrower: borrower_opt
    };

    database_connection::connection(|connection| {
        diesel::insert(&new_post).into(items::table)
            .execute(connection)
    })
}

pub fn insert(title_str: &str, owner_str: &str) -> QueryResult<usize> {
    insert_borrower(title_str, owner_str, None)
}

pub fn select(item: &str) -> QueryResult<Vec<Item>> {
    database_connection::connection(|connection| {
        items.filter(title.like(format!("%{}%", item))).load::<Item>(connection)
    })
}

pub fn delete(item: &str) -> QueryResult<usize> {
    database_connection::connection((|connection| {
        diesel::delete(items.filter(title.eq(item))).execute(connection)
    }))
}

pub fn update(item: &str, borrower_str: &str) -> QueryResult<usize> {
     database_connection::connection((|connection| {
        diesel::update(items.filter(title.eq(item)))
            .set(borrower.eq(borrower_str))
            .execute(connection)
    }))
}

