mod parsers;
mod network;
mod utility;
mod models;

use std::{path::Path, error::Error, time::Duration, fs::{create_dir_all}, env};
use std::collections::HashMap;
use colored::Colorize;
use rand::Rng;
use wreq::Client;
use tokio::time::sleep;

// Custom
use network::{ net, client };
use parsers::rule34_xxx;
use utility::utils;
use models::types;
use crate::models::types::{BadProgressState, Site};
use crate::parsers::{danbooru, gelbooru, nozomi, rule34_us, rule34video};
use crate::utility::{easy_print, initializer};
use crate::utility::utils::detect_site;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Statistic
    let mut success = 0;
    let mut not_found = 0;
    let mut not_found_links: HashMap<usize, String> = HashMap::new();

    let mut failed = 0;
    let mut failed_links: HashMap<usize, String> = HashMap::new();

    let mut skipped = 0;
    let mut skipped_links: HashMap<usize, String> = HashMap::new();
///////////////////
//--Common Properties
    const USER_AGENT:&str = "Mozilla/5.0 (X11; Linux x86_64; rv:139.0) Gecko/20100101 Firefox/139.0";


    // Config Loading
    let config = initializer::load_config()?;
    let current_dir = env::current_dir()?;
    let path_to_save = current_dir.join(&config.paths.save_root);
    let path_for_links_file = current_dir.join(&config.paths.links_file);
    let start_offset = &config.paths.start_name.parse::<usize>()?;

    let request_delay = &config.timings.request_delay.parse::<u64>()?;


//--Specific Properties
    //R34
    let r34cookie = config.get_cookie_for_site("rule34.xxx");
    //R34US
    let r34us_cookie = config.get_cookie_for_site("rule34.us");
    //R34Video
    let r34_video_cookie = config.get_cookie_for_site("rule34video.com");
    //GB
    let gbcookie = config.get_cookie_for_site("gelbooru.com");
    //DB
    let dbcookie = config.get_cookie_for_site("danbooru.com");
    //Nozomi
    let nozomi_cookie = config.get_cookie_for_site("nozomi.la");

///////////////////
//--Clients build
    let rule34_xxx_client = client::create_net_client(USER_AGENT, r34cookie).await?;
    let rule34_us_client = client::create_net_client(USER_AGENT, r34us_cookie).await?;
    let rule34video_client = client::create_net_client(USER_AGENT, r34_video_cookie).await?;

    let gbclient = client::create_net_client(USER_AGENT, gbcookie).await?;
    let dbclient = client::create_net_client(USER_AGENT, dbcookie).await?;

    let nozomi_client = client::create_net_client(USER_AGENT, nozomi_cookie).await?;

//////////////////////
//--File System Deal
    let directory = &config.paths.sub_dir;
    let folder_path_str = path_to_save.join(directory);
    let folder_path = Path::new(&folder_path_str);
    create_dir_all(folder_path)?;

    // Loading links from file import.txt
    let links_file = Path::new(&path_for_links_file);
    let link_to_download = utils::load_links_from_file(links_file)?;

//////////////////
//--Main Part
    for (i, link) in link_to_download.iter().enumerate() {
        println!("Try link {}", link.blue());
        let client_to_use: &Client;
        let html: String;

        let site_type = detect_site(&link);

        let media_to_dl: types::MediaType;
        let tag_data: types::ImageInfo;
        let media_bytes: Vec<u8>;

        let referer: Option<String>;
        let origin: Option<String>;

        match site_type {
            Site::Rule34xxx => {
                println!("This link is rule34.xxx");
                client_to_use = &rule34_xxx_client;
                html = net::send_request_like_browser(client_to_use, &link).await?;
                media_to_dl = rule34_xxx::extract_media_link(&html)?;
                tag_data = rule34_xxx::extract_tags(&html)?;
                referer = Some(utils::get_base_domain(&link));
                origin = None;
            }

            Site::Rule34us => {
                println!("This link is rule34.us");
                client_to_use = &rule34_us_client;
                html = net::send_request_like_browser(client_to_use, &link).await?;
                media_to_dl = rule34_us::extract_media_link(&html)?;
                tag_data = rule34_us::extract_tags(&html)?;
                referer = Some(utils::get_base_domain(&link));
                origin = None;
            }
            Site::Rule34Video => {
                println!("This link is rule34video.com");
                client_to_use = &rule34video_client;
                html = net::send_request_like_browser(client_to_use, &link).await?;
                media_to_dl = rule34video::extract_media_link(&html, &client_to_use).await?;
                tag_data = rule34video::extract_tags(&html)?;
                referer = Some(utils::get_base_domain(&link));
                origin = None;
            }

            Site::Gelbooru => {
                println!("This link is gelbooru.com");
                client_to_use = &gbclient;
                html = net::send_request_like_browser(client_to_use, &link).await?;
                media_to_dl = gelbooru::extract_media_link(&html)?;
                tag_data = gelbooru::extract_tags(&html)?;
                referer = Some(utils::get_base_domain(&link));
                origin = None;
            }

            Site::Danbooru => {
                println!("This link is danbooru.com");
                client_to_use = &gbclient;
                html = net::send_request_like_browser(&dbclient, &link).await?;
                media_to_dl = danbooru::extract_media_link(&html)?;
                tag_data = danbooru::extract_tags(&html)?;
                referer = Some(utils::get_base_domain(&link));
                origin = None;
            }
            Site::Nozomi => {
                println!("This link is nozomi.la");
                client_to_use = &nozomi_client;
                html = net::fetch_nozomi_with_headers(&client_to_use, &link).await?;
                media_to_dl = nozomi::extract_media_link(&html)?;
                tag_data = nozomi::extract_tags(&html)?;

                referer = Some(utils::get_base_domain(&link));
                origin = None;
            }

            _ => {
                skipped += 1;
                skipped_links.insert(i, link.to_string());
                println!("{}","Unsupported site".red());
                continue;
            }
        }

        if let Some((media_link_to_dl, file_extension)) = media_to_dl.get_link_and_extension() {
            match net::download_media_bytes(&client_to_use, &media_link_to_dl, referer.as_deref(), origin.as_deref()).await {
                Ok(bytes) => {
                    success +=  1;
                    media_bytes = bytes;
                }
                Err(e) => {
                    failed += 1;
                    failed_links.insert(i, link.clone());
                    println!("{} because of {}","[ERROR] Failed to download".red(), e);
                    continue;
                }
            }
            utils::save_media(folder_path, &(i + start_offset).to_string(), &media_bytes, &file_extension)?;
            utils::save_tags(folder_path, &(i + start_offset).to_string(), &tag_data)?;
        } else {
            not_found += 1;
            not_found_links.insert(i, link.clone());
            println!("{}","[ERROR] Media Not Found on page.".red());
        }

        let mut rng = rand::rng();
        let random_timeout_offset = rng.random_range(500..800);

        sleep(Duration::from_millis(request_delay + random_timeout_offset)).await;
    }

    if failed == 0 && not_found == 0  && skipped == 0 {
        println!("{} {}", "All downloaded successfully: ".bright_green(), success);
    } else {

        easy_print::print_success(success);
        easy_print::print_bad_state_links(&skipped_links, skipped, BadProgressState::Skipped);
        easy_print::print_bad_state_links(&not_found_links, not_found, BadProgressState::NotFound);
        easy_print::print_bad_state_links(&failed_links, failed, BadProgressState::Failed);

    }

    Ok(())
}
