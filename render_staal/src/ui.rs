use papier::papervm::Pos;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Paragraph, Scrollbar, ScrollbarOrientation, Wrap},
    Frame,
};

use crate::app::App;

fn pos_is_in_view(frame: Rect, pos: Pos) -> bool {
    pos.0 >= 0 && pos.0 < frame.width as i64 && pos.1 >= 0 && pos.1 < frame.height as i64
}

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    app.resize(frame.size().width, frame.size().height);

    let cursor_pos = app.cursor();
    let cursor = Paragraph::new("").style(Style::default().bg(Color::Yellow));

    if pos_is_in_view(frame.size(), cursor_pos) {
        frame.render_widget(
            cursor,
            Rect {
                x: cursor_pos.0 as u16,
                y: cursor_pos.1 as u16,
                width: 1,
                height: 1,
            },
        );
    }

    let view: String = app.get_view_as_string(frame.size());
    let p = Paragraph::new(view).style(Style::default().fg(Color::Black));

    frame.render_widget(
        p,
        Rect {
            x: 0,
            y: 1,
            width: frame.size().width - 1,
            height: frame.size().height - 1,
        },
    );

    let last_cursor_pos = app.last_cursor();
    for word in app.highlight_words() {
        let x = last_cursor_pos.0 + word.0 .0;
        let y = last_cursor_pos.1 + word.0 .1;
        let width = word.1;

        if pos_is_in_view(frame.size(), Pos(x, y))
            || pos_is_in_view(frame.size(), Pos(x + width as i64, y))
        {
            frame.render_widget(
                Paragraph::new("").style(Style::default().bg(Color::Cyan)),
                Rect {
                    x: x.clamp(0, frame.size().width as i64) as u16,
                    y: y.clamp(0, frame.size().height as i64) as u16,
                    width: (width as i64
                        + 0.min(x)
                        + 0.min(frame.size().width as i64 - x - width as i64))
                    .clamp(0, u16::MAX as i64) as u16,
                    height: 1,
                },
            );
        }
    }

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
