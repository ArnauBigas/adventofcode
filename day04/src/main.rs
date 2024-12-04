use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn get_xy(pos: usize, puzzle_size: usize) -> (i32, i32) {
    let x = pos as i32 % puzzle_size as i32;
    let y = pos as i32 / puzzle_size as i32;
    (x, y)
}

fn get_pos(x: usize, y: usize, puzzle_size: usize) -> usize {
    y * puzzle_size + x
}

fn find_xmas(puzzle: &String, puzzle_size: usize, pos: usize, x_offset: i32, y_offset: i32) -> bool {
    let magic = "XMAS".as_bytes();
    let bytes = puzzle.as_bytes();
    let mut current_pos = pos;

    for char in magic {
        // Check current character
        if bytes[current_pos] != *char { return false; }

        // All is good, calculate next position
        let (x, y) = get_xy(current_pos, puzzle_size);
        let (next_x, next_y) = (x + x_offset, y + y_offset);

        // Check bounds
        if *char != b'S' && (next_x < 0 || next_y < 0 || next_x >= puzzle_size as i32 || next_y >= puzzle_size as i32) {
            return false; // Out of bounds, couldn't find XMAS :(
        } else {
            if *char != b'S' { // If we have to check more letters, advance pos
                current_pos = get_pos(next_x as usize, next_y as usize, puzzle_size);
            }
        }
    }

    return true;
}


fn find_mas(puzzle: &String, puzzle_size: usize, pos: usize) -> bool {
    // First of all, check bounds
    let (x, y) = get_xy(pos, puzzle_size);
    if x == 0 || y == 0 || x == (puzzle_size-1) as i32 || y == (puzzle_size-1) as i32 {
        return false; // Out of bounds, couldn't find XMAS :(
    }

    let bytes = puzzle.as_bytes();

    let tl = bytes[get_pos((x-1) as usize, (y-1) as usize, puzzle_size)];
    let tr = bytes[get_pos((x+1) as usize, (y-1) as usize, puzzle_size)];
    let bl = bytes[get_pos((x-1) as usize, (y+1) as usize, puzzle_size)];
    let br = bytes[get_pos((x+1) as usize, (y+1) as usize, puzzle_size)];

    if (tl == b'M' && br == b'S') || (tl == b'S' && br == b'M') {
        if (bl == b'M' && tr == b'S') || (bl == b'S' && tr == b'M') {
            return true
        }
    }

    return false;
}

fn solve_puzzle(input: &str) -> std::io::Result<()>  {
    println!("Input: {}", input);

    let mut puzzle = String::new();

    // Read the file
    BufReader::new(File::open(input)?).read_to_string(&mut puzzle)?;

    // Find size by finding first line break position
    let puzzle_size = puzzle.find("\n").expect("Wrong file format");

    // Delete all line breaks to have continuous puzzle data
    puzzle = puzzle.replace("\n", "");

    let mut xmas_found = 0; // First Half
    let directions = vec![(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];
    for (pos, _) in puzzle.match_indices("X") {
        for (x_offset, y_offset) in &directions {
            if find_xmas(&puzzle, puzzle_size, pos, *x_offset, *y_offset) { xmas_found += 1; }
        }
    }

    let mut mas_found = 0; // Second Half
    for (pos, _) in puzzle.match_indices("A") {
        if find_mas(&puzzle, puzzle_size, pos) { mas_found += 1; }
    }

    println!("\tXMAS found: {}", xmas_found);
    println!("\tMAS found: {}", mas_found);

    Ok(())
}

fn main() -> std::io::Result<()> {
    solve_puzzle("example.txt")?;
    solve_puzzle("input.txt")?;

    Ok(())
}