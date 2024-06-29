mod stacker;

use render_staal::run_program;
use stacker::*;

fn main() {
    let mut program = gcd();
    program.insert(0, vec![StackInstr::Write(1223.0), StackInstr::Write(127.0)]);

    let compiled = compile_stacker(program);

    run_program(compiled).unwrap();
}
