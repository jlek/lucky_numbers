#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::module_inception, clippy::module_name_repetitions)]

//! This library crate contains the code required to count the number of occurences of a sequence of digits in a range of numbers.

mod count_lucky_numbers;
mod lucky_numbers_iterator;
mod number_of_matches;

pub use self::count_lucky_numbers::count_lucky_numbers;
