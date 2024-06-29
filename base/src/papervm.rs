use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::{self, Display};
use std::hash::Hash;
use std::sync::Arc;

pub const CHARS_PER_FLOAT: usize = 10;

#[derive(Debug, Clone)]
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

pub trait MemoryCell: Default + Debug + Clone {
    fn write(&mut self, value: char);
    fn read(&self) -> char;
}

pub trait FromChars: Debug + Send + Sync {
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

pub trait IntoChars: Debug + Send + Sync {
    fn chars_ref(&self) -> Vec<char>;
}

impl IntoChars for &dyn IntoChars {
    fn chars_ref(&self) -> Vec<char> {
        (*self).chars_ref()
    }
}

impl IntoChars for Arc<dyn IntoChars> {
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

impl IntoChars for str {
    fn chars_ref(&self) -> Vec<char> {
        self.chars().collect()
    }
}

impl IntoChars for String {
    fn chars_ref(&self) -> Vec<char> {
        self.chars().collect()
    }
}

impl IntoChars for Vec<char> {
    fn chars_ref(&self) -> Vec<char> {
        self.clone()
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Write(Arc<dyn IntoChars>),
    Call(Vec<Instruction>, Vec<Word>),
    Circle(Word),
    Add(Word, Word),
    Sub(Word, Word),
    Mod(Word, Word),
    Copy(Word),
    TrimmedCopy(Word),
    Jump(i64),
    JumpRelIf(Word, Ordering, f64, i64),
    STOP,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Write(chars) => {
                write!(
                    f,
                    "Write `{}'",
                    chars
                        .chars_ref()
                        .into_iter()
                        .filter(|c| *c != '\n')
                        .collect::<String>()
                )
            }
            Instruction::Call(instructions, args) => {
                write!(f, "Call prog[{}]({:?})", instructions.len(), args)
            }
            Instruction::Circle(w) => write!(f, "Circle {}", w),
            Instruction::Add(w1, w2) => write!(f, "Add {} {}", w1, w2),
            Instruction::Sub(w1, w2) => write!(f, "Sub {} {}", w1, w2),
            Instruction::Mod(w1, w2) => write!(f, "Mod {} {}", w1, w2),
            Instruction::Copy(w) => write!(f, "Copy {}", w),
            Instruction::TrimmedCopy(w) => write!(f, "TrimmedCopy {}", w),
            Instruction::Jump(val) => write!(f, "Jump {}", val),
            Instruction::JumpRelIf(w, ord, compare_value, jump) => {
                write!(f, "JumpRelIf {} {:?} {} {}", w, ord, compare_value, jump)
            }
            Instruction::STOP => write!(f, "STOP"),
        }
    }
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

impl Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0 .0, self.0 .1, self.1)
    }
}

impl<A, B, C> From<(A, B, C)> for Word
where
    A: Into<i64>,
    B: Into<i64>,
    C: Into<usize>,
{
    fn from((a, b, c): (A, B, C)) -> Self {
        Word(Pos(a.into(), b.into()), c.into())
    }
}

#[derive(Debug)]
pub struct SimStepState {
    pub instruction: Instruction,
    pub cursor: Pos,
}

#[derive(Debug)]
pub enum StepResult {
    Finished,
    Running(SimStepState),
}

impl StepResult {
    pub fn is_finished(&self) -> bool {
        matches!(self, StepResult::Finished)
    }
}

#[derive(Clone)]
pub struct PaperVM<T: MemoryCell> {
    memory: HashMap<Pos, T>,
    cursor: Pos,
    program: Vec<Instruction>,
    circled: Option<Word>,
    instruction_counter: i64,
    pub subroutine: Option<Box<PaperVM<T>>>,
    pub finished_papers: Vec<PaperVM<T>>,
}

impl<T: MemoryCell> PaperVM<T> {
    pub fn new(program: Vec<Instruction>) -> PaperVM<T> {
        PaperVM {
            memory: HashMap::new(),
            cursor: Pos(0, 0),
            program,
            circled: None,
            instruction_counter: 0,
            subroutine: None,
            finished_papers: vec![],
        }
    }

    pub fn get_memory(&self) -> &HashMap<Pos, T> {
        &self.memory
    }

    pub fn lowest_subroutine(&self) -> &PaperVM<T> {
        if let Some(vm) = &self.subroutine {
            vm.lowest_subroutine()
        } else {
            self
        }
    }

    pub fn get_circled(&self) -> Option<Word> {
        self.circled.map(|mut x| {
            x.0 .0 += self.cursor.0;
            x.0 .1 += self.cursor.1;
            x
        })
    }

    pub fn cursor(&self) -> Pos {
        self.cursor
    }

    fn op(&mut self, a: Word, b: Word, op: fn(f64, f64) -> f64) {
        let a: f64 = self.read(a);
        let b: f64 = self.read(b);

        let result = op(a, b);
        self.write(&result);
    }

