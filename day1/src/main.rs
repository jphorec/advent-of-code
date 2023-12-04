use std::fs::{File, read_to_string};
use std::io::Read;

fn main() {
    let lines = read_lines("input.txt");

    let mut sum = 0;
    lines.iter().for_each(|line| {
       let numbers = parse_ints(line);
       match numbers.len() {
           0 => return,
           1 => {
               sum = sum + concat_numbers(numbers[0], numbers[0]); }
           _ => {
               sum = sum + concat_numbers(numbers[0], numbers[numbers.len() - 1]);
           },
       }
   });

    println!("sum: {}", sum);
}

fn concat_numbers(num1: u32, num2: u32) -> u32 {
    return (num1.to_string() + &num2.to_string()).parse::<u32>().unwrap();
}

// parse all ints from a string
fn parse_ints(input: &str) -> Vec<u32> {
    input.chars()
        .filter_map(|a| a.to_digit(10))
        .collect()
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)
        .collect()  // gather them together into a vector
}