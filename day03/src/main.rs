use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;

fn solve_puzzle(input: &str) -> std::io::Result<()>  {
    println!("Input: {}", input);

    // Create a regex with a matching group with 3 possibilities:
    // - Group mul: contains a mul instruction, with group x being a 3 digit integer and group y, also an integer.
    // - Group do: contains a do instruction
    // - Group dont: contains a don't instruction
    let re = Regex::new(r"((?<mul>mul\((?<x>\d{1,3}),(?<y>\d{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\)))").unwrap();

    let mut sum = 0; // First Half
    let mut sum_conditional = 0; // Second Half
    let mut enabled = true;
    
    // Read the file
    let file = File::open(input)?;
    let buf_reader = BufReader::new(file);
    
    // Parse line by line, 
    for line in buf_reader.lines() {
        let line_str = line?;
        for capture_matches in re.captures_iter(&line_str) {
            if let Some(_) = capture_matches.name("do") {
                enabled = true;
            } else if let Some(_) = capture_matches.name("dont") {
                enabled = false;
            } else { // We must be in mul() instruction
                let x = capture_matches.name("x").expect("Group x should be present by regex construction")
                            .as_str().parse::<u32>().expect("Should be an int by regex construction");
                let y = capture_matches.name("y").expect("Group y should be present by regex construction")
                            .as_str().parse::<u32>().expect("Should be an int by regex construction");

                sum += x*y; // First Half

                if enabled {
                    sum_conditional += x*y; // Second Half
                }
            }
        }
    }

    println!("\tSum: {}", sum);
    println!("\tSum (conditional): {}", sum_conditional);

    Ok(())
}

fn main() -> std::io::Result<()> {
    solve_puzzle("example1.txt")?;
    solve_puzzle("example2.txt")?;
    solve_puzzle("input.txt")?;

    Ok(())
}