use std::{env, fs, io};
use std::io::Read;

const BUFFER_SIZE: usize = 30000;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Please input a file `./brainfuck script.bf`");
        return ();
    }

    let script = fs::read_to_string(args.get(1).unwrap()).expect("Unable to read file.");
    interpret(&script, "");
}

fn interpret(code: &str, input: &str) -> String {
    let mut memory = [0u8; BUFFER_SIZE];
    let mut current_index: usize = 0;
    let mut loop_stack: Vec<usize> = vec![];
    let mut output: Vec<char> = vec![];
    let mut input: Vec<u8> = input.bytes().rev().collect();
    let mut byte_ptr = 0usize;

    let code_bytes: Vec<char> = code.chars().filter(|c| !c.is_whitespace()).map(|b| b as char).collect();

    loop {
        match code_bytes[byte_ptr] {
            '>' => { if current_index < BUFFER_SIZE { current_index += 1 } else { current_index = 0; } }
            '<' => { if current_index > 0 { current_index -= 1 } else { current_index = BUFFER_SIZE; } }
            '+' => { memory[current_index] += 1; }
            '-' => { memory[current_index] -= 1; }
            '.' => { print!("{}", memory[current_index] as char); output.push(memory[current_index] as char); }
            ',' => {
                if input.len() != 0 { memory[current_index] = input.pop().unwrap() as u8; }
                else { memory[current_index] = get_input(); }
            }
            '[' => {
                if memory[current_index] == 0 {
                    let mut bracket_counter = 0;
                    let mut found = false;
                    for i in (byte_ptr + 1)..code_bytes.len() {
                        byte_ptr += 1;
                        match code_bytes[i] {
                            '[' => { bracket_counter += 1; }
                            ']' => {
                                if bracket_counter > 0 {
                                    bracket_counter -= 1;
                                } else {
                                    found = true;
                                    break;
                                }
                            }
                            _ => {continue;}
                        }
                    }

                    if !found {
                        eprintln!("Opening bracket at position: {}, has no matching closing bracket", byte_ptr);
                        break;
                    }
                } else { loop_stack.push(byte_ptr); }
            }
            ']' => { if loop_stack.len() > 0 { byte_ptr = loop_stack.pop().unwrap() - 1; } }
            _ => {},
        }

        if byte_ptr + 1 >= code_bytes.len() {
            break
        } else {
            byte_ptr += 1;
        }
    }

    output.into_iter().collect::<String>().trim().to_string()
}

fn get_input() -> u8 {
    io::stdin().bytes().next().and_then(|result| result.ok()).map(|byte| byte as u8).unwrap_or(0u8)
}

#[cfg(test)]
mod test {
    use crate::interpret;

    #[test]
    fn loop_test() {
        assert_eq!(interpret("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.", ""), "Hello World!");
        assert_eq!(interpret("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.", ""), "Hello World!");
        assert_eq!(interpret(",[.,]", "Rust\0"), "Rust");
    }
}