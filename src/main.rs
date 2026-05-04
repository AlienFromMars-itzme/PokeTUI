use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode};
use firecli::{
    core::state_machine::{GameState, StateMachine},
    ui::renderer,
};
use ratatui::{Terminal, backend::CrosstermBackend};

fn main() -> anyhow::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut sm = StateMachine::new();
    sm.transition_to(GameState::Overworld);
    let mut status = String::from("Welcome to FireCLI");
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    let res = loop {
        terminal.draw(|f| renderer::render(f, sm.current(), &status))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)?
            && let Event::Key(key) = event::read()?
        {
            match key.code {
                KeyCode::Char('q') => break Ok(()),
                KeyCode::Char('i') => sm.transition_to(GameState::Inventory),
                KeyCode::Char('b') => sm.transition_to(GameState::Battle),
                KeyCode::Esc => sm.transition_to(GameState::Overworld),
                KeyCode::Char('w') | KeyCode::Up => status = "Moved north".into(),
                KeyCode::Char('a') | KeyCode::Left => status = "Moved west".into(),
                KeyCode::Char('s') | KeyCode::Down => status = "Moved south".into(),
                KeyCode::Char('d') | KeyCode::Right => status = "Moved east".into(),
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    };

    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;
    res
}
