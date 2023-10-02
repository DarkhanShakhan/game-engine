use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::{
        canvas::{Canvas, Line, Rectangle},
        Block, BorderType, Borders, Paragraph, Wrap,
    },
    Frame,
};

use crate::game::{city::Owner, Game};

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
    draw_middle(f, game, chunks[1]);
    draw_control(f, chunks[2]);
}

fn draw_control<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let text = Spans::from(Span::styled(
        "[Down] - pause, [Esc] - quit, [Enter] - restart, [Left] - backward, [Right] - forward",
        Style::default().fg(Color::DarkGray),
    ));
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Control")
        .title_alignment(Alignment::Left)
        .border_type(BorderType::Rounded);
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_middle<B: Backend>(f: &mut Frame<B>, game: &mut Game, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(area);
    draw_board(f, game, chunks[0]);
    draw_logs(f, game, chunks[1]);
}

fn draw_logs<B: Backend>(f: &mut Frame<B>, game: &mut Game, area: Rect) {
    let mut text = vec![Spans::from(Span::styled(
        "Movements",
        Style::default().bg(Color::Cyan),
    ))];

    for m in &game.board.moves {
        if !m.is_complete() {
            text.push(Spans::from(Span::styled(
                format!(
                    "From: {} To: {} Units: {} Tick: {}",
                    m.from_city, m.to_city, m.units, m.ticks_to_finish
                ),
                Style::default().fg({
                    if m.from_owner == "p1" {
                        Color::LightMagenta
                    } else {
                        Color::LightBlue
                    }
                }),
            )));
        }
    }
    text.push(Spans::from(Span::styled(
        "Debug",
        Style::default().bg(Color::Cyan),
    )));
    text.push(Spans::from(Span::styled(
        "Player 1",
        Style::default().bg(Color::LightMagenta),
    )));
    text.push(Spans::from(Span::styled(
        game.board.p1_logs.to_string(),
        Style::default()
            .fg(Color::LightMagenta)
            .add_modifier(Modifier::ITALIC),
    )));
    text.push(Spans::from(Span::styled(
        "Player 2",
        Style::default().bg(Color::LightBlue),
    )));
    text.push(Spans::from(Span::styled(
        game.board.p2_logs.to_string(),
        Style::default()
            .fg(Color::LightBlue)
            .add_modifier(Modifier::ITALIC),
    )));

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Logs")
        .title_alignment(Alignment::Left)
        .border_type(BorderType::Rounded);
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

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
            format!(
                "{}: {} ",
                game.bots[0]
                    .filename
                    .strip_prefix("./bots/")
                    .unwrap_or_else(|| &game.bots[0].filename),
                stats.player_1_count
            ),
            Style::default().fg(Color::LightMagenta),
        ),
        Span::styled(
            format!(
                "{}: {} ",
                game.bots[1]
                    .filename
                    .strip_prefix("./bots/")
                    .unwrap_or_else(|| &game.bots[0].filename),
                stats.player_2_count
            ),
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
                                x1: ((from.position.x - 350.0) / 10.0) * 4.0,
                                y1: ((from.position.y - 350.0) / 10.0) * 3.0,
                                x2: ((m.position.x.ceil() - 350.0) / 10.0) * 4.0,
                                y2: ((m.position.y.ceil() - 350.0) / 10.0) * 3.0,
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
                    x: ((c.position.x - 350.0) / 10.0) * 4.0,
                    y: ((c.position.y - 350.0) / 10.0) * 3.0,
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
                    ((c.position.x - 350.0) / 10.0) * 4.0,
                    ((c.position.y - 350.0) / 10.0) * 3.0,
                    Spans::from(format!("{}", c.units)),
                )
            }
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);
    f.render_widget(c, area);
}
