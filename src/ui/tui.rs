use crate::states::AppState;
use crate::{model::game_map::GameMap, views::VisualizeView};
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
    message_style: Style,
    previous_input: String,
}

impl TUI {
    pub fn new() -> Result<Self, io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).ok();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear().ok();
        Result::Ok(Self {
            terminal,
            user_input: String::new(),
            message: String::new(),
            message_style: Style::default(),
            previous_input: String::new(),
        })
    }

    fn redraw(&mut self, state: &dyn AppState) {
        let viz_data = state.visualize_view().as_ref().map(VisualizeView::map);
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
                        .x_bounds([0.0, 255.0])
                        .y_bounds([0.0, 255.0])
                        .block(viz_block)
                        .paint(|ctx| {
                            visualize(ctx, viz_data);
                        });
                    f.render_widget(viz, chunks[0]);
                } else {
                    f.render_widget(viz_block, chunks[0]);
                }
                let input_box = Block::default()
                    .title(Spans::from(vec![
                        Span::raw(" "),
                        Span::styled(&self.message, self.message_style),
                        Span::raw(" "),
                    ]))
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

fn visualize(ctx: &mut Context, data: &GameMap) {
    data.routes.iter().for_each(|route| {
        let from = &data.cities.cities()[route.from()];
        let to = &data.cities.cities()[route.to()];
        let color = &data.colors.colors()[route.color()];
        const LERP_AMOUNT: f64 = 0.03;
        let from_x = from.x() as f64;
        let from_y = from.y() as f64;
        let to_x = to.x() as f64;
        let to_y = to.y() as f64;
        let x1 = from_x.lerp(to_x, LERP_AMOUNT) as f64;
        let x2 = to_x.lerp(from_x, LERP_AMOUNT) as f64;
        let y1 = from_y.lerp(to_y, LERP_AMOUNT) as f64;
        let y2 = to_y.lerp(from_y, LERP_AMOUNT) as f64;
        let color = Color::Rgb(color.r(), color.g(), color.b());
        ctx.draw(&Line {
            x1,
            y1,
            x2,
            y2,
            color,
        });
    });
    ctx.layer();
    data.cities.cities().iter().for_each(|city| {
        ctx.print(
            city.x() as f64,
            city.y() as f64,
            Span::styled(
                format!("• {}", city.name()),
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
    fn get_input(&mut self, app_state: &dyn AppState) -> String {
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
        self.message_style = Style::default();
    }

    fn display_error(&mut self, error: &str) {
        self.display_message(error);
        self.message_style = Style::default().fg(Color::Red).add_modifier(Modifier::BOLD);
    }
}
