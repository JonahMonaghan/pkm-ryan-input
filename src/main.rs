use std::fs::File;
use std::io::{self, BufReader, Read};
use std::thread::sleep;
use std::time::Duration;
use enigo::{Enigo, Keyboard, Key, Direction, Settings};
use std::path::PathBuf;
use dirs::home_dir;

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
            sleep(Duration::from_millis(50));
            enigo.key(Key::UpArrow, Direction::Release).unwrap();
        }
        '4' | 'b' => {
            enigo.key(Key::RightArrow, Direction::Press).unwrap();
            sleep(Duration::from_millis(50));
            enigo.key(Key::RightArrow, Direction::Release).unwrap();
        }
        '5' | 'c' => {
            enigo.key(Key::DownArrow, Direction::Press).unwrap();
            sleep(Duration::from_millis(50));
            enigo.key(Key::DownArrow, Direction::Release).unwrap();
        }
        '6' | 'd' => {
            enigo.key(Key::LeftArrow, Direction::Press).unwrap();
            sleep(Duration::from_millis(50));
            enigo.key(Key::LeftArrow, Direction::Release).unwrap();
        }
        _ => println!("Action: No action for {}", digit),
    }
    sleep(Duration::from_millis(100)); // Delay after each action
}

fn read_digits_in_fixed_chunks(path: &str, chunk_size: usize, enigo: &mut Enigo) -> io::Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = vec![0; chunk_size]; // Fixed-size buffer
    let delay = Duration::from_secs(5); // 5-second delay

    // Read the file in fixed-size chunks
    loop {
        let bytes_read = reader.read(&mut buffer)?; // Read up to `chunk_size` bytes
        if bytes_read == 0 {
            break; // End of file reached
        }

        // Convert only the bytes read into a string and process the chunk
        let chunk = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Chunk: {}", chunk);
        for digit in chunk.chars(){
            println!("{}", digit);
            emulate_action(digit, enigo);
            sleep(delay); // 1-second delay after each action
        }
        // Optionally, you can process each chunk here instead of printing.
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut file_path = home_dir().unwrap_or_else(|| PathBuf::from("/"));
    file_path.push("Constant/e-const.txt");
    let chunk_size = 1000; // Adjust chunk size to your needs

    read_digits_in_fixed_chunks(file_path.to_str().unwrap(), chunk_size,  &mut enigo)
}