use std::error::Error;
use select::document::Document;
use select::predicate::{Attr, Name};
use crate::models::types::{ImageInfo, MediaType};
use crate::utility::utils;

pub fn extract_tags(html_content: &str) -> Result<ImageInfo, Box<dyn Error>> {
    println!("Getting info...");

    let document = Document::from(html_content);
    let mut info = ImageInfo::default();

    let sidebar_node = match document.find(Attr("id","tag-list ")).next(){
        Some(node) => node,
        None => {
            println!("[Warn] Sidebar not found");
            return Ok(info);
        },
    };

    println!("Sidebar found. Searching for tags inside...");

    let li_selector = Name("li");

    for li_node in sidebar_node.find(li_selector) {
        let search_class = li_node.attr("class").unwrap_or("");

        let tag_list: &mut Vec<String> = match search_class {
            s if s.contains("copyright-tag") => &mut info.titles,
            s if s.contains("artist-tag") => &mut info.authors,
            s if s.contains("character-tag") => &mut info.characters,
            s if s.contains("general-tag") => &mut info.general,
            _ => {
                println!("Found Shit instead of tag: {}", search_class);
                continue;
            },
        };

        for a_node in li_node.find(Name("a")) {
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
    let content_selector = Attr("class", "content_push");

    if let Some(content_node) = document.find(content_selector).next() {
        if let Some(img_node) = content_node.find(Name("img")).next() {
            if let Some(src) = img_node.attr("src") {
                println!("Found image link via container: {}", src);
                if utils::check_image(src)
                {
                    return Ok(Some(src.to_string()));
                }
            }
        }
    } else {
        return Ok(None);
    }


    Ok(None)
}
fn try_extract_video (html_content: &str) -> Result<Option<String>, Box<dyn Error>> {
    let document = Document::from(html_content);

    let link = document.find(Name("video"))
        .next()
        .and_then(|video_node| video_node.find(Name("source")).skip(1).next())
        .and_then(|source_node| source_node.attr("src"))
        .map(|src| {
            println!("Found video link: {}", src);
            src.to_string()
        });

    Ok(link)
}