    pub fn aread(&self, x: i64, y: i64) -> String {
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

    pub fn step(&mut self) -> StepResult {
        if let Some(result) = self.subroutine.as_mut().map(|x| x.step()) {
            if result.is_finished() {
                let word = self.subroutine.as_ref().unwrap().circled.unwrap();
                self.write(&self.subroutine.as_ref().unwrap().read::<Vec<char>>(word));
                self.finished_papers.push(*self.subroutine.take().unwrap());
            } else {
                return result;
            }
        }

        let instruction = self.program[self.instruction_counter as usize].clone();

        let sim_step_state = SimStepState {
            instruction: instruction.clone(),
            cursor: self.cursor,
        };

        match instruction.clone() {
            Instruction::Write(chars) => self.write(&chars),
            Instruction::Call(instructions, args) => {
                let mut vm: PaperVM<T> = PaperVM::new(instructions);
                for arg in args {
                    vm.write(&self.read::<Vec<char>>(arg));
                }
                self.subroutine = Some(Box::new(vm));
                self.instruction_counter += 1;
                return StepResult::Running(sim_step_state);
            }
            Instruction::Circle(arg) => {
                self.circled = Some(arg);
                return StepResult::Finished;
            }
            Instruction::Add(a, b) => self.op(a, b, |a, b| a + b),
            Instruction::Sub(a, b) => self.op(a, b, |a, b| a - b),
            Instruction::Mod(a, b) => self.op(a, b, |a, b| a % b),

            Instruction::Copy(a) => self.write(&self.read::<Vec<char>>(a)),
            Instruction::TrimmedCopy(a) => {
                let mut a: Vec<char> = self.read(a);
                a.retain(|x| !x.is_whitespace());
                self.write(&a);
            }
            Instruction::Jump(rel_jump) => {
                self.instruction_counter += rel_jump;
                return StepResult::Running(sim_step_state);
            }
            Instruction::JumpRelIf(a, ordering, val, rel_jump) => {
                let a: f64 = self.read(a);
                let cmp = a.partial_cmp(&val);
                if cmp == Some(ordering)
                        // special case for floating point equality
                        || (ordering == Ordering::Equal && (a - val).abs() < f32::EPSILON as f64)
                {
                    self.instruction_counter += rel_jump;
                    return StepResult::Running(sim_step_state);
                }
            }
            Instruction::STOP => panic!("STOP"),
        }
        self.instruction_counter += 1;

        StepResult::Running(sim_step_state)
    }

    pub fn run(&mut self) {
        loop {
            if self.step().is_finished() {
                break;
            }
        }

        println!("---\n{}---\n", self.print());
    }

    pub fn read<O: FromChars>(&self, word: Word) -> O {
        let mut chars = Vec::new();
        let length = word.1;
        let mut pos = word.0;
        for _ in 0..length {
            let cell = self
                .memory
                .get(&pos.rel_to_cursor(self.cursor))
                .map(|x| x.read())
                .unwrap_or(' ');
            pos = pos.next();
            chars.push(cell);
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
            if *c == ' ' {
                self.cursor = self.cursor.next();
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

pub mod instructions {
    use super::*;

    pub fn write(chars: impl IntoChars) -> Instruction {
        Instruction::Write(Arc::new(chars.chars_ref()))
    }

    pub fn call<A>(instructions: Vec<Instruction>, args: Vec<A>) -> Instruction
    where
        A: Into<Word>,
    {
        Instruction::Call(instructions, args.into_iter().map(Into::into).collect())
    }

    pub fn circle(word: impl Into<Word>) -> Instruction {
        Instruction::Circle(word.into())
    }

    pub fn copy(word: impl Into<Word>) -> Instruction {
        Instruction::Copy(word.into())
    }

    pub fn copy_trimmed(word: impl Into<Word>) -> Instruction {
        Instruction::TrimmedCopy(word.into())
    }

    pub fn jump(rel_jump: i64) -> Instruction {
        Instruction::Jump(rel_jump)
    }

    pub fn jump_rel_if(
        word: impl Into<Word>,
        ordering: Ordering,
        val: f64,
        rel_jump: i64,
    ) -> Instruction {
        Instruction::JumpRelIf(word.into(), ordering, val, rel_jump)
    }

    pub fn add(a: impl Into<Word>, b: impl Into<Word>) -> Instruction {
        Instruction::Add(a.into(), b.into())
    }

    pub fn sub(a: impl Into<Word>, b: impl Into<Word>) -> Instruction {
        Instruction::Sub(a.into(), b.into())
    }

    pub fn modulo(a: impl Into<Word>, b: impl Into<Word>) -> Instruction {
        Instruction::Mod(a.into(), b.into())
    }

    pub fn stop() -> Instruction {
        Instruction::STOP
    }
}
