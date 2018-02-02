use diesel;
use diesel::prelude::*;
use schema::rockys;
use schema::rockys::dsl::*;
use chrono::prelude::*;

use database_connection;
use models::record::rockys::*;

pub fn insert(word_str: &str) -> QueryResult<usize> {
    let new_rocky = NewRocky {
        word: word_str,
    };

    database_connection::connection(|connection| {
        diesel::insert(&new_rocky).into(rockys::table)
            .execute(connection)
    })
}



