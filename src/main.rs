use papier::{
    papervm::{CharCell, Instruction, PaperVM, Pos, Word, CHARS_PER_FLOAT},
    papier::{Numbers, Papier},
};

fn main() {
    let a = 127.;
    let b = 1322.;
    let program = vec![
        Instruction::Write(Box::new("b,")),
        Instruction::Write(Box::new("a,")),
        Instruction::Write(Box::new("t,\n")),
        Instruction::Write(Box::new(b)),
        Instruction::Write(Box::new(a)),
        // :start
        // t := b
        Instruction::Copy(Word(Pos(-2 * (CHARS_PER_FLOAT as i64), 0), CHARS_PER_FLOAT)),
        Instruction::Write(Box::new("\n")),
        // b := a % b
        Instruction::Mod(
            Word(Pos(CHARS_PER_FLOAT as i64, -1), CHARS_PER_FLOAT),
            Word(Pos(0, -1), CHARS_PER_FLOAT),
        ),
        Instruction::JumpRelIf(
            Word(Pos(-(CHARS_PER_FLOAT as i64), 0), CHARS_PER_FLOAT),
            0.,
            3,
        ),
        // a := t
        Instruction::Copy(Word(Pos(CHARS_PER_FLOAT as i64, -1), CHARS_PER_FLOAT)),
        // jump to start
        Instruction::Jump(-5),
        Instruction::Circle(Word(Pos(CHARS_PER_FLOAT as i64, -1), CHARS_PER_FLOAT)),
    ];

    let mut vm = PaperVM::<CharCell>::new(&program);
    vm.run();

    let result: f64 = vm.result().unwrap();
    println!("GCD: {}", result);
}

fn gcd(papier: &mut Papier, a: u64, b: u64) {
    papier.writes("   b ");
    papier.writes("   a ");
    papier.writes("   t \r\n");
    Numbers::write(papier, a as i64, 4);
    Numbers::write(papier, b as i64, 4);

    loop {
        // t := b
        let b = Numbers::read::<i64>(papier, -5 * 2, 0, 4).unwrap();
        Numbers::write(papier, b, 4);
        papier.writes("\r\n");

        if b == 0 {
            break;
        }

        // b := a % b
        let a: i64 = Numbers::read(papier, 5, -1, 4).unwrap();
        let b: i64 = Numbers::read(papier, 0, -1, 4).unwrap();
        Numbers::write(papier, a % b, 4);

        // a := t
        let t: i64 = Numbers::read(papier, 5, -1, 4).unwrap();
        Numbers::write(papier, t, 4);
    }
}
