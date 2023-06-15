// https://github.com/dmitmel/brainwhat.git

#![allow(dead_code)]

use brainfuck::*;
use std::time::*;

fn factorial(brainfuck: &mut BrainFuck) {
    brainfuck.take_input("Enter some number to calculate factorial: ");
}

#[rustfmt::skip]
fn test(brainfuck: &mut BrainFuck) {
    brainfuck.take_input("Enter some number: ");
    // brainfuck.set_current_cell_value('9' as u8, 0, true);
    brainfuck.subtract_from_current_cell('0' as u8, None, true);
    brainfuck.if_elif_else(
        vec![
            (0, |brainfuck: &mut BrainFuck| {brainfuck.print_string("You entered zero!")}),
            (1, |brainfuck: &mut BrainFuck| {brainfuck.print_string("You entered one!")}),
            (2, |brainfuck: &mut BrainFuck| {brainfuck.print_string("You entered two!")}),
            (3, |brainfuck: &mut BrainFuck| {brainfuck.print_string("You entered three!")}),
            (4, |brainfuck: &mut BrainFuck| {brainfuck.print_string("You entered four!")}),
            (5, |brainfuck: &mut BrainFuck| {brainfuck.print_string("You entered five!")}),
            (6, |brainfuck: &mut BrainFuck| {brainfuck.print_string("You entered six!")}),
            (7, |brainfuck: &mut BrainFuck| {brainfuck.print_string("You entered seven!")}),
            (8, |brainfuck: &mut BrainFuck| {brainfuck.print_string("You entered eight!")}),
            (9, |brainfuck: &mut BrainFuck| {brainfuck.print_string("You entered nine!")}),
        ],
        |brainfuck: &mut BrainFuck| { brainfuck.print_string("You didn't enter a digit! Try entering a digit in the input") },
        false,
        true,
    );
}

#[rustfmt::skip]
fn main() {
    let mut brainfuck = BrainFuck::new(1);
    // brainfuck.print_string("Hello World!");
    // brainfuck.print_string("This is a huge text which is printed for testing my brainfuck code generator!");
    test(&mut brainfuck);
    let mut clock = Instant::now();
    let code = brainfuck.get_optimised_code();
    let code_generation_time = clock.elapsed();
    println!("{}\n\n", code);
    clock = Instant::now();
    brainfuck.run_code();
    let code_running_time = clock.elapsed();
    println!("\n\n");
    brainfuck.print_interpreter();
    println!(
        "\n\nCode Generation Time: {}\nCode Runnng Time: {}\n Num Steps: {}",
        code_generation_time.as_secs_f64(),
        code_running_time.as_secs_f64(),
        brainfuck.interpreter.get_num_steps()
    );
}
