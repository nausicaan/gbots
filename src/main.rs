use std::{collections::HashMap, env};
use colored::Colorize;
mod tasks;

const HALT: &'static str = "program halted ";

fn main() {
    let passed: Vec<String> = env::args().collect();
    let mut months: HashMap<String, String> = HashMap::new();

    // Insert some key-value pairs into months
    months.insert(String::from(tasks::NAMES[0]), String::from("01"));
    months.insert(String::from(tasks::NAMES[1]), String::from("02"));
    months.insert(String::from(tasks::NAMES[2]), String::from("03"));
    months.insert(String::from(tasks::NAMES[3]), String::from("04"));
    months.insert(String::from(tasks::NAMES[4]), String::from("05"));
    months.insert(String::from(tasks::NAMES[5]), String::from("06"));
    months.insert(String::from(tasks::NAMES[6]), String::from("07"));
    months.insert(String::from(tasks::NAMES[7]), String::from("08"));
    months.insert(String::from(tasks::NAMES[8]), String::from("09"));
    months.insert(String::from(tasks::NAMES[9]), String::from("10"));
    months.insert(String::from(tasks::NAMES[10]), String::from("11"));
    months.insert(String::from(tasks::NAMES[11]), String::from("12"));

    if passed.len() > 1 {
        let task: &String = &passed[1];
        let month: &String = &passed[2];
        let days: Vec<String> = tasks::generate(&months, &month);
        // let site: &str = &passed[3];
        if task == "download" {
            message("Downloading Compressed Log Files");
            tasks::download(&months, &month, &days);
            println!();
        } else if task == "unzip" {
            message("Unzipping Compressed Log Files");
			tasks::unzip(&month);
            println!();
        } else if task == "filter" {
            message("Searching for Hits to Target URL");
            tasks::manipulate(&("filtered for ".to_owned() + &passed[3] + " hits"), &month, "/unzipped/",  &passed[3]);
            println!();
        } else if task == "divide" {
            message("Dividing Data into Google and Non-Google Hits");
            tasks::manipulate("divided", &month, "/filtered/",  &passed[3]);
            println!();
        } else {
            warn(" Arguments not recognized ");
        }
    } else {
        alert(" No arguments provided, ");
    }
}

// Print informational messages
fn message(content: &str) {
    println!("\n{} {} {}", "**".yellow(), content, "**".yellow());
}

// Print colourized warning messages
fn warn(content: &str) {
    println!("\n{}", content.on_yellow());
}

// Print colourized error messages
fn alert(content: &str) {
    println!("\n{}{}", content.on_bright_red(), HALT.on_bright_red());
}