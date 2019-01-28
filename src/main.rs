#[macro_use]
extern crate lazy_static;

extern crate regex;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(String::from("input/") + &args[1]).unwrap();
    let buf_reader = BufReader::new(file);
    let input: Vec<String> = buf_reader.lines().map(|x| x.unwrap()).collect();
    let args_ref: Vec<&str> = args.iter().map(|x| x.as_str()).collect();
    let result = match &args_ref[..] {
        [_, "day1", "std"] => day1::std(input),
        [_, "day1", "plus"] => day1::plus(input),
        [_, "day2", "std"] => day2::std(input),
        [_, "day2", "plus"] => day2::plus(input),
        [_, "day3", "std"] => day3::std(input),
        [_, "day3", "plus"] => day3::plus(input),
        [_, "day4", "std"] => day4::std(input),
        [_, "day4", "plus"] => day4::plus(input),
        [_, "day5", "std"] => day5::std(input),
        [_, "day5", "plus"] => day5::plus(input),
        _ => panic!("Function not found"),
    };
    println!("The result is {}", result.unwrap());
}
