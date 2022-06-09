use scraper::{Html, Selector};
use thirtyfour::prelude::*;

use crate::models::Receipt;

pub async fn get_html(url: &str) -> WebDriverResult<Receipt> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    // Navigate to https://wikipedia.org.
    driver.get(url).await?;

    // Look for header to implicitly wait for the page to load.
    driver.find_element(By::Tag("script")).await?;

    println!("{}", driver.title().await?);
 
    while let Err(_) =
    driver.find_element(By::ClassName("invoice-items")).await {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
 
    let html = driver.find_element(By::ClassName("invoice-amount")).await?.find_element(By::Tag("h1")).await?.find_element(By::Tag("strong")).await?.html(true).await?;

    // Always explicitly close the browser. There are no async destructors.
    driver.quit().await?;
 
    Ok(Receipt { nslf: html, ..Default::default() })
}

// pub fn get_receipt_from_url(html: &str) -> Receipt {
//     let fragment = Html::parse_fragment(html);
//     unimplemented!();
// }