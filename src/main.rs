extern crate clap;
use clap::{App, Arg};

use std::env;
use std::io;

mod init;

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
        )
        .arg(
          Arg::with_name("deprecate")
            .help("Note the fact this record deprecates a previous Decision Record. Can be used several times.")
            .long("deprecate...")
            .visible_alias("deprecates...")
            .short("d")
            .takes_value(true)
            .value_name("record")
        )
        .arg(
          Arg::with_name("amend")
            .help("Note the fact this record amends a previous Decision Record. Can be used several times.")
            .long("amend...")
            .visible_alias("amends...")
            .short("a")
            .takes_value(true)
            .value_name("record")
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

    _ => println!("decision-record command not recognised. Please call --help for options.")
  }
  Ok(())
}
