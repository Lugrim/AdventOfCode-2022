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

//FIXME: Change this on the day you start working on the puzzle
#![allow(unused_variables)]

/// Solve Advent of Code day 03 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 03.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_one(data: &str) -> usize {
    data.split('\n')
        .map(|s| (s.chars().take(s.chars().count()/2), s.chars().skip(s.chars().count()/2)))
        .map(|(mut a, mut b)| bitflags(&mut a) & bitflags(&mut b))
        .map(|a| a.trailing_zeros())
        .fold(0, |acc, i| acc + i)
        .try_into()
        .unwrap()
}

fn priority(c: u8) -> u8 {
    if c >= 65 && c <=90 {
        c - 38
    } else {
        c - 96
    }
}

fn bitflags(data: &mut dyn std::iter::Iterator<Item=char>) -> u64 {
    data.fold(0, |a, c| a | 1 << priority(c as u8))
}

/// Solve Advent of Code day 03 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 03.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_two(data: &str) -> usize {
    data.split('\n')
        .collect::<Vec<&str>>()[..]
        .windows(3)
        .step_by(3)
        .map(|a| bitflags(&mut a[0].chars())
            & bitflags(&mut a[1].chars())
            & bitflags(&mut a[2].chars()))
        .map(|a| a.trailing_zeros())
        .fold(0, |acc, i| acc + i)
        .try_into()
        .unwrap()
}

// vim: set tw=80:
