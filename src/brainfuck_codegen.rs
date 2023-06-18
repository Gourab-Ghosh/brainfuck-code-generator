use core::num;
use std::vec;

use super::*;

#[derive(Clone)]
pub struct BrainFuck {
    curr_index: usize,
    stacks: Vec<Stack>,
    code: String,
    value_changer_threshold: CellData,
    pub interpreter: BrainFuckInterpreter,
}

impl BrainFuck {
    pub fn new(initial_stack_size: usize) -> BrainFuck {
        BrainFuck {
            curr_index: 0,
            stacks: vec![Stack::new(0, initial_stack_size)],
            code: String::new(),
            value_changer_threshold: if OPTIMISE_CODE { CellData::MAX } else { 15 },
            interpreter: BrainFuckInterpreter::new(),
        }
    }

    pub fn get_value_changer_threshold(&self) -> CellData {
        self.value_changer_threshold
    }

    pub fn set_value_changer_threshold(&mut self, threshold: CellData) {
        self.value_changer_threshold = threshold;
    }

    pub fn get_last_empty_cell(&self) -> usize {
        if self.stacks.len() == 0 {
            return 0;
        }
        self.stacks
            .iter()
            .map(|stack| stack.get_end_index())
            .max()
            .unwrap()
    }

    pub fn get_current_index(&self) -> usize {
        self.curr_index
    }

    pub fn go_to_cell(&mut self, index: usize) {
        let difference = self.curr_index.abs_diff(index);
        if difference == 0 {
            return;
        }
        if index > self.curr_index {
            self.shift_right(difference)
        } else {
            self.shift_left(difference)
        };
    }

    pub fn shift_left(&mut self, num_times: usize) {
        for _ in 0..num_times {
            self.code += "<";
        }
        self.curr_index -= num_times;
    }

    pub fn shift_right(&mut self, num_times: usize) {
        for _ in 0..num_times {
            self.code += ">";
        }
        self.curr_index += num_times;
    }

    pub fn clear_current_cell(&mut self) {
        self.code += "[-]";
    }

    pub fn take_input(&mut self, message: &str) {
        if message != "" {
            self.print_string(message);
        }
        self.code += ",";
    }

    pub fn print_current_cell(&mut self) {
        self.code += ".";
    }

    // pub fn print_current_cell_value(&mut self, restore_index: bool) {
    //     let curr_index = self.curr_index;
    //     let stack = self.generate_stack(2);
    //     self.copy_value_without_overwriting(curr_index, stack.get_start_index(), false);
    //     self.jump_to_stack(stack);
    //     self.code += "[";
    //     self.divide_current_cell_by(10, None, stack.get_start_index() + 1, 0, true);
    //     self.shift_right(1);
    //     self.add_to_current_cell('0' as CellData, true);
    //     self.print_current_cell();
    //     self.clear_current_cell();
    //     self.shift_left(1);
    //     self.code += "]";
    //     self.delete_stack(stack, false, vec![0; 2]);
    //     if restore_index {
    //         self.go_to_cell(curr_index);
    //     }
    // }

    pub fn print_current_cell_value(&mut self, restore_index: bool) {
        let curr_index = self.curr_index;
        let cell_data_num_digits = (CellData::MAX as f64).log10().floor() as usize + 1;
        let stack = self.generate_stack(cell_data_num_digits);
        self.copy_value_without_overwriting(curr_index, stack.get_start_index(), false);
        self.jump_to_stack(stack);
        for idx in 1..cell_data_num_digits {
            self.divide_current_cell_by(10, None, stack.get_start_index() + cell_data_num_digits - idx, 0, true);
        }
        for idx in 0..cell_data_num_digits {
            self.go_to_cell(stack.get_start_index() + idx);
            self.add_to_current_cell('0' as CellData, true);
            self.print_current_cell();
        }
        self.delete_stack(stack, false, None);
        if restore_index {
            self.go_to_cell(curr_index);
        }
    }

    pub fn generate_stack(&mut self, stack_len: usize) -> Stack {
        let stack = Stack::new(self.get_last_empty_cell(), stack_len);
        self.stacks.push(stack);
        stack
    }

    pub fn delete_stack(
        &mut self,
        stack: Stack,
        restore_index: bool,
        optional_stack_vals: impl Into<Option<Vec<CellData>>>,
    ) {
        let stack_vals = optional_stack_vals.into().unwrap_or(vec![]);
        let curr_index = self.curr_index;
        for index in (stack.get_start_index()..stack.get_end_index()).rev() {
            if stack_vals.get(index - stack.get_start_index()) != Some(&0) {
                self.go_to_cell(index);
                self.clear_current_cell();
            }
        }
        if restore_index {
            self.go_to_cell(curr_index);
        }
        self.stacks = self
            .stacks
            .iter()
            .map(|&s| s)
            .filter(|&s| s != stack)
            .collect_vec();
    }

