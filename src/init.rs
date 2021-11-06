extern crate pathdiff;

use pathdiff::diff_paths;
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;

pub fn init(
    root_dir: PathBuf,
    doc_path: PathBuf,
    template_file: &str,
    format: &str,
    language: &str,
    template_directory: &str,
    adr_format: bool,
    default_proposed: bool,
    force: bool,
) {
    let str_root_dir = root_dir.display().to_string();
    let str_doc_path = doc_path.display().to_string();
    let absolute_root_dir = Path::new(&str_root_dir);
    let absolute_doc_path = Path::new(&str_doc_path);

    let relative_doc_path = diff_paths(absolute_doc_path, absolute_root_dir)
        .unwrap()
        .display()
        .to_string();

    println!("root_dir: {}", fs::canonicalize(absolute_root_dir).unwrap().display().to_string());
    println!("doc_path: {}", str_doc_path);
    println!("relative_doc_path: {}", relative_doc_path);
    println!("template_file: {}", template_file);
    println!("format: {}", format);
    println!("language: {}", language);
    println!("template_directory: {}", template_directory);
    println!("adr_format: {}", adr_format);
    println!("default_proposed: {}", default_proposed);
    println!("force: {}", force);

    let absolute_template_directory_path = absolute_root_dir.join(template_directory);
    let mut complete_language_template_filename = String::from(template_file);
    complete_language_template_filename.push_str(".");
    complete_language_template_filename.push_str(language);
    complete_language_template_filename.push_str(".");
    let mut complete_language_template_ref = String::from(&complete_language_template_filename);
    complete_language_template_ref.push_str("ref");
    complete_language_template_filename.push_str(format);
    let complete_absolute_template_path = absolute_template_directory_path.join(&complete_language_template_filename);

    let re = Regex::new("([a-zA-Z]+)([-_][a-zA-Z]+)").unwrap();
    let mut partial_language_template_filename = String::from(template_file);
    partial_language_template_filename.push_str(".");
    partial_language_template_filename.push_str(&re.replace(&language, "${1}"));
    partial_language_template_filename.push_str(".");
    let mut partial_language_template_ref = String::from(&partial_language_template_filename);
    partial_language_template_ref.push_str("ref");
    partial_language_template_filename.push_str(format);
    let partial_absolute_template_path = absolute_template_directory_path.join(&partial_language_template_filename);

    let mut no_language_template_filename = String::from(template_file);
    no_language_template_filename.push_str(".");
    let mut no_language_template_ref = String::from(&no_language_template_filename);
    no_language_template_ref.push_str("ref");
    no_language_template_filename.push_str(format);
    let no_absolute_template_path = absolute_template_directory_path.join(&no_language_template_filename);

    println!("Making Doc directory");
    let mut create_dir = fs::create_dir_all(absolute_doc_path);
    if create_dir.is_ok() {
        println!("Done");
    }
    println!("Making Template directory");
    create_dir = fs::create_dir_all(absolute_template_directory_path.display().to_string());
    if create_dir.is_ok() {
        println!("Done");
    }
    // Based on https://stackoverflow.com/a/32384768
    // let template_exists = .exists()
    println!("Complete: {} Partial: {} No: {}", complete_absolute_template_path.display().to_string(), partial_absolute_template_path.display().to_string(), no_absolute_template_path.display().to_string());
}
