use aoc_runner_derive::{aoc, aoc_generator};
use permutohedron::LexicalPermutation;

use crate::intcode::{IntCode, State};

#[aoc_generator(day7)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.split(',').map(|it| it.parse().unwrap()).collect()
}

fn permutate(mut data: [i64; 5]) -> Vec<Vec<i64>> {
    let mut permutations = vec![data.to_vec()];

    while data.next_permutation() {
        permutations.push(data.to_vec());
    }

    permutations
}

#[aoc(day7, part1)]
pub fn part1(mem: &[i64]) -> Option<i64> {
    permutate([0, 1, 2, 3, 4])
        .iter()
        .map(|it| {
            it.iter().fold(0, |acc, elem| {
                let mut vm = IntCode::new(mem);

                match vm.run_with_input(0, &[*elem, acc]) {
                    State::Halted(n) => n,
                    State::Write(n) => n,
                    _ => panic!("Too many inputs?"),
                }
            })
        })
        .max()
}

#[aoc(day7, part2)]
pub fn part2(mem: &[i64]) -> Option<i64> {
    permutate([5, 6, 7, 8, 9])
        .iter()
        .map(|it| {
            let mut last = 0;
            let mut last_output = 0;
            let mut needs_phases = true;

            let mut vms = vec![
                IntCode::new(mem),
                IntCode::new(mem),
                IntCode::new(mem),
                IntCode::new(mem),
                IntCode::new(mem),
            ];

            loop {
                let inp_idx = if needs_phases { 0 } else { 1 };

                for i in 0..it.len() {
                    let it_val = it[i];
                    let cur_vm = &mut vms[i];

                    match cur_vm.run_with_input(inp_idx, &[it_val, last]) {
                        State::Halted(_) => return last_output,
                        State::Write(n) => {
                            last_output = n;
                            last = n;
                        }
                        _ => panic!("Too many inputs?"),
                    }
                }

                needs_phases = false;
            }
        })
        .max()
}