    pub fn jump_to_stack(&mut self, stack: Stack) {
        self.go_to_cell(stack.get_start_index());
    }

    pub fn move_value_without_overwriting(
        &mut self,
        from_index: usize,
        to_index: usize,
        restore_index: bool,
    ) {
        let curr_index = self.curr_index;
        if from_index == to_index {
            return;
        }
        self.go_to_cell(from_index);
        self.code += "[";
        self.go_to_cell(to_index);
        self.code += "+";
        self.go_to_cell(from_index);
        self.code += "-]";
        if restore_index {
            self.go_to_cell(curr_index);
        }
    }

    pub fn move_value(
        &mut self,
        from_index: usize,
        to_index: usize,
        from_index_optional_prev_value: impl Into<Option<CellData>>,
        to_index_optional_prev_value: impl Into<Option<CellData>>,
        restore_index: bool,
    ) {
        let from_index_optional_prev_value = from_index_optional_prev_value.into();
        let to_index_optional_prev_value = to_index_optional_prev_value.into();
        if from_index_optional_prev_value == to_index_optional_prev_value
            || from_index_optional_prev_value == Some(0)
        {
            return;
        }
        if to_index_optional_prev_value != Some(0) {
            self.go_to_cell(to_index);
            self.clear_current_cell();
        }
        self.move_value_without_overwriting(from_index, to_index, restore_index);
    }

    pub fn copy_value_without_overwriting(
        &mut self,
        from_index: usize,
        to_index: usize,
        restore_index: bool,
    ) {
        let curr_index = self.curr_index;
        if from_index == to_index {
            return;
        }
        let stack = self.generate_stack(1);
        self.go_to_cell(from_index);
        self.code += "[";
        self.go_to_cell(to_index);
        self.code += "+";
        self.jump_to_stack(stack);
        self.code += "+";
        self.go_to_cell(from_index);
        self.code += "-]";
        self.move_value_without_overwriting(stack.get_start_index(), from_index, false);
        self.delete_stack(stack, false, vec![0]);
        if restore_index {
            self.go_to_cell(curr_index);
        }
    }

    pub fn copy_value(
        &mut self,
        from_index: usize,
        to_index: usize,
        from_index_optional_prev_value: impl Into<Option<CellData>>,
        to_index_optional_prev_value: impl Into<Option<CellData>>,
        restore_index: bool,
    ) {
        let from_index_optional_prev_value = from_index_optional_prev_value.into();
        let to_index_optional_prev_value = to_index_optional_prev_value.into();
        if from_index_optional_prev_value == to_index_optional_prev_value
            || from_index_optional_prev_value == Some(0)
        {
            return;
        }
        if to_index_optional_prev_value != Some(0) {
            self.go_to_cell(to_index);
            self.clear_current_cell();
        }
        self.copy_value_without_overwriting(from_index, to_index, restore_index);
    }

    pub fn reverse_current_cell_value(&mut self, base: CellData, restore_index: bool) {
        let curr_index = self.curr_index;
        let stack = self.generate_stack(2);
        self.code += "[";
        self.divide_current_cell_by(base, None, stack.get_start_index(), 0, false);
        self.go_to_cell(stack.get_start_index() + 1);
        self.multiply_current_cell_by(base, None, false);
        self.jump_to_stack(stack);
        self.move_value_without_overwriting(stack.get_start_index(), stack.get_start_index() + 1, false);
        self.go_to_cell(curr_index);
        self.code += "]";
        self.move_value_without_overwriting(stack.get_start_index() + 1, curr_index, false);
        self.delete_stack(stack, false, vec![0, 0]);
        if restore_index {
            self.go_to_cell(curr_index);
        }
    }

    pub fn add_to_current_cell_from_another_cell(
        &mut self,
        another_cell_index: usize,
        restore_cell: bool,
        restore_index: bool,
    ) {
        if restore_cell {
            self.copy_value_without_overwriting(another_cell_index, self.curr_index, restore_index);
        } else {
            self.move_value_without_overwriting(another_cell_index, self.curr_index, restore_index);
        }
    }

    pub fn add_to_current_cell(&mut self, value: CellData, restore_index: bool) {
        if value <= self.value_changer_threshold {
            for _ in 0..value {
                self.code += "+";
            }
            return;
        }
        let curr_index = self.curr_index;
        let stack = self.generate_stack(1);
        self.jump_to_stack(stack);
        self.set_current_cell_value(value, 0, false);
        self.go_to_cell(curr_index);
        self.add_to_current_cell_from_another_cell(stack.get_start_index(), false, restore_index);
        self.delete_stack(stack, false, vec![0]);
        if restore_index {
            self.go_to_cell(curr_index);
        }
    }

