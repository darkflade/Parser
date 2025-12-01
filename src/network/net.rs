use std::error::Error;
use wreq::Client;
use wreq::header::{HeaderMap, HeaderValue, ORIGIN, REFERER};
use crate::models::types::DataType;
use crate::utility::utils;
use crate::utility::utils::get_base_domain;

////////////
//Network//
///////////
pub async fn send_request_like_browser(client: &Client, link: &str) -> Result<String, Box<dyn Error>> {
    let response = client.get(link).send().await?;
    
    let text = response.text().await?;

    Ok(text)
}

pub async fn download_media_bytes(client: &Client, link: &str, referer: Option<&str>, origin: Option<&str>) -> Result<Vec<u8>, Box<dyn Error>> {
    println!("Downloading image from {}", link);

    let mut headers = HeaderMap::new();
    if let Some(referer) = referer {
        headers.insert(REFERER, HeaderValue::from_str(referer).unwrap());
    }
    if let Some(origin) = origin {
        headers.insert(ORIGIN, HeaderValue::from_str(origin).unwrap());
    }


    let response = client.get(link)
        .headers(headers)
        .send().await?;

    let bytes = response.bytes().await?;
    let image_bytes: Vec<u8> = bytes.to_vec();

    Ok(image_bytes)

/*
    if let Some(data_start) = link.find("base64,") {
        let encoded_data = &link[data_start + "base64,".len()..];

        let image_bytes = general_purpose::STANDARD.decode(encoded_data)?;

        Ok(image_bytes)
    } else {
        Err("Invalid Base64 format: 'base64,' marker not found.".into())
    }
*/
}

pub async fn fetch_nozomi_with_headers(client: &Client, base_link: &str) -> Result<String, Box<dyn Error>> {

    let referer = &get_base_domain(base_link);

    let request_id = utils::get_nozomi_id_from_link(base_link)?;
    let request_link = utils::generate_nozomi_link_from_id(&request_id, DataType::Json)?;

    let response = client.get(request_link)
        .header("Referer", referer)
        .header("Origin", referer)
        .header("X-Requested-With", "XMLHttpRequest")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("API Error: {}", response.status()).into());
    }

    let text = response.text().await?;
    Ok(text)
}

