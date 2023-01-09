use std::env;
use std::process;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];

    // Your code here :)
    let file = File::open(filename).expect("Fail to open file!");
    let mut s: u32 = 0;

    for line in io::BufReader::new(file).lines() {
        let line_str = line.unwrap();
        for word in line_str.split(" ") {
            if word.len() > 0 {
                s += 1;
            }
        }
    }
    println!("{} has {} words.", filename, s);
}
