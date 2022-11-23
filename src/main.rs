use std::io;
use std::io::Write;

const BASE_URL: &str = "https://www.toptal.com/developers/gitignore/api/";

fn main() {
    println!("---------------------------------------------------------");
    println!("--                                                     --");
    println!("--   gig => .gitignore generator using Rust + Toptal   --");
    println!("--                                                     --");
    println!("---------------------------------------------------------");

    let terms = prompt_user_for_terms();
    let url = format!("{}{}", BASE_URL, terms);
    let body = fetch_gitignore_body(&url);

    save_gitignore_file(&body.unwrap());

    println!();
    println!("Finished! .gitignore file saved in directory: {}", std::env::current_dir().unwrap().display());

    std::process::exit(0);
}

fn prompt_user_for_terms() -> String {
    let mut terms = String::new();
    println!();
    println!("Please enter each .gitignore term seperated by a space:");
    println!();

    io::stdin()
        .read_line(&mut terms)
        .expect("Failed to read line");

    if terms.trim().is_empty() {
        println!("You must enter at least one term.");
        return "".to_string();
    }

    terms.replace(" ", ",")
}

fn fetch_gitignore_body(url: &str) -> Result<String, ureq::Error> {
    let body: String = ureq::get(url)
        .set("Example-Header", "header value")
        .call()?
        .into_string()?;

    Ok(body)
}

fn save_gitignore_file(body: &str) {
    let mut file = std::fs::File::create(".gitignore").expect("Unable to create file");
    file.write_all(body.as_bytes()).expect("Unable to write data");
}