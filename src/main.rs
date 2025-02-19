use chrono::{Local, Timelike};
use std::{thread, time};

const BLOCK_DIGITS: [[&str; 6]; 10] = [
    /*
        Migrating to Unicode bitmap :: Saves memory
        Got to change &str either to u16 or u8
        u64 if bit-packing to be used ( 6 rows x 10 bits per digit)
    */
    [
        // 0
        " ████████ ", // 0111111110 -> 0x1FE
        " ██    ██ ", // 0100000010 -> 0x102
        " ██    ██ ", // 0100000010 -> 0x102
        " ██    ██ ", // 0100000010 -> 0x102
        " ████████ ", // 0111111110 -> 0x1FE
        "          ", // 0000000000 -> 0x000
    ],
    [
        // 1
        " ██  ", // 000011000 -> 0x060
        " ██  ", // 000110000 -> 0x060
        " ██  ", // 000110000 -> 0x060
        " ██  ", // 000110000 -> 0x060
        " ██  ", // 000110000 -> 0x060
        "     ", // 000110000 -> 0x060
    ],
    [
        // 2
        " ██████  ", // 0x1FE
        "     ██  ", // 0x006 <- 0000000110
        " ██████  ", // 0X1FE
        " ██      ", // 0x180 <- 0110000000
        " ██████  ", // 0X1FE
        "         ", // 0x000
    ],
    [
        // 3
        " ██████  ", // 0x1FE
        "     ██  ", // 0X006
        "  █████  ", // 0X07E
        "     ██  ", // 0X006
        " ██████  ", // 0X1FE
        "         ", // 0x000
    ],
    [
        // 4
        " ██   ██ ", // 0x082
        " ██   ██ ", // 0x182
        " ███████ ", // 0x1FE
        "      ██ ", // 0x002
        "      ██ ", // 0x002
        "         ", // 0x000
    ],
    [
        // 5
        " ███████ ", // 0x1FE
        " ██      ", // 0x180
        " ███████ ", // 0x1FE
        "      ██ ", // 0x180
        " ███████ ", // 0x1FE
        "         ", // 0x000
    ],
    [
        // 6
        " ███████  ", // 0x1FE
        " ██       ", // 0x180
        " ████████ ", // 0x1FE
        " ██    ██ ", // 0x182
        " ████████ ", // 0x1FE
        "          ", // 0x000
    ],
    [
        // 7
        " ███████ ", // 0x1FE
        "      ██ ", // 0x006
        "      ██ ", // 0x006
        "      ██ ", // 0x006
        "      ██ ", // 0x006
        "         ", // 0x000
    ],
    [
        // 8
        " ███████ ", // 0x1FE
        " ██   ██ ", // 0x182
        " ███████ ", // 0x1FE
        " ██   ██ ", // 0x182
        " ███████ ", // 0x1FE
        "         ", // 0x000
    ],
    [
        // 9
        " ███████ ", // 0x1FE
        " ██   ██ ", // 0x182
        " ███████ ", // 0x1FE
        "      ██ ", // 0x180
        " ███████ ", // 0x1FE
        "         ", // 0x000
    ],
];

/// Prints ASCII clock digits in color
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

    // ANSI color codes
    let color_reset = "\x1B[0m"; // Reset to default color
    let color_digit = "\x1B[1;36m"; // Bright Cyan for numbers
    let color_colon = "\x1B[1;31m"; // Bright Red for colon `:`

    // Colon ASCII art
    let colon = ["    ", " ██ ", "    ", " ██ ", "    ", "    "];

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
                print!(
                    "{}{}{}",
                    color_digit, BLOCK_DIGITS[digit as usize][row], color_reset
                ); // Colored Digits
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
