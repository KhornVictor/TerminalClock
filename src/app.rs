use std::io::{self, Write};
use std::time::Duration;
use chrono::{Local, Timelike};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    execute, queue,
    style::{Print, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::digits::{render_colon, render_digit};
use crate::theme::THEMES;

/// RAII Guard to guarantee terminal state restoration on normal exit or panic.
pub struct TerminalGuard;

impl TerminalGuard {
    pub fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, Hide)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let mut stdout = io::stdout();
        let _ = execute!(stdout, Show, LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}

/// Runs the main terminal clock loop.
pub fn run() -> io::Result<()> {
    // Set custom panic hook to restore terminal if panic occurs
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = execute!(io::stdout(), Show, LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
        default_panic(info);
    }));

    let _guard = TerminalGuard::new()?;
    let mut stdout = io::stdout();

    let mut theme_idx = 0;
    let mut use_12h = true;

    loop {
        let now = Local::now();
        let (hour, is_pm) = if use_12h {
            let (is_pm, h) = now.hour12();
            (h, is_pm)
        } else {
            (now.hour(), now.hour() >= 12)
        };

        let min = now.minute();
        let sec = now.second();

        let time_str = format!("{:02}{:02}{:02}", hour, min, sec);
        let digits: Vec<char> = time_str.chars().collect();

        // Format date matching reference image: "Saturday, September 14, 2024 [PM]"
        let am_pm = if is_pm { "PM" } else { "AM" };
        let date_str = if use_12h {
            format!("{} [{}]", now.format("%A, %B %e, %Y"), am_pm)
        } else {
            now.format("%A, %B %e, %Y").to_string()
        };

        // Terminal window size
        let (term_w, term_h) = terminal::size()?;

        // Total width calculations:
        // 6 digits (10 chars each) + 2 colons (6 chars each) + 7 gaps (2 chars each) = 86 chars wide
        let clock_width = 86;
        let clock_height = 7; // 5 digit rows + 1 gap row + 1 date row

        let start_x = if term_w > clock_width {
            (term_w - clock_width) / 2
        } else {
            0
        };

        let start_y = if term_h > clock_height {
            (term_h - clock_height) / 2
        } else {
            0
        };

        queue!(stdout, Clear(ClearType::All))?;
        queue!(stdout, SetForegroundColor(THEMES[theme_idx].color))?;

        // Render 5 rows of large digits
        for r in 0..5 {
            queue!(stdout, MoveTo(start_x, start_y + r as u16))?;

            // Hours H1, H2
            render_digit(&mut stdout, digits[0], r)?;
            queue!(stdout, Print("  "))?;
            render_digit(&mut stdout, digits[1], r)?;

            // Colon 1
            queue!(stdout, Print("  "))?;
            render_colon(&mut stdout, r)?;
            queue!(stdout, Print("  "))?;

            // Minutes M1, M2
            render_digit(&mut stdout, digits[2], r)?;
            queue!(stdout, Print("  "))?;
            render_digit(&mut stdout, digits[3], r)?;

            // Colon 2
            queue!(stdout, Print("  "))?;
            render_colon(&mut stdout, r)?;
            queue!(stdout, Print("  "))?;

            // Seconds S1, S2
            render_digit(&mut stdout, digits[4], r)?;
            queue!(stdout, Print("  "))?;
            render_digit(&mut stdout, digits[5], r)?;
        }

        // Render centered date underneath time
        let date_len = date_str.chars().count() as u16;
        let date_x = if term_w > date_len {
            (term_w - date_len) / 2
        } else {
            0
        };
        let date_y = start_y + 6;

        queue!(stdout, MoveTo(date_x, date_y))?;
        queue!(stdout, Print(&date_str))?;

        stdout.flush()?;

        // Poll for user keyboard input with 100ms timeout
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('c') | KeyCode::Char('t') => {
                            theme_idx = (theme_idx + 1) % THEMES.len();
                        }
                        KeyCode::Char('f') | KeyCode::Char(' ') => {
                            use_12h = !use_12h;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}
