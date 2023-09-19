use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    text::{Span, Spans},
    widgets::{
        canvas::{Canvas, Line, Rectangle},
        Block, BorderType, Borders, Paragraph, Wrap,
    },
    Frame,
};

use crate::{city::Owner, game::Game};

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
    draw_board(f, game, chunks[1]);
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

fn draw_board<B: Backend>(f: &mut Frame<B>, game: &mut Game, area: Rect) {
    let c = Canvas::default()
        .marker(symbols::Marker::Braille)
        .block(
            Block::default()
                .title("Board")
                .borders(Borders::ALL)
                .title_alignment(Alignment::Left)
                .border_type(BorderType::Rounded),
        )
        .paint(|ctx| {
            for m in &game.board.moves {
                if !m.is_complete() {
                    if let Some(from) = game.board.cities.get(&m.from_city) {
                        if game.board.cities.get(&m.to_city).is_some() {
                            ctx.draw(&Line {
                                x1: ((from.x - 350) / 10) as f64 * 4.0,
                                y1: ((from.y - 350) / 10) as f64 * 3.0,
                                x2: ((m.x.ceil() - 350.0) / 10.0) * 4.0,
                                y2: ((m.y.ceil() - 350.0) / 10.0) * 3.0,
                                color: {
                                    if m.from_owner == "p1" {
                                        Color::LightMagenta
                                    } else {
                                        Color::LightBlue
                                    }
                                },
                            })
                        }
                    }
                }
            }
            ctx.layer();
            for c in game.board.cities.values() {
                ctx.draw(&Rectangle {
                    x: ((c.x - 350) / 10) as f64 * 4.0,
                    y: ((c.y - 350) / 10) as f64 * 3.0,
                    width: 5.0,
                    height: 5.0,
                    color: {
                        if let Owner::Player(name) = &c.owner {
                            if name == "p1" {
                                Color::LightMagenta
                            } else {
                                Color::LightBlue
                            }
                        } else {
                            Color::LightGreen
                        }
                    },
                });
                ctx.print(
                    ((c.x - 350) / 10) as f64 * 4.0,
                    ((c.y - 350) / 10) as f64 * 3.0,
                    Spans::from(format!("{}", c.units)),
                )
            }
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);
    f.render_widget(c, area);
}
