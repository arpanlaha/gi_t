#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use gi_t::GiError;
use std::env;

fn main() {
    gi_t::process_args(env::args().skip(1)).unwrap_or_else(GiError::print);
}
