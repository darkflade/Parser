mod parsers;
mod network;
mod utility;
mod models;

use std::{
    path::Path,
    error::Error,
    time::Duration,
    fs::{create_dir_all},
};
use reqwest::Client;
use tokio::time::sleep;

// Custom
use network::{ net, client };
use parsers::rule34_xxx;
use utility::utils;
use models::types;
use network::gigaclient;
use crate::models::types::Site;
use crate::network::giganet;
use crate::parsers::{danbooru, gelbooru, rule34_us, rule34video};
use crate::utility::utils::detect_site;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
///////////////////
//--Common Properties
    const USER_AGENT:&str = "Mozilla/5.0 (X11; Linux x86_64; rv:139.0) Gecko/20100101 Firefox/139.0";

    let path_for_links_file = "/home/darkflade/dl/source.txt";
    let path_to_save = "/home/darkflade/dl";
    let character_name = "Anna Annon";


//--Specific Properties
    //R34
    let r34cookie = "cf_clearance=VLgvyNBOO9oPGi8SpW2ecHl2cUEzfjL9QCYjGMtAYPM-1764318621-1.2.1.1-3OgirKtblXxW2wdFDamU9fHsIR8S54YYqrpG2gKAUIZgshqklmTDKbK4iL5OqgB87JutubeJK5_li6u_M7e9682CeEiooOl1bsijaoGUk5vMAno1rSVyD66msTzCidJTZn7At9qgD1rzyKgAZPatCvuWsmkSZdmYUZVmX8oDv5d_Jk8w.Dd7GwZE6pVp5KBqyt6j94ibJNYyMghI68pwqBptCiCmwv.kMCFW5smw70idKb5X3caKgoOOKpz1Pm2k; gdpr=1; gdpr-consent=1";
    //R34US
    let r34us_cookie = "cf_clearance=lT8b4rWqbnxSHYkoU3D1K9bqYlKOmxbLUZuvVjTc.bU-1764337137-1.2.1.1-Emk1ar4kNC2TjmWiXr4VyZZba_X3XUO.vtL2tHFsmib9nOFIyZmqFGpIBZ3SI4jSns0n3RMs9R1riLBhuvAI2ATj0ree9dn0idlkKqnKTePsAYnOrAQu4eAdRne9sQzawaM8lHuUmLoxm7tA1sNY.p_c9ednykU5SljDqB3pjuHOzm_cy_yCABQFRKvlGIMKR6MZDYBvuRthOc0w67wGFU3bzcffH_A9kPAn.7XRO2U; _ga_7TL9KTS11R=GS2.1.s1764319473$o1$g1$t1764323345$j60$l0$h0; _ga=GA1.2.1521054167.1764319473; _gid=GA1.2.1593694751.1764319474";
    //R34Video
    let r34_video_cookie = "kt_rt_popAccess=1; __ddg8_=ivr8IeDhQrOvSw4h; __ddg10_=1764344117; __ddg9_=194.87.240.193; __ddg1_=5JePupEfsN7bQshxzSqd; PHPSESSID=viftfdbs7vanbg9dq19po0h4ma; kt_qparams=id%3D3252829%26dir%3Dnew-character; kt_ips=194.87.240.193; kt_vast_955770=fd500355baba9c26216513ea43f1a952; kt_tcookie=1; kt_is_visited=1; kt_vast_806972=fd500355baba9c26216513ea43f1a952";
    //GB
    let gbcookie = "";
    //DB
    let dbcookie = "_danbooru2_session=lE7ZZVlGE1GuHmqDzMxiW9q3A4OPdYczgcAKi67jTQCLJGgfMsh%2B%2BVlYZMGcCtsSsKeVta1Zt6KzWhUkISDp8byF1sjlwbnUEXp2zAY1UNF5ni3EyIZvsVlpBSJhDhhcIlpmIEYdBphM8zKTn7Zvk5dQZb%2FYuMDSz4pZgyhUJLUNvTgIEq0Ed3O2fezvyuXWbSYmehWEFyE%2FJaGy11MXC4cyHRnT4ulQIG64T1y4dRvM6JVjHVahPfNbhjywLzRTQGGY0C7uBeXKEHdxauojzpMk6%2FsX9abCTIStq4UqOPQbn6DmmZM9Jjrk%2Fadjk%2FVcgKOseP9r9%2F5EZKrYDBaR8gqYzb8zOF%2BPPU6n5MkRs7wNQTZXp4pNl2gQTC7uKVFDqZ2kd7hB3jukwBqpFu0bnA%3D%3D--o%2BF9pyq%2B0t%2FSyrAt--7UuX1BBiJLxZD%2B4T50OCxQ%3D%3D";

