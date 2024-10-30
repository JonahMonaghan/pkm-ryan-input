use std::fs::File;
use std::io::{self, BufReader, Read};
use std::thread::sleep;
use std::time::Duration;
use enigo::{Enigo, Keyboard, Key, Direction, Settings};
use std::path::PathBuf;
use dirs::home_dir;

fn emulate_action(digit: char){
    let mut e = Enigo::new(&Settings::default()).unwrap();
    match digit {
        '0' | '7' => e.key(Key::Return, Direction::Click).unwrap(),
        '1' | '8' | 'e' => e.key(Key::Unicode('z'), Direction::Click).unwrap(),
        '2' | '9' | 'f' => e.key(Key::Unicode('x'), Direction::Click).unwrap(),
        '3' | 'a'=> e.key(Key::UpArrow, Direction::Click).unwrap(),
        '4' | 'b'=> e.key(Key::RightArrow, Direction::Click).unwrap(),
        '5' | 'c'=> e.key(Key::DownArrow, Direction::Click).unwrap(),
        '6' | 'd'=> e.key(Key::LeftArrow, Direction::Click).unwrap(),
        _ => println!("Action: No action for {}", digit),
    }
}

fn read_digits_in_fixed_chunks(path: &str, chunk_size: usize) -> io::Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = vec![0; chunk_size]; // Fixed-size buffer
    let delay = Duration::from_secs(1); // 1-second delay

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
            emulate_action(digit);
            sleep(delay); // 1-second delay after each action
        }
        // Optionally, you can process each chunk here instead of printing.
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut file_path = home_dir().unwrap_or_else(|| PathBuf::from("/"));
    file_path.push("Constant/e-const.txt");
    let chunk_size = 1000; // Adjust chunk size to your needs

    read_digits_in_fixed_chunks(file_path.to_str().unwrap(), chunk_size)
}