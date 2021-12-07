extern crate pathdiff;
extern crate slug;
use pathdiff::diff_paths;
use slug::slugify;

use crate::config;
use chrono::Local;
use regex::Regex;
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::prelude::*;
use std::io::{self, Error, ErrorKind};
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
) -> Result<(), Error> {
    // Regex Statements here
    let filename_structure = Regex::new(r".*[\\/](\d{4})([^\\/]*)$").unwrap();
    let re_number = Regex::new("NUMBER").unwrap();
    let re_title = Regex::new("TITLE").unwrap();
    let re_date = Regex::new("DATE").unwrap();
    let re_status = Regex::new("STATUS").unwrap();

    // Default values here
    let mut max_file_prefix: i32 = 0;
    let mut status: String = String::from(&config.default_status);
    let mut new_file_content: String = String::from(&config.template_string);
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
    create_file(&absolute_filename, new_file_content)?;

    // Run all linking activities
    if supersedes.len() > 0 {
        supersede(supersedes, max_file_prefix.to_string())?;
    }
    if deprecates.len() > 0 {
        deprecate(deprecates, max_file_prefix.to_string())?;
    }
    if amends.len() > 0 {
        amend(amends, max_file_prefix.to_string())?;
    }
    if links.len() > 0 {
        link(links, max_file_prefix.to_string(), "Linked".to_string())?;
    }
    println!("Created file {}", this_filename);
    return Ok(());
}

// Linking activities, referenced either above, or in the main.rs
pub fn approve(records: String) -> Result<(), Error> {
    println!("records: {}", records);
    return Ok(());
}

pub fn proposed(records: String) -> Result<(), Error> {
    println!("records: {}", records);
    return Ok(());
}

pub fn link(from: String, to: String, reason: String) -> Result<(), Error> {
    // Create the regexes
    let re_inject_filename_link = Regex::new("(#)").unwrap();
    let re_inject_reason = Regex::new("(%)").unwrap();

    // Load config values
    let config: config::Config = config::load_config().unwrap();
    let template_format = String::from(&config.template_format);
    let config_record_path = PathBuf::from(&config.record_path);

    // Translate some strings, using the config references
    let translated_status_header_string =
        translate_string("Status".to_string(), &config.template_references)?;
    let mut translated_linked_from_string =
        translate_string("Linked to #".to_string(), &config.template_references)?;
    let mut translated_linked_to_string =
        translate_string("Linked to #".to_string(), &config.template_references)?;

    if reason.len() > 0 {
        translated_linked_from_string.push_str(" ");
        translated_linked_from_string.push_str(&translate_string(
            "for reason %".to_string(),
            &config.template_references,
        )?);
        translated_linked_to_string.push_str(" ");
        translated_linked_to_string.push_str(&translate_string(
            "for reason %".to_string(),
            &config.template_references,
        )?);
    }

    // Get the path and the formatted title of the record to be superseded by
    let pathbuf_record_to = find_record(to.parse().unwrap(), &config_record_path)?;
    let title_to: String = formatted_title_and_file_of_record(
        &pathbuf_record_to,
        &config_record_path,
        &template_format,
    )?;
    let mut inject_to_link = re_inject_filename_link
        .replace(&translated_linked_from_string, &title_to)
        .to_string();
    inject_to_link = re_inject_reason
        .replace(&inject_to_link, &reason)
        .to_string();

    // Process the list of records to supersede
    let from_records: Vec<&str> = from.split_terminator(',').collect();
    for record in from_records {
        // Get the path and formatted title of the record to supersede
        let pathbuf_record_from = find_record(record.parse().unwrap(), &config_record_path)?;
        let title_from: String = formatted_title_and_file_of_record(
            &pathbuf_record_from,
            &config_record_path,
            &template_format,
        )?;
        let mut inject_from_link = re_inject_filename_link
            .replace(&translated_linked_to_string, title_from)
            .to_string();
        inject_from_link = re_inject_reason
            .replace(&inject_from_link, &reason)
            .to_string();

        // Update the "from" and "to" records with the respective amended and amends links
        inject_text_in_status_block_of_a_record(
            &pathbuf_record_from,
            &translated_status_header_string,
            &inject_to_link,
            false,
            false,
            &[],
        )?;
        inject_text_in_status_block_of_a_record(
            &pathbuf_record_to,
            &translated_status_header_string,
            &inject_from_link,
            false,
            false,
            &[],
        )?;
    }
    return Ok(());
}

