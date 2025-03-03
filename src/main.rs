use std::env;
use std::io;
use std::path::Path;

use clap::{Arg, ArgAction, Command};
use pathdiff::diff_paths;

mod config;
mod decision_record;
mod init;

fn main() -> Result<(), io::Error> {
    let app = Command::new("decision-record")
        .version("0.0.4")
        .author("Jon Spriggs <jon@sprig.gs>")
        .about("Making Decision Records easier. See https://github.com/DecisionRecords/ for more details.")
        .subcommand(
            Command::new("init")
                .about("Initializes the directory structures for new decision records.")
                .arg(Arg::new("doc_path")
                    .help("The directory to create your decision records in.")
                    .default_value("")
                    .num_args(1))
                .arg(Arg::new("template_file")
                    .help("Set the filename prefix for the Decision Record template to use.")
                    .long("template")
                    .short('t')
                    .default_value("template"))
                .arg(Arg::new("format")
                    .help("Set the Decision Record template format to use.")
                    .long("format")
                    .short('f')
                    .default_value("md"))
                .arg(Arg::new("language")
                    .help("The two or four-letter code defining the language to prefer.")
                    .long("language")
                    .short('l')
                    .default_value("en"))
                .arg(Arg::new("template_directory")
                    .help("The template directory to use. [default: DOC_PATH/.template/]")
                    .long("template-directory")
                    .short('d')
                    .num_args(1))
                .arg(Arg::new("adr_format")
                    .help("Use the old ADR format for finding the directory structure.")
                    .long("adr")
                    .action(ArgAction::SetTrue))
                .arg(Arg::new("default_proposed")
                    .help("Default new records as 'proposed' rather than 'accepted'.")
                    .long("default-proposed")
                    .short('p')
                    .action(ArgAction::SetTrue))
                .arg(Arg::new("force")
                    .help("Force overwriting of an existing config.")
                    .long("force")
                    .action(ArgAction::SetTrue)),
        )
        .subcommand(
            Command::new("new")
                .about("Creates a new decision record.")
                .arg(Arg::new("title")
                    .help("The title of the new record")
                    .required(true)
                    .num_args(1..))
                .arg(Arg::new("supersede")
                    .help("This record supersedes a previous Decision Record.")
                    .long("supersede")
                    .short('s')
                    .num_args(1..))
                .arg(Arg::new("deprecate")
                    .help("This record deprecates a previous Decision Record.")
                    .long("deprecate")
                    .short('d')
                    .num_args(1..))
                .arg(Arg::new("amend")
                    .help("This record amends a previous Decision Record.")
                    .long("amend")
                    .short('a')
                    .num_args(1..))
                .arg(Arg::new("link")
                    .help("This record links to another Decision Record.")
                    .long("link")
                    .short('l')
                    .num_args(1..))
                .arg(Arg::new("proposed")
                    .help("Sets this decision record as Proposed.")
                    .long("proposed")
                    .short('P')
                    .conflicts_with("approved")
                    .action(ArgAction::SetTrue))
                .arg(Arg::new("approved")
                    .help("Sets this decision record as Approved.")
                    .long("approved")
                    .short('A')
                    .conflicts_with("proposed")
                    .action(ArgAction::SetTrue)),
        )
        .subcommand(
          Command::new("approve")
            .about("Change the status of a proposed Decision Record to approved.")
            .visible_alias("accept")
            .arg(
              Arg::new("record")
                .help("The record or records to change the status to approved")
                .required(true)
                .num_args(1..)
            )
        )
        .subcommand(
          Command::new("reject")
            .about("Change the status of a proposed Decision Record to rejected.")
            .visible_alias("deny")
            .arg(
              Arg::new("record")
                .help("The record or records to change the status to rejected")
                .required(true)
                .num_args(1..)
            )
        )
        .subcommand(
          Command::new("proposed")
            .about("Change the status of a proposed Decision Record to proposed.")
            .arg(
              Arg::new("record")
                .help("The record or records to change the status to proposed")
                .required(true)
                .num_args(1..)
            )
        )
        .subcommand(
          Command::new("link")
            .about("Link two decision records.")
            .arg(
              Arg::new("from")
                .help("Link from a record")
                .required(true)
            )
            .arg(
              Arg::new("to")
                .help("Link to a record")
                .required(true)
            )
            .arg(
              Arg::new("reason")
                .help("The reason to link the two records")
                .num_args(0..)
            )
        )
        .subcommand(
          Command::new("deprecate")
            .about("Change the status of a Decision Record to deprecated.")
            .arg(
              Arg::new("from")
                .help("Link from a record")
                .required(true)
            )
            .arg(
              Arg::new("to")
                .help("Link to a record")
                .required(true)
            )
        )
        .subcommand(
          Command::new("amend")
            .about("Amend a Decision Record with an additional Decision Record.")
            .arg(
              Arg::new("from")
                .help("Link from a record")
                .required(true)
            )
            .arg(
              Arg::new("to")
                .help("Link to a record")
                .required(true)
            )
        )
        .subcommand(
          Command::new("supersede")
            .about("Change the status of a Decision Record to superseded.")
            .alias("supercede")
            .arg(
              Arg::new("from")
                .help("Link from a record")
                .required(true)
            )
            .arg(
              Arg::new("to")
                .help("Link to a record")
                .required(true)
            )
        )
        ;

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("init", submatch)) => {
            let root_dir = env::current_dir()?;

            let force = submatch.get_flag("force");
            let adr_format = submatch.get_flag("adr_format");

            let mut doc_path = root_dir.join("doc").join("decision_records");
            if let Some(path) = submatch.get_one::<String>("doc_path") {
                if !path.is_empty() {
                    doc_path = root_dir.join(path);
                }
            }

            let str_root_dir = root_dir.display().to_string();
            let str_doc_path = doc_path.display().to_string();
            let absolute_root_dir = Path::new(&str_root_dir);
            let absolute_doc_path = Path::new(&str_doc_path);

            let relative_doc_path = diff_paths(absolute_doc_path, absolute_root_dir)
                .unwrap()
                .display()
                .to_string();

            let default_template_file = "template".to_string();
            let template_file = submatch
                .get_one::<String>("template_file")
                .unwrap_or(&default_template_file);
            let default_format = "md".to_string();
            let format = submatch
                .get_one::<String>("format")
                .unwrap_or(&default_format);
            let default_language = "en".to_string();
            let language = submatch
                .get_one::<String>("language")
                .unwrap_or(&default_language);

            let default_template_directory = Path::new(&relative_doc_path)
                .join(".template")
                .display()
                .to_string();

            let template_directory = submatch
                .get_one::<String>("template_directory")
                .unwrap_or(&default_template_directory);

            let default_proposed = submatch.get_flag("default_proposed");

            if adr_format {
                init::short_init(root_dir, doc_path, force)?;
            } else {
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
        Some(("new", submatch)) => {
            let title: String = submatch
                .get_many::<String>("title")
                .unwrap()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" ");

            let supersede = submatch
                .get_many::<String>("supersede")
                .map(|vals| vals.map(|s| s.to_string()).collect::<Vec<_>>().join(","))
                .unwrap_or_default();

            let deprecate = submatch
                .get_many::<String>("deprecate")
                .map(|vals| vals.map(|s| s.to_string()).collect::<Vec<_>>().join(","))
                .unwrap_or_default();

            let amend = submatch
                .get_many::<String>("amend")
                .map(|vals| vals.map(|s| s.to_string()).collect::<Vec<_>>().join(","))
                .unwrap_or_default();

            let link = submatch
                .get_many::<String>("link")
                .map(|vals| vals.map(|s| s.to_string()).collect::<Vec<_>>().join(","))
                .unwrap_or_default();

            let proposed = submatch.get_flag("proposed");
            let approved = submatch.get_flag("approved");

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
        Some(("approve", submatch)) => {
            let mut records: String = "".to_owned();
            if let Some(record_items) = submatch.get_many::<String>("record") {
                for record_item in record_items {
                    if !records.is_empty() {
                        records.push(',');
                    }
                    records.push_str(record_item);
                }
            }
            decision_record::approve(records)?;
        }
        Some(("reject", submatch)) => {
            let mut records: String = "".to_owned();
            if let Some(record_items) = submatch.get_many::<String>("record") {
                for record_item in record_items {
                    if !records.is_empty() {
                        records.push(',');
                    }
                    records.push_str(record_item);
                }
            }
            decision_record::reject(records)?;
        }
        Some(("proposed", submatch)) => {
            let mut records: String = "".to_owned();
            if let Some(record_items) = submatch.get_many::<String>("record") {
                for record_item in record_items {
                    if !records.is_empty() {
                        records.push(',');
                    }
                    records.push_str(record_item);
                }
            }
            decision_record::proposed(records)?;
        }
        Some(("link", submatch)) => {
            let from_record = submatch.get_one::<String>("from").unwrap().to_string();
            let to_record = submatch.get_one::<String>("to").unwrap().to_string();

            let mut reason = String::new();
            if let Some(reason_items) = submatch.get_many::<String>("reason") {
                for reason_item in reason_items {
                    if !reason.is_empty() {
                        reason.push(' ');
                    }
                    reason.push_str(reason_item);
                }
            }
            decision_record::link(from_record, to_record, reason)?;
        }
        Some(("deprecate", submatch)) => {
            let from_record = submatch.get_one::<String>("from").unwrap().to_string();
            let to_record = submatch.get_one::<String>("to").unwrap().to_string();

            decision_record::deprecate(from_record, to_record)?;
        }
        Some(("amend", submatch)) => {
            let from_record = submatch.get_one::<String>("from").unwrap().to_string();
            let to_record = submatch.get_one::<String>("to").unwrap().to_string();

            decision_record::amend(from_record, to_record)?;
        }
        Some(("supersede", submatch)) => {
            let from_record = submatch.get_one::<String>("from").unwrap().to_string();
            let to_record = submatch.get_one::<String>("to").unwrap().to_string();

            decision_record::supersede(from_record, to_record)?;
        }
        _ => println!("decision-record command not recognized. Use --help for options."),
    }

    Ok(())
}
