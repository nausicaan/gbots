use std::{collections::{HashMap, HashSet}, fs::{self, File, OpenOptions}, io::{stdout, BufRead, BufReader, Read, Result, Write}, process::Command, thread::sleep, time::Duration};
use flate2::read::GzDecoder;
use colored::Colorize;
pub mod vars;

// Insert some key-value pairs into days
pub static NAMES: [&str; 12] = ["january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december"];

// Create a slice with the appropriate number of days for that month
pub fn generate(months: &HashMap<String, String>, month: &String) -> Vec<String> {
    let mut days: HashMap<String, i32> = HashMap::new();
    let mut digits: Vec<String> = Vec::new();
    let mut counter: i32 = 10;

    // Insert some key-value pairs into days
    days.insert(String::from(NAMES[0]), 31);
    days.insert(String::from(NAMES[1]), 28);
    days.insert(String::from(NAMES[2]), 31);
    days.insert(String::from(NAMES[3]), 30);
    days.insert(String::from(NAMES[4]), 31);
    days.insert(String::from(NAMES[5]), 30);
    days.insert(String::from(NAMES[6]), 31);
    days.insert(String::from(NAMES[7]), 31);
    days.insert(String::from(NAMES[8]), 30);
    days.insert(String::from(NAMES[9]), 31);
    days.insert(String::from(NAMES[10]), 30);
    days.insert(String::from(NAMES[11]), 31);

    // Push leading zero digits 01 - 09
    digits.push(String::from("01"));
    digits.push(String::from("02"));
    digits.push(String::from("03"));
    digits.push(String::from("04"));
    digits.push(String::from("05"));
    digits.push(String::from("06"));
    digits.push(String::from("07"));
    digits.push(String::from("08"));
    digits.push(String::from("09"));

    for element in months {
        if month == element.0 {
            while counter <= days[element.0] {
                digits.push(counter.to_string());
                counter += 1;
            }
        }
    }
    digits
}

// Download the log files from freedom.bcgov
pub fn download(months: &HashMap<String, String>, month: &String, days: &Vec<String>) {

    for server in vars::SERVERS {
        let destination: String = vars::PREFIX.to_owned() + server + "/zipped/" + month;

        for day in days {
            let source: String = String::from(vars::IDENTITY.to_owned() + server + "/nginx_access.log-2024" + &months[month] + &day + ".gz");
            sleep(Duration::from_millis(200));
            Command::new("scp")
            .args([&source, &destination])
            .spawn()
            .expect("scp command failed to start");
        }
    }
}


// Extract data from .gz compressed files
pub fn unzip(month: &String) {

    for server in vars::SERVERS {
        let mut index: usize = 0;
        let files: Vec<String> = directory(vars::PREFIX.to_owned() + server  + "/zipped/" + month);
        let mut stdout = stdout();
        let total: usize = files.len();

        for file in files {
            sleep(Duration::from_millis(200));
            let result: &str = &file.replace(".gz", "");
            let result: &str = &result.replace(".log-", "_");
            let result = result.to_owned() + ".log";
            decompress(vars::PREFIX.to_owned() + server  + "/zipped/" + month + "/" + &file, vars::PREFIX.to_owned() + server + "/unzipped/" + month + "/" + &result);
            stdout.flush().unwrap();
            index += 1;
            print!("Unzipped file {} ( {} of {} ) in {}\n", file.green(), (index), total, server.yellow());
        }
    }
}


// Direct the program towards either the separate or isolate function depending on objective
pub fn manipulate(action: &str, month: &String, source: &str, site: &str) {

    for server in vars::SERVERS {
        let files: Vec<String> = directory(vars::PREFIX.to_owned() + server + "/" + source + "/" + month);
        let mut stdout = stdout();
        let total: usize = files.len();
        let mut index: usize = 0;

        for file in files {
            sleep(Duration::from_millis(200));
            if action == "Divided" {
                let _ = separate(&server, &file, &month);
            } else {
                let _ = isolate(&server, &file, &month, &site);
            }
            stdout.flush().unwrap();
            index += 1;
            print!("{} file {} ( {} of {} ) in {}\n", action, file.green(), (index), total, server.yellow());
        }
    }
}