pub fn deprecate(from: String, to: String) -> Result<(), Error> {
    // Create the regexes
    let re_inject_filename_link = Regex::new("(#)").unwrap();

    // Load config values
    let config: config::Config = config::load_config().unwrap();
    let template_format = String::from(&config.template_format);
    let config_record_path = PathBuf::from(&config.record_path);

    // Translate some strings, using the config references
    let translated_status_header_string =
        translate_string("Status".to_string(), &config.template_references)?;
    let translated_deprecated_string =
        translate_string("Deprecated by #".to_string(), &config.template_references)?;
    let translated_deprecates_string =
        translate_string("Deprecates #".to_string(), &config.template_references)?;
    let slice_prune_strings: &[&String] = &[
        &translate_string("Approved".to_string(), &config.template_references).unwrap(),
        &translate_string("Proposed".to_string(), &config.template_references).unwrap(),
    ];

    // Get the path and the formatted title of the record to be deprecated by
    let pathbuf_record_to = find_record(to.parse().unwrap(), &config_record_path)?;
    let title_to: String = formatted_title_and_file_of_record(
        &pathbuf_record_to,
        &config_record_path,
        &template_format,
    )?;
    let inject_to_link = re_inject_filename_link
        .replace(&translated_deprecated_string, &title_to)
        .to_string();

    // Process the list of records to deprecate
    let from_records: Vec<&str> = from.split_terminator(',').collect();
    for record in from_records {
        // Get the path and formatted title of the record to deprecate
        let pathbuf_record_from = find_record(record.parse().unwrap(), &config_record_path)?;
        let title_from: String = formatted_title_and_file_of_record(
            &pathbuf_record_from,
            &config_record_path,
            &template_format,
        )?;
        let inject_from_link = re_inject_filename_link
            .replace(&translated_deprecates_string, title_from)
            .to_string();

        // Update the "from" and "to" records with the respective deprecate and deprecates links
        inject_text_in_status_block_of_a_record(
            &pathbuf_record_from,
            &translated_status_header_string,
            &inject_to_link,
            false,
            false,
            &slice_prune_strings,
        )?;
        inject_text_in_status_block_of_a_record(
            &pathbuf_record_to,
            &translated_status_header_string,
            &inject_from_link,
            false,
            false,
            &[],
        )?;
    }
    return Ok(());
}

pub fn amend(from: String, to: String) -> Result<(), Error> {
    // Create the regexes
    let re_inject_filename_link = Regex::new("(#)").unwrap();

    // Load config values
    let config: config::Config = config::load_config().unwrap();
    let template_format = String::from(&config.template_format);
    let config_record_path = PathBuf::from(&config.record_path);

    // Translate some strings, using the config references
    let translated_status_header_string =
        translate_string("Status".to_string(), &config.template_references)?;
    let translated_amended_string =
        translate_string("Amended by #".to_string(), &config.template_references)?;
    let translated_amends_string =
        translate_string("Amends #".to_string(), &config.template_references)?;

    // Get the path and the formatted title of the record to be superseded by
    let pathbuf_record_to = find_record(to.parse().unwrap(), &config_record_path)?;
    let title_to: String = formatted_title_and_file_of_record(
        &pathbuf_record_to,
        &config_record_path,
        &template_format,
    )?;
    let inject_to_link = re_inject_filename_link
        .replace(&translated_amended_string, &title_to)
        .to_string();

    // Process the list of records to supersede
    let from_records: Vec<&str> = from.split_terminator(',').collect();
    for record in from_records {
        // Get the path and formatted title of the record to supersede
        let pathbuf_record_from = find_record(record.parse().unwrap(), &config_record_path)?;
        let title_from: String = formatted_title_and_file_of_record(
            &pathbuf_record_from,
            &config_record_path,
            &template_format,
        )?;
        let inject_from_link = re_inject_filename_link
            .replace(&translated_amends_string, title_from)
            .to_string();

        // Update the "from" and "to" records with the respective amended and amends links
        inject_text_in_status_block_of_a_record(
            &pathbuf_record_from,
            &translated_status_header_string,
            &inject_to_link,
            false,
            false,
            &[],
        )?;
        inject_text_in_status_block_of_a_record(
            &pathbuf_record_to,
            &translated_status_header_string,
            &inject_from_link,
            false,
            false,
            &[],
        )?;
    }
    return Ok(());
}

