use bfops::BFFFile;
use bfops::BFOp;
use std::env;
mod bfops;

const MAX_ARRAY_SIZE: usize = 30000;
const LOOP_START: usize = 30001;
const LOOP_END: usize = 30002;
fn process_file(contents: &String) -> String {
    contents.trim().replace(" ", "").to_string()
}
fn get_operations(fcontents: &String) -> Vec<BFOp> {
    let mut ops: Vec<BFOp> = Vec::new();
    for f in fcontents.chars() {
        ops.push(BFOp::new(f))
    }
    ops
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let test_file = BFFFile::new(&args[1]);
    let file_contents = process_file(&test_file.read());
    let ops = get_operations(&file_contents);

    let mut pointer = 0;
    let mut prog: [u32; MAX_ARRAY_SIZE] = [0; MAX_ARRAY_SIZE];
    let mut is_loop: bool = false;
    let mut loop_count_index: usize = 0;
    let mut loop_end_index: usize = 0;
    let mut jump_to_end_of_loop: bool = false;


    'outer: for (index, op) in ops.iter().enumerate() {
        if jump_to_end_of_loop {
            for _ in 1..=loop_end_index - index {
                continue 'outer;
            }
            jump_to_end_of_loop = false;

        }
        if is_loop {
            let mut loop_count = prog[loop_count_index];
            let mut next_op = op;
            let mut next_op_count = 1;
            if loop_count != 0 {
                while loop_count != 0 {
                    match BFOp::run_op(next_op, &mut prog, pointer, &ops) {
                        Ok(got_pointer) => {
                            
                            if got_pointer  == LOOP_END {
                                //jump to start of loop
                                next_op = &&ops[index];
                                loop_end_index = index - 1 + next_op_count; 
                                next_op_count = 1;
                            }
                            else {
                                if got_pointer != LOOP_START {
                                    pointer = got_pointer;
                                }
                                loop_count = prog[loop_count_index];
                                next_op = &&ops[index+next_op_count];
                                next_op_count += 1;

                            }
                        }
                        Err(why) => {
                            if why != "OpNull" {
                                panic!(
                                    "Error running operation {:?} ({:?}). Error: {}",
                                    op.get_op_char(),
                                    op.get_op_type(),
                                    why
                                );
                            }
                        }
                    }
                    
                }
                is_loop = false;
                jump_to_end_of_loop = true;

            } else {
                is_loop = false;
            }
        } else {
            match BFOp::run_op(&op, &mut prog, pointer, &ops) {
                Ok(got_pointer) => {
                    
                    if got_pointer == LOOP_START && is_loop == false {
                        is_loop = true;
                        loop_count_index = pointer;
                    } else if is_loop{
                        continue;
                    }
                    else if got_pointer == LOOP_END {
                        continue;
                    }
                    else {
                        pointer = got_pointer;
                    }
                }
                Err(why) => {
                    if why != "OpNull" {
                        panic!(
                            "Error running operation {:?} ({:?}). Error: {}",
                            op.get_op_char(),
                            op.get_op_type(),
                            why
                        );
                    }
                }
            }
        }
    }
}
