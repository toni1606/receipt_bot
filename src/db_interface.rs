use diesel::{MysqlConnection, Connection};

pub fn establish_connection(url: &str) -> MysqlConnection {
    MysqlConnection::establish(&url).expect("Could not connect to database")
}