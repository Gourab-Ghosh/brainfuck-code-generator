// https://github.com/dmitmel/brainwhat.git
// https://esolangs.org/wiki/brainfuck_algorithms

#![allow(dead_code)]

use brainfuck::*;
use std::time::*;

fn factorial(brainfuck: &mut BrainFuck) {
    brainfuck.take_input("Enter some number to calculate factorial: ");
}

#[rustfmt::skip]
fn test_if_else(brainfuck: &mut BrainFuck) {
    brainfuck.take_input("Enter some number: ");
    // brainfuck.set_current_cell_value('9' as CellData, 0, true);
    brainfuck.subtract_from_current_cell('0' as CellData, None, true);
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
        |brainfuck: &mut BrainFuck| { brainfuck.print_string("You didn't enter a digit! Try entering a single digit in the input!") },
        false,
        true,
    );
}

#[rustfmt::skip]
fn test_div_mod(brainfuck: &mut BrainFuck) {
    brainfuck.set_current_cell_value(100, 0, true);
    let stack = brainfuck.generate_stack(3);
    brainfuck.divide_current_cell_by(6, None, stack.get_start_index(), 0, false);
    brainfuck.delete_stack(stack, false, vec![0; 3]);
}

#[rustfmt::skip]
fn test_print_cell_value(brainfuck: &mut BrainFuck) {
    brainfuck.set_current_cell_value(245, 0, true);
    brainfuck.print_current_cell_value(false);
}

#[rustfmt::skip]
fn main() {
    let mut brainfuck = BrainFuck::new(1);
    // brainfuck.print_string("Hello World!");
    // brainfuck.print_string("This is a huge text which is printed for testing my brainfuck code generator!");
    // test_if_else(&mut brainfuck);
    // test_div_mod(&mut brainfuck);
    test_print_cell_value(&mut brainfuck);
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

// fn main() {
//     let mut interpreter = BrainFuckInterpreter::new();
//     interpreter.interpret(">++++[-<+++++++++++>]>[>++++++[-<-------->]>+++++++++[-<<<[->+>+<<]>>[-<<+
//     >>]>]<<[-<+>]]<<+++++.-----.+++++.----->-->+>+<<[-<.>>>[->+>+<<]<[->>>+<<<
//     ]>>[-<<+>>]>[->+<<<+>>]>[>>>>++++++++++<<<<[->+>>+>-[<-]<[->>+<<<<[->>>+<<<
//     ]>]<<]>+[-<+>]>>>[-]>[-<<<<+>>>>]<<<<]<[>++++++[<++++++++>-]<-.[-]<]<<<<]", false);
// }