///////////////////

    // Clients build
    let rule34_xxx_client = client::create_net_client(USER_AGENT, r34cookie).await?;
    let rule34_us_client = client::create_net_client(USER_AGENT, r34us_cookie).await?;
    let rule34video_client = client::create_net_client(USER_AGENT, r34_video_cookie).await?;

    let gbclient = client::create_net_client(USER_AGENT, gbcookie).await?;
    let dbclient = gigaclient::create_giga_client(USER_AGENT, dbcookie).await?;

    // File System Deal
    let directory = character_name;
    let folder_path_str = format!("{}/{}", path_to_save, directory);
    let folder_path = Path::new(&folder_path_str);
    create_dir_all(folder_path)?;

    // Loading links from file import.txt
    let links_file = Path::new(&path_for_links_file);
    let link_to_download = utils::load_links_from_file(links_file)?;

    // Main Part
    for (i, link) in link_to_download.iter().enumerate() {
        println!("Try link {}", link);
        let client_to_use: &Client;
        let html: String;

        let site_type = detect_site(&link);

        let media_to_dl: types::MediaType;
        let tag_data: types::ImageInfo;
        let media_bytes: Vec<u8>;

        match site_type {
            Site::Rule34xxx => {
                println!("This link is rule34.xxx");
                client_to_use = &rule34_xxx_client;
                html = net::send_request_like_browser(client_to_use, &link).await?;
                media_to_dl = rule34_xxx::extract_media_link(&html)?;
                tag_data = rule34_xxx::extract_tags(&html)?;
            }

            Site::Rule34us => {
                println!("This link is rule34.us");
                client_to_use = &rule34_us_client;
                html = net::send_request_like_browser(client_to_use, &link).await?;
                media_to_dl = rule34_us::extract_media_link(&html)?;
                tag_data = rule34_us::extract_tags(&html)?;
            }
            Site::Rule34Video => {
                println!("This link is rule34video.com");
                client_to_use = &rule34video_client;
                html = net::send_request_like_browser(client_to_use, &link).await?;
                media_to_dl = rule34video::extract_media_link(&html)?;
                tag_data = rule34video::extract_tags(&html)?;
            }

            Site::Gelbooru => {
                println!("This link is gelbooru.com");
                client_to_use = &gbclient;
                html = net::send_request_like_browser(client_to_use, &link).await?;
                media_to_dl = gelbooru::extract_media_link(&html)?;
                tag_data = gelbooru::extract_tags(&html)?;
            }

            Site::Danbooru => {
                println!("This link is danbooru.com");
                client_to_use = &gbclient;
                html = giganet::send_request_like_gigachad(&dbclient, &link).await?;
                media_to_dl = danbooru::extract_media_link(&html)?;
                tag_data = danbooru::extract_tags(&html)?;
            }
            Site::Nozomi => {
                println!("This link is nozomi.la");
                println!("[Skip] Now not supported");
                continue;
            }

            _ => {
                println!("Unsupported site");
                continue;
            }
        }

        if  let Some((media_link_to_dl, file_extension)) = media_to_dl.get_link_and_extension() {
            match site_type {
                Site::Danbooru => media_bytes = giganet::download_media_bytes(&dbclient, &media_link_to_dl).await?,
                _ => media_bytes = net::download_media_bytes(&client_to_use, &media_link_to_dl).await?,
            }
            utils::save_media(folder_path, &(i + 0).to_string(), &media_bytes, &file_extension)?;
            utils::save_tags(folder_path, &(i+0).to_string(), &tag_data)?;
        } else {
            println!("[Warning] Media Not Found on page.");
        }

        sleep(Duration::from_millis(600)).await;
    }

    Ok(())
}
