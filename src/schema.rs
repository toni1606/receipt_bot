table! {
    company (company_id) {
        company_id -> Varchar,
        location -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
    }
}

table! {
    employee (emp_code) {
        emp_code -> Varchar,
        comp_id -> Varchar,
    }
}

table! {
    receipt (nslf) {
        nslf -> Varchar,
        nivf -> Varchar,
        value_before_tvsh -> Nullable<Decimal>,
        tvsh -> Nullable<Decimal>,
        value -> Decimal,
        location -> Nullable<Varchar>,
        release_date -> Datetime,
        receipt_type -> Nullable<Varchar>,
        sw_code -> Nullable<Varchar>,
        payment_deadline -> Nullable<Datetime>,
        status -> Nullable<Varchar>,
        business_id -> Varchar,
        operator_id -> Varchar,
        user_id -> Bigint,
    }
}

table! {
    user (user_id) {
        user_id -> Bigint,
        registered_on -> Nullable<Datetime>,
        last_upload -> Nullable<Datetime>,
        is_admin -> Nullable<Bool>,
    }
}

joinable!(employee -> company (comp_id));
joinable!(receipt -> company (business_id));
joinable!(receipt -> employee (operator_id));
joinable!(receipt -> user (user_id));

allow_tables_to_appear_in_same_query!(
    company,
    employee,
    receipt,
    user,
);
