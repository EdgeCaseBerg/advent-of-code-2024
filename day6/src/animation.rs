use std::fs;
use std::{thread, time};

fn main() {
    let raw_data = fs::read_to_string("anim.txt").expect("bad input data");
    let raw_data = raw_data.as_str();
    
    let lines = raw_data.lines();

    let delay_between_frames = time::Duration::from_millis(100);

    let mut buffer = String::new();
    for line in lines {
        if line.is_empty() {
            // std::process::Command::new("clear").status().unwrap();
            print!("\x1B[2J\x1B[H");
            print!("{}", buffer);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            thread::sleep(delay_between_frames);
            buffer.clear();
        }
        buffer.push_str(line);
        buffer.push_str("\n");
    }
    print!("\x1B[2J\x1B[H");
    print!("{}", buffer);
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    thread::sleep(time::Duration::from_millis(1000));
}



