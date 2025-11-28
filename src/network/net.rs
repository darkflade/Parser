use std::error::Error;
use reqwest::Client;
use reqwest::header::REFERER;

////////////
//Network//
///////////
pub async fn send_request_like_browser(client: &Client, link: &str) -> Result<String, Box<dyn Error>> {
    let response = client.get(link).send().await?;
    println!("Response status: {}", response.status());

    let text = response.text().await?;

    Ok(text)
}

pub async fn download_media_bytes(client: &Client, link: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    println!("Downloading image from {}", link);

    let response = client.get(link).header(REFERER, link).send().await?;
    println!("Response status: {}", response.status());

    let bytes = response.bytes().await?;
    let image_bytes: Vec<u8> = bytes.to_vec();

    Ok(image_bytes)

}
