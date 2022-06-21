use crate::{app::AppState, model::common::game::GameCommon};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use lerp::Lerp;
use std::io::{self, Stdout};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        canvas::{Canvas, Context, Line},
        Block, Borders, Paragraph,
    },
    Terminal,
};

use super::UI;

pub struct TUI {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    user_input: String,
    message: String,
    previous_input: String,
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
            previous_input: String::new(),
        })
    }

    fn redraw(&mut self, state: &AppState) {
        let viz_data = match state {
            AppState::GameNotStarted(builder) => builder.common.as_ref(),
            _ => None,
        };
        self.terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(0), Constraint::Length(3)])
                    .split(f.size());
                let viz_block = Block::default()
                    .title("Visualization")
                    .borders(Borders::ALL);
                if let Some(viz_data) = viz_data {
                    let viz = Canvas::default()
                        .x_bounds([0.0, 1.0])
                        .y_bounds([0.0, 1.0])
                        .block(viz_block)
                        .paint(|ctx| {
                            visualize(ctx, viz_data);
                        });
                    f.render_widget(viz, chunks[0]);
                } else {
                    f.render_widget(viz_block, chunks[0]);
                }
                let input_box = Block::default()
                    .title(self.message.clone())
                    .borders(Borders::ALL);
                let user_input = Paragraph::new(Spans::from(vec![
                    Span::styled(" > ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(self.user_input.clone()),
                    Span::styled("⎸", Style::default().add_modifier(Modifier::SLOW_BLINK)),
                ]))
                .block(input_box);
                f.render_widget(user_input, chunks[1]);
            })
            .expect("Failed to render a frame");
    }
}

fn visualize(ctx: &mut Context, data: &GameCommon) {
    data.routes.iter().for_each(|route| {
        let from = &data.cities[route.from.0 as usize];
        let to = &data.cities[route.to.0 as usize];
        let color = &data.colors[route.color.0 as usize];
        const LERP_AMOUNT: f32 = 0.03;
        let x1 = from.x.lerp(to.x, LERP_AMOUNT) as f64;
        let x2 = to.x.lerp(from.x, LERP_AMOUNT) as f64;
        let y1 = from.y.lerp(to.y, LERP_AMOUNT) as f64;
        let y2 = to.y.lerp(from.y, LERP_AMOUNT) as f64;
        let color = Color::Rgb(color.r, color.g, color.b);
        ctx.draw(&Line {
            x1,
            y1,
            x2,
            y2,
            color,
        });
    });
    ctx.layer();
    data.cities.iter().for_each(|city| {
        ctx.print(
            city.x as f64,
            city.y as f64,
            Span::styled(
                format!("• {}", &city.name),
                Style::default().fg(Color::White),
            ),
        );
    });
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
                if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                    return "exit".to_owned();
                }
                match key.code {
                    KeyCode::Char(c) => {
                        self.user_input.push(c);
                    }
                    KeyCode::Enter => {
                        self.previous_input = self.user_input.clone();
                        return std::mem::take(&mut self.user_input);
                    }
                    KeyCode::Backspace => {
                        self.user_input.pop();
                    }
                    KeyCode::Up => {
                        self.user_input = self.previous_input.clone();
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
