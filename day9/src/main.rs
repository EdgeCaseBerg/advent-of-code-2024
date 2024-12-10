use std::fs;
use std::path;
use std::env;
use std::collections::VecDeque;

fn main() {
    let no_arg = String::from("../sample.txt");
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
    

// what if this... though....
///         let foo = uncompressed_data.iter().find(|i| match i {
        //     I::File { id, blocks: _ } => *id == file_id,
        //     I::Empty { blocks: _ } => false
        // });

    let mut compressed_data_left: Vec<I> = Vec::new();
    let mut compressed_data_right_reverse: Vec<I> = Vec::new();
    let mut right_side_all_empty = false;
    loop {
        println!("ud {:?}", uncompressed_data.clone().into_iter().map(|i| i.to_string()).collect::<Vec<String>>().join(""));
        println!("rd {:?}", compressed_data_right_reverse.clone().into_iter().map(|i| i.to_string()).collect::<Vec<String>>().join(""));
        
        if right_side_all_empty || uncompressed_data.is_empty() {
            break;
        }
        let left = uncompressed_data.pop_front().unwrap();
        match left {
            I::File { id: _, blocks: _ } => {
                compressed_data_left.push(left);
            }
            I::Empty { blocks: _ } => {
                let maybe_right = uncompressed_data.pop_back();
                if maybe_right.is_none() {
                    right_side_all_empty = true;
                    break; // nothing left.
                }

                //FORWARD!
                uncompressed_data.push_back(maybe_right.unwrap());
                loop {
                    match uncompressed_data.pop_back() {
                        Some(right) => {
                            if right.is_empty() {
                                compressed_data_right_reverse.push(right);        
                            } else {
                                uncompressed_data.push_front(right);
                                break;        
                            }
                        }
                        None => {
                            right_side_all_empty = true;
                            break
                        },
                    }
                }

                if right_side_all_empty {
                    break;
                }

                // Guaranteed to exist.
                let right = uncompressed_data.pop_front().unwrap();
                println!("Looking to seat {:?} in left {:?}", right, left);
                
                // We now have the first right side file, find a hole that fits.
                if left.can_fit(&right) {
                    println!("{:?} can fit {:?}", left, right);
                    let remaining_blocks = left.blocks() - right.blocks();
                    compressed_data_right_reverse.push(I::new_empty(right.blocks()));
                    compressed_data_left.push(right);
                    if remaining_blocks > 0 {
                        uncompressed_data.push_front(I::new_empty(remaining_blocks));
                    }
                } else {
                    // advance forward from the left side to see if we can something
                    uncompressed_data.push_front(left);
                    let mut to_restore_after_advancement_done: VecDeque<I> = VecDeque::new();
                    loop {
                        let peek_ahead = uncompressed_data.pop_front();
                        if peek_ahead.is_none() {
                            println!("restoring {:?} with {:?}", uncompressed_data,  to_restore_after_advancement_done.clone().into_iter().map(|i| i.to_string()).collect::<Vec<String>>().join(""));
                            compressed_data_right_reverse.push(right); // RIGHT CANNOT FIT                            
                            for restore in to_restore_after_advancement_done {
                                uncompressed_data.push_back(restore);
                            }
                            break;
                        }
                        // left segment to consider
                        let next_left = peek_ahead.unwrap();
                        if next_left.can_fit(&right) {
                            println!("{:?} {:?}", next_left.blocks(), right.blocks());
                            let remaining_blocks = next_left.blocks() - right.blocks();
                            compressed_data_right_reverse.push(I::new_empty(right.blocks()));
                            compressed_data_left.push(right);
                            if remaining_blocks > 0 {
                                to_restore_after_advancement_done.push_front(I::new_empty(remaining_blocks));
                            }
                            break;
                        } else {
                            to_restore_after_advancement_done.push_back(next_left);
                        }
                    }
                    
                }
            }
        }
        println!("cd {:?}", compressed_data_left.clone().into_iter().map(|i| i.to_string()).collect::<Vec<String>>().join(""));
    }
    for backside in compressed_data_right_reverse.into_iter().rev() {
        compressed_data_left.push(backside.clone());
    }
    println!("{:?}", compressed_data_left.clone().into_iter().map(|i| i.to_string()).collect::<Vec<String>>().join(""));

    let mut idx = 0;
    let mut check_sum = 0;
    for segment in compressed_data_left {
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

    // part 1
    // loop {
    //     if uncompressed_data.is_empty() {
    //         break;
    //     }
    //     let left = uncompressed_data.pop_front().unwrap();
    //     match left {
    //         I::File { id, blocks } => {
    //             for _ in 0..blocks {
    //                 check_sum += id * idx;
    //                 idx += 1;    
    //             }
    //             compressed_data.push(left);
    //         }
    //         I::Empty { blocks } => {
    //             let available_blocks = blocks;
    //             let mut right_most_file = None;
    //             loop {
    //                 if let Some(right) = uncompressed_data.pop_back() {
    //                     if !right.is_empty() {
    //                         right_most_file = Some(right);
    //                         break;
    //                     }
    //                 } else {
    //                     // No more elements in the list.
    //                     break
    //                 }
    //             }
    //             if let Some(right) = right_most_file {
    //                 match right {
    //                     I::Empty { blocks: _ } => {
    //                         println!("not possible");
    //                     } // not  possible due to right.is_empty checks
    //                     I::File { id, blocks } => {
    //                         for _ in 0..available_blocks.min(blocks) {
    //                             check_sum += id * idx;
    //                             idx += 1
    //                         }
    //                         // Bug is here, need to still take more from teh right hand side
    //                         // if there is space available for it. Helper function time?
    //                         if available_blocks < blocks {
    //                             uncompressed_data.push_back(I::new_file(id, blocks - available_blocks));
    //                         } else if blocks < available_blocks {
    //                             uncompressed_data.push_front(I::new_empty(available_blocks - blocks));
    //                         }
    //                         compressed_data.push(I::new_file(id, available_blocks.min(blocks)));
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    // println!("{:?}", compressed_data.clone().into_iter().map(|i| i.to_string()).collect::<Vec<String>>().join(""));
    // println!("{:?}", check_sum);
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