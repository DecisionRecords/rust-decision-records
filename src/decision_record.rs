extern crate slug;
use slug::slugify;

use crate::config;
use chrono::Local;
use regex::Regex;
use std::fs::{read_dir, File};
use std::io::prelude::*;
use std::io::Error;
use std::path::{Path, PathBuf};

pub fn new_record(
    title: String,
    supersedes: String,
    deprecates: String,
    amends: String,
    links: String,
    proposed: bool,
    approved: bool,
    config: config::Config,
) {
    // Regex Statements here
    let filename_structure = Regex::new(r".*[\\/](\d{4})([^\\/]*)$").unwrap();
    let re_number = Regex::new("NUMBER").unwrap();
    let re_title = Regex::new("TITLE").unwrap();
    let re_date = Regex::new("DATE").unwrap();
    let re_status = Regex::new("STATUS").unwrap();

    // Default values here
    let mut max_file_prefix: i32 = 0;
    let mut status: String = config.default_status;
    let mut new_file_content: String = String::from(config.template_string);
    let mut absolute_filename: PathBuf = PathBuf::from(&config.record_path.display().to_string());
    let date_now = Local::now().format("%Y-%m-%d");

    // Look throuh the paths and find any files which match the naming convention ([0-9][0-9][0-9][0-9]*)
    // then see if that digit at the start is greater than the calculated "max_file_prefix" and if so,
    // set max_file_prefix to that. Then after looping through all the files, add one to that number.
    let paths = read_dir(&config.record_path);
    if paths.is_ok() {
        for path in paths.unwrap() {
            let str_path = path.unwrap().path().display().to_string();
            if filename_structure.is_match(&str_path) {
                let file_number: i32 = filename_structure.replace(&str_path, "$1").parse().unwrap();
                if max_file_prefix < file_number {
                    max_file_prefix = file_number;
                }
            }
        }
    }
    max_file_prefix = max_file_prefix + 1;

    // Format the file prefix as 4 digits long, zero padded. Add the title, as a slug (unicode characters, replacing symbols with hyphens)
    // and the format, like this `0001-some-title.md`
    let mut filename: String = format!("{:0>4}", max_file_prefix);
    filename.push_str("-");
    filename.push_str(&String::from(slugify(&title)));
    filename.push_str(".");
    filename.push_str(&config.template_format);

    // Add the filename to the record path.
    let this_filename: String = String::from(&filename);
    absolute_filename.push(&filename);

    // Set the status string, if the status is forced (otherwise use the default, pulled from the config)
    if proposed {
        status = "Proposed".to_string();
    } else if approved {
        status = "Approved".to_string();
    }

    // Apply translation to the status value.
    for (key, value) in &config.template_references {
        if String::from(key) == String::from(&status) {
            status = String::from(value);
        }
    }

    // Replace the marker values in the template (NUMBER, TITLE, DATE, STATUS) with their values from above
    new_file_content = re_number
        .replace_all(&String::from(new_file_content), max_file_prefix.to_string())
        .parse()
        .unwrap();
    new_file_content = re_title
        .replace_all(&String::from(new_file_content), &String::from(&title))
        .parse()
        .unwrap();
    new_file_content = re_date
        .replace_all(&String::from(new_file_content), date_now.to_string())
        .parse()
        .unwrap();
    new_file_content = re_status
        .replace_all(&String::from(new_file_content), &String::from(&status))
        .parse()
        .unwrap();

    // Write the file.
    let temp_create_file = create_file(absolute_filename, new_file_content);
    if temp_create_file.is_ok() {
        println!("Created file {}", this_filename);
    }

    // Run all linking activities
    if supersedes.len() > 0 {
        supersede(supersedes, max_file_prefix.to_string());
    }
    if deprecates.len() > 0 {
        deprecate(deprecates, max_file_prefix.to_string());
    }
    if amends.len() > 0 {
        amend(amends, max_file_prefix.to_string());
    }
    if links.len() > 0 {
        link(links, max_file_prefix.to_string(), "Linked".to_string());
    }
}

// Linking activities, referenced either above, or in the main.rs
pub fn approve(records: String) {
    println!("records: {}", records);
}

pub fn proposed(records: String) {
    println!("records: {}", records);
}

pub fn link(from: String, to: String, reason: String) {
    println!("from: {}", from);
    println!("to: {}", to);
    println!("reason: {}", reason);
}

pub fn deprecate(from: String, to: String) {
    println!("from: {}", from);
    println!("to: {}", to);
}

pub fn amend(from: String, to: String) {
    println!("from: {}", from);
    println!("to: {}", to);
}

pub fn supersede(from: String, to: String) {
    println!("from: {}", from);
    println!("to: {}", to);
}

fn create_file(filename: PathBuf, content: String) -> Result<(), Error> {
    // write_all requires bytes. Convert content to bytes.
    let bytes_content = content.as_bytes();
    // Convert the filename to a (temporary) string
    let string_filename = filename.display().to_string();
    // Then convert that into a path
    let path_filename = Path::new(&string_filename);
    // Then create the file object using that path
    let mut file_object = File::create(path_filename)?;
    // And then write everything to the file
    file_object.write_all(bytes_content)?;
    return Ok(());
}
