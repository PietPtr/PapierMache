use std::collections::HashMap;
use std::fmt::Debug;

pub const CHARS_PER_FLOAT: usize = 10;

#[derive(Debug)]
pub struct CharCell {
    value: char,
}

impl Default for CharCell {
    fn default() -> Self {
        CharCell { value: ' ' }
    }
}

impl MemoryCell for CharCell {
    fn write(&mut self, value: char) {
        self.value = value;
    }

    fn read(&self) -> char {
        self.value
    }
}

pub trait MemoryCell: Default + Debug {
    fn write(&mut self, value: char);
    fn read(&self) -> char;
}

pub trait FromChars {
    fn from_chars(chars: Vec<char>) -> Self;
}

impl FromChars for i64 {
    fn from_chars(chars: Vec<char>) -> i64 {
        chars.iter().collect::<String>().trim().parse().unwrap()
    }
}

impl FromChars for f64 {
    fn from_chars(chars: Vec<char>) -> f64 {
        chars.iter().collect::<String>().trim().parse().unwrap()
    }
}

impl FromChars for Vec<char> {
    fn from_chars(chars: Vec<char>) -> Vec<char> {
        chars
    }
}

pub trait IntoChars: Debug {
    fn chars_ref(&self) -> Vec<char>;
}

impl IntoChars for Box<dyn IntoChars> {
    fn chars_ref(&self) -> Vec<char> {
        self.as_ref().chars_ref()
    }
}

impl IntoChars for char {
    fn chars_ref(&self) -> Vec<char> {
        vec![*self]
    }
}

impl IntoChars for f64 {
    fn chars_ref(&self) -> Vec<char> {
        format!("{:width$}", self, width = CHARS_PER_FLOAT)
            .chars()
            .collect()
    }
}

impl IntoChars for &str {
    fn chars_ref(&self) -> Vec<char> {
        self.chars().collect()
    }
}

impl IntoChars for Vec<char> {
    fn chars_ref(&self) -> Vec<char> {
        self.clone()
    }
}

#[derive(Debug)]
pub enum Instruction {
    Write(Box<dyn IntoChars>),
    Call(Vec<Instruction>, Vec<Word>),
    Circle(Word),
    Add(Word, Word),
    Mod(Word, Word),
    Copy(Word),
    Jump(i64),
    JumpRelIf(Word, f64, i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(pub i64, pub i64);

impl Pos {
    fn next(&self) -> Pos {
        Pos(self.0 + 1, self.1)
    }

    fn down(&self) -> Pos {
        Pos(0, self.1 + 1)
    }

    fn rel_to_cursor(&self, cursor: Pos) -> Pos {
        Pos(self.0 + cursor.0, self.1 + cursor.1)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Word(pub Pos, pub usize);

pub struct PaperVM<'a, T: MemoryCell> {
    memory: HashMap<Pos, T>,
    cursor: Pos,
    program: &'a Vec<Instruction>,
    circled: Option<Word>,
}

impl<'a, T: MemoryCell> PaperVM<'a, T> {
    pub fn new(program: &'a Vec<Instruction>) -> PaperVM<T> {
        PaperVM {
            memory: HashMap::new(),
            cursor: Pos(0, 0),
            program,
            circled: None,
        }
    }

    fn op(&mut self, a: Word, b: Word, op: fn(f64, f64) -> f64) {
        let a: f64 = self.read(a);
        let b: f64 = self.read(b);

        let result = op(a, b);
        self.write(&result);
    }

    fn aread(&self, x: i64, y: i64) -> String {
        self.memory
            .get(&Pos(x, y))
            .map(|c| c.read().to_string())
            .unwrap_or(" ".to_string())
    }

    pub fn print(&self) -> String {
        if self.memory.is_empty() {
            return String::new();
        }

        let &min_x = self.memory.keys().map(|Pos(x, _)| x).min().unwrap();
        let &max_x = self.memory.keys().map(|Pos(x, _)| x).max().unwrap();
        let &min_y = self.memory.keys().map(|Pos(_, y)| y).min().unwrap();
        let &max_y = self.memory.keys().map(|Pos(_, y)| y).max().unwrap();

        let mut result = String::new();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let ch = self.aread(x, y);
                result.push_str(&ch);
            }
            result.push('\n');
        }

        result
    }

    pub fn run(&mut self) {
        let mut instruction_counter: i64 = 0;

        loop {
            println!("---\n{}---\n", self.print());
            let instruction = &self.program[instruction_counter as usize];
            println!("Instruction: {:?}, cursor: {:?}", instruction, self.cursor);
            match instruction {
                Instruction::Write(chars) => self.write(chars),
                Instruction::Call(instructions, args) => {
                    let mut vm: PaperVM<T> = PaperVM::new(instructions);
                    for arg in args {
                        vm.write(&self.read::<Vec<char>>(*arg));
                    }
                    vm.run();
                    let word = vm.circled.unwrap();
                    self.write(&vm.read::<Vec<char>>(word));
                }
                Instruction::Circle(arg) => {
                    self.circled = Some(*arg);
                    break;
                }
                Instruction::Add(a, b) => self.op(*a, *b, |a, b| a + b),
                Instruction::Mod(a, b) => self.op(*a, *b, |a, b| {
                    dbg!(a, b, a % b);
                    a % b
                }),
                Instruction::Copy(a) => self.write(&self.read::<Vec<char>>(*a)),
                Instruction::Jump(rel_jump) => {
                    instruction_counter += rel_jump;
                    continue;
                }
                Instruction::JumpRelIf(a, val, rel_jump) => {
                    if (self.read::<f64>(*a) - val) < (std::f32::EPSILON as f64) {
                        instruction_counter += rel_jump;
                        continue;
                    }
                }
            }
            instruction_counter += 1;
        }
    }

    pub fn read<O: FromChars>(&self, word: Word) -> O {
        let mut chars = Vec::new();
        let length = word.1;
        let mut pos = word.0;
        for _ in 0..length {
            let cell = self.memory.get(&pos.rel_to_cursor(self.cursor)).unwrap();
            pos = pos.next();
            chars.push(cell.read());
        }
        O::from_chars(chars)
    }

    pub fn write(&mut self, value: &impl IntoChars) {
        let chars = value.chars_ref();
        for c in chars.iter() {
            if *c == '\n' {
                self.cursor = self.cursor.down();
                continue;
            }
            let cell = self.memory.entry(self.cursor).or_default();

            self.cursor = self.cursor.next();
            cell.write(*c);
        }
    }

    pub fn result<O: FromChars>(&mut self) -> Option<O> {
        self.circled
            .map(|word| O::from_chars(self.read::<Vec<char>>(word)))
    }
}
