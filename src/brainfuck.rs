use super::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct Stack {
    start_index: usize,
    end_index: usize,
}

fn get_smallest_factor(number: u8) -> u8 {
    for prime in PRIMES {
        if number % prime == 0 {
            return prime;
        }
    }
    unreachable!()
}

fn is_prime(number: u8) -> bool {
    PRIMES.contains(&number)
}

impl Stack {
    pub fn new(start_index: usize, length: usize) -> Stack {
        Stack {
            start_index,
            end_index: start_index + length,
        }
    }

    pub fn get_start_index(&self) -> usize {
        self.start_index
    }

    pub fn get_end_index(&self) -> usize {
        self.end_index
    }
}

pub struct BrainFuck {
    curr_index: usize,
    stacks: Vec<Stack>,
    index_backups: Vec<usize>,
    code: String,
}

impl BrainFuck {
    pub fn new(initial_stack_size: usize) -> BrainFuck {
        BrainFuck {
            curr_index: 0,
            stacks: vec![Stack::new(0, initial_stack_size)],
            index_backups: vec![],
            code: String::new(),
        }
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

    pub fn take_input(&mut self) {
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
        self.move_value(stack.get_start_index(), from_index, 0, None, false);
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
        if value <= THRESHOLD {
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
        if value == 0 || optional_prev_value == Some(0) {
            return;
        }
        if let Some(prev_value) = optional_prev_value {
            if prev_value < value {
                return;
            }
            if prev_value == value {
                self.clear_current_cell();
                return;
            }
            let difference = prev_value - value;
            if difference <= THRESHOLD {
                for _ in 0..difference {
                    self.code += "-";
                }
                return;
            }
        }
        if value <= THRESHOLD {
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
        self.move_value_without_overwriting(stack.start_index, curr_index, false);
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
        while multiplier != 1 {
            let factor = get_smallest_factor(multiplier);
            self.sub_multiply(factor, curr_index);
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
        if optional_prev_value == Some(value) {
            return;
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
        if is_prime(value) && value > THRESHOLD {
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
            let multiplier = value / prev_value;
            if multiplier > 1 {
                self.multiply_current_cell_by(multiplier, prev_value, true);
                self.set_current_cell_value(value, prev_value * multiplier, restore_index);
                return;
            }
            let difference = value.abs_diff(prev_value);
            if value > prev_value {
                self.add_to_current_cell(difference, restore_index);
            } else {
                self.subtract_from_current_cell(difference, prev_value, restore_index);
            }
        } else {
            self.set_curr_cell_val(value, None, restore_index);
        }
    }

    pub fn print_string(&mut self, string: &str) {
        let stack = self.generate_stack(1);
        self.jump_to_stack(stack);
        let mut prev_value = 0;
        for c in string.chars() {
            self.set_current_cell_value(c as u8, prev_value, true);
            prev_value = c as u8;
            self.print_current_cell();
        }
        self.delete_stack(stack, false, vec![prev_value]);
        self.optimise_code()
    }

    pub fn println_string(&mut self, string: &str) {
        self.print_string(&(string.to_string() + "\n"));
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
        self.optimise_code();
        let optimised_code = self.code.clone();
        // let last_input_or_output = optimised_code
        //     .rfind(|c| c == '.' || c == ',')
        //     .unwrap_or(0);
        // optimised_code[..last_input_or_output + 1].to_string()
        optimised_code
    }

    // def run_code(self):
    //     print(list(i for i in evaluate(self.code)))
    //     print("".join(chr(i) for i in list(i for i in evaluate(self.code))))
}
