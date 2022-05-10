use diesel::{MysqlConnection, Connection, prelude::*, sql_types::{BigInt, Bool}};

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

    pub fn get_user(&self, id: i64) -> QueryResult<Vec<User>> {
        user::table.filter(user::user_id.eq(id))
                   .load::<User>(&self.connection)
    }

    pub fn insert_user(&self, id: i64, is_admin: bool) -> Result<usize, diesel::result::Error> {
        diesel::sql_query("CALL sp_insertUser(?, ?);")
                .bind::<BigInt, _>(id)
                .bind::<Bool, _>(is_admin)
                .execute(&self.connection)
    }
}