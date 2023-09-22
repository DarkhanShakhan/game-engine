use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::game::Game;

use super::ui::draw;

pub fn run(tick_rate: Duration, bot_1_path: &str, bot_2_path: &str) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = run_game(&mut terminal, bot_1_path, bot_2_path, tick_rate);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_game<B: Backend>(
    terminal: &mut Terminal<B>,
    bot_1_path: &str,
    bot_2_path: &str,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut game = Game::new(bot_1_path, bot_2_path);
    let mut last_tick = Instant::now();
    let mut is_pause = false;
    loop {
        terminal.draw(|f| draw(f, &mut game))?;
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(1));
        if poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Left => {
                        if is_pause {
                            game.on_left()
                        }
                    }
                    KeyCode::Right => {
                        if is_pause {
                            game.on_right()
                        }
                    }
                    KeyCode::Down => {
                        is_pause = !is_pause;
                    }
                    KeyCode::Enter => {
                        game = Game::new(bot_1_path, bot_2_path);
                    }
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate && !game.is_finished() && !is_pause {
            game.tick();
            last_tick = Instant::now();
        }
    }
    Ok(())
}