pub fn supersede(from: String, to: String) -> Result<(), Error> {
    // Create the regexes
    let re_inject_filename_link = Regex::new("(#)").unwrap();

    // Load config values
    let config: config::Config = config::load_config().unwrap();
    let template_format = String::from(&config.template_format);
    let config_record_path = PathBuf::from(&config.record_path);

    // Translate some strings, using the config references
    let translated_status_header_string =
        translate_string("Status".to_string(), &config.template_references)?;
    let translated_superseded_string =
        translate_string("Superseded by #".to_string(), &config.template_references)?;
    let translated_supersedes_string =
        translate_string("Supersedes #".to_string(), &config.template_references)?;
    let slice_prune_strings: &[&String] = &[
        &translate_string("Approved".to_string(), &config.template_references).unwrap(),
        &translate_string("Proposed".to_string(), &config.template_references).unwrap(),
    ];

    // Get the path and the formatted title of the record to be superseded by
    let pathbuf_record_to = find_record(to.parse().unwrap(), &config_record_path)?;
    let title_to: String = formatted_title_and_file_of_record(
        &pathbuf_record_to,
        &config_record_path,
        &template_format,
    )?;
    let inject_to_link = re_inject_filename_link
        .replace(&translated_superseded_string, &title_to)
        .to_string();

    // Process the list of records to supersede
    let from_records: Vec<&str> = from.split_terminator(',').collect();
    for record in from_records {
        // Get the path and formatted title of the record to supersede
        let pathbuf_record_from = find_record(record.parse().unwrap(), &config_record_path)?;
        let title_from: String = formatted_title_and_file_of_record(
            &pathbuf_record_from,
            &config_record_path,
            &template_format,
        )?;
        let inject_from_link = re_inject_filename_link
            .replace(&translated_supersedes_string, title_from)
            .to_string();

        // Update the "from" and "to" records with the respective supersede and supersedes links
        inject_text_in_status_block_of_a_record(
            &pathbuf_record_from,
            &translated_status_header_string,
            &inject_to_link,
            false,
            false,
            &slice_prune_strings,
        )?;
        inject_text_in_status_block_of_a_record(
            &pathbuf_record_to,
            &translated_status_header_string,
            &inject_from_link,
            false,
            false,
            &[],
        )?;
    }
    return Ok(());
}

// Internal functions for use in this crate
fn translate_string(
    needle_string: String,
    haystack_kv: &HashMap<String, String>,
) -> Result<String, Error> {
    for (key, value) in haystack_kv {
        if String::from(key) == String::from(&needle_string) {
            return Ok(String::from(value));
        }
    }
    return Ok(needle_string);
}

