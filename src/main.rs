#![allow(dead_code)]

use brainfuck::*;

fn factorial(brainfuck: &mut BrainFuck) {
    brainfuck.take_input("Enter some number to calculate factorial: ");
}

fn test(brainfuck: &mut BrainFuck) {
    brainfuck.take_input("Enter some number: ");
    brainfuck.subtract_from_current_cell('0' as u8, None, true);
    // brainfuck.if_current_cell_is_zero_else(
    //     |brainfuck: &mut BrainFuck| brainfuck.print_string("You entered zero!"),
    //     |brainfuck: &mut BrainFuck| brainfuck.print_string("You didn't enter zero!"),
    //     false,
    //     true,
    // );
    // brainfuck.if_elif_else(
    //     vec![
    //             (0, Box::new(|brainfuck: &mut BrainFuck| brainfuck.print_string("You entered zero!"))),
    //             (0, Box::new(|brainfuck: &mut BrainFuck| brainfuck.print_string("You didn't enter zero!"))),
    //         ],
    //     false,
    //     true,
    // );
}

#[rustfmt::skip]
fn main() {
    let mut brainfuck = BrainFuck::new(1);
    // brainfuck.print_string("Hello World!");
    brainfuck.print_string("This is a huge text which is printed for testing my brainfuck code generator!");
    // test(&mut brainfuck);
    let code = brainfuck.get_optimised_code();
    println!("{}\n\n", code);
    brainfuck.run_code();
    println!("\n\n");
    brainfuck.print_interpreter();

}
