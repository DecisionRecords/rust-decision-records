// use std::env;
// use std::io;
// use std::path::{Path, PathBuf};

extern crate clap;
extern crate pathdiff;

use clap::{Parser, Subcommand};
// use pathdiff::diff_paths;

// mod config;
// mod decision_record;
// mod init;

// TODO: OK, so I ripped a LOAD out of this, and it needs to be put back in using the new format! Good luck Jon!

// Making Decision Records easier. See 
// https://github.com/DecisionRecords/ for more details.
//
// Decision Records are a method of storing the reasons why decisions were
// made. This tool helps you to create those records, using a standard naming
// convention and method of updating those records.
#[derive(Parser)]
#[command(author)]
#[command(version)]
#[command(about)]
#[command(long_about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes the directory structures for new decision records.
    Init {
        /// The directory to create your decision records in.
        #[arg(
            long,
            alias = "doc_path",
            value_name = "PATH",
            default_value = "."
        )]
        doc_path: String,
        /// Set the filename prefix for the Decision Record template to use.
        #[arg(
            long,
            alias = "template_file",
            short = 't',
            value_name = "FILENAME",
            default_value = "template"
        )]
        template_file: String,
        /// Set the Decision Record template format to use.
        #[arg(
            long,
            short = 'f',
            value_name = "FORMAT",
            value_parser(["md", "rst"]),
            default_value = "md"
        )]
        format: String,
        /// The two or four letter code defining the language to prefer.
        #[arg(
            long,
            short = 'l',
            value_name = "LANGUAGE",
            default_value = "en"
        )]
        language: String,
        /// The template directory to use. [default: DOC_PATH/.template/]
        #[arg(
            long,
            alias = "template_directory",
            short = 'd',
            value_name = "DIRECTORY",
            required = false,
            default_value = "./.template/"
        )]
        template_directory: String,
        /// Use the old ADR format for finding the directory structure.
        #[arg(
            long,
            value_parser = clap::builder::BoolishValueParser::new(),
            default_value = "false"
        )]
        adr_format: bool,
        /// Default new records as 'proposed' rather than 'accepted'.
        #[arg(
            long,
            alias = "default_proposed",
            short = 'p',
            value_parser = clap::builder::BoolishValueParser::new(),
            default_value = "false"
        )]
        default_proposed: bool,
        /// Force overwriting of an existing config.
        #[arg(
            long,
            value_parser = clap::builder::BoolishValueParser::new(),
            default_value = "false"
        )]
        force: bool,
    },

    /// Creates a new decision record.
    #[clap(visible_alias = "create")]
    New {
        /// This record supersedes the identified Decision Record.
        #[arg(
            long,
            aliases = ["supercede", "supersedes", "supercedes"],
            short,
            value_name = "RECORD_ID",
            num_args = 1
        )]
        supersede: Vec<u64>,
        /// This record deprecates the identified Decision Record.
        #[arg(
            long,
            aliases = ["deprecates"],
            short,
            value_name = "RECORD_ID",
            num_args = 1
        )]
        deprecate: Vec<u64>,
        /// This record amends the identified Decision Record.
        #[arg(
            long,
            aliases = ["amends"],
            short,
            value_name = "RECORD_ID",
            num_args = 1
        )]
        amend: Vec<u64>,
        /// This record is linked to the identified Decision Record.
        #[arg(
            long,
            aliases = ["links"],
            short = 'l',
            value_name = "RECORD_ID",
            num_args = 1
        )]
        link: Vec<u64>,
        /// Class this record as Proposed. Conflicts with "--approved".
        #[arg(
            long,
            aliases = ["propose"],
            short = 'P',
            conflicts_with = "approved"
        )]
        proposed: bool,
        /// Class this record as Approved. Conflicts with "--proposed".
        #[arg(
            long,
            aliases = ["approve", "accept", "accepted"],
            short = 'A',
            conflicts_with = "proposed"
        )]
        approved: bool,
        /// The title of the new record.
        #[arg(
            value_name = "A TITLE",
            required = true,
            trailing_var_arg = true
        )]
        title: Vec<String>
    },
    /// Change the status of a proposed Decision Record to "approved".
    #[clap(visible_alias = "accept")]
    #[clap(visible_alias = "approved")]
    #[clap(visible_alias = "promote")]
    Approve {
        /// The record or records to change the status to approved
        #[arg(
            value_name = "RECORD_ID",
            required = true,
            trailing_var_arg = true
        )]
        records: Vec<u64>
    },
    /// Change the status of a proposed Decision Record to "proposed".
    #[clap(visible_alias = "propose")]
    #[clap(visible_alias = "demote")]
    Proposed {
        /// The record or records to change the status to proposed
        #[arg(
            value_name = "RECORD_ID",
            required = true,
            trailing_var_arg = true
        )]
        records: Vec<u64>
    },
    /// Link two decision records.
    Link {
        /// Link from a Decision Record ID
        #[arg(
            value_name = "FROM_RECORD_ID",
            required = true
        )]
        from: u64,

        /// Link to a Decision Record ID
        #[arg(
            value_name = "TO_RECORD_ID",
            required = true
        )]
        to: u64,
        /// The reason to link two records.
        #[arg(
            value_name = "REASON",
            required = true,
            trailing_var_arg = true
        )]
        reason: Vec<String>
    },
    /// Deprecate one Decision Record with a second Decision Record.
    Deprecate {
        /// Link from a Decision Record ID
        #[arg(
            value_name = "FROM_RECORD_ID",
            required = true
        )]
        from: u64,

        /// Link to a Decision Record ID
        #[arg(
            value_name = "TO_RECORD_ID",
            required = true
        )]
        to: u64
    },
    /// Amend one Decision Record with a second Decision Record.
    Amend {
        /// Link from a Decision Record ID
        #[arg(
            value_name = "FROM_RECORD_ID",
            required = true
        )]
        from: u64,

        /// Link to a Decision Record ID
        #[arg(
            value_name = "TO_RECORD_ID",
            required = true
        )]
        to: u64
    },
    /// Supersed one Decision Record with a second Decision Record.
    #[clap(alias = "supercede")]
    Supersede {
        /// Link from a Decision Record ID
        #[arg(
            value_name = "FROM_RECORD_ID",
            required = true
        )]
        from: u64,

        /// Link to a Decision Record ID
        #[arg(
            value_name = "TO_RECORD_ID",
            required = true
        )]
        to: u64
    }
}

