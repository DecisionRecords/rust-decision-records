use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

pub fn load_config() -> Config {
    println!("{}", "loading config...");
    let current_dir = env::current_dir();
    let path = walk_path(Path::new(&current_dir.unwrap()));
    return path.unwrap();
}

// Based on https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct Config {
    pub record_path: PathBuf,
    pub template_path: PathBuf,
    pub template_language: String,
    pub template_file: String,
    pub template_format: String
}

// Based on https://gist.github.com/rust-play/66643f002522e9a19f9261d0a7e762ff
fn walk_path(path: &Path) -> Result<Config, io::Error> {
    let mut config = Config {
        record_path: path.to_path_buf(),
        template_path: path.to_path_buf(),
        template_language: String::from("en"),
        template_file: String::from("INTERNAL"),
        template_format: String::from("md")
    };
    let mut pathbuf = PathBuf::new();
    let mut lastpath = String::new();
    let mut record_path = PathBuf::new();
    let path_regex = Regex::new(r"\\$|/$").unwrap();
    let unix_path = Regex::new(r"/").unwrap();
    let windows_path = Regex::new(r"\\").unwrap();
    pathbuf.push(path);
    record_path.push(path);

    while pathbuf.exists() {
        pathbuf.push(".adr-dir");

        if pathbuf.exists() {
            let mut file_read: bool = false;
            let mut theline: String = String::from("");
            if let Ok(lines) = read_lines(&pathbuf) {
                record_path.pop();
                for line in lines {
                    if let Ok(line) = line {
                        if line.chars().count() > 0 {
                            if file_read {
                                panic!(".adr-dir contains multiple lines which is against spec.");
                            } else {
                                theline = line;
                                file_read = true;
                            }
                        }
                    }
                }
                
                if unix_path.is_match(&theline) {
                    let split_path_array = theline.split("/");
                    for split_path in split_path_array {
                        record_path.push(split_path);
                    }
                } else {
                    if windows_path.is_match(&theline) {
                        let split_path_array = theline.split(r"\\");
                        for split_path in split_path_array {
                            record_path.push(split_path);
                        }
                    } else {
                        record_path.push(theline);
                    }
                }
                config.record_path = PathBuf::from(record_path);
            }
        
            return Ok(config);
        }

        pathbuf.pop();

        pathbuf.push(".decisionrecords-config");

        if pathbuf.exists() {
            let mut root_path: PathBuf = pathbuf.clone();
            let re_record_path = Regex::new(r"^records=(.*)$").unwrap();
            let re_system_language = Regex::new(r"^language=(.*)$").unwrap();
            let re_template_dir = Regex::new(r"^templateDir=(.*)$").unwrap();
            let re_template = Regex::new(r"^template=(.*)$").unwrap();
            let re_filetype = Regex::new(r"^fileType=(.*)$").unwrap();

            if let Ok(lines) = read_lines(pathbuf) {
                root_path.pop();
                for line in lines {
                    if let Ok(line) = line {
                        if line.chars().count() > 0 {
                            if re_record_path.is_match(&line) {
                                let the_record_path = re_record_path.replace(&line, "$1");
                                let mut record_path: PathBuf = root_path.clone();
                                record_path.push(String::from(the_record_path));
                                config.record_path = record_path;
                            }
                            if re_system_language.is_match(&line) {
                                let system_language = re_system_language.replace(&line, "$1");
                                config.template_language = String::from(system_language);
                            }
                            if re_template_dir.is_match(&line) {
                                let template_dir = re_template_dir.replace(&line, "$1");
                                let mut template_path: PathBuf = root_path.clone();
                                template_path.push(String::from(template_dir));
                                config.template_path = template_path;
                            }
                            if re_template.is_match(&line) {
                                let template = re_template.replace(&line, "$1");
                                config.template_file = String::from(template);
                            }
                            if re_filetype.is_match(&line) {
                                let filetype = re_filetype.replace(&line, "$1");
                                config.template_format = String::from(filetype);
                            }
                        }
                    }
                }
            }
            return Ok(config);
        }

        pathbuf.pop();

        pathbuf.push("doc");
        pathbuf.push("adr");

        if pathbuf.exists() {
            config.record_path = pathbuf;
            return Ok(config);
        }

        pathbuf.pop();

        pathbuf.push("decision_records");

        if pathbuf.exists() {
            config.record_path = pathbuf;
            return Ok(config);
        }

        pathbuf.pop();

        pathbuf.pop();

        pathbuf.pop();

        if path_regex.is_match(&pathbuf.display().to_string()) {
            if pathbuf.display().to_string() == lastpath {
                panic!("File not found at {}", lastpath);
            }
        }

        lastpath = pathbuf.display().to_string();
    }
    panic!("Path not found!")
}
