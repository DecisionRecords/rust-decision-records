extern crate slug;
use slug::slugify;

use crate::config;
use regex::Regex;
use std::fs;

pub fn new_record(
    title: String,
    supersede: String,
    deprecate: String,
    amend: String,
    link: String,
    proposed: bool,
    approved: bool,
    config: config::Config,
) {
    println!("title: {}", title);
    println!("supersede: {}", supersede);
    println!("deprecate: {}", deprecate);
    println!("amend: {}", amend);
    println!("link: {}", link);
    println!("proposed: {}", proposed);
    println!("approved: {}", approved);
    println!("config.record_path: {:?}", config.record_path);
    println!("config.template_path: {:?}", config.template_path);
    println!("config.template_language: {}", config.template_language);
    println!("config.template_file: {}", config.template_file);
    println!("config.template_format: {}", config.template_format);
    println!("config.template_string: {}", config.template_string);
    for (key, value) in config.template_placeholders {
        println!("config.template_placeholders[{}]: {}", key, value);
    }

    let filename_structure = Regex::new(r"^(\d{4})(.*)$").unwrap();
    let mut max_file_prefix: i32 = 0;

    let paths = fs::read_dir(config.record_path);
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
    let mut filename: String = format!("{:0>4}", max_file_prefix);
    filename.push_str("-");
    filename.push_str(&String::from(slugify(title)));
    filename.push_str(".");
    filename.push_str(&config.template_format);
    println!("Create file: {}", filename);
}

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
