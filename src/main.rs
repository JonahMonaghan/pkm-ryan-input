use std::fs::File;
use std::io::{self, BufReader, Read};
use std::thread::sleep;
use std::time::Duration;
use enigo::{Enigo, Keyboard, Key, Direction, Settings};
use std::path::PathBuf;
use dirs::home_dir;

fn print_action_table() {
    println!("+-----------+------------------+");
    println!("| Digit(s)  | Action           |");
    println!("+-----------+------------------+");
    println!("| 0, 7      | Start            |");
    println!("| 1, 8, e   | B                |");
    println!("| 2, 9, f   | A                |");
    println!("| 3, a      | Up               |");
    println!("| 4, b      | Right            |");
    println!("| 5, c      | Down             |");
    println!("| 6, d      | Left             |");
    println!("+-----------+------------------+\n");
}

fn emulate_action(digit: char, enigo: &mut Enigo) {
    match digit {
        '0' | '7' => {
            enigo.key(Key::Return, Direction::Press).unwrap();
            sleep(Duration::from_millis(50));
            enigo.key(Key::Return, Direction::Release).unwrap();
        }
        '1' | '8' | 'e' => {
            enigo.key(Key::Unicode('z'), Direction::Press).unwrap();
            sleep(Duration::from_millis(50));
            enigo.key(Key::Unicode('z'), Direction::Release).unwrap();
        }
        '2' | '9' | 'f' => {
            enigo.key(Key::Unicode('x'), Direction::Press).unwrap();
            sleep(Duration::from_millis(50));
            enigo.key(Key::Unicode('x'), Direction::Release).unwrap();
        }
        '3' | 'a' => {
            enigo.key(Key::UpArrow, Direction::Press).unwrap();
            sleep(Duration::from_millis(200));
            enigo.key(Key::UpArrow, Direction::Release).unwrap();
        }
        '4' | 'b' => {
            enigo.key(Key::RightArrow, Direction::Press).unwrap();
            sleep(Duration::from_millis(200));
            enigo.key(Key::RightArrow, Direction::Release).unwrap();
        }
        '5' | 'c' => {
            enigo.key(Key::DownArrow, Direction::Press).unwrap();
            sleep(Duration::from_millis(200));
            enigo.key(Key::DownArrow, Direction::Release).unwrap();
        }
        '6' | 'd' => {
            enigo.key(Key::LeftArrow, Direction::Press).unwrap();
            sleep(Duration::from_millis(200));
            enigo.key(Key::LeftArrow, Direction::Release).unwrap();
        }
        _ => print!(""),
    }
    sleep(Duration::from_millis(1000)); // Delay after each action
}

fn display_digits_with_arrow(digits: &[char], current_index: usize, digit_count: &i64) {
    // Move the cursor below the table to display digits
    print!("\x1B[15;1H"); // Move cursor to row 15, column 1 (adjust if needed)

    let start = current_index.saturating_sub(5);
    let end = (current_index + 6).min(digits.len());
    let display_chunk: Vec<_> = digits[start..end].iter().collect();

    // Clear the current line, then print the digit view
    print!("\x1B[K"); // Clear the line
    for &digit in &display_chunk {
        print!("{}", digit);
    }

    // Move to the next line and print the arrow under the current digit
    print!("\n\x1B[K"); // Clear the line before printing the arrow
    let arrow_position = (5.min(current_index)).min(display_chunk.len() - 1);
    for _ in 0..arrow_position {
        print!(" ");
    }
    println!("▲");

    // Move to the next line and print the arrow under the current digit
    print!("\n\x1B[K"); // Clear the line before printing the arrow
    println!("Number of Digits: {}", digit_count);

    // Flush output and add a small delay
    std::io::Write::flush(&mut std::io::stdout()).expect("Flush failed");
    sleep(Duration::from_millis(500));
}

fn read_digits_in_fixed_chunks(path: &str, chunk_size: usize, enigo: &mut Enigo) -> io::Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = vec![0; chunk_size]; // Fixed-size buffer
    let mut current_digit_count: i64 = -1;

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of file reached
        }

        let chunk = String::from_utf8_lossy(&buffer[..bytes_read]);
        let digits: Vec<char> = chunk.chars().collect();

        for (i, &digit) in digits.iter().enumerate() {
            current_digit_count += 1;
            display_digits_with_arrow(&digits, i, &current_digit_count); // Show current, previous, and next digits with arrow
            emulate_action(digit, enigo); // Perform action based on the current digit
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    print!("\x1B[2J\x1B[H"); // Clear screen and move cursor to top-left
    print_action_table();

    sleep(Duration::from_millis(10000)); //Startup Delay

    let mut file_path = home_dir().unwrap_or_else(|| PathBuf::from("/"));
    file_path.push("Constant/e-const.txt");
    let chunk_size = 1000; // Adjust chunk size to your needs

    read_digits_in_fixed_chunks(file_path.to_str().unwrap(), chunk_size,  &mut enigo)
}