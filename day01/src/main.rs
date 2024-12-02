use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::zip;

fn main() -> std::io::Result<()> {
    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();
    
    // Read the file
    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);
    
    // Parse line by line, put ID into corresponding list
    for line in buf_reader.lines() {
        let line_str = line?;
        let values: Vec<&str> = line_str.split_ascii_whitespace().collect();
        list_1.push(values[0].parse::<u32>().unwrap());
        list_2.push(values[1].parse::<u32>().unwrap());
    }
    
    // Sort lists
    list_1.sort();
    list_2.sort();
    
    // *** First half of the puzzle ***
    
    let mut total_distance = 0;
    
    // Find distance between two IDs
    for (a, b) in zip(&list_1, &list_2) {
        if a > b { total_distance += a - b; }
        else { total_distance += b - a; }
    }
    
    println!("Total Distance: {}", total_distance);
    
    // *** Second half of the puzzle ***
    
    let mut similarity_score = 0;
    
    // Contains the number of times a particular ID appears in list_2
    let mut id_histogram = HashMap::new();
    
    for id in &list_2 {
        let hashmap_entry = id_histogram.get(&id);
        let _ = match hashmap_entry {
            Some(&count) => id_histogram.insert(id, count+1),
            None => id_histogram.insert(id, 1)
        };
    }
    
    for id in &list_1 {
        let hashmap_entry = id_histogram.get(&id);
        let _ = match hashmap_entry {
            Some(&count) => similarity_score += id*count,
            None => similarity_score += 0
        };
    }
    
    println!("Similarity Score: {}", similarity_score);
    
    Ok(())
}