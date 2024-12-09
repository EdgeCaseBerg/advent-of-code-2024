use std::fs;
use std::path;
use std::env;
use std::collections::VecDeque;

fn main() {
    let no_arg = String::from("../input.txt");
    let maybe_file_contents = get_filename_from_args().or(Some(no_arg)).and_then(|name| load_file_to_str(&name));
    println!("{:?}", maybe_file_contents);
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

    println!("{:?}", uncompressed_data.clone().into_iter().map(|i| i.to_string()).collect::<Vec<String>>().join(""));
    
    let mut check_sum = 0; 
    let mut idx = 0;
    let mut compressed_data = Vec::new();
    loop {
        if uncompressed_data.is_empty() {
            break;
        }
        let left = uncompressed_data.pop_front().unwrap();
        match left {
            I::File { id, blocks } => {
                for _ in 0..blocks {
                    check_sum += id * idx;
                    idx += 1;    
                }
                compressed_data.push(left);
            }
            I::Empty { blocks } => {
                let available_blocks = blocks;
                let mut right_most_file = None;
                loop {
                    if let Some(right) = uncompressed_data.pop_back() {
                        if !right.is_empty() {
                            println!("Block to shift {:?}", right);
                            right_most_file = Some(right);
                            break;
                        }
                    } else {
                        // No more elements in the list.
                        break
                    }
                }
                if let Some(right) = right_most_file {
                    match right {
                        I::Empty { blocks: _ } => {
                            println!("not possible");
                        } // not  possible due to right.is_empty checks
                        I::File { id, blocks } => {
                            for _ in 0..available_blocks.min(blocks) {
                                check_sum += id * idx;
                                idx += 1
                            }
                            // Bug is here, need to still take more from teh right hand side
                            // if there is space available for it. Helper function time?
                            if available_blocks < blocks {
                                uncompressed_data.push_back(I::new_file(id, blocks - available_blocks));
                            } else if blocks < available_blocks {
                                uncompressed_data.push_front(I::new_empty(available_blocks - blocks));
                            }
                            compressed_data.push(I::new_file(id, available_blocks.min(blocks)));
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", compressed_data.clone().into_iter().map(|i| i.to_string()).collect::<Vec<String>>().join(""));
    println!("{:?}", check_sum);
}

#[derive(Debug, Clone)]
enum I {
    File { id: u64, blocks: u64 },
    Empty { blocks: u64 },
}

impl I {
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