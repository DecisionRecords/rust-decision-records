extern crate pathdiff;

use pathdiff::diff_paths;
use regex::Regex;
use std::fs::{canonicalize, create_dir_all, remove_file, File};
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

// In here we have two versions - the "short init" for use with the adr format
// and the "normal init" used with the new decision-record format directory.

// TODO: Document this script
pub fn short_init(root_dir: PathBuf, doc_path: PathBuf, force: bool) -> Result<(), Error> {
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

    if (absolute_config_path.exists() && force) || !absolute_config_path.exists() {
        if absolute_config_path.exists() && force {
            remove_file(&absolute_config_path).unwrap();
        }
        create_file(absolute_config_path, relative_doc_path).unwrap();

        if !absolute_doc_path.exists() {
            create_dir_all(absolute_doc_path).unwrap();
        }
    } else {
        panic!("Unable to initialize directory, config file already exists");
    }
    return Ok(());
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
) -> Result<(), Error> {
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
    config_string.push_str("\u{000A}");

    config_string.push_str("templateDir=");
    config_string.push_str(template_directory);
    config_string.push_str("\u{000A}");

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
    let complete_language_template_ref_path =
        absolute_template_directory_path.join(&complete_language_template_ref);

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
    let partial_language_template_ref_path =
        absolute_template_directory_path.join(&partial_language_template_ref);

    let mut no_language_template_filename = String::from(template_file);
    no_language_template_filename.push_str(".");
    let mut no_language_template_ref = String::from(&no_language_template_filename);
    no_language_template_ref.push_str("ref");
    no_language_template_filename.push_str(format);
    let no_absolute_template_path =
        absolute_template_directory_path.join(&no_language_template_filename);
    let no_language_template_ref_path =
        absolute_template_directory_path.join(&no_language_template_ref);

    if language != "" {
        config_string.push_str("language=");
        config_string.push_str(language);
        config_string.push_str("\u{000A}");
    }

    if template_file != "" {
        config_string.push_str("template=");
        config_string.push_str(template_file);
        config_string.push_str("\u{000A}");
    }

    if format != "" {
        config_string.push_str("fileType=");
        config_string.push_str(format);
        config_string.push_str("\u{000A}");
    }

    if default_proposed {
        config_string.push_str("defaultProposed=");
        config_string.push_str("true");
        config_string.push_str("\u{000A}");
    } else {
        config_string.push_str("defaultProposed=");
        config_string.push_str("false");
        config_string.push_str("\u{000A}");
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
        if !absolute_template_directory_path.exists() {
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
        if !complete_language_template_ref_path.exists()
            && !partial_language_template_ref_path.exists()
            && !no_language_template_ref_path.exists()
        {
            let result_reference = load_template(language.to_string(), "ref".to_string());
            if result_reference.is_ok() {
                let template_string = result_reference.unwrap();
                println!("Writing default template reference");
                let create_template =
                    create_file(complete_language_template_ref_path, template_string);
                if create_template.is_ok() {
                    println!("Done");
                }
            }
        }

        let create_file = create_file(absolute_config_path, config_string);
        if create_file.is_ok() {
            println!("Done");
        }
        return Ok(());
    } else {
        panic!("Config file already exists");
    }
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

fn load_template(language: String, format: String) -> Result<String, Error> {
    let short_language = String::from(&language);
    let re = Regex::new("^([a-zA-Z]+)([-_][a-zA-Z]+|)$").unwrap();
    re.replace(&short_language, "${1}");

    if language == "en" || short_language == "en" {
        if format == "md" {
            return Ok("# NUMBER. TITLE\u{000A}\u{000A}Date: DATE\u{000A}\u{000A}## Status\u{000A}\u{000A}STATUS\u{000A}\u{000A}## Context\u{000A}\u{000A}This is the context.\u{000A}\u{000A}## Decision\u{000A}\u{000A}This is the decision that was made.\u{000A}\u{000A}## Consequence\u{000A}\u{000A}This is the consequence of the decision.\u{000A}".to_string());
        } else if format == "rst" {
            return Ok("#################\u{000A}NUMBER. TITLE\u{000A}#################\u{000A}\u{000A}Date: DATE\u{000A}\u{000A}******\u{000A}Status\u{000A}******\u{000A}\u{000A}STATUS\u{000A}\u{000A}*******\u{000A}Context\u{000A}*******\u{000A}\u{000A}This is the context.\u{000A}\u{000A}********\u{000A}Decision\u{000A}********\u{000A}\u{000A}This is the decision that was made.\u{000A}\u{000A}***********\u{000A}Consequence\u{000A}***********\u{000A}\u{000A}This is the consequence of the decision.\u{000A}".to_string());
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "Invalid Language/Format Match.",
            ));
        }
    } else if language == "fr" || short_language == "fr" {
        if format == "md" {
            return Ok("# NUMBER. TITLE\u{000A}\u{000A}Date: DATE\u{000A}\u{000A}## Statut\u{000A}\u{000A}STATUS\u{000A}\u{000A}## Le contexte\u{000A}\u{000A}C'est le Contexte.\u{000A}\u{000A}## Décision\u{000A}\u{000A}Pris une décision.\u{000A}\u{000A}## Conséquence\u{000A}\u{000A}C'est la conséquence de la décision.\u{000A}".to_string());
        } else if format == "ref" {
            return Ok("Status=\"Statut\"\u{000A}Context=\"Le contexte\"\u{000A}Decision=\"Décision\"\u{000A}Consequence=\"Conséquence\"\u{000A}Proposed=\"Proposé\"\u{000A}Approved=\"Approuvé\"\u{000A}".to_string());
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "Invalid Language/Format Match.",
            ));
        }
    } else {
        if format == "md" {
            return Ok("# NUMBER. TITLE\u{000A}\u{000A}Date: DATE\u{000A}\u{000A}## Status\u{000A}\u{000A}STATUS\u{000A}\u{000A}## Context\u{000A}\u{000A}This is the context.\u{000A}\u{000A}## Decision\u{000A}\u{000A}This is the decision that was made.\u{000A}\u{000A}## Consequence\u{000A}\u{000A}This is the consequence of the decision.\u{000A}".to_string());
        } else if format == "rst" {
            return Ok("#################\u{000A}NUMBER. TITLE\u{000A}#################\u{000A}\u{000A}Date: DATE\u{000A}\u{000A}******\u{000A}Status\u{000A}******\u{000A}\u{000A}STATUS\u{000A}\u{000A}*******\u{000A}Context\u{000A}*******\u{000A}\u{000A}This is the context.\u{000A}\u{000A}********\u{000A}Decision\u{000A}********\u{000A}\u{000A}This is the decision that was made.\u{000A}\u{000A}***********\u{000A}Consequence\u{000A}***********\u{000A}\u{000A}This is the consequence of the decision.\u{000A}".to_string());
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "Invalid Language/Format Match.",
            ));
        }
    }
}
