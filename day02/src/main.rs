use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn level_is_safe(increasing: bool, previous: u32, current: u32) -> bool {
    if increasing && current <= previous {return false;}
    if !increasing && current >= previous {return false;}
    if (increasing && (current - previous) > 3) || (!increasing && (previous - current) > 3) {return false;}
    return true;
}

fn report_is_safe(line: &str, dampening: bool) -> bool {
    let mut levels = line.split_ascii_whitespace().map(|s| s.parse::<u32>().expect("Element in report is not an integer")).peekable();
    let mut previous_level = levels.next().unwrap();
    let increasing = levels.peek().unwrap() > &previous_level;
    let mut has_been_dampened = false;
    for current_level in levels {
        if dampening {
            if !level_is_safe(increasing, previous_level, current_level) {
                if !has_been_dampened {has_been_dampened = true;}
                else {return false;}
            } else {
                previous_level = current_level;
            }
        } else {
            if !level_is_safe(increasing, previous_level, current_level) {return false;}
            previous_level = current_level;
        }
    }
    return true;
}

fn solve_puzzle(input: &str) -> std::io::Result<()>  {
    println!("Input: {}", input);

    let mut safe_reports = 0; // First Half
    let mut safe_reports_dampened = 0; // Second Half

    // Read the file
    let file = File::open(input)?;
    let buf_reader = BufReader::new(file);
    
    // Parse line by line, check report is safe
    for line in buf_reader.lines() {
        let line_str = line?;
        if report_is_safe(line_str.as_str(), false) {safe_reports += 1;}
        // *** Second half of the puzzle ***
        if report_is_safe(line_str.as_str(), true) {safe_reports_dampened += 1;}
    }

    println!("\tSafe Reports: {}", safe_reports);
    println!("\tSafe Reports (Dampening): {}", safe_reports_dampened);

    Ok(())
}

fn main() -> std::io::Result<()> {
    solve_puzzle("example.txt")?;
    solve_puzzle("input.txt")?;

    Ok(())
}