fn main() {
    let args = Cli::parse();
    
    match args.command {
        Commands::Init {
            doc_path,
            template_file,
            format,
            language,
            template_directory,
            adr_format,
            default_proposed,
            force
        } => {
            println!("doc_path          : {}", doc_path);
            println!("template_file     : {}", template_file);
            println!("format            : {}", format);
            println!("language          : {}", language);
            println!("template_directory: {}", template_directory);
            println!("adr_format        : {}", adr_format);
            println!("default_proposed  : {}", default_proposed);
            println!("force             : {}", force);
        }
        Commands::New {
            title,
            supersede,
            deprecate,
            amend,
            link,
            proposed,
            approved
        } => {
            let the_title: String = _merge_vec_of_strings_to_string(title);
            let supersedes: String = _merge_vec_of_integer_to_string(supersede);
            let deprecates: String = _merge_vec_of_integer_to_string(deprecate);
            let amends: String = _merge_vec_of_integer_to_string(amend);
            let links: String = _merge_vec_of_integer_to_string(link);

            println!("Title: {}", the_title);
            println!("Supersede: {}", supersedes);
            println!("Deprecate: {}", deprecates);
            println!("Amends: {}", amends);
            println!("Links: {}", links);
            println!("Proposed: {}", proposed);
            println!("Approved: {}", approved);
        }
        Commands::Approve { records } => {
            let the_records = _merge_vec_of_integer_to_string(records);
            println!("Records: {}", the_records);
        }
        Commands::Proposed { records } => {
            let the_records = _merge_vec_of_integer_to_string(records);
            println!("Records: {}", the_records);
        }
        Commands::Link {
            from,
            to,
            reason
        } => {
            let the_reason = _merge_vec_of_strings_to_string(reason);
            println!("From: {}", from);
            println!("To: {}", to);
            println!("Reason: {}", the_reason);
        }
        Commands::Deprecate {
            from,
            to
        } => {
            println!("From: {}", from);
            println!("To: {}", to);
        }
        Commands::Amend {
            from,
            to
        } => {
            println!("From: {}", from);
            println!("To: {}", to);
        }
        Commands::Supersede {
            from,
            to
        } => {
            println!("From: {}", from);
            println!("To: {}", to);
        }
    }
}

fn _merge_vec_of_strings_to_string(
    source: Vec<String>
) -> String {
    let mut res: String = "".to_owned();
    for ref mut source_item in source {
        if !res.is_empty() {
            res.push(' ');
        }
        res.push_str(&source_item.to_string());
    }
    return res;
}

fn _merge_vec_of_integer_to_string(
    source: Vec<u64>
) -> String {
    let mut res: String = "".to_owned();
    for ref mut source_item in source {
        if !res.is_empty() {
            res.push(' ');
        }
        res.push_str(&source_item.to_string());
    }
    return res;
}
