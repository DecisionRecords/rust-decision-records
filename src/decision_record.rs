pub fn new_record(
    title: String,
    supersede: String,
    deprecate: String,
    amend: String,
    link: String,
    proposed: bool,
    approved: bool
) {
    println!("title: {}", title);
    println!("supersede: {}", supersede);
    println!("deprecate: {}", deprecate);
    println!("amend: {}", amend);
    println!("link: {}", link);
    println!("proposed: {}", proposed);
    println!("approved: {}", approved);
}