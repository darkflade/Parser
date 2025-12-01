use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use colored::Colorize;
use regex::Regex;
use crate::models::types::{DataType, ImageInfo, Site};

/////////////
// Utility//
////////////
pub fn load_links_from_file(file_path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    let mut links = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;

        let trimmed_line = line.trim();

        if !trimmed_line.is_empty() && !trimmed_line.starts_with('#') {
            links.push(trimmed_line.to_string());
        }
    }

    Ok(links)
}
pub fn save_media(folder_path: &Path, name: &str, image_data: &[u8], extension: &str) -> Result<(), Box<dyn Error>> {

    let image_path = folder_path.join(format!("{}.{}", name, extension));

    let mut image_file = File::create(&image_path)?;
    image_file.write_all(image_data)?;
    
    let formated_string = format!("Image Saved! To path: {:?}", image_path).on_green().black();
    println!("{}", formated_string);

    Ok(())

}

pub fn save_tags(path: &Path, name: &str, tags_info: &ImageInfo) -> Result<(), Box<dyn Error>> {
    let tags_path = path.join(format!("{}.txt", name));

    let mut tags_file = File::create(&tags_path)?;

    let mut content = String::new();
    let mut format_section = |title: &str, tags: &Vec<String>| {
        if !tags.is_empty() {
            content.push_str(&format!("[{}]\n", title));
            content.push_str(&tags.join("\n"));
            content.push_str("\n\n");
        }
    };

    format_section("author", &tags_info.authors);
    format_section("title", &tags_info.titles);
    format_section("character", &tags_info.characters);
    format_section("general", &tags_info.general);

    tags_file.write_all(content.trim().as_bytes())?;

    Ok(())
}

pub fn detect_site(url: &str) -> Site {

    match url {
        s if s.contains("/rule34.xxx") => Site::Rule34xxx,
        s if s.contains("/rule34.us") => Site::Rule34us,
        s if s.contains("/rule34video.com") => Site::Rule34Video,
        s if s.contains("/gelbooru.com") => Site::Gelbooru,
        s if s.contains("/danbooru.donmai.us") => Site::Danbooru,
        s if s.contains("/nozomi.la") => Site::Nozomi,
        _ => Site::Unknown,
    }
}

pub fn check_image (link: &str) -> bool {
    link.ends_with(".png") 
        || link.ends_with(".jpg") 
        || link.ends_with(".jpeg") 
        || link.ends_with(".gif") 
        || link.ends_with(".webp")
}

pub fn get_base_domain(url: &str) -> String {

    let start_search = if url.starts_with("https://") { 8 } else { 7 };

    if let Some(slash_index) = url[start_search..].find('/') {
        let end = start_search + slash_index + 1;
        url[..end].to_string()
    } else {
        format!("{}/", url)
    }
}

pub fn get_nozomi_id_from_link(original_link: &str) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(r"/post/(\d+)\.html")?;

    let id = match re.captures(original_link).and_then(|c| c.get(1)) {
        Some(m) => m.as_str(),
        None => return Err("Could not extract ID from link".into()),
    };
    
    Ok(id.to_string())
}

pub fn generate_nozomi_link_from_id(id: &str, data_type: DataType) -> Result<String, Box<dyn Error>> {

    let link_prefix: &str;
    let link_postfix: &str;
    let link_additional: &str;

    match data_type {
        DataType::Image => {
            link_prefix = "w";
            link_postfix = "webp";
            link_additional = "";
        },
        DataType::Json => {
            link_prefix = "j";
            link_postfix = "json";
            link_additional = "post/";
        },
    }

    let chars: Vec<char> = id.chars().collect();
    let len = chars.len();

    if len < 3 {
        return Ok(format!("https://{}.gold-usergeneratedcontent.net/{}.{}", link_prefix, id, link_postfix))
    }

    let last_char = chars[len - 1];

    let next_two = format!("{}{}", chars[len - 3], chars[len - 2]);

    let api_url = format!("https://{}.gold-usergeneratedcontent.net/{}{}/{}/{}.{}", link_prefix, link_additional, last_char, next_two, id, link_postfix);
    
    Ok(api_url)
}