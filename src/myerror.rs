use diesel;

use std::fmt;
use std::error;

#[derive(Debug)]
pub enum MyError {
    Diesel(diesel::result::Error),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // 下層のエラーは両方ともすでに `Display` を実装しているので、
            // それらの実装に従います。
            MyError::Diesel(ref err) => write!(f, "Diesel error: {}", err),
        }
    }
}

impl error::Error for MyError {
    fn description(&self) -> &str {
        // 下層のエラーは両方ともすでに `Error` を実装しているので、
        // それらの実装に従います。
        match *self {
            MyError::Diesel(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // 注意：これらは両方とも `err` を、その具象型（`&io::Error` か
            // `&num::ParseIntError` のいずれか）から、トレイトオブジェクト
            // `&Error` へ暗黙的にキャストします。どちらのエラー型も `Error` を
            // 実装しているので、問題なく動きます。
            MyError::Diesel(ref err) => Some(err),
        }
    }
}