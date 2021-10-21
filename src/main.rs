extern crate clap;
use clap::{App, Arg};

use std::env;
use std::io;

mod init;
mod decision_record;

fn main() -> Result<(), io::Error> {
  let app = App::new("decision-record")
    .version("0.1.0")
    .author("Jon Spriggs <jon@sprig.gs>")
    .about("Making Decision Records easier")
    .subcommand(
      App::new("init")
        .about("Initializes the directory structures for new decision records.")
        .arg(
          Arg::with_name("doc_path")
            .help("The directory to create your decision records in.")
            .default_value(".")
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
            .help("The template directory to use. [default: PATH/.templates/]")
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
            .index(1)
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
    // TODO: Add the following
    //   program
    //   .command(
    //     'approve',
    //     "Change the status of a proposed Decision Record to approved."
    //   )
    //   .argument(
    //     '<record...>',
    //     "The record or records to change the status to approved",
    //     {
    //       validator: program.ARRAY | program.NUMBER
    //     }
    //   )
    //   .action(({ args, options }) => { set_approved_records(args, options) });
    
    // program
    //   .command(
    //     'proposed',
    //     "Change the status of a proposed Decision Record to proposed."
    //   )
    //   .argument(
    //     '<record...>',
    //     "The record or records to change the status to proposed",
    //     {
    //       validator: program.ARRAY | program.NUMBER
    //     }
    //   )
    //   .action(({ args, options }) => { set_proposed_records(args, options) });
    
    // program
    //   .command(
    //     'link',
    //     "Link two Decision Records."
    //   )
    //   .argument(
    //     '<from_record>',
    //     "Link from a record",
    //     {
    //       validator: program.NUMBER
    //     }
    //   )
    //   .argument(
    //     '<to_record>',
    //     "Link to a record",
    //     {
    //       validator: program.NUMBER
    //     }
    //   )
    //   .argument(
    //     '[reason...]',
    //     "The optional reason to link the two records."
    //   )
    //   .action(({ args, options }) => { link_records(args, options) });
    
    // program
    //   .command(
    //     'deprecate',
    //     "Change the status of a Decision Record to deprecated."
    //   )
    //   .argument(
    //     '<deprecate_record>',
    //     "Deprecate this record number",
    //     {
    //       validator: program.NUMBER
    //     }
    //   )
    //   .argument(
    //     '<replace_record>',
    //     "Identify this record as the record which deprecates the old record",
    //     {
    //       validator: program.NUMBER
    //     }
    //   )
    //   .action(({ args, options }) => { deprecate_record(args, options) });
    
    // program
    //   .command(
    //     'amend',
    //     "Amend a Decision Record with an additional Decision Record."
    //   )
    //   .argument(
    //     '<original_record>',
    //     "Amend this Decision Record.",
    //     {
    //       validator: program.NUMBER
    //     }
    //   )
    //   .argument(
    //     '<additional_record>',
    //     "Identify this record as the Decision Record which amends the previous one.",
    //     {
    //       validator: program.NUMBER
    //     }
    //   )
    //   .action(({ args, options }) => { amend_record(args, options) });
    
    // program
    //   .command(
    //     'supersede',
    //     "Change the status of a Decision Record to superseded."
    //   )
    //   .argument(
    //     '<supersede_record>',
    //     "Supersede this record number",
    //     {
    //       validator: program.NUMBER
    //     }
    //   )
    //   .argument(
    //     '<replace_record>',
    //     "Identify this record as the record which supersedes the old record",
    //     {
    //       validator: program.NUMBER
    //     }
    //   )
    //   .action(({ args, options }) => { supersede_record(args, options) });
    .get_matches();

  match app.subcommand() {
    ("init", Some(submatch)) => {
      let root_dir = env::current_dir()?;
      let mut doc_path = env::current_dir()?;
      if submatch.value_of("doc_path").unwrap_or_default() != "." {
        doc_path = root_dir.join(submatch.value_of("doc_path").unwrap_or_default());
      }
      let template_file = submatch.value_of("template_file").unwrap_or_default();
      let format = submatch.value_of("format").unwrap_or_default();
      let language = submatch.value_of("language").unwrap_or_default();

      let template_directory = doc_path.join(submatch.value_of("template_directory").unwrap_or(".templates"));
      
      let mut adr_format = false;
      if submatch.is_present("adr_format") {
        adr_format = true;
      }
      
      let mut default_proposed = false;
      if submatch.is_present("default_proposed") {
        default_proposed = true;
      }
      
      let mut force = false;
      if submatch.is_present("force") {
        force = true;
      }

      init::init(
        doc_path,
        template_file,
        format,
        language,
        template_directory,
        adr_format,
        default_proposed,
        force
      );
    }
    ("new", Some(submatch)) => {
      let title = submatch.value_of("title").unwrap_or("NO_TITLE_DEFILED").to_string();

      let mut supersede: String = "".to_owned();
      if submatch.is_present("supersede") {
        let mut supersede_items = submatch.values_of("supersede").unwrap();
        while let Some(ref mut supersede_item) = supersede_items.next() {
          if supersede.len() > 0 {
            supersede.push_str(",");
          }
          supersede.push_str(&supersede_item.to_string());
        }
      }
      
      
      let mut deprecate: String = "".to_owned();
      if submatch.is_present("deprecate") {
        let mut deprecate_items = submatch.values_of("deprecate").unwrap();
        while let Some(ref mut deprecate_item) = deprecate_items.next() {
          if deprecate.len() > 0 {
            deprecate.push_str(",");
          }
          deprecate.push_str(&deprecate_item.to_string())
        }
      }
      
      let mut amend: String = "".to_owned();
      if submatch.is_present("amend") {
        let mut amend_items = submatch.values_of("amend").unwrap();
        while let Some(ref mut amend_item) = amend_items.next() {
          if amend.len() > 0 {
            amend.push_str(",");
          }
          amend.push_str(&amend_item.to_string())
        }
      }
      
      let mut link: String = "".to_owned();
      if submatch.is_present("link") {
        let mut link_items = submatch.values_of("link").unwrap();
        while let Some(ref mut link_item) = link_items.next() {
          if link.len() > 0 {
            link.push_str(",");
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
        approved
      );
    }
    // TODO: Parse "approve", "proposed", "link", "deprecate", "amend" and "supersede"
    _ => println!("decision-record command not recognised. Please call --help for options.")
  }
  Ok(())
}
