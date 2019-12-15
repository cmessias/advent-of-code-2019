use std::fs;
use intcode::IntcodeMachine;

fn read_input(path: &str) -> Vec<isize> {
    return fs::read_to_string(path)
        .expect("Failed to read input file. Place it in the root of the module.")
        .trim()
        .split(",")
        .map(|num| num.parse::<isize>().unwrap())
        .collect();
}

fn main() {
    let tape: Vec<isize> = read_input("input");

    // Part 1
    let mut machine = IntcodeMachine::init(tape.clone(), 12, 02);
    println!("Part 1: {}", machine.run());

    // part 2
    let desired_output = 19690720;

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut machine = IntcodeMachine::init(tape.clone(), noun, verb);
            if machine.run() == desired_output {
                println!("Part 2: 100 * {} + {} = {}", noun, verb, 100* noun + verb);
            }
        }
    }
}