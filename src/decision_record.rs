use crate::config;

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
