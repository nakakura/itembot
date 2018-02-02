use schema::rockys;
use chrono::naive::NaiveDateTime;

#[derive(Queryable, Debug, Clone, PartialOrd, PartialEq)]
pub struct Rocky {
    // https://github.com/diesel-rs/dieselt sta/issues/80
    pub word: String,
}

#[derive(Insertable, Debug, Clone, PartialOrd, PartialEq)]
#[table_name="rockys"]
pub struct NewRocky<'a> {
    pub word: &'a str,
}
