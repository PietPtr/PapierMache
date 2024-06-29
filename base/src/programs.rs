use std::cmp::Ordering;

use crate::papervm::{instructions::*, Instruction, CHARS_PER_FLOAT};

const CPF: usize = CHARS_PER_FLOAT;
const CPFI: i64 = CHARS_PER_FLOAT as i64;

pub fn gcd_main(a: f64, b: f64) -> Vec<Instruction> {
    vec![
        write(a),
        write(b),
        call(gcd(), vec![(-CPFI * 2, 0, CPF), (-CPFI, 0, CPF)]),
        circle((-CPFI, 0, CPF)),
    ]
}

pub fn gcd() -> Vec<Instruction> {
    vec![
        write("\n         b"),
        write("         a"),
        write("         t\n"),
        copy((0, -2, CPF)),
        copy((0, -2, CPF)),
        // :start
        // t := b
        copy((-2 * CPFI, 0, CPF)),
        write("\n"),
        // b := a % b
        modulo((CPFI, -1, CPF), (0, -1, CPF)),
        jump_rel_if((-CPFI, 0, CPF), Ordering::Equal, 0., 3),
        // a := t
        copy((CPFI, -1, CPF)),
        // jump to start
        jump(-5),
        circle((CPFI, -1, CPF)),
    ]
}

pub fn modulo_prog() -> Vec<Instruction> {
    vec![
        write("\n"),
        copy_trimmed((0, -1, CPF)),
        write(" % "),
        copy_trimmed((CPFI - 3, -1, CPF)),
        write("\n"),
        copy((0, -2, CPF)),
        write(" - "),
        copy((-3, -2, CPF)),
        write(" = "),
        sub((-(CPFI * 2 + 6), 0, CPF), (-(CPFI + 3), 0, CPF)),
        jump_rel_if((-CPFI, 0, CPF), Ordering::Less, 0., 9),
        write("\n"),
        copy((CPFI * 2 + 6, -1, CPF)),
        write(" - "),
        copy((0, -1, CPF)),
        write(" = "),
        sub((-(CPFI * 2 + 6), 0, CPF), (-(CPFI + 3), 0, CPF)),
        jump_rel_if((-CPFI, 0, CPF), Ordering::Greater, 0., -6),
        circle((-CPFI, -1, CPF)),
        write("\n"),
        circle((0, -1, CPF)),
    ]
}

pub fn gcd_with_mod() -> Vec<Instruction> {
    vec![
        write("\n         b"),
        write("         a"),
        write("         t\n"),
        copy((0, -2, CPF)),
        copy((0, -2, CPF)),
        // :start
        // t := b
        copy((-2 * CPFI, 0, CPF)),
        write("\n"),
        // b := a % b
        call(modulo_prog(), vec![(CPFI, -1, CPF), (0, -1, CPF)]),
        jump_rel_if((-CPFI, 0, CPF), Ordering::Equal, 0., 3),
        // a := t
        copy((CPFI, -1, CPF)),
        // jump to start
        jump(-5),
        circle((CPFI, -1, CPF)),
    ]
}
