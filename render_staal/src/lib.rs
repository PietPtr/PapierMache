use crate::app::{App, AppResult};
use crate::event::{Event, EventHandler};
use crate::handler::{handle_key_events, handle_mouse_events};
use crate::tui::Tui;
use papier::papervm::*;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

pub mod app;
pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;

pub fn run_program(program: Vec<Instruction>) -> AppResult<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let mut app = App::new(program);

            let backend = CrosstermBackend::new(io::stderr());
            let terminal = Terminal::new(backend)?;
            let events = EventHandler::new(10);
            let mut tui = Tui::new(terminal, events);
            tui.init()?;
            app.init().await?;

            while app.running {
                tui.draw(&mut app)?;

                match tui.events.next().await? {
                    Event::Tick => app.tick(),
                    Event::Key(key_event) => handle_key_events(key_event, &mut app).await?,
                    Event::Mouse(mouse_event) => handle_mouse_events(mouse_event, &mut app)?,
                    Event::Resize(width, height) => app.resize(width, height),
                }
            }

            tui.exit()?;
            Ok(())
        })
}
