use pyo3::prelude::*;

/// Each day is in a different module
mod day01;
mod day02;

#[pyfunction]
fn run_day_part(day: u8, part: u8, data: &str) {
    match (day, part) {
        (1, 1) => day01::run_part1(data),
        (1, 2) => day01::run_part2(data),
        (2, 1) => day02::run_part1(data),
        _ => println!("Need to specify day/part number"),
    }
}

#[pymodule]
fn advent2023(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_day_part, m)?)?;
    Ok(())
}
