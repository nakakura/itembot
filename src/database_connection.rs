use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use std::env;
use std::sync::Mutex;

lazy_static! {
  static ref CONNECTION: Mutex<MysqlConnection> = {
    Mutex::new(establish_connection())
  };
}

fn db_uri() -> String {
    env::var("DATABASE_URL").ok().unwrap_or("mysql://root:mysql@127.0.0.1:3306/mono".to_string())
}

fn establish_connection() -> MysqlConnection {
    let uri = db_uri();
    MysqlConnection::establish(&uri)
        .expect(&format!("Error connecting to {}", uri))
}

pub fn connection<F, T>(closure: F) -> QueryResult<T>
    where F: Fn(&MysqlConnection) -> QueryResult<T> {
    closure(&*CONNECTION.lock().unwrap())
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::db_uri;
    use super::connection;

    #[test]
    fn test_db_url_without_env() {
        env::remove_var("DATABASE_URL");
        assert_eq!(db_uri(), "mysql://root:mysql@127.0.0.1:3306/mono");
    }

    #[test]
    fn test_db_url_with_env() {
        env::set_var("DATABASE_URL", "somewhere");
        assert_eq!(db_uri(), "somewhere");
    }

    /*
    #[test]
    fn test_connection() {
        let x = connection(|_r| {
            Ok(0)
        });

        assert_eq!(x, Ok(0));
    }
    */
}
