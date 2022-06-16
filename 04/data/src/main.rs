/*
 * Advent of Code 2021
 * Day 04
*/
use std::fs;

fn main() {
    let input_file = "src/input.txt";
    let input = fs::read_to_string(input_file).expect("Something went wrong reading the file");

    let numbers = input
        .split(',')
        .map(|s| s.parse::<i32>().expect("Could not parse"))
        .collect::<Vec<i32>>();

    println!("{:?}", numbers);
}
