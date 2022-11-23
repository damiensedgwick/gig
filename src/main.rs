use std::io;

const BASE_URL: &str = "https://www.toptal.com/developers/gitignore/api/";

fn main() {
    // Make a pretty banner for the program
    println!("---------------------------------------------------------");
    println!("--                                                     --");
    println!("--   gig => .gitignore generator using Rust + Toptal   --");
    println!("--                                                     --");
    println!("---------------------------------------------------------");

    // Ask the user for the .gitignore terms.
    let mut terms = String::new();
    println!();
    println!("Please enter each .gitignore term seperated by a space:");
    println!();

    io::stdin()
        .read_line(&mut terms)
        .expect("Failed to read line");

    // Check that the user entered some terms
    if terms.trim().is_empty() {
        println!("You must enter at least one term.");
        return;
    }

    // Replace each space with a comma.
    terms = terms.replace(" ", ",");

    // Create a new url from the base url and the terms.
    let url = format!("{}{}", BASE_URL, terms);

    // Print the url
    println!("URL: {}", url);

    // Make the API call to Toptal, previously, the web scraping worked nicely.

    // Save the .gitignore file to the current directory (this should be a project root folder).
}
