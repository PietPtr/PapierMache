use crossterm::event::KeyCode;
use papier::convenience::*;
use papier::papervm::Instruction;
use papier::papervm::*;
use papier::programs::*;
use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::ScrollbarState,
};
use std::{
    collections::HashMap,
    error::{self, Error},
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct App {
    pub running: bool,
    last_sim_step: SimStepState,
    free_running: bool,
    vm: PaperVM<CharCell>,
    view_pos: Pos,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(program: Vec<Instruction>) -> Self {
        let mut vm = PaperVM::<CharCell>::new(program);
        let last_sim_step = vm.step();
        Self {
            running: true,
            last_sim_step: match last_sim_step {
                StepResult::Finished => todo!(),
                StepResult::Running(s) => s,
            },
            free_running: false,
            vm,
            view_pos: Pos(0, 0),
        }
    }

    pub async fn init(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        if self.free_running {
            self.advance_sim();
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn advance_sim(&mut self) {
        match self.vm.step() {
            StepResult::Finished => panic!("what do on finish"),
            StepResult::Running(step_state) => self.last_sim_step = step_state,
        }
        if let Instruction::BreakPoint = self.last_sim_step.instruction {
            self.free_running = false;
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {}

    pub fn scroll(&mut self, key: KeyCode) {
        match key {
            KeyCode::Left => self.view_pos.0 -= 1,
            KeyCode::Right => self.view_pos.0 += 1,
            KeyCode::Up => self.view_pos.1 -= 1,
            KeyCode::Down => self.view_pos.1 += 1,
            _ => {}
        }
    }

    pub fn current_instruction(&self) -> Instruction {
        self.last_sim_step.instruction.clone()
    }

    pub fn get_view_as_string(&self, size: Rect) -> String {
        let memory = self.vm.lowest_subroutine().get_memory();

        let mut result = String::new();
        for y in self.view_pos.1..(self.view_pos.1 + size.height as i64) {
            for x in self.view_pos.0..(self.view_pos.0 + size.width as i64) {
                let ch = memory
                    .get(&Pos(x, y))
                    .map(|c| c.read().to_string())
                    .unwrap_or(" ".to_string());
                result.push_str(&ch);
            }
            result.push('\n');
        }
        result
    }

    fn apply_view(&self, pos: Pos) -> Pos {
        Pos(pos.0 - self.view_pos.0, pos.1 - self.view_pos.1 + 1)
    }

    pub fn last_cursor(&self) -> Pos {
        self.apply_view(self.last_sim_step.cursor)
    }
    pub fn cursor(&self) -> Pos {
        self.apply_view(self.vm.lowest_subroutine().cursor())
        // self.apply_view(self.last_sim_step.cursor)
    }

    pub fn highlight_words(&self) -> Vec<Word> {
        match self.current_instruction() {
            Instruction::Circle(word) => vec![word],
            Instruction::Add(w1, w2) => vec![w1, w2],
            Instruction::Sub(w1, w2) => vec![w1, w2],
            Instruction::Mod(w1, w2) => vec![w1, w2],
            Instruction::Copy(word) => vec![word],
            Instruction::TrimmedCopy(word) => vec![word],
            Instruction::Write(_) => vec![],
            Instruction::Call(_, _) => vec![],
            Instruction::Jump(_) => vec![],
            Instruction::JumpRelIf(word, _, _, _) => vec![word],
            Instruction::JumpRelIfStr(word, _, _) => vec![word],
            Instruction::MoveCursor(_) => vec![],
            Instruction::Stop => vec![],
            Instruction::BreakPoint => vec![],
            Instruction::JumpRelCmp(a, b, _, _) => vec![a, b],
        }
    }

    pub(crate) fn toggle_free_running(&mut self) {
        self.free_running = !self.free_running
    }
}
