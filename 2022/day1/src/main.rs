use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut elfes: Vec<i32> = Vec::new();

    let mut calories = 0;
    for line in reader.lines() {
	let data = line.unwrap();
        if !data.is_empty() {
            calories += data.parse::<i32>().unwrap();
	}
	else {
	    elfes.push(calories);
	    calories = 0;
	}
    }
    elfes.sort();
    let top1: i32 = *elfes.last().unwrap();
    let top3: i32 = elfes.iter().rev().take(3).sum();
    
    println!("answer 1 : {}", top1);
    println!("answer 2 : {}", top3);
    Ok(())
}

