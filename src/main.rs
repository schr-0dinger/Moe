use chrono::{Local, Timelike};
use std::{io::{stdout, Write}, thread, time};

// ANSI color codes
const COLOR_RESET: &str = "\x1B[0m";
const COLOR_DIGIT: &str = "\x1B[1;36m"; // Bright Cyan for numbers
const COLOR_COLON: &str = "\x1B[1;31m"; // Bright Red for colon `:`

// 6-row representation of numbers
// The rendered digits 1 and colon are thinner
const BLOCK_DIGITS: [[u16; 6]; 10] = [
    [0x1FE, 0x186, 0x186, 0x186, 0x1FE, 0x0], // 0
    [ 14, 6, 6, 6, 6, 0], // 1
    [0x1FE, 0x006, 0x1FE, 0x180, 0x1FE, 0x0], // 2
    [0x1FE, 0x006, 0x07E, 0x006, 0x1FE, 0x0], // 3
    [0x186, 0x186, 0x1FE, 0x006, 0x006, 0x0], // 4
    [0x1FE, 0x180, 0x1FE, 0x006, 0x1FE, 0x0], // 5
    [0x1FE, 0x180, 0x1FE, 0x186, 0x1FE, 0x0], // 6
    [0x1FE, 0x006, 0x006, 0x006, 0x006, 0x0], // 7
    [0x1FE, 0x186, 0x1FE, 0x186, 0x1FE, 0x0], // 8
    [0x1FE, 0x186, 0x1FE, 0x006, 0x1FE, 0x0], // 9
];

// Colon separator (6 rows)
const COLON: [u16; 6] = [0, 6, 0, 6, 0, 0];

/// Renders a row-wise formatted output of digits
fn render_row(digits: &[u8], row: usize, scale: usize) {
    for _ in 0..scale {
        for &digit in digits {
            let (bitmap, color) = if digit == 10 {
                (&COLON, COLOR_COLON)
            } else {
                (&BLOCK_DIGITS[digit as usize], COLOR_DIGIT)
            };

            let row_value = bitmap[row];
            
            // 4-bit for 1 and colon ; Rest has 10-bit
            let width = if digit == 1 || digit == 10 { 4 } else { 10 };

            for col in (0..width).rev() {
                let pixel = (row_value >> col) & 1;
                let symbol = if pixel == 1 { "█" } else { " " };
                for _ in 0..scale {
                    print!("{}{}{}", color, symbol, COLOR_RESET);
                }
            }
        }
        println!();
    }
}

/// Prints the time in a digital block style
fn print_time(hours: u8, minutes: u8, seconds: u8) {
    let digits = [
        hours / 10,
        hours % 10,
        10, // Colon separator
        minutes / 10,
        minutes % 10,
        10, // Colon separator
        seconds / 10,
        seconds % 10,
    ];

    // Clear screen
    print!("\x1B[H\x1B[J");
    stdout().flush().unwrap(); // Flush output for instant effect

    // Add top padding
    for _ in 0..2 {
        println!();
    }

    // Render each row separately
    for row in 0..6 {
        render_row(&digits, row, 1);
    }
    println!();
}

/// Main function to continuously update the time
fn main() {
    loop {
        let now = Local::now();
        let hours = now.hour() as u8;
        let minutes = now.minute() as u8;
        let seconds = now.second() as u8;

        print_time(hours, minutes, seconds);

        thread::sleep(time::Duration::from_secs(1));
    }
}