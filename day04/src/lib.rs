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

/// Solve Advent of Code day 04 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 04.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_one(data: &str) -> usize {
    data.split('\n')
        .map(|s| s.split_once(',').unwrap())
        .map(|(a, b)| (parse_pair(a.split_once('-')), parse_pair(b.split_once('-'))))
        .filter(|&a| included(a))
        .count()
        // .filter(|(a, b)| included(a, b))
        // .count()
}

fn parse_pair(p: Option<(&str, &str)>) -> (usize, usize) {
    let (a, b) = p.unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

fn included(((al, ar), (bl, br)): ((usize, usize), (usize, usize))) -> bool {
    (al >= bl && ar <= br) || (bl >= al && br <= ar)
}

/// Solve Advent of Code day 04 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 04.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_two(data: &str) -> usize {
    data.split('\n')
        .map(|s| s.split_once(',').unwrap())
        .map(|(a, b)| (parse_pair(a.split_once('-')), parse_pair(b.split_once('-'))))
        .filter(|&a| overlap(a))
        .count()
}

fn overlap(((al, ar), (bl, br)): ((usize, usize), (usize, usize))) -> bool {
    !((al < bl && ar < bl) || (al > br && ar > br))
}
// vim: set tw=80:
