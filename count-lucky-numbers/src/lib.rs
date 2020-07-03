#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::module_inception, clippy::module_name_repetitions)]

mod count_lucky_numbers;
mod lucky_numbers_iterator;
mod number_of_matches;

pub use self::count_lucky_numbers::count_lucky_numbers;
