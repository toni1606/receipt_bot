use diesel::{
    prelude::*,
    sql_types::{BigInt, Bool},
    Connection, MysqlConnection,
};

use crate::{models::User, schema::*, models::Company};

pub struct Database {
    connection: MysqlConnection,
}

impl Database {
    pub fn connect(url: &str) -> Result<Self, ConnectionError> {
        match MysqlConnection::establish(&url) {
            Ok(con) => Ok(Database { connection: con }),
            Err(err) => Err(err),
        }
    }

    pub fn get_users(&self) -> QueryResult<Vec<User>> {
        user::table.load::<User>(&self.connection)
    }

    pub fn get_user(&self, id: i64) -> QueryResult<Vec<User>> {
        user::table
            .filter(user::user_id.eq(id))
            .load::<User>(&self.connection)
    }

    pub fn insert_user(&self, id: i64, is_admin: bool) -> Result<usize, diesel::result::Error> {
        diesel::sql_query("CALL sp_insertUser(?, ?);")
            .bind::<BigInt, _>(id)
            .bind::<Bool, _>(is_admin)
            .execute(&self.connection)
    }

    pub fn has_business(&self, id: i64) -> Result<bool, diesel::result::Error> {
        let res = company::table.limit(1).load::<Company>(&self.connection)?;

        Ok(match res.len() {
            1 => true,
            _ => false
        })
    }
}
