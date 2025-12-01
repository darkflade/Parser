use std::error::Error;
use wreq::Client;
use wreq::header::{HeaderMap, HeaderValue, COOKIE, USER_AGENT};
use wreq_util::Emulation;

pub async fn create_net_client(user_agent: &str, cookie: &str) -> Result<Client, Box<dyn Error>> {
    let mut headers = HeaderMap::new();

    let user_agent = user_agent;

    headers.insert(USER_AGENT, user_agent.parse()?);
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("document"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("navigate"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-origin"));
    headers.insert("Accept", HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"));
    if !cookie.is_empty() {
        headers.insert(COOKIE, cookie.parse()?);
    }

    let client = Client::builder()
        .emulation(Emulation::Firefox139)
        .default_headers(headers)
        .build()?;

    Ok(client)
}
