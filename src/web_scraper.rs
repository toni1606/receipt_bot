use std::str::FromStr;

use chrono::NaiveDateTime;
use thirtyfour::prelude::*;

use crate::models::Receipt;

pub async fn get_html(url: &str) -> WebDriverResult<Receipt> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    driver.get(url).await?;

    driver.find_element(By::Tag("script")).await?;

    println!("{}", driver.title().await?);

    while let Err(_) = driver.find_element(By::ClassName("invoice-items")).await {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    
    let ret = get_receipt_from_url(&driver).await?;

    driver.quit().await?;

    Ok(ret)
}

async fn get_receipt_from_url(driver: &WebDriver) -> WebDriverResult<Receipt> {
    let invoice_header = driver
        .find_element(By::ClassName("invoice-amount"))
        .await?;
    let value = invoice_header.find_element(By::XPath("//h1/strong"))
        .await?
        .html(true)
        .await?;
    let tvsh = invoice_header.find_element(By::XPath("//small[2]")).await?.find_element(By::Tag("strong")).await?.html(true).await?;
    let release_date = invoice_header.find_element(By::XPath("//ul/li[3]")).await?.html(true).await?;
    

    let value: f64 = value.replace("&nbsp;", "").replace(" LEK", "").replace(",", ".").trim().parse().unwrap();
    let tvsh: Option<f64> = match tvsh.replace("&nbsp;", "").replace(" LEK", "").parse() {
        Ok(o) => Some(o),
        Err(_) => None
    };

    let release_date = NaiveDateTime::parse_from_str(release_date.replace("::before", "").replace(r#"""#, "").trim(), "%d/%m/%Y %H:%M").unwrap();
    let value_before_tvsh = Some(value - tvsh.unwrap_or(0.0)); 
    
    Ok(Receipt {
        value,
        tvsh,
        release_date,
        value_before_tvsh,
        ..Default::default()
    })
}
