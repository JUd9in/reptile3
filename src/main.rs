use undetected_chromedriver::chrome;
use thirtyfour::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let driver = chrome().await?;

    driver.goto("https://www.rust-lang.org/").await?;

    let title = driver.title().await?;
    println!("Title: {}", title);

    // let elem = driver.find(By::Css("pre")).await?;
    // let raw_json = elem.text().await?;

    // println!("Target_Json: {}", raw_json);

    // let body = driver.source().await?;
    // println!("Body: {}", body);

    driver.quit().await?;

    Ok(())
}