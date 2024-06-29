use papier::convenience::*;
use papier::papervm::*;
use papier::programs::*;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use render_staal::app::{App, AppResult};
use render_staal::event::{Event, EventHandler};
use render_staal::handler::{handle_key_events, handle_mouse_events};
use render_staal::tui::Tui;
use std::io;

pub fn main() -> AppResult<()> {
    let program = call_static(
        sort(),
        vec![10., 8., 9., 7., 6., 3., 4., 5., 1., 2.],
        CHARS_PER_FLOAT,
    );

    render_staal::run_program(program)
}