// Discover the different types of search strings in use
pub fn pattern(month: &String) {
    
    for server in vars::SERVERS {
        let files: Vec<String> = directory(vars::PREFIX.to_owned() + server + "/divided/" + month);
        let mut stdout = stdout();
        let total: usize = files.len();
        let mut index: usize = 0;

        for file in files {
            sleep(Duration::from_millis(200));
            let _ = search(server, &file, month);
            stdout.flush().unwrap();
            index += 1;
            print!("Captured file {} ( {} of {} ) in {}\n", file.green(), (index), total, server.yellow());
        }
    }
}


// Collate all the search strings to find the most popular
pub fn tally(month: &String) {
    
    for server in vars::SERVERS {
        let files: Vec<String> = directory(vars::PREFIX.to_owned() + server + "/captured/" + month);
        let mut stdout = stdout();
        let total: usize = files.len();
        let mut index: usize = 0;

        for file in files {
            sleep(Duration::from_millis(200));
            if let Ok(s) = transform(vars::PREFIX.to_owned() + server + "/captured/" + month + "/" + &file) {
                let trimfile: String = vars::PREFIX.to_owned() + server + "/analyzed/" + month + "/" + &file;
                let slash:Option<&str> = trimfile.strip_suffix(".log");
                let writefile: &str = slash.unwrap_or_default();

                let mut headrow: File = File::create(writefile.to_owned() + ".csv").expect("Unable to create file");
                let _ = headrow.write("Count,Search_String\n".as_bytes());

                let duplicates: Vec<String> = doppleganger(&s);
                for d in duplicates {
                    let occurrences: usize = s.clone().iter().filter(|&s| s == &d).count();
                    let data_to_append: String = occurrences.to_string() + "," + &d + "\n";
                    let _ = dictate(&data_to_append, writefile);
                }
            }
            stdout.flush().unwrap();
            index += 1;
            print!("Analyzed file {} ( {} of {} ) in {}\n", file.green(), (index), total, server.yellow());
        }
    }
}


/* ---------- Private Functions ---------- */


// Read the contents of a folder
fn directory(location: String) -> Vec<String> {

    let mut nginx: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(location) {
        for entry in entries {
            if let Ok(entry) = entry {
                let filename: std::ffi::OsString = entry.file_name();
                nginx.push(filename.to_string_lossy().into_owned());
            }
        }
    } else {
        println!("Failed to read directory contents.");
    }
    nginx
}


// Decompress a zipped file and write the value to a new file
fn decompress(readfile: String, writefile: String) {
    // Open the .gz file
    let file: File = File::open(readfile).expect("Unable to open file");

    // Create a GzDecoder
    let mut decoder: GzDecoder<File> = GzDecoder::new(file);

    // Read the decompressed data
    let mut decompressed_data: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut decompressed_data).expect("Unable to read data");

    // Convert decompressed data to string (if it's text data)
    let content: String = String::from_utf8(decompressed_data).expect("Invalid UTF-8");

    let mut file: File = File::create(writefile).expect("Unable to create file");
    file.write_all(content.as_bytes()).expect("Unable to write to file");
}


// Search for Hits to the supplied Target URL
fn isolate(server: &str, filename: &String, month: &String, site: &str) -> Result<()> {

    let readfile: String = vars::PREFIX.to_owned() + server + "/unzipped/" + month + "/" + filename;
    let openfile: File = File::open(&readfile).expect("Unable to open file");

    let reader: BufReader<File> = BufReader::new(openfile);
    let mut eibc: Vec<String> = Vec::new();

    for line in reader.lines() {
        let instance: String = line?;
        if instance.contains(site) {
            eibc.push(instance);
        }
    }

    let writefile: String = vars::PREFIX.to_owned() + server + "/filtered/" + month + "/" + filename;
    let _ = iterwrite(&eibc, &writefile, "");

    Ok(())
}


