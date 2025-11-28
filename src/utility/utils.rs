use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use crate::models::types::{ImageInfo, Site};

/////////////
// Utility//
////////////
pub fn load_links_from_file(file_path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    println!("Loading links...");

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
    println!("Saving image...");

    let image_path = folder_path.join(format!("{}.{}", name, extension));

    let mut image_file = File::create(&image_path)?;
    image_file.write_all(image_data)?;
    println!("Saved image! To path: {:?}", image_path);

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
    println!("Successfully saved tags to {}", tags_path.to_str().unwrap());

    Ok(())
}

pub fn detect_site(url: &str) -> Site {

    match url {
        s if s.contains("rule34.xxx") => Site::Rule34xxx,
        s if s.contains("rule34.us") => Site::Rule34us,
        s if s.contains("rule34video.com") => Site::Rule34Video,
        s if s.contains("gelbooru.com") => Site::Gelbooru,
        s if s.contains("donmai.us") => Site::Danbooru,
        s if s.contains("nozomi.la") => Site::Nozomi,
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