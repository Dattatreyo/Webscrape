use reqwest::blocking::get;
use scraper::{Html, Selector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // URL to scrape
    let url = "https://www.rust-lang.org/";

    // Fetch the HTML content
    let body = get(url)?.text()?;

    // Parse the HTML
    let document = Html::parse_document(&body);

    // Create a selector for the elements you want to scrape
    let selector = Selector::parse("h1").unwrap();

    // Extract and print the text content of the selected elements
    for element in document.select(&selector) {
        println!("Found h1: {}", element.text().collect::<Vec<_>>().join(""));
    }

    Ok(())
}