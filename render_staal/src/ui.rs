use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Paragraph, Scrollbar, ScrollbarOrientation, Wrap},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    app.resize(frame.size().width, frame.size().height);

    // Title bar
    frame.render_widget(
        Paragraph::new(format!("{}", app.current_instruction()))
            .style(Style::default().fg(Color::White).bg(Color::LightBlue)),
        Rect {
            x: 0,
            y: 0,
            width: frame.size().width,
            height: 1,
        },
    );
}
