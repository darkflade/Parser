use std::error::Error;
use wreq::Client;
use wreq::header::{HeaderMap, COOKIE, USER_AGENT};
use wreq_util::{Emulation};

pub async fn create_giga_client(user_agent: &str, cookie: &str) -> Result<Client, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, user_agent.parse()?);
    if !cookie.is_empty() {
        headers.insert(COOKIE, cookie.parse()?);
    }
    let client = Client::builder()
        .emulation(Emulation::Firefox139)
        .default_headers(headers)
        .build()?;

    Ok(client)
}