use std::collections::HashMap;
use colored::Colorize;
use crate::models::types::BadProgressState;


pub fn print_success(count: i32) {
    println!("{} {}", "Downloaded successfully: ".bright_green(), count);
    println!();
}

pub fn print_bad_state_links(links: &HashMap<usize, String>, count: i32, state: BadProgressState) {
    if count == 0 { return }

    println!("{} {}", state.label(), count);

    for (position, link) in links.iter() {
        let formated_position = state.apply(&format!("[{}] ", position));
        let formated_link = state.apply(&link);
        
        println!("{}{}", formated_position, formated_link)
    }
    println!();
}
