use crate::MAX_ARRAY_SIZE;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;
#[derive(Debug, PartialEq, Clone)]
pub enum BFOpType {
    OpMoveRight,
    OpMoveLeft,
    OpInc,
    OpDec,
    OpOut,
    OpNumOut,
    OpInp,
    OpLoopStart,
    OpLoopEnd,
    OpNull,
}
#[derive(Debug)]
pub struct BFFFile {
    path: String,
}

#[derive(Debug)]
pub struct BFOp {
    op_type: BFOpType,
    op_char: char,
}

impl BFFFile {
    pub fn new(path: &str) -> Self {
        match path.ends_with(".bf") {
            true => Self {
                path: String::from(path),
            },
            false => panic!("File {} is not a brainfuck file", path),
        }
    }
    pub fn read(&self) -> String {
        let path = Path::new(&self.path);
        let display = path.display();

        let mut file = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => s,
        }
    }
}
impl BFOp {
    pub fn new(op_char: char) -> Self {
        Self {
            op_type: BFOpType::get_op_type(op_char),
            op_char,
        }
    }
    pub fn get_op_type(&self) -> BFOpType {
        self.op_type.clone()
    }
    pub fn get_op_char(&self) -> char {
        self.op_char
    }
    pub fn run_op(
        op: &BFOp,
        arr: &mut [u32; MAX_ARRAY_SIZE],
        pointer: usize,
        op_vec: &Vec<BFOp>,
    ) -> Result<usize, String> {
        let optype = op.get_op_type();
        match optype {
            BFOpType::OpMoveRight => {
                if pointer != MAX_ARRAY_SIZE - 1 {
                    Ok(pointer + 1)
                } else {
                    Err("Index out of range".to_string())
                }
            }
            BFOpType::OpMoveLeft => {
                if pointer != 0 {
                    Ok(pointer - 1)
                } else {
                    Err("Index out of range ".to_string())
                }
            }
            BFOpType::OpInc => {
                if pointer != MAX_ARRAY_SIZE {
                    arr[pointer] += 1;
                    Ok(pointer)
                } else {
                    Err("Index out of range".to_string())
                }
            }
            BFOpType::OpDec => {
                if pointer != MAX_ARRAY_SIZE - 1 {
                    if arr[pointer] != 0 {
                        arr[pointer] -= 1;
                        Ok(pointer)
                    } else {
                        Err(format!("Cannot decrement cell {} as it is 0", pointer))
                    }
                } else {
                    Err("Index out of range".to_string())
                }
            }
            BFOpType::OpOut => {
                if pointer != MAX_ARRAY_SIZE {
                    match char::from_u32(arr[pointer]) {
                        Some(ch) => {
                            print!("{}\n", ch);
                            Ok(pointer)
                        }
                        None => {
                            print!("{}\n", arr[pointer]);
                            Ok(pointer)
                        }
                    }
                } else {
                    Err("Index out of range".to_string())
                }
            }
            BFOpType::OpInp => {
                if pointer != MAX_ARRAY_SIZE {
                    let inp = get_cell_inp();
                    if inp > 0 {
                        arr[pointer] = inp as u32;
                        Ok(pointer)
                    } else {
                        Err(
                            "Error taking input: Cannot assign non-numeric value to cell"
                                .to_string(),
                        )
                    }
                } else {
                    Err("Index out of range".to_string())
                }
            }
            BFOpType::OpLoopStart => {
                if check_for_end_of_loop(&op_vec) {
                    Ok(MAX_ARRAY_SIZE + 1) // return loop start
                } else {
                    Err("Syntax error: end of loop not found.".to_string())
                }
            }
            BFOpType::OpLoopEnd => {
                Ok(MAX_ARRAY_SIZE + 2) // return loop end
            },
            BFOpType::OpNumOut => {
                if pointer != MAX_ARRAY_SIZE {
                    print!("{}\n", arr[pointer]);
                    Ok(pointer)
                } else {
                    Err("Index out of range".to_string())
                }
            }
            BFOpType::OpNull => Err("OpNull".to_string()),
        }
    }
}
impl BFOpType {
    fn get_op_type(op_char: char) -> BFOpType {
        match op_char {
            '>' => BFOpType::OpMoveRight,
            '<' => BFOpType::OpMoveLeft,
            '+' => BFOpType::OpInc,
            '-' => BFOpType::OpDec,
            '.' => BFOpType::OpOut,
            ',' => BFOpType::OpInp,
            '[' => BFOpType::OpLoopStart,
            ']' => BFOpType::OpLoopEnd,
            '=' => BFOpType::OpNumOut,
            _ => BFOpType::OpNull,
        }
    }
}
fn get_cell_inp() -> i32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num: Result<u32, _> = input.trim().parse();

    match num {
        Ok(n) => n as i32,
        Err(_) => -1,
    }
}
fn check_for_end_of_loop(ops: &Vec<BFOp>) -> bool {
    let index_of_start = ops.iter().position(|op| op.get_op_char() == '[').unwrap();
    ops.iter().skip(index_of_start).any(|op| op.get_op_type() == BFOpType::OpLoopEnd)
}
