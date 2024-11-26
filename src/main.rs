use std::fs;
use emdb_lib::word_endings_freq::final_ly_ily_to_l;
use emdb_lib::punctuation::substitute_punctuation;

fn main() -> std::io::Result<()> {
    let _ = fs::remove_file("output.txt");
    
    let content = fs::read_to_string("input.txt")?;
    
    let result = substitute_punctuation(&content);

    println!("{}", result);

    fs::write("output.txt", result);
    println!("");
    println!("contents written to output.txt");
    let greaty = final_ly_ily_to_l("greatly");
    println!("{}", greaty);

    Ok(())
}