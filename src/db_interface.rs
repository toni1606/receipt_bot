use diesel::{MysqlConnection, Connection, prelude::*};

use crate::{models::User, schema::*};

pub struct Database {
    connection: MysqlConnection
}

impl Database {
    pub fn connect(url: &str) -> Result<Self, ConnectionError> {
        match MysqlConnection::establish(&url) {
            Ok(con) => Ok(Database { connection: con }),
            Err(err) => Err(err)
        }
    }

    pub fn get_users(&self) -> QueryResult<Vec<User>> {
        user::table.load::<User>(&self.connection)
    }

    pub fn get_user(&self, id: i64) -> User {
        unimplemented!()
    }
}