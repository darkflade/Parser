use std::error::Error;
use colored::Colorize;
use regex::Regex;
use select::document::Document;
use select::predicate::{ Class, Name};
use wreq::Client;
use wreq::header::LOCATION;
use crate::models::types::{ImageInfo, MediaType};

// Tags Extractor
pub fn extract_tags(html_content: &str) -> Result<ImageInfo, Box<dyn Error>> {

    let mut info = ImageInfo::default();

    let re = Regex::new(r"video_tags:\s*'([^']+)'")?;

    if let Some(captures) = re.captures(html_content) {
        if let Some(tags_str) = captures.get(1) {
            let tags_text = tags_str.as_str();

            info.general = tags_text.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            println!("Tags found: {}", info.general.len());
        }
    }

    Ok(info)

}

pub async fn extract_media_link(html_content: &str, client: &Client) -> Result<MediaType, Box<dyn Error>> {

    let document = Document::from(html_content);

    let wrap_selector = Class("wrap");

    for wrap_node in document.find(wrap_selector) {
        let label_text = wrap_node.find(Class("label"))
            .next()
            .map(|n| n.text())
            .unwrap_or_default();

        if label_text.trim() == "Download" {

            let links: Vec<(String, String)> = wrap_node.find(Name("a"))
                .filter_map(|n| {
                    let text = n.text();
                    let href = n.attr("href")?.to_string();
                    Some((text, href))
                })
                .collect();

            for (text, href) in &links {
                if text.contains("2160") {
                    return get_hidden_link(href, client).await;
                }
            }

            for (text, href) in &links {
                if text.contains("1080") {
                    return get_hidden_link(href, client).await;
                }
            }

            for (text, href) in &links {
                if text.contains("720") {
                    return get_hidden_link(href, client).await;
                }
            }

            for (text, href) in &links {
                if text.contains("480") {
                    return get_hidden_link(href, client).await;
                }
            }

            if let Some((_, href)) = links.first() {
                return get_hidden_link(href, client).await;
            }
        }
    }

    println!("{}","Download section or video links not found".red());


    Ok(MediaType::NotFound)

}

async fn get_hidden_link(href: &str, client: &Client) -> Result<MediaType, Box<dyn Error>> {

    let response = client
        .get(href)
        .redirect(wreq::redirect::Policy::none())
        .send()
        .await?;

    if response.status().is_redirection() {
        if let Some(location_header) = response.headers().get(LOCATION) {
            let final_url = location_header.to_str()?;
            return Ok(MediaType::Video(final_url.to_string()));
        } else {
            return Err(format!(
                "Redirection occurred ({}), but Location header is missing.",
                response.status()
            ).into());
        }
    }

    if response.status().is_success() {
        return Ok(MediaType::Video(href.to_string()));
    }

    Err(format!("Failed to fetch hidden link. Status: {}", response.status()).into())
}
