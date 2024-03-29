use crate::intcode::{IntCode, State};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::izip;

#[aoc_generator(day15)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.split(',').filter_map(|it| it.parse().ok()).collect()
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct DroidState {
    vm: IntCode,
    oxygen: bool,
}

fn successors(ds: &DroidState) -> Vec<DroidState> {
    let mut result = vec![];

    for direction in 1..=4 {
        let mut vm = ds.vm.clone();
        if vm.run() == State::Waiting {
            vm.input(direction);

            if let State::Write(n) = vm.run() {
                match n {
                    0 => { /* hit a wall */ }
                    1 | 2 => {
                        // moved one step - new state
                        let new_state = DroidState { vm, oxygen: n == 2 };
                        result.push(new_state);
                    }
                    _ => unreachable!("Unknown response code!"),
                }
            }
        }
    }

    result
}

const fn found_oxygen(ds: &DroidState) -> bool {
    ds.oxygen
}

fn find_oxygen(ds: &DroidState) -> Option<(DroidState, usize)> {
    let states = pathfinding::prelude::bfs(ds, successors, found_oxygen)?;
    Some((
        states
            .last()
            .cloned()
            .expect("There should be at least 1 state"),
        states.len() - 1,
    ))
}

#[aoc(day15, part1)]
pub fn part1(inp: &[i64]) -> Option<usize> {
    let vm = IntCode::new(inp);
    let ds = DroidState { vm, oxygen: false };

    let (_, cost) = find_oxygen(&ds)?;

    Some(cost)
}

#[aoc(day15, part2)]
pub fn part2(inp: &[i64]) -> Option<usize> {
    let vm = IntCode::new(inp);
    let ds = DroidState { vm, oxygen: false };

    let (ds, _) = find_oxygen(&ds)?;

    let all_nodes =
        pathfinding::prelude::dijkstra_all(&ds, |it| izip!(successors(it), std::iter::repeat(1)));

    all_nodes
        .values()
        .max_by_key(|(_, cost)| cost)
        .map(|(_, cost)| *cost - 1)
}
