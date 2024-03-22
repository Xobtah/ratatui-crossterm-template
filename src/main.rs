use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

struct Model {
    name: String,
}

impl Model {
    fn new(name: &str) -> Self {
        Self { name: name.into() }
    }
}

impl Widget for &Model {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Min(1), Constraint::Length(1)]);
        let [message_area, footer_area] = vertical.areas(area);

        <&str as Into<Text>>::into("Hello world !")
            .fg(Color::White)
            .bg(Color::Black)
            .render(message_area, buf);

        Text::from(Line::from(vec![
            "Template made by ".into(),
            self.name.clone().bold(),
            ", press ".into(),
            "Esc".bold(),
            " to exit.".into(),
        ]))
        .render(footer_area, buf);
    }
}

fn handle_input() -> io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Esc => Ok(false),
            _ => Ok(true),
        }
    } else {
        Ok(true)
    }
}

fn run() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let model = Model::new("Sylvain Garant");

    while handle_input()? {
        let _ = terminal.draw(|frame| frame.render_widget(&model, frame.size()));
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err:#?}");
    }
}
