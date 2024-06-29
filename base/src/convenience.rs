use crate::papervm::{instructions::*, Instruction, IntoChars};

pub fn call_static<X: IntoChars>(
    program: Vec<Instruction>,
    inputs: Vec<X>,
    return_size: usize,
) -> Vec<Instruction> {
    let mut main = vec![];

    let word_sizes: Vec<_> = inputs
        .iter()
        .scan((0i64, 0i64, 0usize), |word_state, inp| {
            let (x, y, size) = word_state;
            let new_x = *x;
            let l = inp.chars_ref().len();
            *x += l as i64;
            *y = 0;
            *size = l;
            Some((new_x, *y, *size))
        })
        .collect();

    let total_length = word_sizes.last().map(|x| x.0 + x.2 as i64).unwrap_or(0);

    for x in inputs {
        main.push(write(x))
    }

    let word_size_for_instr = word_sizes
        .iter()
        .map(|(x, y, size)| (x - total_length, *y, *size))
        .collect();

    main.push(call(program, word_size_for_instr));
    main.push(circle((-(return_size as i64), 0, return_size)));

    main
}
