extern crate pathdiff;

use pathdiff::diff_paths;
use regex::Regex;
use std::fs::{canonicalize, create_dir_all, remove_file, File};
use std::io::prelude::*;
use std::io::Error;
use std::path::{Path, PathBuf};

fn create_file(filename: PathBuf, content: String) -> Result<(), Error> {
    let bytes_content = content.as_bytes();

    let string_filename = filename.display().to_string();
    let path_filename = Path::new(&string_filename);
    let mut file_object = File::create(path_filename)?;
    file_object.write_all(bytes_content)?;
    return Ok(());
}

fn load_template(language: String, format: String) -> Result<String, Error> {
    let short_language = String::from(&language);
    let re = Regex::new("^([a-zA-Z]+)([-_][a-zA-Z]+|)$").unwrap();
    re.replace(&short_language, "${1}");

    if language == "en" || short_language == "en" {
        if format == "md" {
            return Ok("# NUMBER. TITLE\u{000D}\u{000D}Date: DATE\u{000D}\u{000D}## Status\u{000D}\u{000D}STATUS\u{000D}\u{000D}## Context\u{000D}\u{000D}This is the context.\u{000D}\u{000D}## Decision\u{000D}\u{000D}This is the decision that was made.\u{000D}\u{000D}## Consequence\u{000D}\u{000D}This is the consequence of the decision.\u{000D}".to_string());
        } else if format == "rst" {
            return Ok("#################\u{000D}NUMBER. TITLE\u{000D}#################\u{000D}\u{000D}Date: DATE\u{000D}\u{000D}******\u{000D}Status\u{000D}******\u{000D}\u{000D}STATUS\u{000D}\u{000D}*******\u{000D}Context\u{000D}*******\u{000D}\u{000D}This is the context.\u{000D}\u{000D}********\u{000D}Decision\u{000D}********\u{000D}\u{000D}This is the decision that was made.\u{000D}\u{000D}***********\u{000D}Consequence\u{000D}***********\u{000D}\u{000D}This is the consequence of the decision.\u{000D}".to_string());
        } else {
            panic!("Invalid Language/Format match.");
        }
    } else if language == "fr" || short_language == "fr" {
        if format == "md" {
            return Ok("# NUMBER. TITLE\u{000D}\u{000D}Date: DATE\u{000D}\u{000D}## Statut\u{000D}\u{000D}STATUS\u{000D}\u{000D}## Le contexte\u{000D}\u{000D}C'est le Contexte.\u{000D}\u{000D}## Décision\u{000D}\u{000D}Pris une décision.\u{000D}\u{000D}## Conséquence\u{000D}\u{000D}C'est la conséquence de la décision.\u{000D}".to_string());
        } else {
            panic!("Invalid Language/Format match.");
        }
    } else {
        panic!("Invalid Language/Format match.");
    }
}

pub fn short_init(root_dir: PathBuf, doc_path: PathBuf, force: bool) {
    println!("ADR format");
    let str_root_dir = root_dir.display().to_string();
    let str_doc_path = doc_path.display().to_string();
    let absolute_root_dir = Path::new(&str_root_dir);
    let absolute_doc_path = Path::new(&str_doc_path);
    let mut absolute_config_path = PathBuf::new();

    absolute_config_path.push(absolute_root_dir);
    absolute_config_path.push(".adr-dir");

    let relative_doc_path: String = diff_paths(absolute_doc_path, absolute_root_dir)
        .unwrap()
        .display()
        .to_string();

    println!(
        "root_dir: {}",
        canonicalize(absolute_root_dir)
            .unwrap()
            .display()
            .to_string()
    );
    println!("doc_path: {}", str_doc_path);
    println!("relative_doc_path: {}", relative_doc_path);

    println!("Checking config file");
    if (absolute_config_path.exists() && force) || !absolute_config_path.exists() {
        if absolute_config_path.exists() && force {
            println!("Found, removing...");
            remove_file(&absolute_config_path).unwrap();
        }
        println!("Making config file");
        let create_file = create_file(absolute_config_path, relative_doc_path);
        if create_file.is_ok() {
            println!("Done");
        }
        println!("Checking Doc directory");
        if !absolute_doc_path.exists() {
            println!("Not found...");
            println!("Making Doc directory");
            let create_dir = create_dir_all(absolute_doc_path);
            if create_dir.is_ok() {
                println!("Done");
            }
        } else {
            println!("Doc directory already exists");
        }
    } else {
        panic!("Config file already exists");
    }
}