// Create a file named with the variable `filename`, populated with content of the variable `content`.
fn create_file(filename: &PathBuf, content: String) -> Result<(), Error> {
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

// Find the file which starts with the 4 character, zero padded string held in number in this directory.
fn find_record(number: i32, config_record_path: &PathBuf) -> Result<PathBuf, Error> {
    // Define the error message we will return if we can't find the file matching the defined structure.
    let err_not_found = Error::new(ErrorKind::NotFound, "Required file was not found");

    // Create a regex, starting by looking for the exact start of the file, an up-to 4-zero-padded string ending with the variable `number`.
    // e.g. 0001, 0099, 0101, 1234
    let mut str_find_file: String = format!("{:0>4}", number);
    // Add the rest of the regex, looking for anything which is not a back-or-forward slash following a hyphen.
    str_find_file.push_str(r"-[^\\/]*");
    // Then create the regex, using this pattern we've just created.
    let re_find_file = Regex::new(&str_find_file).unwrap();
    // Step through the directory
    let paths = read_dir(config_record_path)?;
    for path in paths {
        // Convert the path into a string we can match with the regex.
        let str_path = path.unwrap().path().display().to_string();
        if re_find_file.is_match(&str_path) {
            // If it matched, turn the string back into a PathBuf and return it.
            return Ok(PathBuf::from(str_path));
        }
    }
    // Otherwise return the error we defined at the start of this function.
    return Err(err_not_found);
}

// This function endevours to read the content of a file, find a search string, and then either inject the string at the start or end of that block
// or replace the whole string entirely.
fn inject_text_in_status_block_of_a_record(
    pathbuf_of_file: &PathBuf,
    translated_status_heading_string: &String,
    inject_line: &String,
    start_of_block: bool,
    replace_block: bool,
    remove_strings: &[&String],
) -> Result<(), Error> {
    let mut temp_file_content: String = String::from("");
    let mut filetype: String = String::from("undefined");
    let mut bool_in_block: bool = false;
    let mut bool_found_search_line: bool = false; // Only used in RST files
    let mut bool_after_block: bool = false;
    let mut last_line: String = String::from("");

    let re_filetype_md = Regex::new(r"\.md$").unwrap();
    let re_filetype_rst = Regex::new(r"\.rst$").unwrap();
    let mut re_search_line = Regex::new(&String::from(translated_status_heading_string)).unwrap();
    let mut re_find_delimiter = Regex::new("").unwrap();

    if re_filetype_md.is_match(&pathbuf_of_file.display().to_string()) {
        filetype = String::from("md");
        re_find_delimiter = Regex::new(r"^\s*#+\s+\S").unwrap();
        let mut str_search_line: String = String::from(r"^\s*#+\s+");
        str_search_line.push_str(&translated_status_heading_string);
        str_search_line.push_str(r"\s*$");
        re_search_line = Regex::new(&str_search_line).unwrap();
    } else if re_filetype_rst.is_match(&pathbuf_of_file.display().to_string()) {
        filetype = String::from("rst");
        re_find_delimiter = Regex::new(r"^\s*([*]+|[#]+)\s*$").unwrap();
    }

    if let Ok(lines) = get_lines_from_a_file(&pathbuf_of_file) {
        for line in lines {
            if let Ok(line) = line {
                let mut prune_line: bool = false;
                for prune_item in remove_strings {
                    let mut str_prune_item: String = String::from(r"^\s*");
                    str_prune_item.push_str(prune_item);
                    str_prune_item.push_str(r"\s*.*$");
                    let re_prune_item = Regex::new(&str_prune_item).unwrap();
                    if re_prune_item.is_match(&line) {
                        prune_line = true;
                    }
                }
                if !prune_line {
                    if !(last_line.len() == 0 && line.len() == 0) {
                        if bool_after_block {
                            temp_file_content.push_str(&line);
                            temp_file_content.push_str("\u{000A}");
                            last_line = line;
                        } else if bool_in_block {
                            if re_find_delimiter.is_match(&line) {
                                bool_after_block = true;
                                if start_of_block == false {
                                    temp_file_content.push_str(&inject_line);
                                    temp_file_content.push_str("\u{000A}\u{000A}");
                                }
                            }
                            temp_file_content.push_str(&line);
                            temp_file_content.push_str("\u{000A}");
                            last_line = line;
                        } else if filetype == "md" {
                            temp_file_content.push_str(&line);
                            temp_file_content.push_str("\u{000A}");
                            if re_search_line.is_match(&line) {
                                bool_in_block = true;
                                if start_of_block || replace_block {
                                    temp_file_content.push_str("\u{000A}");
                                    temp_file_content.push_str(&inject_line);
                                    temp_file_content.push_str("\u{000A}");
                                    if replace_block {
                                        bool_after_block = true;
                                        temp_file_content.push_str("\u{000A}");
                                    }
                                }
                            }
                            last_line = line;
                        } else if filetype == "rst" {
                            temp_file_content.push_str(&line);
                            temp_file_content.push_str("\u{000A}");

                            if re_search_line.is_match(&line) {
                                bool_found_search_line = true;
                            } else if bool_found_search_line {
                                bool_in_block = true;
                                if start_of_block || replace_block {
                                    temp_file_content.push_str("\u{000A}");
                                    temp_file_content.push_str(&inject_line);
                                    temp_file_content.push_str("\u{000A}");
                                    if replace_block {
                                        bool_after_block = true;
                                        temp_file_content.push_str("\u{000A}");
                                    }
                                }
                            }
                            last_line = line;
                        }
                    }
                }
            }
        }
    }

    create_file(pathbuf_of_file, temp_file_content)?;
    return Ok(());
}

// Based on https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn get_lines_from_a_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// This function turns a PathBuf object into a markdown link. If the Object is readable as a Markdown link or a Restructured Text
// file, with the title as the first heading, then it will encapsulate the title in the relevant formatted link.
fn formatted_title_and_file_of_record(
    pathbuf_of_record: &PathBuf,
    base_path: &PathBuf,
    format: &String,
) -> Result<String, Error> {
    // Define Regexes
    let re_title_md = Regex::new(r"^# (\d+\.?\s+.*)\s?$").unwrap();
    let re_title_rst = Regex::new(r"^\s*[\*#]\s*+$").unwrap();

    // Set flags
    let mut past_delimiter: bool = false; // Used for RST only

    // Calculate the relative path from the pathbuf to the base path
    let relative_path_to = diff_paths(pathbuf_of_record, base_path)
        .unwrap()
        .display()
        .to_string();

    // Build a default "title" to return
    let mut link_text: String = String::from(&relative_path_to);

    // Replace it with a formatted version, if we are working with a known file format
    if format == "md" {
        link_text = String::from("[");
        link_text.push_str(&relative_path_to);
        link_text.push_str("](");
        link_text.push_str(&relative_path_to);
        link_text.push_str(")");
    } else if format == "rst" {
        link_text = String::from(":doc:`");
        link_text.push_str(&relative_path_to);
        link_text.push_str(" <");
        link_text.push_str(&relative_path_to);
        link_text.push_str(">`");
    }

    // Then loop through the pathbuf to replace the file format
    if let Ok(lines) = get_lines_from_a_file(pathbuf_of_record) {
        for line in lines {
            if let Ok(line) = line {
                if format == "md" {
                    if re_title_md.is_match(&line) {
                        link_text = re_title_md.replace(&line, "[$1](").to_string();
                        link_text.push_str(&relative_path_to);
                        link_text.push_str(")");
                        return Ok(link_text);
                    }
                } else if format == "rst" {
                    if past_delimiter {
                        link_text = String::from(":doc:`");
                        link_text.push_str(&line);
                        link_text.push_str(" <");
                        link_text.push_str(&relative_path_to);
                        link_text.push_str(">`");
                        return Ok(link_text);
                    } else if re_title_rst.is_match(&line) {
                        past_delimiter = true;
                    }
                }
            }
        }
    }

    // If we couldn't find the title, return at least a formatted link string
    return Ok(link_text);
}
