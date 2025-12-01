use std::fs;
use std::path;
use std::env;
use std::collections::VecDeque;

fn main() {
    let no_arg = String::from("../input.txt");
    let maybe_file_contents = get_filename_from_args().or(Some(no_arg)).and_then(|name| load_file_to_str(&name));
    if maybe_file_contents.is_none() {
        println!("no data given");
        return;
    }
    let file_contents: Vec<u64> = maybe_file_contents.unwrap().chars().filter_map(|c| {
        match c.to_string().parse::<u64>() {
            Ok(_) => {
                let n: u64 = c.to_string().parse().unwrap();
                Some(n)
            }
            Err(_) => {
                None
            }
        }
    }).collect();
    let mut file_contents = file_contents.iter();
    let mut file_id = 0;
    let mut uncompressed_data: VecDeque<I> = VecDeque::new();
    loop {
        let file_length = file_contents.next();
        if file_length.is_none() {
            break;
        }
        let file_length = file_length.unwrap();
        let free_blocks = file_contents.next().unwrap_or(&0);

        uncompressed_data.push_back(I::new_file(file_id, *file_length));
        uncompressed_data.push_back(I::new_empty(*free_blocks));

        file_id += 1;
    }

    loop {
        let mut compressed_data_right_reverse: Vec<I> = Vec::new();

        file_id -= 1;
        let right_file_position = uncompressed_data.iter().position(|i| match i {
            I::File { id, blocks: _ } => *id == file_id,
            I::Empty { blocks: _ } => false
        }).unwrap();
        
        // Move any empty blocks to the right of the file we're about to move into the reversed result vec
        loop {    
            if right_file_position < uncompressed_data.len() - 1 {
                compressed_data_right_reverse.push(uncompressed_data.pop_back().unwrap())
            } else {
                break;
            }
        }


        let right = uncompressed_data.pop_back().unwrap();

        // Move forward from the left until an empty position can fit this file
        // advance forward from the left side to see if we can something
        let mut to_restore_after_advancement_done: VecDeque<I> = VecDeque::new();
        loop {
            let peek_ahead = uncompressed_data.pop_front();
            if peek_ahead.is_none() {
                compressed_data_right_reverse.push(right); // RIGHT CANNOT FIT
                break;
            }

            // left segment to consider
            let left = peek_ahead.unwrap();
            if left.can_fit(&right) {
                let remaining_blocks = left.blocks() - right.blocks();
                compressed_data_right_reverse.push(I::new_empty(right.blocks()));
                if remaining_blocks > 0 {
                    uncompressed_data.push_front(I::new_empty(remaining_blocks));
                }
                uncompressed_data.push_front(right);
                break;
            } else {
                to_restore_after_advancement_done.push_back(left);
            }
        }
        
        for restore in to_restore_after_advancement_done.into_iter().rev() {
            uncompressed_data.push_front(restore);
        }

        for backside in compressed_data_right_reverse.into_iter().rev() {
            uncompressed_data.push_back(backside.clone());
        }

        if file_id == 1 {
            break;
        }
    }
     
    let mut idx = 0;
    let mut check_sum = 0;
    for segment in uncompressed_data {
        match segment {
            I::Empty { blocks } => {
                for _ in 0..blocks {
                    idx += 1
                }
            }
            I::File { id, blocks } => {
                for _ in 0..blocks {
                    check_sum += id * idx;
                    idx += 1
                }
            }
        }
    }
    println!("{:?}", check_sum);
}

#[derive(Debug, Clone)]
enum I {
    File { id: u64, blocks: u64 },
    Empty { blocks: u64 },
}

impl I {
    fn blocks(&self) -> u64 {
        match self {
            I::Empty { blocks } => *blocks,
            I::File { id:_ , blocks } => *blocks,
        }
    }
    fn is_empty(&self) -> bool {
        match self {
            I::Empty { blocks: _ } => true,
            _ => false
        }
    }
    fn new_empty(blocks: u64) -> I {
        I::Empty {
            blocks
        }
    }
    fn new_file(id: u64, blocks: u64) -> I {
        I::File {
            id, blocks
        }
    }

    fn to_string(&self) -> String {
        match self {
            I::Empty { blocks } => {
                String::from(".").repeat(*blocks as usize)
            }
            I::File { id, blocks } => {
                id.to_string().repeat(*blocks as usize)
            }
        }
    }

    fn can_fit(&self, other: &I) -> bool {
        match self {
            I::Empty { blocks } => {
                if other.is_empty() {
                    return false;
                }
                other.blocks() <= *blocks
            },
            _ => false
        }
    }
}


fn get_filename_from_args() -> Option<String> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.is_empty() {
        println!("No filename passed to file. Defaulting to sample.txt");
        return None;
    }
    let mut arguments = arguments.iter();
    arguments.next(); // skip the name of the program being ran
    arguments.next().cloned()
}

fn load_file_to_str(filename: &String) -> Option<String> {
    if !path::Path::new(filename).exists() {
        println!("File does not exist. {}", filename);
        return None;
    }
    Some(fs::read_to_string(filename).unwrap())
}