use std::fmt::format;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::game::Game;

pub fn draw<B: Backend>(f: &mut Frame<B>, game: &mut Game) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());
    draw_stats(f, game, chunks[0]);
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title("Board")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded),
        chunks[1],
    );
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title("Control")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded),
        chunks[2],
    );
}

// fn draw_info()
fn draw_stats<B: Backend>(f: &mut Frame<B>, game: &mut Game, area: Rect) {
    let stats = game.stats();
    let text = vec![Spans::from(vec![
        Span::styled(
            format!("tick: {} ", stats.tick),
            Style::default().fg(Color::LightCyan),
        ),
        Span::styled(
            format!("neutral: {} ", stats.neutral_count),
            Style::default().fg(Color::LightGreen),
        ),
        Span::styled(
            format!("player1: {} ", stats.player_1_count),
            Style::default().fg(Color::LightMagenta),
        ),
        Span::styled(
            format!("player2: {} ", stats.player_2_count),
            Style::default().fg(Color::LightBlue),
        ),
    ])];
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Stats")
        .title_alignment(Alignment::Left)
        .border_type(BorderType::Rounded);
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
