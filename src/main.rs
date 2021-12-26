use scraper::{Html, Selector};

mod errors;

fn get_url(word: &str) -> String {
    format!("https://www.ldoceonline.com/dictionary/{}", word)
}

#[tokio::main]
async fn main() -> errors::Result<()> {
    let body = reqwest::get(get_url("fierce")).await?.text().await?;
    let document = Html::parse_document(body.as_str());
    let selector = Selector::parse("span.DEF").unwrap();
    for (i, element) in document.select(&selector).enumerate() {
        let explanation = element.text().fold(String::new(), |a, b| a + b);
        println!("{}: {}", i, explanation.trim());
    }
    Ok(())
}
