use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn compute_distances(levels: &Vec<i32>) -> Vec<i32>
{
    let mut distances = Vec::new();
    let mut iter = levels.iter();
    let mut prev_level = iter.next().expect("Should have at least one level");

    for level in iter {
        distances.push(level - prev_level);
        prev_level = level;
    }

    return distances;
}


fn report_is_safe(levels: &Vec<i32>, dampening: bool) -> bool {
    // Compute distances
    let distances = compute_distances(&levels);

    // Check if any of the distances are too high or too low
    if let Some(index) = distances.iter().position(|d| *d == 0 || d.abs() > 3) {
        if (!dampening) {
            return false; // No dampening, report is fucked right away
        }
        
        // Try to remove level on the left and see if it's safe
        let mut new_list = levels.clone();
        new_list.remove(index);
        if (report_is_safe(&new_list, false)) {
            return true;
        } else {
            // Left level didn't fix the report, let's hope it's the right then...
            new_list = levels.clone();
            new_list.remove(index+1);
            return report_is_safe(&new_list, false);
        }
    }

    // Count positive increments
    let num_increasing = distances.iter().filter(|d| **d > 0).count();

    if num_increasing == distances.len() || num_increasing == 0 {
        // If all distances are positive (i.e. all increments) or we have exactly 0 positive distances (i.e. all are negative/decrementing) then report is safe
        return true;
    } else if dampening && (num_increasing == (distances.len()-1) || num_increasing == 1){
        // Only one of the reports is fucked, let's try and fix this
        // Find the odd level
        let index;
        if (num_increasing == 1) {
            index = distances.iter().position(|d| *d > 0).expect("We already checked there is exactly one positive distance");
        } else {
            index = distances.iter().position(|d| *d < 0).expect("We already checked there is exactly one negative distance");
        }
        // Try to remove level on the left and see if it's safe
        let mut new_list = levels.clone();
        new_list.remove(index);
        if (report_is_safe(&new_list, false)) {
            return true;
        } else {
            // Left level didn't fix the report, let's hope it's the right then...
            new_list = levels.clone();
            new_list.remove(index+1);
            return report_is_safe(&new_list, false);
        }
    } else {
        // It's fucked beyond repair
        return false;
    }
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
        let levels = line?.as_str().split_ascii_whitespace().map(|s| s.parse::<i32>().expect("Element in report is not an integer")).collect();
        if report_is_safe(&levels, false) {safe_reports += 1;}
        // *** Second half of the puzzle ***
        if report_is_safe(&levels, true) {safe_reports_dampened += 1;}
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