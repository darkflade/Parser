use std::error::Error;
use regex::Regex;
use select::document::Document;
use select::predicate::{ Class, Name};
use crate::models::types::{ImageInfo, MediaType};

// Tags Extractor
pub fn extract_tags(html_content: &str) -> Result<ImageInfo, Box<dyn Error>> {
    println!("Getting info...");

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

pub fn extract_media_link(html_content: &str) -> Result<MediaType, Box<dyn Error>> {

    let document = Document::from(html_content);

    let wrap_selector = Class("wrap");

    for wrap_node in document.find(wrap_selector) {
        let label_text = wrap_node.find(Class("label"))
            .next()
            .map(|n| n.text())
            .unwrap_or_default();

        if label_text.trim() == "Download" {
            println!("Found Download section!");

            let links: Vec<(String, String)> = wrap_node.find(Name("a"))
                .filter_map(|n| {
                    let text = n.text();
                    let href = n.attr("href")?.to_string();
                    Some((text, href))
                })
                .collect();

            for (text, href) in &links {
                if text.contains("2160") {
                    println!("Selected 2160p video");
                    return Ok(MediaType::Video(href.clone()));
                }
            }

            for (text, href) in &links {
                if text.contains("1080") {
                    println!("Selected 1080p video");
                    return Ok(MediaType::Video(href.clone()));
                }
            }

            for (text, href) in &links {
                if text.contains("720") {
                    println!("Selected 720p video");
                    return Ok(MediaType::Video(href.clone()));
                }
            }

            for (text, href) in &links {
                if text.contains("480") {
                    println!("Selected 480p video");
                    return Ok(MediaType::Video(href.clone()));
                }
            }

            if let Some((_, href)) = links.first() {
                println!("Selected default (first) video");
                return Ok(MediaType::Video(href.clone()));
            }
        }
    }

    println!("Download section or video links not found");
    Ok(MediaType::NotFound)

}