// Divide Data into Google and Non-Google Hits
fn separate(server: &str, filename: &String, month: &String) -> Result<()> {

    let readfile: String = vars::PREFIX.to_owned() + server + "/filtered/" + month + "/" + filename;
    let openfile: File = File::open(&readfile).expect("Unable to open file");
    let reader: BufReader<File> = BufReader::new(openfile);
    let mut obot: Vec<String> = Vec::new();
    let mut gbot: Vec<String> = Vec::new();

    for line in reader.lines() {
        let instance: String = line?;
        if instance.contains("Googlebot") {
            gbot.push(instance);
        } else {
            obot.push(instance);
        }
    }

    let trimfile: String = vars::PREFIX.to_owned() + server + "/divided/" + month + "/" + filename;
    let slash:Option<&str> = trimfile.strip_suffix(".log");
    let writefile: &str = slash.unwrap_or_default();
    let _ = iterwrite(&gbot, &writefile, "_google.log");
    let _ = iterwrite(&obot, &writefile, "_others.log");

    Ok(())
}


// Search for legitimate HTTP request data
fn search(server: &str, filename: &String, month: &String) -> Result<()> {

    let readfile: String = vars::PREFIX.to_owned() + server + "/divided/" + "/" + month + "/" + filename;
    let openfile: File = File::open(&readfile).expect("Unable to open file");
    let reader: BufReader<File> = BufReader::new(openfile);
    let mut extracted: Vec<String> = Vec::new();
    let mut gotcha: String;

    for line in reader.lines() {
        let instance: String = line?;
        if instance.contains("\"GET ") {
            gotcha = extract(instance, "\"GET ", " HTTP/");
        } else if instance.contains("\"POST ") {
            gotcha = extract(instance, "\"POST ", " HTTP/");
        } else if instance.contains("\"HEAD ") {
            gotcha = extract(instance, "\"HEAD ", " HTTP/");
        } else {
            gotcha = extract(instance, "] \"", "\"");
        }
        extracted.push(gotcha);
    }

    let trimfile: String = vars::PREFIX.to_owned() + server + "/captured/" + month + "/" + filename;
    let slash:Option<&str> = trimfile.strip_suffix(".log");
    let writefile: &str = slash.unwrap_or_default();
    let _ = iterwrite(&extracted, &writefile, "_strings.log");

    Ok(())
}


// Pull out only specified data
fn extract(part: String, focus: &str, delim: &str) -> String {
    let mut gotcha: String = String::from("");
    match part.split_once(focus) {
        Some((_first, second)) => {
            match second.split_once(delim) {
                Some((first, _second)) => {
                    gotcha = first.to_string();
                }
                None => println!("Inner: {}", part)
            }
        },
        None => { println!("Outer: {}", part);
        },
    }
    gotcha
}


// Read a file line by line and append the results to a String Vector
fn transform(filepath: String) -> Result<Vec<String>> {
    let f: File = File::open(&filepath).expect("Unable to open file");
    let reader: BufReader<File> = BufReader::new(f);
    let mut contents: Vec<String> = Vec::new();

    for line in reader.lines() {
        contents.push(line?);
    }

    Ok(contents)
}


// Search a vector for duplicate items
fn doppleganger<T: Eq + std::hash::Hash + Clone>(vec: &Vec<T>) -> Vec<T> {
    let mut seen: HashSet<T> = HashSet::new();
    let mut duplicates: HashSet<T> = HashSet::new();
    let mut result: Vec<T> = Vec::new();

    for item in vec {
        if seen.contains(&item) {
            duplicates.insert(item.clone());
        }
        seen.insert(item.clone());
    }

    for item in duplicates {
        result.push(item);
    }

    result
}


// Write to a file
fn iterwrite(contents: &Vec<String>, destination: &str, extension: &str) -> Result<()> {
    let mut f2: File = File::create(destination.to_owned() + extension).expect("Unable to create file");

    for element in contents {
        writeln!(f2, "{}", element)?;
    }

    Ok(())
}

fn dictate(data: &str, destination: &str) -> std::io::Result<()> {
    // Open the file in append mode, creating it if it doesn't exist
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(destination.to_owned() + ".csv")?;

    // Write the data to the file
    file.write_all(data.as_bytes())?;

    Ok(())
}