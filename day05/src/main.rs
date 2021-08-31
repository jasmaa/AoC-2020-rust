use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

// Converts seating code to id.
fn seating2id(seating: &String) -> u32 {
    let mut row: u32 = 0;
    let mut col: u32 = 0;
    for (i, c) in seating.chars().enumerate() {
        if i < 7 {
            if c == 'F' {
                row = row << 1;
            } else if c == 'B' {
                row = row << 1 | 1;
            } else {
                panic!("Invalid seating.");
            }
        } else {
            if c == 'L' {
                col = col << 1
            } else if c == 'R' {
                col = col << 1 | 1;
            } else {
                panic!("Invalid seating.")
            }
        }
    }
    row * 8 + col
}

// Finds missing seat in boarding pass id range.
fn find_my_seat_id(ids: Vec<u32>) -> u32 {
    let mut boarding_ids: HashSet<u32> = HashSet::new();
    for id in &ids {
        boarding_ids.insert(*id);
    }
    let lowest: u32 = *ids.iter().min().unwrap();
    let highest: u32 = *ids.iter().max().unwrap();
    for id in lowest..highest + 1 {
        if !boarding_ids.contains(&id) {
            return id;
        }
    }
    panic!("No seat found.");
}

fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut seatings: Vec<String> = Vec::new();
    loop {
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        seatings.push(line.clone().trim().to_string());
        line.clear();
    }

    let highest_pair = seatings
        .iter()
        .map(|x| (x, seating2id(&x)))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    println!("{}", highest_pair.1);

    let ids = seatings.iter().map(seating2id).collect::<Vec<u32>>();
    println!("{:?}", find_my_seat_id(ids));

    Ok(())
}
