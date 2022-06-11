use diesel::{
    prelude::*,
    sql_types::{BigInt, Bool},
    Connection, MysqlConnection,
};

use crate::{
    models::Company,
    models::{Employee, Receipt, User},
    schema::*,
};

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

    pub fn insert_user(&self, id: i64, is_admin: bool) -> QueryResult<usize> {
        diesel::sql_query("CALL sp_insertUser(?, ?);")
            .bind::<BigInt, _>(id)
            .bind::<Bool, _>(is_admin)
            .execute(&self.connection)
    }

    pub fn has_business(&self, id: &str) -> QueryResult<bool> {
        let res = company::table
            .filter(company::columns::company_id.eq(id))
            .limit(1)
            .load::<Company>(&self.connection)?;

        Ok(match res.len() {
            1 => true,
            _ => false,
        })
    }

    pub fn insert_business(&self, comp: Company) -> QueryResult<usize> {
        use crate::schema::company::dsl::*;

        diesel::insert_into(company)
            .values((
                company_id.eq(comp.company_id),
                location.eq(comp.location),
                name.eq(comp.name),
            ))
            .execute(&self.connection)
    }

    pub fn has_employee(&self, id: &str) -> QueryResult<bool> {
        let res = employee::table
            .filter(employee::columns::emp_code.eq(id))
            .limit(1)
            .load::<Employee>(&self.connection)?;

        Ok(match res.len() {
            1 => true,
            _ => false,
        })
    }

    pub fn insert_employee(&self, emp: Employee) -> QueryResult<usize> {
        use crate::schema::employee::dsl::*;

        diesel::insert_into(employee)
            .values((emp_code.eq(emp.emp_code), comp_id.eq(emp.comp_id)))
            .execute(&self.connection)
    }

    pub fn has_receipt(&self, rec: Receipt) -> QueryResult<bool> {
        let res = receipt::table
            .filter(receipt::columns::nivf.eq(rec.nivf))
            .limit(1)
            .load::<Receipt>(&self.connection)?;

        Ok(match res.len() {
            1 => true,
            _ => false,
        })
    }

    pub fn insert_receipt(&self, rec: Receipt) -> QueryResult<usize> {
        use crate::schema::receipt::dsl::*;

        diesel::insert_into(receipt)
            .values((
                nslf.eq(rec.nslf),
                nivf.eq(rec.nivf),
                value_before_tvsh.eq(rec.value_before_tvsh),
                tvsh.eq(rec.tvsh),
                value.eq(rec.value),
                location.eq(rec.location),
                release_date.eq(rec.release_date),
                receipt_type.eq(rec.receipt_type),
                sw_code.eq(rec.sw_code),
                payment_deadline.eq(rec.payment_deadline),
                status.eq(rec.status),
                business_id.eq(rec.business_id),
                operator_id.eq(rec.operator_id),
                user_id.eq(rec.user_id),
            ))
            .execute(&self.connection)
    }
}
