use std::path::PathBuf;

pub fn init(
    doc_path: PathBuf,
    template_file: &str,
    format: &str,
    language: &str,
    template_directory: PathBuf,
    adr_format: bool,
    default_proposed: bool,
    force: bool,
) {
    println!(
        "doc_path: {}",
        doc_path.into_os_string().into_string().unwrap()
    );
    println!("template_file: {}", template_file);
    println!("format: {}", format);
    println!("language: {}", language);
    println!(
        "template_directory: {}",
        template_directory.into_os_string().into_string().unwrap()
    );
    println!("adr_format: {}", adr_format);
    println!("default_proposed: {}", default_proposed);
    println!("force: {}", force);
}
