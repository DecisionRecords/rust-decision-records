use std::env;
use std::io;
use std::path::Path;

extern crate clap;
extern crate pathdiff;

use clap::{App, Arg};
use pathdiff::diff_paths;

mod config;
mod decision_record;
mod init;

fn main() -> Result<(), io::Error> {
    let app = App::new("decision-record")
    .version("0.0.1")
    .author("Jon Spriggs <jon@sprig.gs>")
    .about("Making Decision Records easier. See https://github.com/DecisionRecords/ for more details.")
    .subcommand(
      App::new("init")
        .about("Initializes the directory structures for new decision records.")
        .arg(
          Arg::with_name("doc_path")
            .help("The directory to create your decision records in.")
            .default_value("")
            .takes_value(true)
            .empty_values(true)
            .index(1),
        )
        .arg(
          Arg::with_name("template_file")
            .help("Set the filename prefix for the Decision Record template to use.")
            .long("template")
            .short("t")
            .default_value("template"),
        )
        .arg(
          Arg::with_name("format")
            .help("Set the Decision Record template format to use.")
            .long("format")
            .short("f")
            .default_value("md"),
        )
        .arg(
          Arg::with_name("language")
            .help("The two or four letter code defining the language to prefer.")
            .long("language")
            .short("l")
            .default_value("en"),
        )
        .arg(
          Arg::with_name("template_directory")
            .help("The template directory to use. [default: DOC_PATH/.template/]")
            .long("template-directory")
            .short("d"),
        )
        .arg(
          Arg::with_name("adr_format")
            .help("Use the old ADR format for finding the directory structure.")
            .long("adr"),
        )
        .arg(
          Arg::with_name("default_proposed")
            .help("Default new records as 'proposed' rather than 'accepted'.")
            .long("default-proposed")
            .short("p"),
        )
        .arg(
          Arg::with_name("force")
            .help("Force overwriting of an existing config.")
            .long("force"),
        ),
    )
    .subcommand(
      App::new("new")
        .about("Creates a new decision record.")
        .arg(
          Arg::with_name("title")
            .help("The title of the new record")
            .takes_value(true)
            .required(true)
            .multiple(true)
        )
        .arg(
          Arg::with_name("supersede")
            .help("Note the fact this record supersedes a previous Decision Record. Can be used several times.")
            .long("supersede...")
            .short("s")
            .visible_alias("supersedes...")
            .alias("supercedes...")
            .alias("supercede...")
            .takes_value(true)
            .value_name("record")
            .multiple(true)
        )
        .arg(
          Arg::with_name("deprecate")
            .help("Note the fact this record deprecates a previous Decision Record. Can be used several times.")
            .long("deprecate...")
            .visible_alias("deprecates...")
            .short("d")
            .takes_value(true)
            .value_name("record")
            .multiple(true)
        )
        .arg(
          Arg::with_name("amend")
            .help("Note the fact this record amends a previous Decision Record. Can be used several times.")
            .long("amend...")
            .visible_alias("amends...")
            .short("a")
            .takes_value(true)
            .value_name("record")
            .multiple(true)
        )
        .arg(
          Arg::with_name("link")
            .help("Note the fact this record is linked to a previous Decision Record. Can be used several times.")
            .long("link...")
            .visible_alias("links...")
            .visible_alias("linked...")
            .short("l")
            .takes_value(true)
            .value_name("record")
            .multiple(true)
        )
        .arg(
          Arg::with_name("proposed")
            .help("Sets the fact that this decision record is classed as Proposed.")
            .long("proposed")
            .visible_alias("propose")
            .short("P")
            .conflicts_with("approved")
        )
        .arg(
          Arg::with_name("approved")
            .help("Sets the fact that this decision record is classed as Approved.")
            .long("approved")
            .visible_alias("approve")
            .visible_alias("accept")
            .visible_alias("accepted")
            .short("A")
            .conflicts_with("proposed")
        )
    )
    .subcommand(
      App::new("approve")
        .about("Change the status of a proposed Decision Record to approved.")
        .visible_alias("accept")
        .arg(
          Arg::with_name("record")
            .help("The record or records to change the status to approved")
            .takes_value(true)
            .required(true)
            .multiple(true)
        )
    )
    .subcommand(
      App::new("proposed")
        .about("Change the status of an approved Decision Record back to proposed.")
        .arg(
          Arg::with_name("record")
            .help("The record or records to change the status to proposed")
            .takes_value(true)
            .required(true)
            .multiple(true)
        )
    )
    .subcommand(
      App::new("link")
        .about("Link two decision records.")
        .arg(
          Arg::with_name("from")
            .help("Link from a record")
            .takes_value(true)
            .required(true)
        )
        .arg(
          Arg::with_name("to")
            .help("Link to a record")
            .takes_value(true)
            .required(true)
        )
        .arg(
          Arg::with_name("reason")
            .help("The reason to link the two records")
            .takes_value(true)
            .multiple(true)
        )
    )
    .subcommand(
      App::new("deprecate")
        .about("Change the status of a Decision Record to deprecated.")
        .arg(
          Arg::with_name("from")
            .help("Link from a record")
            .takes_value(true)
            .required(true)
        )
        .arg(
          Arg::with_name("to")
            .help("Link to a record")
            .takes_value(true)
            .required(true)
        )
    )
    .subcommand(
      App::new("amend")
        .about("Amend a Decision Record with an additional Decision Record.")
        .arg(
          Arg::with_name("from")
            .help("Link from a record")
            .takes_value(true)
            .required(true)
        )
        .arg(
          Arg::with_name("to")
            .help("Link to a record")
            .takes_value(true)
            .required(true)
        )
    )
    .subcommand(
      App::new("supersede")
        .about("Change the status of a Decision Record to superseded.")
        .alias("supercede")
        .arg(
          Arg::with_name("from")
            .help("Link from a record")
            .takes_value(true)
            .required(true)
        )
        .arg(
          Arg::with_name("to")
            .help("Link to a record")
            .takes_value(true)
            .required(true)
        )
    )
    .get_matches();

    match app.subcommand() {
        ("init", Some(submatch)) => {
            let root_dir = env::current_dir()?;

            let mut force = false;
            if submatch.is_present("force") {
                force = true;
            }

            let adr_format: bool;
            match submatch.occurrences_of("adr_format") {
                0 => adr_format = false,
                _ => adr_format = true,
            }
            if adr_format {
                let mut doc_path = root_dir.join("doc").join("adr");
                if !submatch.value_of("doc_path").unwrap_or_default().is_empty() {
                    doc_path = root_dir.join(submatch.value_of("doc_path").unwrap_or_default());
                }

                init::short_init(root_dir, doc_path, force)?;
            } else {
                let mut doc_path = root_dir.join("doc").join("decision_records");
                if !submatch.value_of("doc_path").unwrap_or_default().is_empty() {
                    doc_path = root_dir.join(submatch.value_of("doc_path").unwrap_or_default());
                }

                let str_root_dir = root_dir.display().to_string();
                let str_doc_path = doc_path.display().to_string();
                let absolute_root_dir = Path::new(&str_root_dir);
                let absolute_doc_path = Path::new(&str_doc_path);

                let relative_doc_path = diff_paths(absolute_doc_path, absolute_root_dir)
                    .unwrap()
                    .display()
                    .to_string();

                let template_file = submatch.value_of("template_file").unwrap_or_default();
                let format = submatch.value_of("format").unwrap_or_default();
                let language = submatch.value_of("language").unwrap_or_default();

                let default_template_directory = &Path::new(&relative_doc_path)
                    .join(".template")
                    .display()
                    .to_string();

                let template_directory = submatch
                    .value_of("template_directory")
                    .unwrap_or(default_template_directory);

                let mut default_proposed = false;
                if submatch.is_present("default_proposed") {
                    default_proposed = true;
                }

                init::init(
                    root_dir,
                    doc_path,
                    template_file,
                    format,
                    language,
                    template_directory,
                    default_proposed,
                    force,
                )?;
            }
        }
        ("new", Some(submatch)) => {
            let mut title: String = "".to_owned();
            if submatch.is_present("title") {
                let title_items = submatch.values_of("title").unwrap();
                for ref mut title_item in title_items {
                    if !title.is_empty() {
                        title.push(' ');
                    }
                    title.push_str(&title_item.to_string());
                }
            }

            let mut supersede: String = "".to_owned();
            if submatch.is_present("supersede") {
                let supersede_items = submatch.values_of("supersede").unwrap();
                for ref mut supersede_item in supersede_items {
                    if !supersede.is_empty() {
                        supersede.push(',');
                    }
                    supersede.push_str(&supersede_item.to_string());
                }
            }

            let mut deprecate: String = "".to_owned();
            if submatch.is_present("deprecate") {
                let deprecate_items = submatch.values_of("deprecate").unwrap();
                for ref mut deprecate_item in deprecate_items {
                    if !deprecate.is_empty() {
                        deprecate.push(',');
                    }
                    deprecate.push_str(&deprecate_item.to_string())
                }
            }

            let mut amend: String = "".to_owned();
            if submatch.is_present("amend") {
                let amend_items = submatch.values_of("amend").unwrap();
                for ref mut amend_item in amend_items {
                    if !amend.is_empty() {
                        amend.push(',');
                    }
                    amend.push_str(&amend_item.to_string())
                }
            }

            let mut link: String = "".to_owned();
            if submatch.is_present("link") {
                let link_items = submatch.values_of("link").unwrap();
                for ref mut link_item in link_items {
                    if !link.is_empty() {
                        link.push(',');
                    }
                    link.push_str(&link_item.to_string())
                }
            }

            let mut proposed = false;
            if submatch.is_present("proposed") {
                proposed = true;
            }

            let mut approved = false;
            if submatch.is_present("approved") {
                approved = true;
            }

            decision_record::new_record(
                title,
                supersede,
                deprecate,
                amend,
                link,
                proposed,
                approved,
                config::load_config().unwrap(),
            )?;
        }
        ("approve", Some(submatch)) => {
            let mut records: String = "".to_owned();
            if submatch.is_present("record") {
                let record_items = submatch.values_of("record").unwrap();
                for ref mut record_item in record_items {
                    if !records.is_empty() {
                        records.push(',');
                    }
                    records.push_str(&record_item.to_string());
                }
            }
            decision_record::approve(records)?;
        }
        ("proposed", Some(submatch)) => {
            let mut records: String = "".to_owned();
            if submatch.is_present("record") {
                let record_items = submatch.values_of("record").unwrap();
                for ref mut record_item in record_items {
                    if !records.is_empty() {
                        records.push(',');
                    }
                    records.push_str(&record_item.to_string());
                }
            }
            decision_record::proposed(records)?;
        }
        ("link", Some(submatch)) => {
            let from_record: String = submatch.value_of("from").unwrap().to_string();
            let to_record: String = submatch.value_of("to").unwrap().to_string();

            let mut reason: String = "".to_owned();
            if submatch.is_present("reason") {
                let reason_items = submatch.values_of("reason").unwrap();
                for ref mut reason_item in reason_items {
                    if !reason.is_empty() {
                        reason.push(' ');
                    }
                    reason.push_str(&reason_item.to_string());
                }
            }
            decision_record::link(from_record, to_record, reason)?;
        }
        ("deprecate", Some(submatch)) => {
            let from_record: String = submatch.value_of("from").unwrap().to_string();
            let to_record: String = submatch.value_of("to").unwrap().to_string();
            decision_record::deprecate(from_record, to_record)?;
        }
        ("amend", Some(submatch)) => {
            let from_record: String = submatch.value_of("from").unwrap().to_string();
            let to_record: String = submatch.value_of("to").unwrap().to_string();
            decision_record::amend(from_record, to_record)?;
        }
        ("supersede", Some(submatch)) => {
            let from_record: String = submatch.value_of("from").unwrap().to_string();
            let to_record: String = submatch.value_of("to").unwrap().to_string();
            decision_record::supersede(from_record, to_record)?;
        }
        _ => println!("decision-record command not recognised. Please call --help for options."),
    }
    Ok(())
}