pub fn init(
    root_dir: PathBuf,
    doc_path: PathBuf,
    template_file: &str,
    format: &str,
    language: &str,
    template_directory: &str,
    default_proposed: bool,
    force: bool,
) {
    println!("Non-ADR format");
    let str_root_dir = root_dir.display().to_string();
    let str_doc_path = doc_path.display().to_string();
    let absolute_root_dir = Path::new(&str_root_dir);
    let absolute_doc_path = Path::new(&str_doc_path);
    let mut absolute_config_path = PathBuf::new();

    absolute_config_path.push(absolute_root_dir);
    absolute_config_path.push(".decisionrecords-config");

    let relative_doc_path = diff_paths(absolute_doc_path, absolute_root_dir)
        .unwrap()
        .display()
        .to_string();

    let mut config_string: String = "".to_string();
    config_string.push_str("records=");
    config_string.push_str(&relative_doc_path);
    config_string.push_str("\u{000D}");

    config_string.push_str("templateDir=");
    config_string.push_str(template_directory);
    config_string.push_str("\u{000D}");

    println!(
        "root_dir: {}",
        canonicalize(absolute_root_dir)
            .unwrap()
            .display()
            .to_string()
    );
    println!("doc_path: {}", str_doc_path);
    println!("relative_doc_path: {}", relative_doc_path);
    println!("template_file: {}", template_file);
    println!("format: {}", format);
    println!("language: {}", language);
    println!("template_directory: {}", template_directory);
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
    let complete_absolute_template_path =
        absolute_template_directory_path.join(&complete_language_template_filename);

    let re = Regex::new("([a-zA-Z]+)([-_][a-zA-Z]+)").unwrap();
    let mut partial_language_template_filename = String::from(template_file);
    partial_language_template_filename.push_str(".");
    partial_language_template_filename.push_str(&re.replace(&language, "${1}"));
    partial_language_template_filename.push_str(".");
    let mut partial_language_template_ref = String::from(&partial_language_template_filename);
    partial_language_template_ref.push_str("ref");
    partial_language_template_filename.push_str(format);
    let partial_absolute_template_path =
        absolute_template_directory_path.join(&partial_language_template_filename);

    let mut no_language_template_filename = String::from(template_file);
    no_language_template_filename.push_str(".");
    let mut no_language_template_ref = String::from(&no_language_template_filename);
    no_language_template_ref.push_str("ref");
    no_language_template_filename.push_str(format);
    let no_absolute_template_path =
        absolute_template_directory_path.join(&no_language_template_filename);

    if language != "" {
        config_string.push_str("language=");
        config_string.push_str(language);
        config_string.push_str("\u{000D}");
    }

    if template_file != "" {
        config_string.push_str("template=");
        config_string.push_str(template_file);
        config_string.push_str("\u{000D}");
    }

    if format != "" {
        config_string.push_str("fileType=");
        config_string.push_str(format);
        config_string.push_str("\u{000D}");
    }

    println!("Checking config file");
    if (absolute_config_path.exists() && force) || !absolute_config_path.exists() {
        if absolute_config_path.exists() && force {
            println!("Found, removing...");
            remove_file(&absolute_config_path).unwrap();
        }
        println!("Checking Doc directory");
        if !absolute_doc_path.exists() {
            println!("Not found...");
            println!("Making Doc directory");
            let create_doc_dir = create_dir_all(absolute_doc_path);
            if create_doc_dir.is_ok() {
                println!("Done");
            }
        } else {
            println!("Doc directory already exists");
        }

        println!("Checking Template directory");
        if !absolute_doc_path.exists() {
            println!("Not found...");
            println!("Making Template directory");
            let create_template_dir = create_dir_all(absolute_template_directory_path);
            if create_template_dir.is_ok() {
                println!("Done");
            }
        } else {
            println!("Template directory already exists");
        }

        println!("Checking Template File");
        if !complete_absolute_template_path.exists()
            && !partial_absolute_template_path.exists()
            && !no_absolute_template_path.exists()
        {
            println!("Not found...");
            let result_template = load_template(language.to_string(), format.to_string());
            if result_template.is_ok() {
                let template_string = result_template.unwrap();
                println!("Writing default template file");
                let create_template = create_file(complete_absolute_template_path, template_string);
                if create_template.is_ok() {
                    println!("Done");
                }
            }
        }

        let create_file = create_file(absolute_config_path, config_string);
        if create_file.is_ok() {
            println!("Done");
        }
    } else {
        panic!("Config file already exists");
    }
}
