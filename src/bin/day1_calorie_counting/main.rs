use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn main() -> io::Result<()> {
    println!("Day 1");

    let file = File::open("./src/bin/day1_calorie_counting/input.txt")?;
    let reader = BufReader::new(file);

    let mut max = 0;
    let mut current = 0;
    for line in reader.lines() {
        if let Ok(line_raw) = line {
            if line_raw.is_empty() {
                if current > max {
                    max = current;
                }
                current = 0;
            } else {
                let value = line_raw.parse::<i32>();
                if let Ok(num) = value {
                    current += num;
                }
            }
            println!("{}", line_raw);
        }  
    }

    println!("da solution is {:?}", max);

    Ok(())
}