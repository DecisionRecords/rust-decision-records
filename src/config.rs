use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::{Path, PathBuf};

// This is the config data we'll pass into any functions that need it
pub struct Config {
    pub record_path: PathBuf,
    pub template_path: PathBuf,
    pub template_language: String,
    pub template_file: String,
    pub template_format: String,
    pub template_string: String,
    pub template_references: HashMap<String, String>,
    pub default_status: String,
}

// This is the supervising function that will load the config and return it.
pub fn load_config() -> Result<Config, Error> {
    let current_dir = env::current_dir();
    let config = find_and_read_configuration(Path::new(&current_dir.unwrap()))?;
    return Ok(config);
}

// Some notes on this function:
// This will start in the directory the script is called from, and look for one of 4 key
// indicators. If one is not found, it will go into the parent directory, and work up from there.
// If it gets to the root directory, and still has not found an indicator it will exit with a
// warning.
//
// The four key indicators are:
// 1. A directory named `doc/adr` is found (the legacy adr-tools method)
// 2. A file named `.adr-dir` is found (the legacy adr-tools method)
// 3. A directory named `doc/decision_records` is found
// 4. A file called `.decisionrecords-config` is found
//
// Indicators 1, 2 and 3 offer no customization, while indicator 4 allows you to specify particular
// templates, configuration defaults and file formats.
fn find_and_read_configuration(path: &Path) -> Result<Config, io::Error> {
    // Define the default config to use
    let mut config = Config {
        record_path: path.to_path_buf(),
        template_path: path.to_path_buf(),
        template_language: String::from("en"),
        template_file: String::from("INTERNAL"),
        template_format: String::from("md"),
        template_string: String::from("# NUMBER. TITLE\u{000A}\u{000A}Date: DATE\u{000A}\u{000A}## Status\u{000A}\u{000A}STATUS\u{000A}\u{000A}## Context\u{000A}\u{000A}This is the context.\u{000A}\u{000A}## Decision\u{000A}\u{000A}This is the decision that was made.\u{000A}\u{000A}## Consequence\u{000A}\u{000A}This is the consequence of the decision.\u{000A}"),
        template_references: HashMap::new(),
        default_status: "Approved".to_string(),
    };

    // Create new variables
    let mut pathbuf = PathBuf::new();
    let mut lastpath = String::new();

    // Define Regexes
    let path_regex = Regex::new(r"\\$|/$").unwrap();
    let unix_path = Regex::new(r"/").unwrap();
    let windows_path = Regex::new(r"\\").unwrap();
    let re_record_path = Regex::new(r"^records=(.*)$").unwrap();
    let re_language = Regex::new(r"^language=(.*)$").unwrap();
    let re_template_dir = Regex::new(r"^templateDir=(.*)$").unwrap();
    let re_template = Regex::new(r"^template=(.*)$").unwrap();
    let re_filetype = Regex::new(r"^fileType=(.*)$").unwrap();
    let re_default_proposed = Regex::new(r"defaultProposed=(.*)$").unwrap();
    let re_short_language = Regex::new("^([a-zA-Z]+)([-_][a-zA-Z]+|)$").unwrap();
    let re_reference_construct = Regex::new("^(.*)=\"(.*)\"").unwrap();

    // path here is (by default) the current directory when the script is called.
    pathbuf.push(path);

    // This is where the tree walking starts
    while pathbuf.exists() {
        // Looking first of all for the indicator `.adr-dir`.
        pathbuf.push(".adr-dir");

        if pathbuf.exists() {
            // Where is our root path?
            let mut root_path: PathBuf = pathbuf.clone();
            // Set the "root path" to be the directory that the config file exists in.
            root_path.pop();
            // Create a pathbuf for the record directory, based on the relative record directory
            let mut record_path: PathBuf = root_path.clone();

            // This will be the relative path that the file directs us to.
            let mut str_doc_path: String = String::from("");
            // It may occur that, outside of "spec", the file has several lines. Panic if this happens.
            let mut this_file_has_multiple_lines: bool = false;
            // Read the lines in the file
            if let Ok(lines) = get_lines_from_a_file(&pathbuf) {
                for line in lines {
                    if let Ok(line) = line {
                        // Only read lines which have content in them!
                        if line.chars().count() > 0 {
                            // If we've read one line already, and another appears, this is a fault.
                            if this_file_has_multiple_lines {
                                panic!(".adr-dir contains multiple lines which is against spec.");
                            } else {
                                str_doc_path = line;
                                this_file_has_multiple_lines = true;
                            }
                        }
                    }
                }

                // So that we get a consistent handling of "Unix" style and "Windows" style
                // paths, check for the regex using either \ or / directory terminators... and
                // split on that. Push each directory terminator into the record path variable.
                if unix_path.is_match(&str_doc_path) {
                    let split_path_array = str_doc_path.split("/");
                    for split_path in split_path_array {
                        record_path.push(split_path);
                    }
                } else {
                    if windows_path.is_match(&str_doc_path) {
                        let split_path_array = str_doc_path.split(r"\\");
                        for split_path in split_path_array {
                            record_path.push(split_path);
                        }
                    } else {
                        record_path.push(str_doc_path);
                    }
                }
                // Turn the path object into a PathBuf which the config object requires.
                config.record_path = PathBuf::from(record_path);
            }
            // Early exit - we've got our config!
            return Ok(config);
        }

        // So .adr-dir didn't exist here! Perhaps it's .decisionrecords-config?
        pathbuf.pop();
        pathbuf.push(".decisionrecords-config");

        if pathbuf.exists() {
            // Have we found a template directory?
            let mut def_template_dir: bool = false;
            // Where is our root path?
            let mut root_path: PathBuf = pathbuf.clone();
            // Set the "root path" to be the directory that the config file exists in.
            root_path.pop();

            // Read the config file and loop through each line
            if let Ok(lines) = get_lines_from_a_file(pathbuf) {
                for line in lines {
                    if let Ok(line) = line {
                        // This line is not empty
                        if line.chars().count() > 0 {
                            // The record path is relative to the root directory (records=<dir>). If it's defined, use it.
                            if re_record_path.is_match(&line) {
                                // Create a pathbuf for the record directory, based on the relative record directory
                                let mut record_path: PathBuf = root_path.clone();

                                // Parse the config value
                                let str_doc_path = re_record_path.replace(&line, "$1");

                                // So that we get a consistent handling of "Unix" style and "Windows" style
                                // paths, check for the regex using either \ or / directory terminators... and
                                // split on that. Push each directory terminator into the record path variable.
                                if unix_path.is_match(&str_doc_path) {
                                    let split_path_array = str_doc_path.split("/");
                                    for split_path in split_path_array {
                                        record_path.push(split_path);
                                    }
                                } else {
                                    if windows_path.is_match(&str_doc_path) {
                                        let split_path_array = str_doc_path.split(r"\\");
                                        for split_path in split_path_array {
                                            record_path.push(split_path);
                                        }
                                    } else {
                                        record_path.push(String::from(str_doc_path));
                                    }
                                }
                                // And store it
                                config.record_path = record_path;
                            }
                            // If a language is defined (language=<code>), store it.
                            if re_language.is_match(&line) {
                                let language = re_language.replace(&line, "$1");
                                config.template_language = String::from(language);
                            }
                            // If a template directory is specified (templateDir=<dir>), use it.
                            if re_template_dir.is_match(&line) {
                                let template_dir = re_template_dir.replace(&line, "$1");
                                let mut template_path: PathBuf = root_path.clone();
                                template_path.push(String::from(template_dir));
                                config.template_path = template_path;
                                def_template_dir = true;
                            }
                            // If the name of the template file to use is defined (template=<file_prefix>), use it.
                            if re_template.is_match(&line) {
                                let template = re_template.replace(&line, "$1");
                                config.template_file = String::from(template);
                            }
                            // If the file type to use is defined (format=<suffix>), use it. As we do basic string conversion, suggest md and rst are the only two used.
                            if re_filetype.is_match(&line) {
                                let filetype = re_filetype.replace(&line, "$1");
                                config.template_format = String::from(filetype);
                            }
                            // If the value to store DRs as "proposed" by default is defined (defaultProposed=<bool>), translate and store it.
                            if re_default_proposed.is_match(&line) {
                                let default_proposed: String =
                                    re_default_proposed.replace(&line, "$1").parse().unwrap();
                                if default_proposed == "true" {
                                    config.default_status = "Proposed".to_string();
                                }
                            }
                        }
                    }
                }
            }

            // If we've defined a template directory, then we should probably check whether a
            // template based on our selected (e.g. en_GB, en) or default (none defined) language
            // is located there.
            if def_template_dir {
                // Find the short-language version of our language string, e.g. `en_GB` becomes `en`
                let short_language =
                    String::from(re_short_language.replace(&config.template_language, "${1}"));

                // Create a whole load of file references for lookups
                let mut long_template_file: PathBuf = config.template_path.clone();
                let mut long_template_reference_file: PathBuf = config.template_path.clone();
                let mut short_template_file: PathBuf = config.template_path.clone();
                let mut short_template_reference_file: PathBuf = config.template_path.clone();
                let mut default_template_file: PathBuf = config.template_path.clone();
                let mut default_template_reference_file: PathBuf = config.template_path.clone();

                // And the respective filenames
                let mut long_template_filename: String = config.template_file.clone();
                let mut long_template_reference_filename: String = config.template_file.clone();
                let mut short_template_filename: String = config.template_file.clone();
                let mut short_template_reference_filename: String = config.template_file.clone();
                let mut default_template_filename: String = config.template_file.clone();
                let mut default_template_reference_filename: String = config.template_file.clone();

                // Amend the relevant suffixes
                long_template_filename.push_str(".");
                long_template_filename.push_str(&String::from(&config.template_language));
                long_template_filename.push_str(".");
                long_template_filename.push_str(&String::from(&config.template_format));
                long_template_reference_filename.push_str(".");
                long_template_reference_filename.push_str(&String::from(&config.template_language));
                long_template_reference_filename.push_str(".ref");
                short_template_filename.push_str(".");
                short_template_filename.push_str(&String::from(short_language));
                short_template_filename.push_str(".");
                short_template_filename.push_str(&String::from(&config.template_format));
                short_template_reference_filename.push_str(".");
                short_template_reference_filename
                    .push_str(&String::from(&config.template_language));
                short_template_reference_filename.push_str(".ref");
                default_template_filename.push_str(".");
                default_template_filename.push_str(&String::from(&config.template_format));
                default_template_reference_filename.push_str(".ref");

                // And update the file pointers
                long_template_file.push(long_template_filename);
                long_template_reference_file.push(long_template_reference_filename);
                short_template_file.push(short_template_filename);
                short_template_reference_file.push(short_template_reference_filename);
                default_template_file.push(default_template_filename);
                default_template_reference_file.push(default_template_reference_filename);

                // Then look to see if those files exist, and if so, read them into the config
                if long_template_file.exists() {
                    if let Ok(lines) = get_lines_from_a_file(long_template_file) {
                        config.template_string = String::from("");
                        for line in lines {
                            if let Ok(line) = line {
                                config.template_string.push_str(&line);
                                config.template_string.push_str("\u{000A}");
                            }
                        }
                    }
                } else if short_template_file.exists() {
                    if let Ok(lines) = get_lines_from_a_file(short_template_file) {
                        config.template_string = String::from("");
                        for line in lines {
                            if let Ok(line) = line {
                                config.template_string.push_str(&line);
                                config.template_string.push_str("\u{000A}");
                            }
                        }
                    }
                } else if default_template_file.exists() {
                    if let Ok(lines) = get_lines_from_a_file(default_template_file) {
                        config.template_string = String::from("");
                        for line in lines {
                            if let Ok(line) = line {
                                config.template_string.push_str(&line);
                                config.template_string.push_str("\u{000A}");
                            }
                        }
                    }
                }

                // Next load the language reference file, starting from the default, then short, then long
                // and write those key/value pairs into the config
                if default_template_reference_file.exists() {
                    if let Ok(lines) = get_lines_from_a_file(default_template_reference_file) {
                        for line in lines {
                            if let Ok(line) = line {
                                let key = re_reference_construct
                                    .replace(&String::from(&line), "$1")
                                    .to_string();
                                let value = re_reference_construct
                                    .replace(&String::from(&line), "$2")
                                    .to_string();
                                config.template_references.insert(key, value);
                            }
                        }
                    }
                }
                if short_template_reference_file.exists() {
                    if let Ok(lines) = get_lines_from_a_file(short_template_reference_file) {
                        for line in lines {
                            if let Ok(line) = line {
                                let key = re_reference_construct
                                    .replace(&String::from(&line), "$1")
                                    .to_string();
                                let value = re_reference_construct
                                    .replace(&String::from(&line), "$2")
                                    .to_string();
                                config.template_references.insert(key, value);
                            }
                        }
                    }
                }
                if long_template_reference_file.exists() {
                    if let Ok(lines) = get_lines_from_a_file(long_template_reference_file) {
                        for line in lines {
                            if let Ok(line) = line {
                                let key = re_reference_construct
                                    .replace(&String::from(&line), "$1")
                                    .to_string();
                                let value = re_reference_construct
                                    .replace(&String::from(&line), "$2")
                                    .to_string();
                                config.template_references.insert(key, value);
                            }
                        }
                    }
                }
            }
            // Early exit - we've got our config!
            return Ok(config);
        }

        // Turns out .decisionrecords-config wasn't it either. Let's see if doc/adr exists?
        pathbuf.pop();
        pathbuf.push("doc");
        pathbuf.push("adr");

        // Yes? OK, we just use that!
        if pathbuf.exists() {
            config.record_path = pathbuf;
            return Ok(config);
        }

        // Lastly, let's look for doc/decision_records
        pathbuf.pop();
        pathbuf.push("decision_records");

        // OK, let's use that instead
        if pathbuf.exists() {
            config.record_path = pathbuf;
            return Ok(config);
        }

        // exit doc/decision_records
        pathbuf.pop();
        pathbuf.pop();

        // Move up a directory
        pathbuf.pop();

        // Check and see if the new directory ends with a slash, and that we've been here (because of how lookups work) twice
        if path_regex.is_match(&pathbuf.display().to_string()) {
            if pathbuf.display().to_string() == lastpath {
                panic!("Decision Record path not found");
            }
        }

        // Update the path we're in for the next loop round and go again.
        lastpath = pathbuf.display().to_string();
    }
    // We've somehow failed if we got here, so say so!
    panic!("Path not found!")
}

// Based on https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn get_lines_from_a_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
