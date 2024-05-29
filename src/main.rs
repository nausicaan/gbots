use std::{collections::HashMap, env};
use colored::Colorize;

use crate::tasks::vars::TARGET;
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

    if passed.len() == 0 {
        alert(" No arguments provided, ");
    }

    if passed.len() == 2 {
        if &passed[1] == "help" || &passed[1] == "h" {
            about();
        } else {
            warn(" More parameters needed ");
        }
    }

    if passed.len() == 3 {
        if passed[1] == "download" || passed[1] == "d" {
            message("Downloading Compressed Log Files");
            let days: Vec<String> = tasks::generate(&months, &passed[2]);
            tasks::download(&months, &passed[2], &days);
            println!();
        } else if passed[1] == "unzip" || passed[1] == "u" {
            message("Unzipping Compressed Log Files");
			tasks::unzip(&passed[2]);
            println!();
        } else if passed[1] == "filter" || passed[1] == "f" {
            message("Searching for Hits to Target URL");
            tasks::manipulate(&("Filtering for ".to_owned() + TARGET + " hits"), &passed[2], "/unzipped/",  TARGET);
            println!();
        } else if passed[1] == "divide" || passed[1] == "v" {
            message("Dividing Data into Google and Non-Google Hits");
            tasks::manipulate("Divided", &passed[2], "/filtered/", TARGET);
            println!();
        } else if passed[1] == "capture" || passed[1] == "c" {
            message("Capturing all existing search strings");
            tasks::pattern(&passed[2]);
            println!();
        } else if passed[1] == "analyze" || passed[1] == "a" {
            message("Discovering if search strings are repeated");
            tasks::tally(&passed[2]);
            println!();
        } else {
            warn(" Task not recognized ");
        }
    }

    if passed.len() >= 4 {
        warn(" Too many arguments supplied ");
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

// Print help information for using the program
fn about() {
	println!("\n{}", "Usage:".yellow());
	println!("  [program] [task] [month] [site]");
	println!("{}", "\nOptions:".yellow());
	println!("{}    Download Compressed Log Files", " d,  download".green());
	println!("{}	 Unzip Compressed Log Files", " u,  unzip".green());
	println!("{}	 Search for Hits to Target URL", " f,  filter".green());
	println!("{}	 Divide Data into Google and Non-Google Hits", " v,  divide".green());
	println!("{}	 Capture all existing search strings", " c,  capture".green());
	println!("{}	 Discover if search strings are repeated", " a,  analyze".green());
	println!("{}        Help Information", " h,  help".green());
	println!("{}", "\nExample:".yellow());
	println!("  Pointing at your target/release folder, run:");
	println!("{}", "    googlebot f september emergencyinfobc    ".green());
	println!("{}", "\nHelp:".yellow());
	println!("  For more information go to:");
	println!("{}", "    https://github.com/nausicaan/gbots.git".green());
    println!();
}