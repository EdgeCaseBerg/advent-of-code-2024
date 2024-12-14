use std::fs;
use std::{thread, time};

pub fn main() {
    let raw_data = fs::read_to_string("../anim-input.txt").expect("bad input data");
    let raw_data = raw_data.as_str();
    
    let lines = raw_data.lines();

    let delay_between_frames = time::Duration::from_millis(42);

    let mut buffer = String::new();
    let frame_size = 103; // just delete the last line in the output to make it 
                          // so it doesn't clear the very last frame ever if you 
                          // want the tree to sit on your screen and you printed it last.
    let mut current_frame = 0;
    for line in lines {
        if current_frame == frame_size {
            print!("\x1B[2J\x1B[H");
            print!("{}", buffer);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            thread::sleep(delay_between_frames);
            buffer.clear();
            current_frame = 0;
        }
        buffer.push_str(line);
        buffer.push_str("\n");
        current_frame += 1;
    }
    print!("\x1B[2J\x1B[H");
    print!("{}", buffer);
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    thread::sleep(time::Duration::from_millis(1000));
}