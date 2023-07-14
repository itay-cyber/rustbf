use bfops::BFFFile;
use bfops::BFOp;
use bfops::BFOpType;
use std::env;
mod bfops;

const MAX_ARRAY_SIZE: usize = 30000;
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

    for op in &ops {
        match BFOp::run_op(&op, &mut prog, pointer, &ops) {
            Ok(got_pointer) => {
                pointer = got_pointer;
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
