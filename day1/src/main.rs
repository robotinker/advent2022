use std::fs;

fn main() {
    let contents = fs::read_to_string("day1/src/input.txt").expect("Input file could not be read!");
    let mut elf_calories: Vec<i32> = Vec::new();
    let mut current_elf = 0;
    
    for line in contents.lines() {
        if line.is_empty() {
            elf_calories.push(current_elf);
            current_elf = 0;
        }
        else {
            current_elf += line.parse::<i32>().unwrap();
        }
    }

    let mut biggest_stash = 0;
    for elf in elf_calories.iter() {
        if elf > &biggest_stash {
            biggest_stash = *elf;
        }
    }
    
    println!("{}", biggest_stash);

    elf_calories.sort();
    elf_calories.reverse();
    println!("{}", elf_calories[0] + elf_calories[1] + elf_calories[2]);
}