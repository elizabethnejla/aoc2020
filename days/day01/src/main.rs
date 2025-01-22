use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("should have been able to read I guess?");
    let part_one = day01::run_part_one(input);
    println!("{}", part_one);
}
