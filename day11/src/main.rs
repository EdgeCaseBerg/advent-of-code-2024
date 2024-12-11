use std::fs;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    // let maybe_filename = Some("../sample2.txt"); 
    let maybe_filename = get_filename_from_args();
    if maybe_filename.is_none() {
        return Err("No file provided".into());
    }
    let input: String = fs::read_to_string(maybe_filename.unwrap())?;
    let mut stones: Vec<Stone> = input.split(" ").map(|s| {
        Stone {
            value: s.parse().unwrap()
        }
    }).collect();

    let mut buffer: Vec<Stone> = Vec::new();

    let times_to_blink = 25;
    for _ in 0..times_to_blink {
        for stone in stones.iter() {
            match stone.blink() {
                OnBlink::ZeroToOne => {
                    buffer.push(Stone::one());
                },
                OnBlink::Split => {
                    let (left, right) = stone.split();
                    buffer.push(left);
                    buffer.push(right);
                },
                OnBlink::ReplaceStone => {
                    let multiplied_by_2024 = stone.to_new_stone();
                    buffer.push(multiplied_by_2024);
                }  
            }
        }
        stones.clear();
        stones.append(&mut buffer);
    }

    println!("{:?}", stones.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(" "));
    println!("Total stones: {:?}", stones.len());
    
    Ok(())
}

#[derive(Debug)]
struct Stone {
    value: u64
}

#[derive(Debug)]
enum OnBlink {
    ZeroToOne,
    Split,
    ReplaceStone,
}

impl Stone {
    fn blink(&self) -> OnBlink {
        if self.value == 0 {
            return OnBlink::ZeroToOne
        }

        let number_of_digits_is_even = self.value.to_string().len() % 2 == 0;
        if number_of_digits_is_even {
            return OnBlink::Split
        }

        return OnBlink::ReplaceStone
    }

    fn one() -> Stone {
        Stone { value : 1 }
    }

    fn split(&self) -> (Stone, Stone) {
        let mut string = self.value.to_string();
        let right = string.split_off(string.len() / 2);
        let left = Stone {
            value: string.parse().unwrap()
        };
        let right = Stone {
            value: right.parse().unwrap()
        };
        (left, right)
    }

    fn to_new_stone(&self) -> Stone {
        Stone { value: self.value * 2024 }
    }

    fn to_string(&self) -> String {
        self.value.to_string()
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