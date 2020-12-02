use std::{env,fs};
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid number of arguments provided! Please specify the file to be read as a single parameter");
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();
    let values: Vec<&str> = contents.split("\n")
                                    .filter(|&x| x != "")
                                    .collect();
    let regex = Regex::new(r"^(\d+)-(\d+)\s([a-z]):\s([a-z]*)").unwrap();

    // part 1
    let mut invalid = 0;
    for line in &values {
        for parsed in regex.captures_iter(line) {
            let min:i32 = parsed[1].parse().unwrap();
            let max:i32 = parsed[2].parse().unwrap();
            let c:char = parsed[3].parse().unwrap();
            let val = &parsed[4];

            // count occurences of c in val
            let mut count = 0;
            for x in val.chars() {
                if c == x {
                    count += 1;
                }
            }

            // register as invalid
            if (count < min) || (count > max) {
                invalid += 1;
            } else {
            }
        }
    }
    println!("part 1: invalid: {}, valid: {}", invalid, &values.len()-invalid);

    // part 2
    
    invalid = 0;
    for line in &values {
        for parsed in regex.captures_iter(line) {
            let min:usize = parsed[1].parse().unwrap();
            let max:usize = parsed[2].parse().unwrap();
            let c:char = parsed[3].parse().unwrap();
            let val = &parsed[4];
            
            // filter out passwords which are too short
            if val.len() < max {
                invalid += 1;
                continue;
            }

            // Debug output
            // println!("{},{},{}", val.as_bytes()[min-1] as char, val.as_bytes()[max-1] as char, c)
            
            // Check passwords
            if !((val.as_bytes()[min-1] as char == c) ^ (val.as_bytes()[max-1] as char == c)) {
                invalid += 1;
            }
        }
    }
    println!("part 2: invalid: {}, valid: {}", invalid, &values.len()-invalid);
}
