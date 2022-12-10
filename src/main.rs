// -*- compile-command: "Cargo run" -*-
use std::fs;

fn main() {
    let data = fs::read_to_string("day1.txt").expect("This failed");
    let raw_elves: Vec<&str> =
	data.split("\n\n").collect();
    let split_elves: Vec<Vec<&str>> = raw_elves.iter().map(|x| {
	x.split_terminator("\n").collect::<Vec<&str>>()
    }).collect();
    let num_elves : Vec<Vec<u32>> = split_elves.iter().map(|one_elf| {
	one_elf.iter().map(|num| num.parse::<u32>().unwrap()).collect()
    }
    ).collect();
    let sum_elves : Vec<u32> =
	num_elves
	.iter()
	.map(|e|
	     e.iter().sum::<u32>()
	).collect();
    // let my_list = vec![1,2,3];
    // let my_sum : i32 = my_list.iter().sum();
    println!("{:?}",sum_elves);
}
