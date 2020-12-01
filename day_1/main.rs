use std::{env,fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid number of arguments provided! Please specify the file to be read as a single parameter");
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();
    let values: Vec<u32> = contents.split("\n")
                                    .filter(|&x| x != "")
                                    .map(|x| x.parse::<u32>()
                                    .unwrap()).collect();

    println!("Searching for two numbers adding up to 2020");
    let (x, y) = find_two(&values);
    println!("Product: {}\n", x * y);
    
    println!("Searching for three numbers adding up to 2020");
    let (x, y, z) = find_three(&values);
    println!("Product: {}\n", x * y * z);
}

fn find_two(values: &Vec<u32>) -> (u32, u32) {
    for (i, l) in values.iter().enumerate() {
        for n in &values[i..values.len()] {
            if n + l == 2020 {
                println!("Found numbers: {} {}", l, n);
                return (*n, *l)
            }
        }
    }
    (0, 0)
}

fn find_three(values: &Vec<u32>) -> (u32, u32, u32) {
    for (i, l) in values.iter().enumerate() {
        for n in &values[i..values.len()] {
            for m in values {
                if m+ n + l == 2020 {
                    println!("Found numbers: {} {} {}", l, n, m);
                    return (*n, *l, *m)
                }
            }
        }
    }
    (0, 0, 0)
}
