use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::{IntCode, State};

#[aoc_generator(day5)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.split(',').filter_map(|it| it.parse().ok()).collect()
}

fn run_vm_with_input(v: &[i64], inp: i64) -> i64 {
    let mut vm = IntCode::new(v);

    loop {
        match vm.run() {
            State::Write(0) => {}
            State::Write(n) | State::Halted(n) => return n,
            State::Waiting => vm.input(inp),
        }
    }
}

#[aoc(day5, part1)]
pub fn part1(v: &[i64]) -> i64 {
    run_vm_with_input(v, 1)
}

#[aoc(day5, part2)]
pub fn part2(v: &[i64]) -> i64 {
    run_vm_with_input(v, 5)
}
