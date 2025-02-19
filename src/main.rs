use std::{thread, time};
use chrono::{Local, Timelike};

const BLOCK_DIGITS: [[&str; 6]; 10] = [
    [ // 0
        " ████████ ",
        " ██    ██ ",
        " ██    ██ ",
        " ██    ██ ",
        " ████████ ",
        "          "
    ],
    [ // 1
        " ██  ",
        " ██  ",
        " ██  ",
        " ██  ",
        " ██  ",
        "     "
    ],
    [ // 2
        " ██████  ",
        "     ██  ",
        " ██████  ",
        " ██      ",
        " ██████  ",
        "         "
    ],
    [ // 3
        " ██████  ",
        "     ██  ",
        "  █████  ",
        "     ██  ",
        " ██████  ",
        "         "
    ],
    [ // 4
        " ██   ██ ",
        " ██   ██ ",
        " ███████ ",
        "      ██ ",
        "      ██ ",
        "         "
    ],
    [ // 5
        " ███████ ",
        " ██      ",
        " ███████ ",
        "      ██ ",
        " ███████ ",
        "         "
    ],
    [ // 6
        " ███████  ",
        " ██       ",
        " ████████ ",
        " ██    ██ ",
        " ████████ ",
        "          "
    ],
    [ // 7
        " ███████ ",
        "      ██ ",
        "      ██ ",
        "      ██ ",
        "      ██ ",
        "         "
    ],
    [ // 8
        " ███████ ",
        " ██   ██ ",
        " ███████ ",
        " ██   ██ ",
        " ███████ ",
        "         "
    ],
    [ // 9
        " ███████ ",
        " ██   ██ ",
        " ███████ ",
        "      ██ ",
        " ███████ ",
        "         "
    ]
];

/// Prints ASCII clock digits in color
fn print_time(hours: u8, minutes: u8, seconds: u8) {
    let digits = [
        hours / 10, hours % 10,
        10, // Colon separator
        minutes / 10, minutes % 10,
        10, // Colon separator
        seconds / 10, seconds % 10,
    ];

    // ANSI color codes
    let color_reset = "\x1B[0m";        // Reset to default color
    let color_digit = "\x1B[1;36m";     // Bright Cyan for numbers
    let color_colon = "\x1B[1;31m";     // Bright Red for colon `:`

    // Colon ASCII art
    let colon = [
        "    ",
        " ██ ",
        "    ",
        " ██ ",
        "    ",
        "    "
    ];

    // Clear screen
    print!("\x1B[2J\x1B[H");

    // Add top padding
    let padding_lines = 5;
    for _ in 0..padding_lines {
        println!();
    }

    // Print each row of the ASCII clock
    for row in 0..6 {
        for &digit in &digits {
            if digit == 10 {
                print!("{}{}{}", color_colon, colon[row], color_reset); // Colored Colon
            } else {
                print!("{}{}{}", color_digit, BLOCK_DIGITS[digit as usize][row], color_reset); // Colored Digits
            }
        }
        println!();
    }
}

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