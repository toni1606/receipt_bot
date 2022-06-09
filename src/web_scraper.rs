use scraper::{Html, Selector};
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

    let html = driver
        .find_element(By::ClassName("invoice-amount"))
        .await?
        .find_element(By::Tag("h1"))
        .await?
        .find_element(By::Tag("strong"))
        .await?
        .html(true)
        .await?;

    driver.quit().await?;

    Ok(Receipt {
        nslf: html,
        ..Default::default()
    })
}

async pub fn get_receipt_from_url(driver: &WebDriver) -> Receipt {
    let invoice_header = driver
        .find_element(By::ClassName("invoice-amount"))
        .await?;

    let value = invoice_header.find_element(By::Tag("h1"))
        .await?
        .find_element(By::Tag("strong"))
        .await?
        .html(true)
        .await?;
    
    let tvsh = invoice_header.find_element(By::XPath("/small[2]")).await?.html(true).await?;
    
}
