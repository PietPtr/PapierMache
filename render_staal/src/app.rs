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
    current_instruction: Instruction,
    vm: PaperVM<CharCell>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            running: true,
            current_instruction: program.first().unwrap().clone(),
            vm: PaperVM::<CharCell>::new(program),
        }
    }

    pub async fn init(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        // self.page_title = format!("{:?}", self.page_content_length);
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn advance_sim(&mut self) {
        match self.vm.step() {
            StepResult::Finished => (),
            StepResult::Running(instr) => self.current_instruction = instr,
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {}

    pub fn scroll(&mut self, key: KeyCode) {
        // TODO: d-pad scrolling
    }

    pub fn set_scroll_params(&mut self, length: usize) {
        // self.vertical_scroll_state = self.vertical_scroll_state.content_length(length);
        // self.page_content_length = length;
    }

    pub fn current_instruction(&self) -> Instruction {
        self.current_instruction.clone()
    }
}
