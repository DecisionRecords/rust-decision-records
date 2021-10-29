use std::env;
use std::fs;
use std::fs::File;
use std::path::{Path};
use glob::glob;
use std::io::{self, BufRead};
use regex::Regex;

pub fn load_config() {
    println!("{}", "loading config...");
    let current_dir = env::current_dir();
    let path = walk_path(Path::new(&current_dir.unwrap()));
    println!("{:?}", path);

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./hosts") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}

// Based on https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn walk_path(path: &Path) -> Result<String, io::Error> {
    let mut found_dir: bool = false;
    let re = Regex::new(r"[A-Z]:\\$").unwrap();
    println!("Checking path: {:?}", fs::canonicalize(path).unwrap().display().to_string());
    if re.is_match(&fs::canonicalize(path).unwrap().display().to_string()) {
        println!("{}", "No path found");
        panic!("No path found");
    }
    
    for entry in glob(&fs::canonicalize(path).unwrap().join("doc").join("adr").display().to_string()).unwrap() {
        if let Ok(a_path) = entry {
            println!("FOUND: {:?}", a_path.display());
            found_dir = true;
        }
    }
    for entry in glob(&fs::canonicalize(path).unwrap().join("doc").join("decision_records").display().to_string()).unwrap() {
        if let Ok(a_path) = entry {
            println!("FOUND: {:?}", a_path.display());
            found_dir = true;
        }
    }
    for entry in glob(&fs::canonicalize(path).unwrap().join(".adr-dir").display().to_string()).unwrap() {
        if let Ok(a_path) = entry {
            println!("FOUND: {:?}", a_path.display());
            found_dir = true;
        }
    }
    for entry in glob(&fs::canonicalize(path).unwrap().join(".decisionrecords-config").display().to_string()).unwrap() {
        if let Ok(a_path) = entry {
            println!("FOUND: {:?}", a_path.display());
            found_dir = true;
        }
    }

    if found_dir == false {
        return walk_path(&path.join(".."));
    }
    Ok(path.display().to_string())
}
