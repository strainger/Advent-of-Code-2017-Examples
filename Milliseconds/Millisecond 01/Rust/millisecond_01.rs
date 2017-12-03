use std::fs::File;
use std::io::prelude::*;
// Created by strainger
fn main() {
    let mut file = File::open("list.txt").expect("File Not Found");
    let mut contents = String::new();
    let mut last_char = ' ';
    let mut sum = 0;
    file.read_to_string(&mut contents).expect("Could not read file");
    for i in 0..contents.trim().chars().count() {
        if contents.trim().chars().nth(i).unwrap() == last_char {
                sum += contents.trim().chars().nth(i).unwrap().to_digit(10).expect("Unable to parse character to string");
        } else if i == contents.trim().chars().count() - 1 {
            if contents.trim().chars().nth(contents.trim().chars().count()-1).unwrap() == 
            contents.trim().chars().nth(0).unwrap() {
                sum += contents.trim().chars().nth(0).unwrap().to_digit(10).expect("Unable to parse character to string");
            }
        }
        last_char = contents.trim().chars().nth(i).unwrap()
    }
    println!("{} total matches", sum);
}
