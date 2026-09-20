use std::io;
use crossterm::{queue, style::Print};

/// Returns a 5x5 boolean matrix representing the 7-segment/pixel font for a digit '0'-'9'.
pub fn get_digit_matrix(ch: char) -> [[bool; 5]; 5] {
    match ch {
        '0' => [
            [true, true, true, true, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, true],
        ],
        '1' => [
            [false, false, true, true, false],
            [false, true, true, true, false],
            [false, false, true, true, false],
            [false, false, true, true, false],
            [true, true, true, true, true],
        ],
        '2' => [
            [true, true, true, true, true],
            [false, false, false, false, true],
            [true, true, true, true, true],
            [true, false, false, false, false],
            [true, true, true, true, true],
        ],
        '3' => [
            [true, true, true, true, true],
            [false, false, false, false, true],
            [true, true, true, true, true],
            [false, false, false, false, true],
            [true, true, true, true, true],
        ],
        '4' => [
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, true],
            [false, false, false, false, true],
            [false, false, false, false, true],
        ],
        '5' => [
            [true, true, true, true, true],
            [true, false, false, false, false],
            [true, true, true, true, true],
            [false, false, false, false, true],
            [true, true, true, true, true],
        ],
        '6' => [
            [true, true, true, true, true],
            [true, false, false, false, false],
            [true, true, true, true, true],
            [true, false, false, false, true],
            [true, true, true, true, true],
        ],
        '7' => [
            [true, true, true, true, true],
            [false, false, false, false, true],
            [false, false, false, false, true],
            [false, false, false, false, true],
            [false, false, false, false, true],
        ],
        '8' => [
            [true, true, true, true, true],
            [true, false, false, false, true],
            [true, true, true, true, true],
            [true, false, false, false, true],
            [true, true, true, true, true],
        ],
        '9' => [
            [true, true, true, true, true],
            [true, false, false, false, true],
            [true, true, true, true, true],
            [false, false, false, false, true],
            [true, true, true, true, true],
        ],
        _ => [[false; 5]; 5],
    }
}

/// Returns whether the colon contains a square block at the given row index (0..4).
pub fn get_colon_has_block(row: usize) -> bool {
    row == 1 || row == 3
}

/// Renders a single row of a digit character (width 10 columns: 5 units * 2 chars).
pub fn render_digit(stdout: &mut io::Stdout, ch: char, row: usize) -> io::Result<()> {
    let matrix = get_digit_matrix(ch);
    for col in 0..5 {
        if matrix[row][col] {
            queue!(stdout, Print("██"))?;
        } else {
            queue!(stdout, Print("  "))?;
        }
    }
    Ok(())
}

/// Renders a single row of a colon separator (width 6 columns: "  ██  " or "      ").
pub fn render_colon(stdout: &mut io::Stdout, row: usize) -> io::Result<()> {
    if get_colon_has_block(row) {
        queue!(stdout, Print("  ██  "))?;
    } else {
        queue!(stdout, Print("      "))?;
    }
    Ok(())
}
