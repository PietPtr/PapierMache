use clap::{Arg, Command};
use papier::convenience::*;
use papier::papervm::*;
use papier::programs::*;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use render_staal::app::{App, AppResult};
use render_staal::event::{Event, EventHandler};
use render_staal::handler::{handle_key_events, handle_mouse_events};
use render_staal::tui::Tui;
use std::{fs, io};

#[tokio::main]
async fn main() -> AppResult<()> {
    let program = call_static(gcd_with_mod(), vec![105., 20.], CHARS_PER_FLOAT);

    let mut app = App::new(program);

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
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
}
