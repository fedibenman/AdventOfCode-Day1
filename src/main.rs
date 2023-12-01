use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("./file.txt")?;
    let reader = io::BufReader::new(file);

    let mut line_numbers: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(text) => {
                let digits: Vec<char> = text.chars().filter(|c| c.is_digit(10)).collect();

                if !digits.is_empty() {
                    let result: String = format!("{}{}", digits[0], digits[digits.len() - 1]);

                    line_numbers.push(result);
                }
            }
            Err(err) => eprintln!("Error reading line: {}", err),
        }
    }

    let sum: i32 = line_numbers
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .sum();

    println!("Sum of parsed integers: {}", sum);

    Ok(())
}