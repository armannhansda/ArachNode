use scraper::{Html, Selector};
use url::Url;

use crate::utils::url_utils::normalize_url;

// Extract links from the page
pub fn extract_links(url: &str, body: &str) -> Vec<String> {
    let mut links = Vec::new();
    let document = Html::parse_document(body);

    // extrct title
    let title_selector = Selector::parse("title").unwrap();

    if let Some(title) = document.select(&title_selector).next() {
        println!("Title: {}", title.inner_html());
    }

    // extract meta description
    let meta_selector = Selector::parse("meta[name=\"description\"]").unwrap();
    for meta in document.select(&meta_selector) {
        if let Some(desc) = meta.value().attr("content") {
            println!("Description: {}", desc);
        }
    }

    // extract links
    let selector = Selector::parse("a").unwrap();

    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            let full_url = if link.starts_with("http") {
                link.to_string()
            } else {
                match Url::parse(&url).and_then(|base| base.join(link)) {
                    Ok(u) => u.to_string(),
                    Err(_) => continue,
                }
            };

            if let Some(normalized) = normalize_url(&full_url) {
                links.push(normalized);
            }
        }
    }

    links
}

pub fn extract_text(body: &str) -> String {
    let document = Html::parse_document(body);
    document.root_element().text().collect::<Vec<_>>().join(" ")
}

pub fn extract_metadata(body: &str) -> (String, String) {
    let document = Html::parse_document(body);

    let title_selector = Selector::parse("title").unwrap();
    let title = document 
        .select(&title_selector)
        .next()
        .map(|t| t.inner_html())
        .unwrap_or_default();

    let meta_selector = Selector::parse("meta[name=\"description\"]").unwrap();
    let description = document
        .select(&meta_selector)
        .next()
        .and_then(|m| m.value().attr("content"))
        .unwrap_or("")
        .to_string();

    (title, description)
}
