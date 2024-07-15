use pyo3::prelude::*;

/// Each day is in a different module
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

#[pyfunction]
fn run_day_part(day: u8, part: u8, data: &str) {
    match (day, part) {
        (1, 1) => day01::run_part1(data),
        (1, 2) => day01::run_part2(data),
        (2, 1) => day02::run_part1(data),
        (2, 2) => day02::run_part2(data),
        (3, 1) => day03::run_part1(data),
        (3, 2) => day03::run_part2(data),
        (4, 1) => day04::run_part1(data),
        (4, 2) => day04::run_part2(data),
        (5, 1) => day05::run_part1(data),
        (5, 2) => day05::run_part2(data),
        (6, 1) => day06::run_part1(data),
        (6, 2) => day06::run_part2(data),
        (7, 1) => day07::run_part1(data),
        (7, 2) => day07::run_part2(data),
        (8, 1) => day08::run_part1(data),
        (8, 2) => day08::run_part2(data),
        (9, 1) => day09::run_part1(data),
        (9, 2) => day09::run_part2(data),
        (10, 1) => day10::run_part1(data),
        (10, 2) => day10::run_part2(data),
        _ => println!("Need to specify day/part number"),
    }
}

#[pymodule]
fn advent2023(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_day_part, m)?)?;
    Ok(())
}
