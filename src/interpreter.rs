use std::{io::{Read, Write}, fmt::Display};

#[derive(Clone)]
pub struct BrainFuckInterpreter {
    memory: Vec<u8>,
    pointer: usize,
    output: String,
}

impl BrainFuckInterpreter {
    pub fn new() -> Self {
        Self {
            memory: vec![0; 1],
            pointer: 0,
            output: String::new(),
        }
    }

    pub fn get_pointer(&self) -> usize {
        self.pointer
    }

    pub fn get_current_cell_value(&self) -> u8 {
        self.memory[self.pointer]
    }
    
    pub fn get_output(&self) -> String {
        self.output.clone()
    }

    pub fn reset(&mut self) {
        self.memory.clear();
        self.memory.push(0);
        self.pointer = 0;
        self.output.clear();
    }

    pub fn interpret(&mut self, code: &str, interpret_raw: bool) {
        let mut loop_stack: Vec<usize> = Vec::new();
        let code_chars: Vec<char> = code.chars().collect();
        let code_len = code_chars.len();
        let mut code_index = 0;

        while code_index < code_len {
            match code_chars[code_index] {
                '>' => {
                    self.pointer += 1;
                    while self.memory.len() < self.pointer + 1 {
                        self.memory.push(0);
                    }
                }
                '<' => self.pointer -= 1,
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
                    self.output += &(c as char).to_string();
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

impl Display for BrainFuckInterpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pointer_position = String::from(" ");
        if self.pointer > 0 {
            for i in self.memory[..self.pointer].iter() {
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