    pub fn subtract_another_cell_from_current_cell(
        &mut self,
        another_cell_index: usize,
        restore_cell: bool,
        restore_index: bool,
    ) {
        let curr_index = self.curr_index;
        let optional_stack = if restore_cell {
            Some(self.generate_stack(1))
        } else {
            None
        };
        if restore_cell {
            self.copy_value_without_overwriting(
                another_cell_index,
                optional_stack.unwrap().get_start_index(),
                false,
            );
        }
        self.go_to_cell(another_cell_index);
        self.code += "[";
        self.go_to_cell(curr_index);
        self.code += "-";
        self.go_to_cell(another_cell_index);
        self.code += "-]";
        if restore_cell {
            self.move_value_without_overwriting(
                optional_stack.unwrap().get_start_index(),
                another_cell_index,
                false,
            );
            self.delete_stack(optional_stack.unwrap(), false, vec![0]);
        }
        if restore_index {
            self.go_to_cell(curr_index);
        }
    }

    pub fn subtract_from_current_cell(
        &mut self,
        value: CellData,
        optional_prev_value: impl Into<Option<CellData>>,
        restore_index: bool,
    ) {
        let optional_prev_value = optional_prev_value.into();
        if value == 0 {
            return;
        }
        if let Some(prev_value) = optional_prev_value {
            if prev_value == value {
                self.clear_current_cell();
                return;
            }
        }
        if value <= self.value_changer_threshold {
            for _ in 0..value {
                self.code += "-";
            }
            return;
        }
        let curr_index = self.curr_index;
        let stack = self.generate_stack(1);
        self.jump_to_stack(stack);
        self.set_current_cell_value(value, 0, false);
        self.go_to_cell(curr_index);
        self.subtract_another_cell_from_current_cell(stack.get_start_index(), false, false);
        self.delete_stack(stack, false, None);
        if restore_index {
            self.go_to_cell(curr_index);
        }
    }

    pub fn checked_subtract_another_cell_from_current_cell(
        &mut self,
        another_cell_index: usize,
        restore_cell: bool,
        restore_index: bool,
    ) {
        let curr_index = self.curr_index;
        let optional_stack = if restore_cell {
            Some(self.generate_stack(1))
        } else {
            None
        };
        if restore_cell {
            self.copy_value_without_overwriting(
                another_cell_index,
                optional_stack.unwrap().get_start_index(),
                false,
            );
        }
        let decrement_value = |brainfuck: &mut BrainFuck| brainfuck.code += "-";
        self.go_to_cell(another_cell_index);
        self.code += "[";
        self.go_to_cell(curr_index);
        self.if_current_cell_is_not_zero(decrement_value, false, true);
        self.go_to_cell(another_cell_index);
        self.if_current_cell_is_not_zero(decrement_value, true, true);
        self.code += "]";
        if restore_cell {
            self.move_value_without_overwriting(
                optional_stack.unwrap().get_start_index(),
                another_cell_index,
                false,
            );
            self.delete_stack(optional_stack.unwrap(), false, vec![0]);
        }
        if restore_index {
            self.go_to_cell(curr_index);
        }
    }

    pub fn checked_subtract_from_current_cell(
        &mut self,
        value: CellData,
        optional_prev_value: impl Into<Option<CellData>>,
        restore_index: bool,
    ) {
        let optional_prev_value = optional_prev_value.into();
        if value == 0 {
            return;
        }
        if let Some(prev_value) = optional_prev_value {
            if prev_value <= value {
                self.clear_current_cell();
                return;
            }
            if value <= self.value_changer_threshold && prev_value > value {
                for _ in 0..value {
                    self.code += "-";
                }
                return;
            }
        }
        let curr_index = self.curr_index;
        let stack = self.generate_stack(1);
        self.jump_to_stack(stack);
        self.set_current_cell_value(value, 0, false);
        self.go_to_cell(curr_index);
        self.checked_subtract_another_cell_from_current_cell(stack.get_start_index(), false, false);
        self.delete_stack(stack, false, None);
        if restore_index {
            self.go_to_cell(curr_index);
        }
    }

    // pub fn multiply_current_cell_by_another_cell(&mut self, multiplier_index: usize, restore_index: bool) {
    //     let curr_index = self.curr_index;
    //     let stack = self.generate_stack(1);
    //     self.go_to_cell(multiplier_index);
    //     self.code += "[";
    //     self.go_to_cell(curr_index);
    //     self.code += "+";
    //     self.jump_to_stack(stack);
    //     self.code += "+";
    //     self.go_to_cell(multiplier_index);
    //     self.code += "-]";
    //     self.move_value_without_overwriting(stack.get_start_index(), multiplier_index, false);
    //     self.delete_stack(stack, false, vec![0]);
    //     if restore_index {
    //         self.go_to_cell(curr_index);
    //     }
    // }

