use crate::app::AppState;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Stdout};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use super::UI;

pub struct TUI {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    user_input: String,
    message: String,
}

impl TUI {
    pub fn new() -> Result<Self, io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).ok();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Result::Ok(Self {
            terminal,
            user_input: String::new(),
            message: String::new(),
        })
    }

    fn redraw(&mut self, state: &AppState) {
        self.terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(0), Constraint::Length(3)])
                    .split(f.size());
                let viz = Block::default()
                    .title("Visualization")
                    .borders(Borders::ALL);
                f.render_widget(viz, chunks[0]);
                let input_box = Block::default()
                    .title(self.message.clone())
                    .borders(Borders::ALL);
                let user_input = Paragraph::new(Spans::from(vec![
                    Span::styled(" > ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(self.user_input.clone()),
                ]))
                .block(input_box);
                f.render_widget(user_input, chunks[1]);
            })
            .expect("Failed to render a frame");
    }
}

impl Drop for TUI {
    fn drop(&mut self) {
        disable_raw_mode().ok();
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .ok();
        self.terminal.show_cursor().ok();
    }
}

impl UI for TUI {
    fn get_input(&mut self, app_state: &AppState) -> String {
        self.redraw(app_state);
        loop {
            if let Event::Key(key) = event::read().expect("Error reading terminal") {
                match key.code {
                    KeyCode::Char(c) => {
                        self.user_input.push(c);
                    }
                    KeyCode::Enter => {
                        return std::mem::take(&mut self.user_input);
                    }
                    KeyCode::Backspace => {
                        self.user_input.pop();
                    }
                    _ => {}
                }
            }
            self.redraw(app_state);
        }
    }
    fn display_message(&mut self, message: &str) {
        self.message = message.to_owned();
    }
}
