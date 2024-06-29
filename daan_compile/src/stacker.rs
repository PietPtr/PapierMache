use papier::papervm::instructions::*;
use papier::papervm::CharCell;
use papier::papervm::Instruction;
use papier::papervm::IntoChars;
use papier::papervm::PaperVM;
use papier::papervm::Word;
use std::cmp::Ordering;

use papier::papervm::CHARS_PER_FLOAT as CPF;

static CPFI: i32 = CPF as i32;

pub struct Pos {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Pos {
    fn from(value: (i32, i32)) -> Self {
        Pos {
            x: value.0,
            y: value.1,
        }
    }
}

impl Pos {
    fn to_word(&self) -> Word {
        Word::from((self.x * CPFI, self.y, CPF))
    }
}

fn text(value: &str) -> StackInstr {
    let mut result = [' '; CPF];

    let chars = value.chars_ref();
    for i in 0..CPF.min(value.len()) {
        result[i] = chars[i];
    }

    StackInstr::Text(result)
}

fn textbox(value: &str) -> String {
    let mut result = [' '; CPF];
    let chars = value.chars_ref();
    for i in 0..CPF.min(value.len()) {
        result[i] = chars[i];
    }

    let mut res = String::new();

    res.extend(result);
    res
}

pub enum StackInstr {
    Text([char; CPF]),
    Write(f64),
    Copy(Pos),
    Add(Pos, Pos),
    Sub(Pos, Pos),
    Mod(Pos, Pos),
    Jump(i64),
    JumpRelIf(Pos, Ordering, f64, i64),
    JumpEmpty(Pos, i64),
    JumpRelCmp(Pos, Pos, Ordering, i64),
    Ret(Pos),
    Break,

    Call {
        substack: Vec<Vec<StackInstr>>,
        inputs: Vec<Pos>,
    },
}

pub fn compile_stacker(lines: Vec<Vec<StackInstr>>) -> Vec<Instruction> {
    let mut stack = Vec::new();
    for line in lines {
        for (i, instruction) in line.into_iter().enumerate() {
            let inst = match instruction {
                StackInstr::Text(text) => write(text.to_vec()),
                StackInstr::Write(val) => write(val),
                StackInstr::Copy(pos) => Instruction::Copy(pos.to_word()),
                StackInstr::Add(pos1, pos2) => Instruction::Add(pos1.to_word(), pos1.to_word()),
                StackInstr::Sub(pos1, pos2) => Instruction::Sub(pos1.to_word(), pos2.to_word()),
                StackInstr::Mod(pos1, pos2) => Instruction::Mod(pos1.to_word(), pos2.to_word()),
                StackInstr::Call { substack, inputs } => {
                    let mut subcalls = vec![write("\n")];
                    subcalls.extend(compile_stacker(substack));

                    Instruction::Call(subcalls, inputs.into_iter().map(|x| x.to_word()).collect())
                }
                StackInstr::Jump(jump) => Instruction::Jump(jump),
                StackInstr::JumpRelIf(pos, ordering, val, dest) => {
                    Instruction::JumpRelIf(pos.to_word(), ordering, val, dest)
                }
                StackInstr::JumpRelCmp(pos1, pos2, ordering, dest) => {
                    Instruction::JumpRelCmp(pos1.to_word(), pos2.to_word(), ordering, dest)
                }
                StackInstr::JumpEmpty(pos, dest) => {
                    Instruction::JumpRelIfStr(pos.to_word(), textbox(""), dest)
                }
                StackInstr::Ret(pos) => Instruction::Circle(pos.to_word()),
                StackInstr::Break => Instruction::BreakPoint,
            };
            stack.push(inst)
        }
        stack.push(write("\n"))
    }

    stack
}

pub fn gcd() -> Vec<Vec<StackInstr>> {
    vec![
        vec![text("a"), text("b")],
        vec![
            StackInstr::Copy(Pos { x: 0, y: -2 }),
            StackInstr::Copy(Pos { x: 0, y: -2 }),
        ],
        vec![
            StackInstr::JumpRelCmp(Pos { x: 0, y: -1 }, Pos { x: 1, y: -1 }, Ordering::Equal, 8),
            StackInstr::JumpRelCmp(Pos { x: 0, y: -1 }, Pos { x: 1, y: -1 }, Ordering::Less, 4),
            StackInstr::Sub(Pos { x: 0, y: -1 }, Pos { x: 1, y: -1 }),
            StackInstr::Copy(Pos { x: 0, y: -1 }),
            StackInstr::Jump(-5),
            StackInstr::Copy(Pos { x: 0, y: -1 }),
            StackInstr::Sub(Pos { x: 0, y: -1 }, Pos { x: -1, y: -1 }),
            StackInstr::Jump(-8),
            StackInstr::Break,
        ],
    ]
}

pub fn sort() -> Vec<Vec<StackInstr>> {
    vec![
        vec![
            StackInstr::JumpEmpty(Pos { x: 1, y: -1 }, 8),
            StackInstr::JumpRelCmp(
                Pos { x: 0, y: -1 },
                Pos { x: 1, y: -1 },
                Ordering::Greater,
                3,
            ),
            StackInstr::Copy(Pos { x: 0, y: -1 }),
            StackInstr::Jump(-3),
            StackInstr::Copy(Pos { x: 1, y: -1 }),
            StackInstr::Copy(Pos { x: -1, y: -1 }),
            StackInstr::Jump(-6),
            StackInstr::Break,
            // Check if we need to copy last value
            StackInstr::Copy(Pos { x: 0, y: -1 }),
        ],
        vec![StackInstr::Jump(-9)],
    ]
}