    fn sub_multiply(&mut self, multiplier: CellData, curr_index: usize) {
        // TODO: Optimize
        self.go_to_cell(curr_index);
        self.code += "[";
        let stack = self.generate_stack(1);
        self.jump_to_stack(stack);
        for _ in 0..multiplier {
            self.code += "+";
        }
        self.go_to_cell(curr_index);
        self.code += "-]";
        self.move_value_without_overwriting(stack.get_start_index(), curr_index, false);
        self.delete_stack(stack, false, vec![0]);
    }

    pub fn multiply_current_cell_by(
        &mut self,
        mut multiplier: CellData,
        optional_prev_value: impl Into<Option<CellData>>,
        restore_index: bool,
    ) {
        let optional_prev_value = optional_prev_value.into();
        if multiplier == 0 {
            self.clear_current_cell();
            return;
        }
        if multiplier == 1 || optional_prev_value == Some(0) {
            return;
        }
        let curr_index = self.curr_index;
        let mut prev_value = optional_prev_value.unwrap_or(0);
        while multiplier != 1 {
            let factor = if multiplier > self.value_changer_threshold {
                get_smallest_prime_factor(multiplier)
            } else {
                multiplier
            };
            if factor > self.value_changer_threshold.max(2) {
                let stack = self.generate_stack(1);
                self.copy_value_without_overwriting(curr_index, stack.get_start_index(), false);
                self.go_to_cell(curr_index);
                self.multiply_current_cell_by(
                    factor - 1,
                    if prev_value == 0 {
                        None
                    } else {
                        Some(prev_value)
                    },
                    false,
                );
                self.move_value_without_overwriting(stack.get_start_index(), curr_index, false);
                self.delete_stack(stack, false, vec![0]);
            } else {
                self.sub_multiply(factor, curr_index);
            }
            prev_value = prev_value.wrapping_mul(factor);
            multiplier /= factor;
        }
        if restore_index {
            self.go_to_cell(curr_index);
        }
    }

    fn sub_divide(
        &mut self,
        divisor: CellData,
        curr_index: usize,
        optional_move_remainder_to: impl Into<Option<usize>>,
    ) {
        // https://stackoverflow.com/questions/27905818/divmod-algorithm-in-brainfuck
        // [->+>-[>+>>]>[+[-<+>]>+>>]<<<<<<]
        let stack = self.generate_stack(6);
        self.jump_to_stack(stack);
        self.shift_right(1);
        self.set_current_cell_value(divisor, 0, false);
        self.go_to_cell(curr_index);
        self.code += "[-";
        for _ in curr_index..stack.get_start_index() {
            self.code += ">";
        }
        self.code += "+>-[>+>>]>[+[-<+>]>+>>]";
        for _ in curr_index..stack.get_end_index() - 1 {
            self.code += "<";
        }
        self.code += "]";
        self.move_value_without_overwriting(stack.get_start_index() + 3, curr_index, false);
        let mut expected_stack = vec![1; 3];
        if let Some(move_remainder_to) = optional_move_remainder_to.into() {
            self.move_value_without_overwriting(
                stack.get_start_index() + 2,
                move_remainder_to,
                false,
            );
            expected_stack[2] = 0;
        }
        expected_stack.append(&mut vec![0, 0, 0]);
        self.delete_stack(stack, false, expected_stack);
    }

    pub fn divide_current_cell_by(
        &mut self,
        divisor: CellData,
        optional_prev_value: impl Into<Option<CellData>>,
        optional_move_remainder_to: impl Into<Option<usize>>,
        optional_move_remainder_to_prev_value: impl Into<Option<CellData>>,
        restore_index: bool,
    ) {
        let optional_prev_value = optional_prev_value.into();
        let optional_move_remainder_to = optional_move_remainder_to.into();
        let optional_move_remainder_to_prev_value = optional_move_remainder_to_prev_value.into();
        if divisor == 0 {
            panic!("Divide by zero");
        }
        let curr_index = self.curr_index;
        if divisor == 1 {
            if let Some(move_remainder_to) = optional_move_remainder_to {
                self.go_to_cell(move_remainder_to);
                self.clear_current_cell();
                if restore_index {
                    self.go_to_cell(curr_index);
                }
            }
            return;
        }
        if let Some(prev_value) = optional_prev_value {
            let (q, r) = (prev_value / divisor, prev_value % divisor);
            self.set_current_cell_value(q, prev_value, true);
            if let Some(move_remainder_to) = optional_move_remainder_to {
                self.go_to_cell(move_remainder_to);
                self.set_current_cell_value(r, optional_move_remainder_to_prev_value, false);
            }
            if restore_index {
                self.go_to_cell(curr_index);
            }
            return;
        }
        self.sub_divide(divisor, curr_index, optional_move_remainder_to);
        if restore_index {
            self.go_to_cell(curr_index);
        }
    }

