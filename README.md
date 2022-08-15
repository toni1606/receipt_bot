# Receipt Bot

A project to create a telegram bot which can read the albanian receipt QR codes.
These codes point to a dynamic website which needs to be scraped using
(WebDriver) and got the datapoints which are then stored in a database. The
whole thing is written in Rust.

# Dependencies
To set up the project you need either [`geckodriver`](https://github.com/mozilla/geckodriver/releases) or the
[`chromedriver`](https://chromedriver.chromium.org/), which needs be run before
starting the app.
It also needs a MYSQL database.

# Building

Like other Rust project, you only need to run:
- `cargo b`           -> for debug
- `cargo b --release` -> for release

# SQL Code

The SQL code can be run to create the needed database (flavour is MySQL)

```mysql
DROP SCHEMA IF EXISTS receipt;
CREATE SCHEMA receipt;
USE receipt;

CREATE TABLE user (
    user_id         BIGINT NOT NULL PRIMARY KEY ,
    registered_on   DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_upload     DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    is_admin        BOOL DEFAULT FALSE
);

CREATE TABLE company (
    company_id  VARCHAR(16) NOT NULL PRIMARY KEY,
    location    VARCHAR(16),
    name        VARCHAR(64)
);

CREATE TABLE employee (
    emp_code    VARCHAR(16) NOT NULL PRIMARY KEY,
    comp_id     VARCHAR(16) NOT NULL,
    CONSTRAINT fKe1 FOREIGN KEY (comp_id) REFERENCES company(company_id)
);

CREATE TABLE receipt (
    nslf                VARCHAR(40) NOT NULL PRIMARY KEY,
    nivf                VARCHAR(40) NOT NULL,
    value_before_tvsh   DECIMAL(9, 2),
    tvsh                DECIMAL(10, 2),
    value               DECIMAL(9, 2) NOT NULL,
    location            VARCHAR(64),
    release_date        DATETIME NOT NULL,
    receipt_type        VARCHAR(64),
    sw_code             VARCHAR(64),
    payment_deadline    DATETIME,
    status              VARCHAR(64),
    business_id         VARCHAR(16) NOT NULL,
    operator_id         VARCHAR(16) NOT NULL,
    user_id             BIGINT NOT NULL,
    CONSTRAINT fKr1 FOREIGN KEY (business_id) REFERENCES company(company_id),
    CONSTRAINT fKr2 FOREIGN KEY (operator_id) REFERENCES employee(emp_code),
    CONSTRAINT fKr3 FOREIGN KEY (user_id) REFERENCES user(user_id)
);

DELIMITER //
CREATE PROCEDURE sp_updateLastUpload(p_user BIGINT)
BEGIN
    UPDATE user
    SET last_upload = CURRENT_TIMESTAMP
    WHERE user_id = p_user;
END//

CREATE PROCEDURE sp_instertReceipt (
    p_nslf                VARCHAR(40),
    p_nivf                VARCHAR(40),
    p_value_before_tvsh   DECIMAL(9, 2),
    p_tvsh                DECIMAL(10, 2),
    p_value               DECIMAL(9, 2),
    p_location            VARCHAR(64),
    p_release_date        DATETIME,
    p_receipt_type        VARCHAR(64),
    p_sw_code             VARCHAR(64),
    p_payment_deadline    DATETIME,
    p_status              VARCHAR(64),
    p_business_id         VARCHAR(16),
    p_operator_id         VARCHAR(16),
    p_user_id             BIGINT
)
BEGIN
    INSERT INTO receipt (nslf, nivf, value_before_tvsh, tvsh, value, location, release_date, receipt_type, sw_code, payment_deadline, status, business_id, operator_id, user_id)
    VALUES (p_nslf, p_nivf, p_value_before_tvsh, p_tvsh, p_value, p_location, p_release_date, p_receipt_type, p_sw_code, p_payment_deadline, p_status, p_business_id, p_operator_id, p_user_id);
END //

CREATE PROCEDURE sp_insertEmployee (
    p_emp_code    VARCHAR(16),
    p_comp_id     VARCHAR(16)
)
BEGIN
    INSERT INTO employee (emp_code, comp_id)
    VALUES (p_emp_code, p_comp_id);
END //

CREATE PROCEDURE sp_insertCompany (
    p_company_id  VARCHAR(16),
    p_location    VARCHAR(16),
    p_name        VARCHAR(64)
)
BEGIN
    INSERT INTO company (company_id, location, name)
    VALUES (p_company_id, p_location, p_name);
END //

CREATE PROCEDURE sp_insertUser (
    p_user_id         BIGINT,
    p_is_admin        BOOL
)
BEGIN
    INSERT INTO user (user_id, is_admin)
    VALUES (p_user_id, p_is_admin);
END //

CREATE PROCEDURE sp_getUserData (
    p_user_id   BIGINT
)
BEGIN
    SELECT *
    FROM vw_everything
    WHERE user_id = p_user_id;
END //

DELIMITER ;

DROP VIEW IF EXISTS vw_everything;
CREATE OR REPLACE VIEW vw_everything AS
SELECT
    r.nslf,
    r.nivf,
    r.value_before_tvsh,
    r.tvsh,
    r.value,
    r.location,
    r.release_date,
    r.receipt_type,
    r.sw_code,
    r.payment_deadline,
    r.status,
    r.business_id,
    r.operator_id,
    r.user_id,
    c.name,
    c.location as business_location
FROM receipt r
JOIN company c ON c.company_id = r.business_id;
```
