use crate::{
    core::state_machine::GameState,
    ui::{screens::title_for, theme},
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, state: GameState, status: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(8),
            Constraint::Length(5),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let main = Block::default()
        .title(title_for(state))
        .borders(Borders::ALL)
        .border_style((theme::BORDER,));
    frame.render_widget(main, chunks[0]);
    frame.render_widget(
        Paragraph::new(status).block(
            Block::default()
                .title("Dialogue / Action")
                .borders(Borders::ALL),
        ),
        chunks[1],
    );
    frame.render_widget(
        Paragraph::new("WASD move • I inventory • B battle • Q quit")
            .block(Block::default().title("Status").borders(Borders::ALL)),
        chunks[2],
    );
}