    fn set_curr_cell_val(
        &mut self,
        value: CellData,
        optional_prev_value: impl Into<Option<CellData>>,
        restore_index: bool,
    ) {
        let mut optional_prev_value = optional_prev_value.into();
        if let Some(prev_value) = optional_prev_value {
            if value == prev_value {
                return;
            }
            if value == 0 {
                self.clear_current_cell();
                return;
            }
            let difference = value.abs_diff(prev_value);
            if difference <= self.value_changer_threshold {
                if value > prev_value {
                    self.add_to_current_cell(difference, restore_index);
                } else {
                    self.subtract_from_current_cell(difference, prev_value, restore_index);
                }
                return;
            }
        } else {
            if value <= self.value_changer_threshold {
                self.clear_current_cell();
                self.add_to_current_cell(value, restore_index);
                return;
            }
        }
        if ![Some(0), Some(1)].contains(&optional_prev_value) {
            self.clear_current_cell();
            optional_prev_value = Some(0);
        }
        if optional_prev_value != Some(1) {
            self.add_to_current_cell(1, true);
            optional_prev_value = Some(1);
        }
        self.multiply_current_cell_by(value, optional_prev_value, restore_index);
    }

    pub fn set_current_cell_value(
        &mut self,
        value: CellData,
        optional_prev_value: impl Into<Option<CellData>>,
        restore_index: bool,
    ) {
        let optional_prev_value = optional_prev_value.into();
        if is_prime(value) && value > self.value_changer_threshold {
            self.set_current_cell_value(value - 1, optional_prev_value, true);
            self.add_to_current_cell(1, restore_index);
            return;
        }
        if let Some(prev_value) = optional_prev_value {
            if value == prev_value {
                return;
            }
            if value == 0 {
                self.clear_current_cell();
                return;
            }
            if prev_value == 0 {
                self.set_curr_cell_val(value, 0, restore_index);
                return;
            }
            let difference = value.abs_diff(prev_value);
            let multiplier = value / prev_value;
            if multiplier > 1 && difference > self.value_changer_threshold {
                self.multiply_current_cell_by(multiplier, prev_value, true);
                self.set_current_cell_value(value, prev_value * multiplier, restore_index);
                return;
            }
            if value > prev_value {
                self.add_to_current_cell(difference, restore_index);
            } else {
                self.subtract_from_current_cell(difference, prev_value, restore_index);
            }
        } else {
            self.set_curr_cell_val(value, None, restore_index);
        }
    }

    // pub fn if_current_cell_is_zero_else<F1, F2>(
    //     &mut self,
    //     f1: F1,
    //     f2: F2,
    //     restore_index: bool,
    //     restore_index_before_calling: bool,
    // ) where
    //     F1: FnOnce(&mut Self),
    //     F2: FnOnce(&mut Self),
    // {
    //     let curr_index = self.curr_index;
    //     let stack_distance = self.get_last_empty_cell().abs_diff(curr_index);
    //     let stack = self.generate_stack(stack_distance + 1);
    //     self.jump_to_stack(stack);
    //     self.code += "+";
    //     self.go_to_cell(curr_index);
    //     self.code += "[";
    //     f2(self);
    //     self.jump_to_stack(stack);
    //     self.code += "-]";
    //     self.curr_index = curr_index;
    //     self.jump_to_stack(stack);
    //     self.code += "[-";
    //     if restore_index_before_calling {
    //         self.go_to_cell(curr_index);
    //     }
    //     f1(self);
    //     self.jump_to_stack(stack);
    //     self.shift_right(stack_distance);
    //     self.code += "]";
    //     self.delete_stack(stack, false, vec![0; stack_distance]);
    //     if restore_index {
    //         self.go_to_cell(curr_index);
    //     }
    // }

    // pub fn if_current_cell_is_not_zero<F>(
    //     &mut self,
    //     f: F,
    //     restore_index: bool,
    //     restore_index_before_calling: bool,
    // ) where
    //     F: FnOnce(&mut Self),
    // {
    //     self.if_current_cell_is_zero_else(|_| {}, f, restore_index, restore_index_before_calling);
    // }

    // pub fn if_current_cell_is_zero<F>(
    //     &mut self,
    //     f: F,
    //     restore_index: bool,
    //     restore_index_before_calling: bool,
    // ) where
    //     F: FnOnce(&mut Self),
    // {
    //     self.if_current_cell_is_zero_else(f, |_| {}, restore_index, restore_index_before_calling);
    // }

    pub fn if_current_cell_is_not_zero<F>(
        &mut self,
        f: F,
        restore_index: bool,
        restore_index_before_calling: bool,
    ) where
        F: FnOnce(&mut Self),
    {
        let curr_index = self.curr_index;
        let stack = self.generate_stack(1);
        self.move_value_without_overwriting(curr_index, stack.get_start_index(), false);
        self.jump_to_stack(stack);
        self.code += "[";
        if restore_index_before_calling {
            self.go_to_cell(curr_index);
        }
        f(self);
        self.jump_to_stack(stack);
        self.code += "[";
        self.go_to_cell(curr_index);
        self.code += "+";
        self.jump_to_stack(stack);
        self.code += "-]]";
        self.delete_stack(stack, false, vec![0]);
        if restore_index {
            self.go_to_cell(curr_index);
        }
    }

