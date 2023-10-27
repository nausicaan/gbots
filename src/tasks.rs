
// use std::io::{BufRead, BufReader, Result, Write, Read};
use std::{
    fs, fs::File,
    io::{stdout, Write, BufRead, BufReader, Result, Read},
    collections::HashMap,
    process::Command,
    thread::sleep,
    time::Duration
};
use flate2::read::GzDecoder;
use colored::Colorize;
mod vars;

// Insert some key-value pairs into days
pub static NAMES: [&str; 12] = ["january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "novemeber", "december"];

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

    digits.push(String::from("01"));
    digits.push(String::from("02"));
    digits.push(String::from("03"));
    digits.push(String::from("04"));
    digits.push(String::from("05"));
    digits.push(String::from("06"));
    digits.push(String::from("07"));
    digits.push(String::from("08"));
    digits.push(String::from("09"));

    for element in months.iter() {
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
    for server in vars::SERVERS.iter() {
        let destination: String = vars::PREFIX.to_owned() + server + "/zipped/" + month;
        let mut stdout = stdout();
        let mut index: usize = 0;

        for day in days.iter() {
            let source: String = String::from(vars::IDENTITY.to_owned() + server + "/nginx_access.log-2023" + &day + ".gz ");
            Command::new("scp")
            .arg(&source)
            .arg(&destination)
            .spawn()
            .expect("scp command failed to start");
            print!("\rFile {}{}{} ( {} of {} ) in {} dowloaded   ", "nginx_access.log-2023".green(), months[month].green(), days[index].green(), (index + 1), days.len(), server.yellow());
            stdout.flush().unwrap();
            sleep(Duration::from_millis(100));
            index += 1;
        }
    }
}


pub fn unzip(month: &String) {
    for server in vars::SERVERS.iter() {
        let mut index: usize = 0;
        let files: Vec<String> = directory(vars::PREFIX.to_owned() + server  + "/zipped/" + month);
        let mut stdout = stdout();
        let total: usize = files.len();

        for file in files.iter() {
            let result: &str = &file.replace(".gz", "");
            let result: &str = &result.replace(".log-", "_");
            let result = result.to_owned() + ".log";
            decompress(vars::PREFIX.to_owned() + server  + "/zipped/" + month + "/" + &file, vars::PREFIX.to_owned() + server + "/unzipped/" + month + "/" + &result);
            print!("\rFile {} ( {} of {} ) in {} unzipped   ", file.green(), (index + 1), total, server.yellow());
            stdout.flush().unwrap();
            sleep(Duration::from_millis(100));
            index += 1;
        }
    }
}


pub fn manipulate(action: &str, month: &String, source: &str, site: &str) {

    for server in vars::SERVERS.iter() {

        let files: Vec<String> = directory(vars::PREFIX.to_owned() + server + source + month);
        let mut stdout = stdout();
        let total: usize = files.len();
        let mut index: usize = 0;

        for file in files.iter() {
            if action == "divided" {
                let _ = separate(&server, &file, &month);
            } else {
                let _ = isolate(&server, &file, &month, &site);
            }
            print!("\rFile {} ( {} of {} ) in {} {}   ", file.green(), (index + 1), total, server.yellow(), action);
            stdout.flush().unwrap();
            sleep(Duration::from_millis(100));
            index += 1;
        }
    }
}


/* ---------- Private Functions ---------- */


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

    let writefile: String = vars::PREFIX.to_owned() + server + "/filtered/" + "/" + month + "/" + filename;
    let _ = iterwrite(&eibc, &writefile, "");

    Ok(())
}


// Divide Data into Google and Non-Google Hits
fn separate(server: &str, filename: &String, month: &String) -> Result<()> {
    let readfile: String = vars::PREFIX.to_owned() + server + "/filtered/" + "/" + month + "/" + filename;
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

    let trimfile: String = vars::PREFIX.to_owned() + server + "/divided/" + "/" + month + "/" + filename;
    let slash:Option<&str> = trimfile.strip_suffix(".log") ;
    let writefile: &str = slash.unwrap_or_default();
    let _ = iterwrite(&gbot, &writefile, "_google.log");
    let _ = iterwrite(&obot, &writefile, "_other.log");

    Ok(())
}


fn iterwrite(container: &Vec<String>, destination: &str, extension: &str) -> Result<()> {
    let mut f2: File = File::create(destination.to_owned() + extension).expect("Unable to create file");

    for element in container.iter() {
        writeln!(f2, "{}", element)?;
    }

    Ok(())
}