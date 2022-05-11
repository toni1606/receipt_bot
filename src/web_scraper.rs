use scraper::{Html, Selector};

use crate::models::Receipt;

pub async fn get_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(reqwest::get(url)
        .await?
        .text()
        .await?)
}

pub fn get_receipt_from_url(html: &str) -> Receipt {
    let fragment = Html::parse_fragment(html);
}