    pub fn if_current_cell_is_zero<F>(
        &mut self,
        f: F,
        restore_index: bool,
        restore_index_before_calling: bool,
    ) where
        F: FnOnce(&mut Self),
    {
        let curr_index = self.curr_index;
        let stack = self.generate_stack(1);
        self.jump_to_stack(stack);
        self.code += "+";
        self.go_to_cell(curr_index);
        let func = |brainfuck: &mut Self| {
            brainfuck.jump_to_stack(stack);
            brainfuck.code += "-";
        };
        self.if_current_cell_is_not_zero(func, false, false);
        self.jump_to_stack(stack);
        self.code += "[";
        if restore_index_before_calling {
            self.go_to_cell(curr_index);
        }
        f(self);
        self.jump_to_stack(stack);
        self.code += "-]";
        self.delete_stack(stack, false, vec![0]);
        if restore_index {
            self.go_to_cell(curr_index);
        }
    }

    pub fn if_current_cell_is_zero_else<F1, F2>(
        &mut self,
        f1: F1,
        f2: F2,
        restore_index: bool,
        restore_index_before_calling: bool,
    ) where
        F1: FnOnce(&mut Self),
        F2: FnOnce(&mut Self),
    {
        let curr_index = self.curr_index;
        let stack = self.generate_stack(1);
        let func1 = |brainfuck: &mut Self| {
            if restore_index_before_calling {
                brainfuck.go_to_cell(curr_index);
            }
            f1(brainfuck);
            brainfuck.jump_to_stack(stack);
            brainfuck.code += "+";
        };
        self.go_to_cell(curr_index);
        self.if_current_cell_is_zero(func1, false, false);
        let func2 = |brainfuck: &mut Self| {
            if restore_index_before_calling {
                brainfuck.go_to_cell(curr_index);
            }
            f2(brainfuck)
        };
        self.jump_to_stack(stack);
        self.if_current_cell_is_zero(func2, false, false);
        self.delete_stack(stack, false, None);
        if restore_index {
            self.go_to_cell(curr_index);
        }
    }

    pub fn check_current_cell_equals(
        &mut self,
        value: CellData,
        result_index: usize,
        result_index_optional_prev_val: impl Into<Option<CellData>>,
        restore_cell: bool,
        restore_index: bool,
    ) {
        let curr_index = self.curr_index;
        self.subtract_from_current_cell(value, None, true);
        let f = |brainfuck: &mut BrainFuck| {
            brainfuck.go_to_cell(result_index);
            if result_index_optional_prev_val.into() != Some(0) {
                brainfuck.clear_current_cell();
            }
            brainfuck.code += "+";
        };
        self.if_current_cell_is_zero(f, false, false);
        if restore_cell {
            self.go_to_cell(curr_index);
            self.add_to_current_cell(value, false);
        }
        if restore_index {
            self.go_to_cell(curr_index);
        }
    }

    pub fn if_current_cell_equals_value_else<F1, F2>(
        &mut self,
        f1: F1,
        f2: F2,
        value: CellData,
        restore_index: bool,
    ) where
        F1: FnOnce(&mut Self),
        F2: FnOnce(&mut Self),
    {
        let curr_index = self.curr_index;
        let stack = self.generate_stack(1);
        self.check_current_cell_equals(value, stack.get_start_index(), 0, true, false);
        let func1 = |brainfuck: &mut Self| {
            brainfuck.go_to_cell(curr_index);
            f1(brainfuck);
        };
        let func2 = |brainfuck: &mut Self| {
            brainfuck.go_to_cell(curr_index);
            f2(brainfuck);
        };
        self.jump_to_stack(stack);
        self.if_current_cell_is_zero_else(func2, func1, false, false);
        self.delete_stack(stack, false, None);
        if restore_index {
            self.go_to_cell(curr_index);
        }
    }

    pub fn if_elif_else(
        &mut self,
        conditions: Vec<(CellData, fn(&mut Self))>,
        default_function: fn(&mut Self),
        restore_index: bool,
        restore_index_before_calling: bool,
    ) {
        if conditions.len() == 0 {
            let curr_index = self.curr_index;
            default_function(self);
            if restore_index {
                self.go_to_cell(curr_index);
            }
            return;
        }
        let (value, f1) = conditions[0];
        let f2 = |brainfuck: &mut Self| {
            brainfuck.if_elif_else(
                conditions[1..].to_vec(),
                default_function,
                false,
                restore_index_before_calling,
            );
        };
        self.if_current_cell_equals_value_else(f1, f2, value, restore_index);
    }

    pub fn print_string(&mut self, string: &str) {
        let stack = self.generate_stack(1);
        let curr_index = self.curr_index;
        self.jump_to_stack(stack);
        let mut prev_value = 0;
        for ch in string.chars() {
            self.set_current_cell_value(ch as CellData, prev_value, true);
            prev_value = ch as CellData;
            self.print_current_cell();
        }
        self.delete_stack(stack, false, vec![prev_value]);
        self.go_to_cell(curr_index);
        self.optimise_code()
    }

    fn contains_bad_code(&self) -> bool {
        for bad_pattern in BAD_PATTERNS {
            if self.code.contains(&bad_pattern) {
                return true;
            }
        }
        false
    }

    pub fn optimise_code(&mut self) {
        while self.contains_bad_code() {
            for bad_pattern in BAD_PATTERNS {
                self.code = self.code.replace(bad_pattern, "");
            }
        }
    }

    pub fn get_optimised_code(&mut self) -> String {
        if self.stacks.len() != 1 {
            panic!("Stacks not deleted properly!");
        }
        self.optimise_code();
        let optimised_code = self.code.clone();
        optimised_code
            .chars()
            .enumerate()
            .map(|(i, ch)| {
                if (i + 1) % WORDWRAP_THRESHOLD == 0 {
                    ch.to_string() + "\n"
                } else {
                    ch.to_string()
                }
            })
            .collect()
    }

    pub fn print_interpreter(&self) {
        println!("{}", self.interpreter);
    }

    pub fn run_code(&mut self) {
        let optimised_code = self.get_optimised_code();
        self.interpreter.reset();
        self.interpreter.interpret(&optimised_code, false);
    }

    pub fn run_code_raw(&mut self) {
        let optimised_code = self.get_optimised_code();
        self.interpreter.reset();
        self.interpreter.interpret(&optimised_code, true);
    }

    pub fn clear_code(&mut self) {
        self.code.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZERO_TEXT: &str = "The value fo the cell is zero! This text should be the output of the code. This text is intensionally made huge so that any mistakes in the code can be spotted more accurately";
    const NOT_ZERO_TEXT: &str = "The value fo the cell is not zero! This text should be the output of the code. This text is intensionally made huge so that any mistakes in the code can be spotted more accurately";

    #[test]
    fn test_add() {
        let mut brainfuck = BrainFuck::new(1);
        let mut val = 0;
        for (threshold, value) in [(5, 10), (1, 20), (50, 30), (CellData::MAX, 50)] {
            brainfuck.set_value_changer_threshold(threshold);
            brainfuck.add_to_current_cell(value, true);
            val += value;
            brainfuck.run_code();
            assert_eq!(brainfuck.interpreter.get_current_cell_value(), val);
        }
    }

    #[test]
    fn test_subtract() {
        let mut brainfuck = BrainFuck::new(1);
        let mut val = 250;
        for _ in 0..val {
            brainfuck.code += "+";
        }
        for (threshold, value) in [(5, 10), (1, 20), (50, 37), (CellData::MAX, 50)] {
            brainfuck.set_value_changer_threshold(threshold);
            brainfuck.subtract_from_current_cell(value, None, true);
            val -= value;
            brainfuck.run_code();
            assert_eq!(brainfuck.interpreter.get_current_cell_value(), val);
            brainfuck.subtract_from_current_cell(value, val, true);
            val -= value;
            brainfuck.run_code();
            assert_eq!(brainfuck.interpreter.get_current_cell_value(), val);
        }
    }

    #[test]
    fn test_checked_subtract() {
        let mut brainfuck = BrainFuck::new(1);
        let mut val: CellData = 250;
        for _ in 0..val {
            brainfuck.code += "+";
        }
        for (threshold, value) in [(5, 10), (1, 20), (50, 37), (CellData::MAX, 50)] {
            for _ in 0..2 {
                brainfuck.set_value_changer_threshold(threshold);
                brainfuck.checked_subtract_from_current_cell(value, None, true);
                val = val.checked_sub(value).unwrap_or(0);
                brainfuck.run_code();
                assert_eq!(brainfuck.interpreter.get_current_cell_value(), val);
                brainfuck.checked_subtract_from_current_cell(value, val, true);
                val = val.checked_sub(value).unwrap_or(0);
                brainfuck.run_code();
                assert_eq!(brainfuck.interpreter.get_current_cell_value(), val);
            }
        }
    }

    #[test]
    fn test_multiply() {
        let mut brainfuck = BrainFuck::new(1);
        let mut val;
        for (idx, (threshold, value)) in [
            (5, 7),
            (15, 4),
            (50, 1),
            (CellData::MAX, 50),
            (1, 13),
            (20, 40),
        ]
        .iter()
        .enumerate()
        {
            for _ in 0..idx {
                brainfuck.code += "+";
            }
            val = idx as CellData;
            brainfuck.set_value_changer_threshold(*threshold);
            brainfuck.multiply_current_cell_by(*value, None, true);
            val *= value;
            brainfuck.run_code();
            assert_eq!(brainfuck.interpreter.get_current_cell_value(), val);
            brainfuck.clear_current_cell();
            for _ in 0..idx {
                brainfuck.code += "+";
            }
            val = idx as CellData;
            brainfuck.multiply_current_cell_by(*value, val, true);
            val *= value;
            brainfuck.run_code();
            assert_eq!(brainfuck.interpreter.get_current_cell_value(), val);
            brainfuck.clear_current_cell();
        }
    }

    #[test]
    fn test_set_value() {
        let mut brainfuck = BrainFuck::new(1);
        let indices = [
            (5, 70),
            (15, 29),
            (50, 15),
            (CellData::MAX, 101),
            (2, 219),
            (20, 50),
            (1, 119),
            (100, 30),
            (30, 50),
            (63, 173),
            (10, 0),
            (5, 1),
            (12, 251),
        ];
        for (threshold, value) in indices {
            brainfuck.set_value_changer_threshold(threshold);
            brainfuck.set_current_cell_value(value, None, true);
            brainfuck.run_code();
            assert_eq!(brainfuck.interpreter.get_current_cell_value(), value);
        }
        let mut prev_val = indices[indices.len() - 1].1;
        for (threshold, value) in indices {
            brainfuck.set_value_changer_threshold(threshold);
            brainfuck.set_current_cell_value(value, prev_val, true);
            prev_val = value;
            brainfuck.run_code();
            assert_eq!(brainfuck.interpreter.get_current_cell_value(), value);
        }
    }

    #[test]
    fn test_if_zero_confition() {
        let mut brainfuck = BrainFuck::new(1);
        for threshold in [1, 10, 20, 100, 150, CellData::MAX] {
            brainfuck.set_value_changer_threshold(threshold);
            brainfuck.code.clear();
            let f = |brainfuck: &mut BrainFuck| brainfuck.print_string(ZERO_TEXT);
            brainfuck.if_current_cell_is_zero(f, true, true);
            brainfuck.run_code();
            assert_eq!(brainfuck.interpreter.get_output(), ZERO_TEXT);
            brainfuck.clear_code();
            brainfuck.code += "+";
            brainfuck.if_current_cell_is_zero(f, true, true);
            brainfuck.run_code();
            assert_eq!(brainfuck.interpreter.get_output(), "");
        }
    }

    #[test]
    fn test_if_not_zero_confition() {
        let mut brainfuck = BrainFuck::new(1);
        for threshold in [1, 10, 20, 100, 150, CellData::MAX] {
            brainfuck.set_value_changer_threshold(threshold);
            brainfuck.code.clear();
            let f = |brainfuck: &mut BrainFuck| brainfuck.print_string(NOT_ZERO_TEXT);
            brainfuck.if_current_cell_is_not_zero(f, true, true);
            brainfuck.run_code();
            assert_eq!(brainfuck.interpreter.get_output(), "");
            brainfuck.clear_code();
            brainfuck.code += "+";
            brainfuck.if_current_cell_is_not_zero(f, true, true);
            brainfuck.run_code();
            assert_eq!(brainfuck.interpreter.get_output(), NOT_ZERO_TEXT);
        }
    }

    #[test]
    fn test_if_zero_else_confition() {
        macro_rules! generate_conditions {
            ($value: literal) => {
                ($value, |brainfuck: &mut BrainFuck| {
                    brainfuck.print_string(&format!("The value is {}!", $value));
                    brainfuck.go_to_cell(0);
                })
            };
        }
        let mut brainfuck = BrainFuck::new(1);
        for threshold in [1, 10, 20, 100, 150, CellData::MAX] {
            brainfuck.set_value_changer_threshold(threshold);
            for value in 0..11 {
                brainfuck.code.clear();
                for _ in 0..value {
                    brainfuck.code += "+"
                }
                brainfuck.if_elif_else(
                    vec![
                        generate_conditions!(0),
                        generate_conditions!(1),
                        generate_conditions!(2),
                        generate_conditions!(3),
                        generate_conditions!(4),
                        generate_conditions!(5),
                        generate_conditions!(6),
                        generate_conditions!(7),
                        generate_conditions!(8),
                        generate_conditions!(9),
                    ],
                    |brainfuck: &mut BrainFuck| {
                        brainfuck.print_string("The value is not between 0 to 9! Test failed!")
                    },
                    true,
                    true,
                );
                brainfuck.run_code();
                let expected_output = if value == 10 {
                    "The value is not between 0 to 9! Test failed!".to_string()
                } else {
                    format!("The value is {}!", value)
                };
                assert_eq!(
                    (brainfuck.interpreter.get_output(), threshold),
                    (expected_output, threshold),
                )
            }
        }
    }
}
