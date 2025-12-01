use std::fs;
use std::error::Error;
use std::env;
use std::collections::HashMap;
use std::fmt;

fn main() -> Result<(), Box<dyn Error>> {
    // let maybe_filename = Some("../sample2.txt"); 
    let maybe_filename = get_filename_from_args();
    if maybe_filename.is_none() {
        return Err("No file provided".into());
    }
    let input: String = fs::read_to_string(maybe_filename.unwrap())?;
    let stones: Vec<Stone> = input.split(" ").map(|s| {
        Stone {
            value: s.parse().unwrap()
        }
    }).collect();
    println!("{:?}", stones.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(" "));

    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();

    let times_to_blink = 75;
    let mut num_stones = 0;
    for stone in stones.iter() {
        num_stones += stone.count_size_after_blinks(times_to_blink, &mut cache);
    }
    println!("Secondary count: {:?}", num_stones);
    
    Ok(())
}

#[derive(Debug, Clone)]
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

        OnBlink::ReplaceStone
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

    fn count_size_after_blinks(&self, number_of_blinks: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
        let key = (self.value, number_of_blinks);
        if let Some(&cached_result) = cache.get(&key) {
            return cached_result;
        }

        let mut num_stones = 1; // ourselves.
        if number_of_blinks == 0 {
            cache.insert(key, num_stones);
            return num_stones;
        }
        
        match self.blink() {
            OnBlink::ZeroToOne => {
                num_stones += Stone::one().count_size_after_blinks(number_of_blinks - 1, cache) - 1;
            },
            OnBlink::Split => {
                let (left, right) = self.split();
                num_stones += left.count_size_after_blinks(number_of_blinks - 1, cache) - 1; // sub 1 to not double count ourselves.
                num_stones += right.count_size_after_blinks(number_of_blinks - 1, cache);
            },
            OnBlink::ReplaceStone => {
                let multiplied_by_2024 = self.to_new_stone();
                num_stones += multiplied_by_2024.count_size_after_blinks(number_of_blinks - 1, cache) - 1;
            }  
        }

        cache.insert(key, num_stones);
        num_stones
    }
}

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
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