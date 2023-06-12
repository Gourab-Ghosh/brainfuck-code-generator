#![allow(unused_imports)]
#![allow(dead_code)]

mod brainfuck;
mod constants;

use brainfuck::*;
use constants::*;
use itertools::*;

fn main() {
    let mut brainfuck = BrainFuck::new(1);
    // brainfuck.println_string("Hello World!");
    brainfuck.println_string(
        "This is a huge text which is printed for testing my brainfuck code generator!",
    );
    let code = brainfuck.get_optimised_code();
    println!("{}", code);
}
