use std::str::FromStr;

use chrono::NaiveDateTime;
use thirtyfour::prelude::*;

use crate::{models::Receipt, schema::receipt::{business_id, nivf}};

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
    let invoice_header = driver.find_element(By::ClassName("invoice-amount")).await?;
    let invoice_details = driver.find_element(By::ClassName("panel-body")).await?;
    
    // invoice-header fields
    let value = invoice_header.find_element(By::XPath("//h1/strong")).await?.html(true).await?;
    let tvsh = invoice_header.find_element(By::XPath("//small[2]")).await?.find_element(By::Tag("strong")).await?.html(true).await?;
    let release_date = invoice_header.find_element(By::XPath("//ul/li[3]")).await?.html(true).await?;
    let location = Some(format!("{}, {}",
        invoice_header.find_element(By::XPath("//ul/li[2]/span[1]")).await?.html(true).await?,
        invoice_header.find_element(By::XPath("//ul/li[2]/span[3]")).await?.html(true).await?
    ));
    
    // invoice-details fields
    let bus_id = invoice_details.find_element(By::XPath("//div[@class='form-group form-column'][2]/p")).await?.html(true).await?;
    let nslf = invoice_details.find_element(By::XPath("//div[3]/p")).await?.html(true).await?;
    let _nivf = invoice_details.find_element(By::XPath("//div[4]/p")).await?.html(true).await?;
    let receipt_type = Some(invoice_details.find_element(By::XPath("//div[5]/p")).await?.html(true).await?);
    let op_id = invoice_details.find_element(By::XPath("//div[6]/p")).await?.html(true).await?;
    let op_id = invoice_details.find_element(By::XPath("//div[6]/p")).await?.html(true).await?;

    

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
        location,
        business_id: bus_id,
        nslf,
        nivf: _nivf,
        ..Default::default()
    })
}
