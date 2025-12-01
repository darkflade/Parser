use std::error::Error;
use select::document::Document;
use select::predicate::{Attr, Name};
use serde::Deserialize;
use crate::models::types::{DataType, ImageInfo, MediaType};

#[derive(Debug, Deserialize)]
struct InnerTag {
    #[serde(rename = "tagname_display")]
    name: String,
}

#[derive(Debug, Deserialize)]
struct InnerImage {
    dataid: String,
}

#[derive(Debug, Deserialize)]
struct ApiJson {
    artist: Option<Vec<InnerTag>>,
    copyright: Option<Vec<InnerTag>>,
    character: Option<Vec<InnerTag>>,
    general: Option<Vec<InnerTag>>,
    imageurls: Option<Vec<InnerImage>>,
}

// Tags Extractor
pub fn extract_tags(json_content: &str) -> Result<ImageInfo, Box<dyn Error>> {
    println!("{}", json_content);

    let data: ApiJson = serde_json::from_str(json_content)?;
    let mut info = ImageInfo::default();

    let add_tags = |source: Option<Vec<InnerTag>>, target: &mut Vec<String>| {
        if let Some(tags) = source {
            for t in tags {
                target.push(t.name);
            }
        }
    };

    add_tags(data.artist, &mut info.authors);
    add_tags(data.copyright, &mut info.titles);
    add_tags(data.character, &mut info.characters);
    add_tags(data.general, &mut info.general);

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

fn try_extract_image(json_content: &str) -> Result<Option<String>, Box<dyn Error>> {
    println!("{}", json_content);
    let data: ApiJson = serde_json::from_str(json_content)?;

    if let Some(images) = data.imageurls {
        if let Some(first_img) = images.first() {
            let image_link = crate::utils::generate_nozomi_link_from_id(&first_img.dataid, DataType::Image)?;

            return Ok(Some(image_link));
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