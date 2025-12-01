use std::error::Error;
use select::document::Document;
use select::predicate::{Attr, Class, Name};
use crate::models::types::{ImageInfo, MediaType};
use crate::utility::utils;

// Tags Extractor
pub fn extract_tags(html_content: &str) -> Result<ImageInfo, Box<dyn Error>> {

    let document = Document::from(html_content);
    let mut info = ImageInfo::default();

    for ul_node in document.find(Name("ul")) {
        let class_name = ul_node.attr("class").unwrap_or("");

        let tag_list = match class_name {
            c if c.contains("artist-tag-list") => &mut info.authors,
            c if c.contains("copyright-tag-list") => &mut info.titles,
            c if c.contains("character-tag-list") => &mut info.characters,
            c if c.contains("general-tag-list") => &mut info.general,
            _ => continue,
        };



        for a_node in ul_node.find(Class("search-tag")) {
            let tag_name = a_node.text().trim().to_string();

            if tag_name.len() > 1 {
                tag_list.push(tag_name);
            }
        }
    }
    println!("Tags found - Authors: {}, General: {}", info.authors.len(), info.general.len());

    Ok(info)

}

pub fn extract_media_link(html_content: &str) -> Result<MediaType, Box<dyn Error>> {
    if let Some(media) = try_extract_image(html_content)? {
        return Ok(MediaType::Image(media));
    }

    if let Some(media) = try_extract_video(html_content)? {
        return Ok(MediaType::Video(media));
    }

    Ok(MediaType::NotFound)

}

fn try_extract_image(html_content: &str) -> Result<Option<String>, Box<dyn Error>> {
    let document = Document::from(html_content);
    for node in document.find(Class("image-container")) {
        if let Some(url) = node.attr("data-file-url") {

            if utils::check_image(url) {
                return Ok(Some(url.to_string()));
            }
        }
    }
    
    Ok(None)
}

// TODO This doesn't work it's just copypaste. Let it just be
fn try_extract_video (html_content: &str) -> Result<Option<String>, Box<dyn Error>> {
    let document = Document::from(html_content);

    let link = document.find(Attr("id", "gelcomVideoPlayer"))
        .next()
        .and_then(|video_node| video_node.find(Name("source")).next())
        .and_then(|source_node| source_node.attr("src"))
        .map(|src| {
            println!("Found video link: {}", src);
            src.to_string()
        });

    Ok(link)
}