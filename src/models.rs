// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use std::fmt::Display;

use chrono::NaiveDateTime;
#[derive(Queryable, Debug)]
// #[primary_key(company_id)]
// #[table_name = "Company"]
pub struct Company {
    pub company_id: String,
    pub location: Option<String>,
    pub name: Option<String>,
}

#[derive(Queryable, Debug)]
// #[primary_key(emp_code)]
// #[table_name = "Employee"]
pub struct Employee {
    pub emp_code: String,
    pub comp_id: String,
}

#[derive(Queryable, Debug)]
// #[primary_key(nslf)]
// #[table_name = "Receipt"]
pub struct Receipt {
    pub nslf: String,
    pub nivf: String,
    pub value_before_tvsh: Option<f64>,
    pub tvsh: Option<f64>,
    pub value: f64,
    pub location: Option<String>,
    pub release_date: NaiveDateTime,
    pub receipt_type: Option<String>,
    pub sw_code: Option<String>,
    pub payment_deadline: Option<NaiveDateTime>,
    pub status: Option<String>,
    pub business_id: String,
    pub operator_id: String,
    pub user_id: i64,
}

#[derive(Queryable, Debug)]
// #[primary_key(user_id)]
// #[table_name = "User"]
pub struct User {
    pub user_id: i64,
    pub registered_on: Option<NaiveDateTime>,
    pub last_upload: Option<NaiveDateTime>,
    pub is_admin: Option<bool>,
}

impl Default for Receipt {
    fn default() -> Self {
        Receipt {
            nslf: "".to_string(),
            nivf: "".to_string(),
            value_before_tvsh: Some(0.0),
            tvsh: Some(0.0),
            value: 0.0,
            location: Some("".to_string()),
            release_date: NaiveDateTime::from_timestamp(0, 0),
            receipt_type: Some("".to_string()),
            sw_code: Some("".to_string()),
            payment_deadline: Some(NaiveDateTime::from_timestamp(0, 0)),
            status: Some("".to_string()),
            business_id: "".to_string(),
            operator_id: "".to_string(),
            user_id: 0,
        }
    }
}

impl Display for Receipt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r"Receipt {{
    nslf: {},
    nivf: {},
    value_before_tvsh: {:?},
    tvsh: {:?},
    value: {},
    location: {:?},
    release_date: {},
    receipt_type: {:?},
    sw_code: {:?},
    payment_deadline: {:?},
    status: {:?},
    business_id: {}.
    operator_id: {},
    user_id: {}
}}",
            self.nslf,
            self.nivf,
            self.value_before_tvsh,
            self.tvsh,
            self.value,
            self.location,
            self.release_date,
            self.receipt_type,
            self.sw_code,
            self.payment_deadline,
            self.status,
            self.business_id,
            self.operator_id,
            self.user_id
        )
    }
}
