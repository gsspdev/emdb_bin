use std::fs;
use regex::Regex;

fn main() -> std::io::Result<()> {
    // Read the content of 'input.txt' into a String
    let content = fs::read_to_string("input.txt")?;

    // Define the regex pattern to match punctuation at the end of paragraphs
    let re = Regex::new(r"([.!?])\s*(\n\s*\n|\n*\z)").unwrap();

    // Perform the substitution to double the punctuation marks
    let result = re.replace_all(&content, "$1$1$2");

    // Write the modified content to 'output.txt'
    fs::write("output.txt", result.as_ref())?;

    Ok(())
}
