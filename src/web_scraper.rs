use std::{fmt::Display, str::FromStr};

use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use thirtyfour::prelude::*;

use crate::models::{Company, Employee, Receipt};

pub struct Scraper {
    pub receipt: Receipt,
    pub comp: Company,
    pub emp: Employee,
}

impl Scraper {
    pub async fn new(url: &str, user_id: i64) -> WebDriverResult<Self> {
        log::info!("Starting WebDriver");

        let caps = DesiredCapabilities::firefox();
        let driver = WebDriver::new("http://localhost:4444", caps).await?;

        log::info!("Started WebDriver");

        driver.get(url).await?;

        driver.find_element(By::Tag("script")).await?;

        while let Err(_) = driver.find_element(By::ClassName("invoice-items")).await {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        log::info!("Page loaded HTML elements");

        let mut receipt = Self::scrape_receipt(&driver).await?;
        receipt.user_id = user_id;
        log::info!("Scraped Receipt");

        let comp = Self::scrape_company(&driver).await?;
        log::info!("Scraped Company");

        let emp = Employee {
            emp_code: receipt.operator_id.clone(),
            comp_id: comp.company_id.clone(),
        };
        log::info!("Scraped Employee");

        driver.quit().await?;

        log::info!("Closed WebDriver");

        Ok(Self { receipt, comp, emp })
    }

    async fn scrape_receipt(driver: &WebDriver) -> WebDriverResult<Receipt> {
        let invoice_header = driver.find_element(By::ClassName("invoice-amount")).await?;
        let invoice_details = driver.find_element(By::ClassName("panel-body")).await?;

        // invoice-header fields
        let value = invoice_header
            .find_element(By::XPath("//h1/strong"))
            .await?
            .html(true)
            .await?;
        let tvsh = invoice_header
            .find_element(By::XPath("//small[2]"))
            .await?
            .find_element(By::Tag("strong"))
            .await?
            .html(true)
            .await?;
        let release_date = invoice_header
            .find_element(By::XPath("//ul/li[3]"))
            .await?
            .html(true)
            .await?;
        let location = Some(format!(
            "{}, {}",
            invoice_header
                .find_element(By::XPath("//ul/li[2]/span[1]"))
                .await?
                .html(true)
                .await?,
            invoice_header
                .find_element(By::XPath("//ul/li[2]/span[3]"))
                .await?
                .html(true)
                .await?
        ));

        // invoice-details fields
        let business_id = invoice_details
            .find_element(By::XPath("//div[@class='form-group form-column'][2]/p"))
            .await?
            .html(true)
            .await?;
        let nslf = invoice_details
            .find_element(By::XPath("//div[3]/p"))
            .await?
            .html(true)
            .await?;
        let nivf = invoice_details
            .find_element(By::XPath("//div[4]/p"))
            .await?
            .html(true)
            .await?;
        let receipt_type = Some(
            invoice_details
                .find_element(By::XPath("//div[5]/p"))
                .await?
                .html(true)
                .await?,
        );
        let operator_id = invoice_details
            .find_element(By::XPath("//div[6]/p"))
            .await?
            .html(true)
            .await?;
        let sw_code = Some(
            invoice_details
                .find_element(By::XPath("//div[7]/p"))
                .await?
                .html(true)
                .await?,
        );
        let payment_deadline: Option<NaiveDateTime> = Some(
            NaiveDate::parse_from_str(
                &invoice_details
                    .find_element(By::XPath("//div[8]/p"))
                    .await?
                    .html(true)
                    .await?,
                "%d/%m/%Y",
            )
            .unwrap()
            .and_hms(0, 0, 0),
        );
        let status = Some(
            invoice_details
                .find_element(By::XPath("//div[9]/div/p"))
                .await?
                .html(true)
                .await?,
        );

        let value: BigDecimal = BigDecimal::from_str(
            value
                .replace("&nbsp;", "")
                .replace(" LEK", "")
                .replace(",", ".")
                .trim(),
        )
        .unwrap();
        let tvsh: Option<BigDecimal> = match BigDecimal::from_str(
            tvsh.replace("&nbsp;", "")
                .replace(" LEK", "")
                .replace(",", ".")
                .trim(),
        ) {
            Ok(o) => Some(o),
            Err(_) => None,
        };
        let release_date = NaiveDateTime::parse_from_str(
            release_date
                .replace("::before", "")
                .replace(r#"""#, "")
                .trim(),
            "%d/%m/%Y %H:%M",
        )
        .unwrap();
        let value_before_tvsh = Some(value.clone() - tvsh.clone().unwrap_or(BigDecimal::default()));

        Ok(Receipt {
            value,
            tvsh,
            release_date,
            value_before_tvsh,
            location,
            business_id,
            nslf,
            nivf,
            receipt_type,
            operator_id,
            sw_code,
            payment_deadline,
            status,
            ..Default::default()
        })
    }

    async fn scrape_company(driver: &WebDriver) -> WebDriverResult<Company> {
        let invoice_details = driver.find_element(By::ClassName("panel-body")).await?;
        let invoice_header = driver.find_element(By::ClassName("invoice-amount")).await?;

        let company_id = invoice_details
            .find_element(By::XPath("//div[@class='form-group form-column'][2]/p"))
            .await?
            .html(true)
            .await?;

        let location = match invoice_details
            .find_element(By::XPath("//div[@class='form-group form-column'][1]/p"))
            .await?
            .html(true)
            .await
        {
            Ok(e) => Some(e),
            Err(_) => None,
        };

        let name = match invoice_header
            .find_element(By::XPath(
                "//ul[@class='invoice-basic-info list-unstyled']/li[1]",
            ))
            .await?
            .html(true)
            .await
        {
            Ok(e) => Some(
                e.replace("::before", "")
                    .replace(r#"""#, "")
                    .trim()
                    .to_owned(),
            ),
            Err(_) => None,
        };

        Ok(Company {
            company_id,
            location,
            name,
        })
    }
}

impl Display for Scraper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r"Scraper {{
    receipt: {},
    comp: {},
    emp: {}
}}",
            self.receipt, self.comp, self.emp
        )
    }
}
