use std::error::Error;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use crate::models::types::{ImageInfo, MediaType, Rule34Parse};
use crate::utility::utils::check_image;

// Tags Extractor
pub fn extract_tags(html_content: &str) -> Result<ImageInfo, Box<dyn Error>> {
    println!("Getting info...");

    let document = Document::from(html_content);
    let mut info = ImageInfo::default();

    let sidebar_node = match document.find(Attr("id","tag-sidebar")).next(){
        Some(node) => node,
        None => {
            println!("[Warn] Sidebar not found");
            return Ok(info);
        },
    };

    println!("Sidebar found. Searching for tags inside...");

    let li_selector = Name("li").and(Class("tag"));

    for li_node in sidebar_node.find(li_selector) {
        let search_class = li_node.attr("class").unwrap_or("");

        let tag_list: &mut Vec<String> = match search_class {
            s if s.contains("tag-type-copyright") => &mut info.titles,
            s if s.contains("tag-type-artist") => &mut info.authors,
            s if s.contains("tag-type-character") => &mut info.characters,
            s if s.contains("tag-type-general") => &mut info.general,
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

fn try_extract_image(html: &str) -> Result<Option<String>, Box<dyn Error>> {
    let start_key = "image = {";
    let end_key = "};";

    let start_index = match html.find(start_key) {
        Some(i) => i + start_key.len() - 1,
        None => return Ok(None),
    };

    let end_index = match html[start_index..].find(end_key) {
        Some(i) => start_index + i + 1,
        None => return Ok(None),
    };

    let js = &html[start_index..end_index];
    let json_str = js.replace('\'', "\"");

    let info: Rule34Parse = match serde_json::from_str(&json_str) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    let link = format!(
        "{}/{}/{}/{}",
        info.domain, info.base_dir, info.dir, info.img
    );

    if check_image(&link) {
        return Ok(Some(link));
    }

    Ok(None)
}
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