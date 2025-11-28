use serde::Deserialize;

#[derive(Debug)]
pub enum Site {
    Rule34xxx,
    Rule34us,
    Rule34Video,
    Gelbooru,
    Danbooru,
    Nozomi,
    Unknown,
}
#[derive(Debug)]
pub enum MediaType {
    Image(String),
    Video(String),
    NotFound,
}

impl MediaType {
    pub fn get_link_and_extension(&self) -> Option<(String, &'static str)> {
        match self {
            MediaType::Image(link) => Some((link.clone(), "png")),
            MediaType::Video(link) => Some((link.clone(), "mp4")),
            _ => None,
        }

    }
}

#[derive(Debug, Default)]
pub struct ImageInfo {
    pub authors: Vec<String>,
    pub titles: Vec<String>,
    pub characters: Vec<String>,
    pub general: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Rule34Parse {
    pub domain: String,
    pub dir: i32,
    pub img: String,
    pub base_dir: String,
}
