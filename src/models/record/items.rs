use schema::items;
use chrono::naive::NaiveDateTime;

#[derive(Queryable, Debug, Clone, PartialOrd, PartialEq)]
pub struct Item {
    // https://github.com/diesel-rs/dieselt sta/issues/80
    pub id: i64, //diesen can only handle i64 not u64;
    pub title: String,
    pub owner: String,
    pub borrower: Option<String>,
    pub registered_date: Option<NaiveDateTime>,
    pub due_date: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Clone, PartialOrd, PartialEq)]
#[table_name="items"]
pub struct NewItem<'a> {
    pub title: &'a str,
    pub owner: &'a str,
    pub borrower: Option<String>,
    pub registered_date: Option<NaiveDateTime>,
    pub due_date: Option<NaiveDateTime>,
}

