use super::*;

#[derive(Clone)]
pub struct BrainFuckInterpreter {
    memory: Vec<CellData>,
    pointer: usize,
    output: String,
    num_steps: u64,
}

impl BrainFuckInterpreter {
    pub fn new() -> Self {
        Self {
            memory: vec![0; 1],
            pointer: 0,
            output: String::new(),
            num_steps: 0,
        }
    }

    pub fn get_pointer(&self) -> usize {
        self.pointer
    }

    pub fn get_current_cell_value(&self) -> CellData {
        self.memory[self.pointer]
    }

    pub fn get_output(&self) -> String {
        self.output.clone()
    }

    pub fn get_num_steps(&self) -> u64 {
        self.num_steps
    }

    pub fn reset(&mut self) {
        self.memory = vec![0; 1];
        self.pointer = 0;
        self.output.clear();
        self.num_steps = 0;
    }

    fn optimise_code(code: &str) -> Vec<(char, u64)> {
        let chars = code.chars().collect_vec();
        let mut optimised_code = vec![(chars[0], 1)];
        for &ch in chars[1..].iter() {
            let mut last_elem = optimised_code.last_mut().unwrap();
            if "+-<>.,".contains(ch) && last_elem.0 == ch {
                last_elem.1 += 1;
            } else {
                optimised_code.push((ch, 1));
            }
        }
        optimised_code
    }

    pub fn interpret(&mut self, code: &str, debug: bool) {
        let code_chars = Self::optimise_code(code);
        let mut loop_stack: Vec<usize> = Vec::new();
        let code_len = code_chars.len();
        let mut code_index = 0;

        while code_index < code_len {
            let (ch, num_repetitions) = code_chars[code_index];
            if debug {
                println!("{:?}\n{} * {}", self.memory, ch, num_repetitions);
            }

            match ch {
                '>' => {
                    self.pointer += num_repetitions as usize;
                    while self.memory.len() <= self.pointer {
                        self.memory.push(0);
                    }
                }
                '<' => {
                    self.pointer -= num_repetitions as usize;
                }
                '+' => {
                    self.memory[self.pointer] =
                        self.memory[self.pointer].wrapping_add(num_repetitions as CellData);
                }
                '-' => {
                    self.memory[self.pointer] =
                        self.memory[self.pointer].wrapping_sub(num_repetitions as CellData);
                }
                '.' => {
                    for _ in 0..num_repetitions {
                        let ch = self.memory[self.pointer];
                        if debug {
                            print!("{} ", ch);
                        } else {
                            print!("{}", (ch as u8) as char);
                        }
                        std::io::stdout().flush().unwrap();
                        self.output.push((ch as u8) as char);
                    }
                }
                ',' => {
                    for _ in 0..num_repetitions {
                        let mut buffer = [0; 1];
                        std::io::stdin().read_exact(&mut buffer).unwrap();
                        self.memory[self.pointer] = buffer[0] as CellData;
                    }
                }
                '[' => {
                    if self.memory[self.pointer] == 0 {
                        let mut open_brackets = 1;
                        while open_brackets > 0 {
                            code_index += 1;
                            if code_index >= code_len {
                                break;
                            }
                            if code_chars[code_index].0 == '[' {
                                open_brackets += 1;
                            } else if code_chars[code_index].0 == ']' {
                                open_brackets -= 1;
                            }
                        }
                    } else {
                        loop_stack.push(code_index);
                    }
                }
                ']' => {
                    if self.memory[self.pointer] != 0 {
                        if let Some(&loop_start) = loop_stack.last() {
                            code_index = loop_start;
                        }
                    } else {
                        loop_stack.pop();
                    }
                }
                '#' => {
                    println!("\n{}", self);
                }
                _ => (),
            }

            code_index += 1;
            if ch != '#' {
                self.num_steps += 1;
            }
        }
    }
}

impl Display for BrainFuckInterpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pointer_position = String::from(" ");
        if self.pointer > 0 {
            for i in &self.memory[..self.pointer] {
                for _ in 0..i.to_string().len() + 2 {
                    pointer_position += " ";
                }
            }
        }
        for _ in 0..self.memory[self.pointer].to_string().len() / 2 {
            pointer_position += " ";
        }
        pointer_position += "^";
        write!(f, "{:?}\n{}", self.memory, pointer_position)
    }
}
