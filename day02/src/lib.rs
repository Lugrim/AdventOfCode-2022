//! Library module with all the logic

// Clippy lints!
#![deny(clippy::cargo)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]

#![deny(missing_docs)]
#![deny(rustdoc::missing_crate_level_docs)]

/// Solve Advent of Code day 02 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 02.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_one(data: &str) -> usize {
    data.trim().split('\n')
        .map(|s| (s.chars().nth(0).unwrap() as i8 - 65, s.chars().nth(2).unwrap() as i8 - 88))
        .map(|(l, r)| ((r+1) + ((r-l+4) % 3) * 3) as usize)
        .sum()
}

/// Solve Advent of Code day 02 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 02.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_two(data: &str) -> usize {
    data.trim().split('\n')
        .map(|s| (s.chars().nth(0).unwrap() as i8 - 65, s.chars().nth(2).unwrap() as i8 - 88))
        .map(|(l, r)| ((((l+r+2) % 3) + 1) + (r * 3)) as usize)
        .sum()
}

// vim: set tw=80:
