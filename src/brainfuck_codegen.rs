use super::*;

#[derive(Clone)]
pub struct BrainFuck {
    curr_index: usize,
    stacks: Vec<Stack>,
    index_backups: Vec<usize>,
    code: String,
    value_changer_threshold: u8,
    pub interpreter: BrainFuckInterpreter,
}

impl BrainFuck {
    pub fn new(initial_stack_size: usize) -> BrainFuck {
        BrainFuck {
            curr_index: 0,
            stacks: vec![Stack::new(0, initial_stack_size)],
            index_backups: vec![],
            code: String::new(),
            value_changer_threshold: 15,
            interpreter: BrainFuckInterpreter::new(),
        }
    }

    pub fn get_value_changer_threshold(&self) -> u8 {
        self.value_changer_threshold
    }

    pub fn set_value_changer_threshold(&mut self, threshold: u8) {
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
        if index == self.curr_index {
            return;
        }
        let direction = if index > self.curr_index { ">" } else { "<" };
        for _ in 0..self.curr_index.abs_diff(index) {
            self.code += direction
        }
        self.curr_index = index;
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

    pub fn backup_index(&mut self) {
        self.index_backups.push(self.curr_index);
    }

    pub fn restore_index(&mut self) {
        let index = self.index_backups.pop().unwrap();
        self.go_to_cell(index);
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
        optional_stack_vals: impl Into<Option<Vec<u8>>>,
    ) {
        let stack_vals = optional_stack_vals.into().unwrap_or(vec![]);
        if restore_index {
            self.backup_index()
        }
        for index in stack.get_start_index()..stack.get_end_index() {
            if stack_vals.get(stack.get_start_index() - index) != Some(&0) {
                self.go_to_cell(index);
                self.clear_current_cell();
            }
        }
        if restore_index {
            self.restore_index()
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
        if restore_index {
            self.backup_index()
        }
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
            self.restore_index()
        }
    }

    pub fn move_value(
        &mut self,
        from_index: usize,
        to_index: usize,
        from_index_optional_prev_value: impl Into<Option<u8>>,
        to_index_optional_prev_value: impl Into<Option<u8>>,
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
        if restore_index {
            self.backup_index()
        }
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
            self.restore_index()
        }
    }

    pub fn copy_value(
        &mut self,
        from_index: usize,
        to_index: usize,
        from_index_optional_prev_value: impl Into<Option<u8>>,
        to_index_optional_prev_value: impl Into<Option<u8>>,
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

    pub fn add_to_current_cell(&mut self, value: u8, restore_index: bool) {
        if value <= self.value_changer_threshold {
            for _ in 0..value {
                self.code += "+";
            }
            return;
        }
        if restore_index {
            self.backup_index()
        }
        let curr_index = self.curr_index;
        let stack = self.generate_stack(1);
        self.jump_to_stack(stack);
        self.set_current_cell_value(value, 0, false);
        self.move_value_without_overwriting(stack.get_start_index(), curr_index, false);
        self.delete_stack(stack, false, vec![0]);
        if restore_index {
            self.restore_index()
        }
    }

    pub fn subtract_from_current_cell(
        &mut self,
        value: u8,
        optional_prev_value: impl Into<Option<u8>>,
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
        if restore_index {
            self.backup_index()
        }
        let curr_index = self.curr_index;
        let stack = self.generate_stack(1);
        self.jump_to_stack(stack);
        self.set_current_cell_value(value, 0, true);
        self.code += "[";
        self.go_to_cell(curr_index);
        self.code += "-";
        self.jump_to_stack(stack);
        self.code += "-]";
        self.delete_stack(stack, false, vec![0]);
        if restore_index {
            self.restore_index()
        }
    }

    fn sub_multiply(&mut self, multiplier: u8, curr_index: usize) {
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
        mut multiplier: u8,
        optional_prev_value: impl Into<Option<u8>>,
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
        if restore_index {
            self.backup_index()
        }
        let curr_index = self.curr_index;
        let mut prev_value = optional_prev_value.unwrap_or(0);
        while multiplier != 1 {
            let factor = get_smallest_prime_factor(multiplier);
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
            self.restore_index()
        }
    }

    fn set_curr_cell_val(
        &mut self,
        value: u8,
        optional_prev_value: impl Into<Option<u8>>,
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
        value: u8,
        optional_prev_value: impl Into<Option<u8>>,
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

    pub fn if_current_cell_equals_value_else<F1, F2>(
        &mut self,
        f1: F1,
        f2: F2,
        value: u8,
        restore_index: bool,
    ) where
        F1: FnOnce(&mut Self),
        F2: FnOnce(&mut Self),
    {
        let curr_index = self.curr_index;
        let stack = self.generate_stack(1);
        self.copy_value_without_overwriting(curr_index, stack.get_start_index(), false);
        self.jump_to_stack(stack);
        self.subtract_from_current_cell(value, None, true);
        let func1 = |brainfuck: &mut Self| {
            brainfuck.go_to_cell(curr_index);
            f1(brainfuck);
        };
        let func2 = |brainfuck: &mut Self| {
            brainfuck.go_to_cell(curr_index);
            f2(brainfuck);
        };
        self.if_current_cell_is_zero_else(func1, func2, false, false);
        self.delete_stack(stack, false, None);
        if restore_index {
            self.go_to_cell(curr_index);
        }
    }

    pub fn if_elif_else(
        &mut self,
        conditions: Vec<(u8, fn(&mut Self))>,
        default_function: fn(&mut Self),
        restore_index: bool,
        restore_index_before_calling: bool,
    ) {
        if conditions.len() == 0 {
            if restore_index {
                self.backup_index();
            }
            default_function(self);
            if restore_index {
                self.restore_index();
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
        self.if_current_cell_equals_value_else(
            f1,
            f2,
            value,
            restore_index,
        );
    }

    pub fn print_string(&mut self, string: &str) {
        let stack = self.generate_stack(1);
        let curr_index = self.curr_index;
        self.jump_to_stack(stack);
        let mut prev_value = 0;
        for ch in string.chars() {
            self.set_current_cell_value(ch as u8, prev_value, true);
            prev_value = ch as u8;
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
        if self.index_backups.len() != 0 {
            panic!("Indices not restored properly!");
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
        for (threshold, value) in [(5, 10), (1, 20), (50, 30), (u8::MAX, 50)] {
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
        for (threshold, value) in [(5, 10), (1, 20), (50, 37), (u8::MAX, 50)] {
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
    fn test_multiply() {
        let mut brainfuck = BrainFuck::new(1);
        let mut val;
        for (idx, (threshold, value)) in
            [(5, 7), (15, 4), (50, 1), (u8::MAX, 50), (1, 13), (20, 40)]
                .iter()
                .enumerate()
        {
            for _ in 0..idx {
                brainfuck.code += "+";
            }
            val = idx as u8;
            brainfuck.set_value_changer_threshold(*threshold);
            brainfuck.multiply_current_cell_by(*value, None, true);
            val *= value;
            brainfuck.run_code();
            assert_eq!(brainfuck.interpreter.get_current_cell_value(), val);
            brainfuck.clear_current_cell();
            for _ in 0..idx {
                brainfuck.code += "+";
            }
            val = idx as u8;
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
            (u8::MAX, 101),
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
        for threshold in [1, 10, 20, 100, 150, u8::MAX] {
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
        for threshold in [1, 10, 20, 100, 150, u8::MAX] {
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
                    brainfuck.print_string(&format!("You entered {}!", $value))
                })
            };
        }
        let mut brainfuck = BrainFuck::new(1);
        for threshold in [1, 10, 20, 100, 150, u8::MAX] {
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
                        brainfuck.print_string(
                            "The value is not between 0 to 9! Try entering a digit in the input",
                        )
                    },
                    true,
                    true,
                );
                brainfuck.run_code();
                let expected_output = if value == 10 {
                    "The value is not between 0 to 9! Try entering a digit in the input".to_string()
                } else {
                    format!("You entered {}!", value)
                };
                assert_eq!(
                    (brainfuck.interpreter.get_output(), threshold),
                    (expected_output, threshold),
                )
            }
        }
    }
}
