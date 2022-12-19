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

/// Solve Advent of Code day 05 part one
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part one of advent of code
/// day 05.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_one(data: &str) -> String {
    let (setup, instructions) = data.split_once("\n\n").unwrap();
    let mut setup = parse_setup(setup);
    

    // println!("{:?}",
    instructions.split('\n')
        .map(|s| s.split(' '))
        .map(|mut s| (s.nth(1).expect("lolnup1").parse::<usize>().unwrap(),
                  s.nth(1).expect("lolnup2").parse::<usize>().unwrap(),
                  s.nth(1).expect("lolnup3").parse::<usize>().unwrap()))
        .for_each(|(n, f, t)| {
            let id = setup[f-1].len()-n;
            let tmp: Vec<u8> = setup[f-1].drain(id..).rev().collect();
            setup[t-1].extend(tmp);
            // setup[t-1].push_str(&tmp)
        })
    // )
    ;
    std::str::from_utf8(setup.into_iter().map(|mut s| s.pop().unwrap()).collect::<Vec<_>>().as_slice()).unwrap().to_string()
}

fn parse_setup(setup: &str)-> Vec<Vec<u8>> {

    let mut setup = setup.split('\n')
                 .rev()
                 .map(|ss| ss.as_bytes().chunks(4));

    let l = setup.next().unwrap().count();

    let mut piles = vec![vec![]; l];

    setup.for_each(|r| r.map(|c| c[1])
              .enumerate()
              .for_each(|(i, c)| if c != 32 { piles[i].push(c as u8)}));

    piles

}

/// Solve Advent of Code day 05 part two
///
/// # Arguments
///
///  - `data` : a `&str` that holds the input for today's puzzle
///
/// # Return value
///
/// This function returns a `usize`, the result for part
/// two of advent of code day 05.
///
/// # Panics
///
/// If any conversion assumed to be valid with the input fails, panics.
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn solve_part_two(data: &str) -> String {
    let (setup, instructions) = data.split_once("\n\n").unwrap();
    let mut setup = parse_setup(setup);
    

    // println!("{:?}",
    instructions.split('\n')
        .map(|s| s.split(' '))
        .map(|mut s| (s.nth(1).expect("lolnup1").parse::<usize>().unwrap(),
                  s.nth(1).expect("lolnup2").parse::<usize>().unwrap(),
                  s.nth(1).expect("lolnup3").parse::<usize>().unwrap()))
        .for_each(|(n, f, t)| {
            let id = setup[f-1].len()-n;
            let tmp: Vec<u8> = setup[f-1].drain(id..).collect();
            setup[t-1].extend(tmp);
            // setup[t-1].push_str(&tmp)
        })
    // )
    ;
    std::str::from_utf8(setup.into_iter().map(|mut s| s.pop().unwrap()).collect::<Vec<_>>().as_slice()).unwrap().to_string()
}

// vim: set tw=80:
