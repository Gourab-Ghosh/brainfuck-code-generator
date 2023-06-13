use std::io::{Read, Write};

pub struct BrainFuckInterpreter {
    memory: Vec<u8>,
    pointer: usize,
}

impl BrainFuckInterpreter {
    pub fn new() -> Self {
        Self {
            memory: vec![0; 30000],
            pointer: 0,
        }
    }

    pub fn interpret(&mut self, code: &str, interpret_raw: bool) {
        let mut loop_stack: Vec<usize> = Vec::new();
        let code_chars: Vec<char> = code.chars().collect();
        let code_len = code_chars.len();
        let mut code_index = 0;

        while code_index < code_len {
            match code_chars[code_index] {
                '>' => self.pointer = (self.pointer + 1) % self.memory.len(),
                '<' => self.pointer = (self.pointer + self.memory.len() - 1) % self.memory.len(),
                '+' => self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1),
                '-' => self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1),
                '.' => {
                    let c = self.memory[self.pointer];
                    if interpret_raw {
                        print!("{} ", c);
                    } else {
                        print!("{}", c as char);
                    }
                    std::io::stdout().flush().unwrap();
                }
                ',' => {
                    let mut buffer = [0; 1];
                    std::io::stdin().read_exact(&mut buffer).unwrap();
                    self.memory[self.pointer] = buffer[0];
                }
                '[' => {
                    if self.memory[self.pointer] == 0 {
                        let mut open_brackets = 1;
                        while open_brackets > 0 {
                            code_index += 1;
                            if code_index >= code_len {
                                break;
                            }
                            if code_chars[code_index] == '[' {
                                open_brackets += 1;
                            } else if code_chars[code_index] == ']' {
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
                            code_index = loop_start - 1;
                        }
                    } else {
                        loop_stack.pop();
                    }
                }
                _ => (),
            }

            code_index += 1;
        }
    }
}
