use diesel::{MysqlConnection, Connection, prelude::*};

use crate::{models::User, schema::*};

pub fn establish_connection(url: &str) -> Result<MysqlConnection, ConnectionError> {
    MysqlConnection::establish(&url)
}

pub fn get_users(con: &MysqlConnection) -> QueryResult<Vec<User>> {
    user::table.load::<User>(con)
}