use chrono::{Local, Timelike};
use std::{
    io::{stdout, Write},
    thread, time,
};

// ANSI color codes
const COLOR_RESET: &str = "\x1B[0m";
const COLOR_DIGIT: &str = "\x1B[1;36m"; // Bright Cyan for numbers
const COLOR_COLON: &str = "\x1B[1;31m"; // Bright Red for colon `:`

const SCALE_FACTOR: i32 = 1;

// Colon blink frequency in milliseconds (500ms = 0.5 seconds)
const COLON_BLINK_FREQ: u128 = 500;

// 6-row representation of numbers
// The rendered digits 1 and colon are thinner
const BLOCK_DIGITS: [[u16; 6]; 10] = [
    [0x1FE, 0x186, 0x186, 0x186, 0x1FE, 0x0], // 0
    [14, 6, 6, 6, 6, 0],                      // 1
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

/* Build a complete frame (as a String) that represents the current time.
The frame is constructed row-by-row into a mutable string (our framebuffer).
 Represent the time digits; we use 10 to indicate a colon. */
fn build_frame(hours: u8, minutes: u8, seconds: u8, blink_state: bool) -> String {
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

    let mut frame = String::new();

    // Clear the screen and add top padding.
    frame.push_str("\x1B[H\x1B[J\n\n");

    // For each of the 6 "pixel" rows:
    for row in 0..6 {
        // (Scale factor can be adjusted; here it's 1 for both rows and columns.)
        for _ in 0..SCALE_FACTOR {
            // Process each digit in the time.
            for &digit in &digits {
                let (bitmap, color, width) = if digit == 10 {
                    // Colon: render only if blink_state is true
                    if blink_state {
                        (&COLON, COLOR_COLON, 4)
                    } else {
                        // Render spaces instead of the colon
                        (&[0; 6], COLOR_RESET, 4)
                    }
                } else {
                    // Digits
                    (&BLOCK_DIGITS[digit as usize], COLOR_DIGIT, if digit == 1 { 4 } else { 10 })
                };

                let row_value = bitmap[row];
                // Iterate over each bit in the row (from MSB to LSB).
                for col in (0..width).rev() {
                    let pixel = (row_value >> col) & 1;
                    let symbol = if pixel == 1 { "█" } else { " " };
                    frame.push_str(color);
                    frame.push_str(symbol);
                    frame.push_str(COLOR_RESET);
                }
            }
            frame.push('\n');
        }
    }
    frame.push('\n');
    frame
}

fn main() {
    let mut blink_state = true;
    let mut last_blink = time::Instant::now();

    loop {
        let now = Local::now();
        let hours = now.hour() as u8;
        let minutes = now.minute() as u8;
        let seconds = now.second() as u8;

        // Toggle blink_state every COLON_BLINK_FREQUENCY milliseconds
        if last_blink.elapsed().as_millis() >= COLON_BLINK_FREQ {
            blink_state = !blink_state;
            last_blink = time::Instant::now();
        }

        // Instead of printing each row immediately, build the full frame first.
        let frame = build_frame(hours, minutes, seconds, blink_state);
        print!("{}", frame);
        stdout().flush().unwrap();

        thread::sleep(time::Duration::from_millis(100)); // Sleep for 100ms to avoid excessive CPU usage
    }
}