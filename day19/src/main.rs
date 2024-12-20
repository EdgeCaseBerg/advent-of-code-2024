pub mod boilerplate;

use std::collections::HashMap;

fn main() {
    let raw_data = crate::boilerplate::get_sample_if_no_input();
    if let Err(ref problem) = raw_data {
        println!("Could not read input data: {:?}", problem);
        return;
    }
    let data = raw_data.unwrap();
    part_1(&data);
    part_2(&data);
}

fn part_1(data: &str) {
    let (parsers, request_designs) = get_parsed_data(data);

    let mut valid_designs = 0;
    for design in &request_designs {
        let leftover = design.clone();
        if try_parsers(&parsers, &leftover) {
            valid_designs += 1;
        }
    }
    println!("Valid designs {:?}", valid_designs);
}

fn part_2(data: &str) {
    let (parsers, request_designs) = get_parsed_data(data);

    let mut designs_to_find_arrangements_of = vec![];
    for design in &request_designs {
        if try_parsers(&parsers, &design) {
            designs_to_find_arrangements_of.push(design);
        }
    }
    let mut trie = Trie::new();
    for prefixes in parsers {
        trie.insert(&prefixes.to_word());
    }

    let mut different_ways_to_make_design = 0;
    for design in request_designs {
        let mut s: String = String::new();
        for c in &design {
            s.push(c.to_str());
        }
        different_ways_to_make_design += count_combos(&trie, &s);
    }

    println!("{:?}", different_ways_to_make_design);
}

fn count_combos(prefix_trie: &Trie, for_this_word: &str) -> u64 {
    let len = for_this_word.len() + 1;
    let mut combos_for_index = vec![0u64; len];
    combos_for_index[0] = 1;
    for i in 0..for_this_word.len() {
        if combos_for_index[i] <= 0 {
            continue;
        }

        let mut node = &prefix_trie.root;
        let mut j = i;
        let chars: Vec<char> = for_this_word.chars().collect();

        while j < for_this_word.len() {
            if let Some(next) = node.children.get(&chars[j]) {
                node = next;
                if node.ends_a_word {
                    combos_for_index[j + 1] += combos_for_index[i];
                }
                j += 1;
            } else {   
                break;
            }
        }
    }

    combos_for_index[for_this_word.len()]
}


fn get_parsed_data(data: &str) -> (Vec<Design>, Vec<Vec<TowelStripe>>) {
    let raw_designs = data
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split(", ").collect::<Vec<&str>>())
        .flatten()
        .collect::<Vec<&str>>();
    let parsers = raw_designs.iter().map(|raw| {
        let design_tokens = raw.chars().filter_map(|c| TowelStripe::from(c)).collect::<Vec<TowelStripe>>();
        Design {
            design: design_tokens
        }
    }).collect::<Vec<Design>>();

    let request_designs = data
        .lines()
        .skip_while(|line| !line.is_empty()).skip(1)
        .map(|raw| {
            raw.chars().filter_map(|c| TowelStripe::from(c)).collect::<Vec<TowelStripe>>()
        })
        .collect::<Vec<Vec<TowelStripe>>>();

    (parsers, request_designs)
}

fn try_parsers(parsers: &Vec<Design>, input: &[TowelStripe]) -> bool {
    for parser in parsers {
        if let Some(to_consume) = parser.matches(input) {
            // Fully matched?
            if to_consume == input.len() {
                return true;
            }

            // Deepen the search for this parse
            let eventually_worked = try_parsers(parsers, &input[to_consume..]);
            if eventually_worked {
                return true;
            }
        }
    }
    return false;
}

#[derive(Debug)]
struct TrieNode {
    ends_a_word: bool,
    // we'll use chars for now because I'm not sure exactly if this work if I store my tokens
    // without doing some extra stuff I'm not familiar with yet. 
    children: HashMap<char, TrieNode>
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            ends_a_word: false,
        }
    }
}

#[derive(Debug)]
struct Trie {
    root: TrieNode
}

impl Trie {
    fn new() -> Trie {
        Trie {
            root: TrieNode::new()
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for character in word.chars() {
            node = node.children.entry(character).or_insert(TrieNode::new())
        }
        node.ends_a_word = true;
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]

enum TowelStripe {
    White,
    Blue,
    Black,
    Red,
    Green
}
impl TowelStripe {
    fn from(c: char) -> Option<TowelStripe> {
        match c {
            'w' | 'u' | 'b' | 'r' | 'g' => Some(TowelStripe::of(c)),
            _ => None
        }
    }

    fn of(c: char) -> TowelStripe {
        match c {
            'w' => TowelStripe::White,
            'u' => TowelStripe::Blue,
            'b' => TowelStripe::Black,
            'r' => TowelStripe::Red,
            'g' => TowelStripe::Green,
            _ => panic!("Invalid character given for towel")
        }
    }

    fn to_str(&self) -> char {
        match self {
            TowelStripe::White  => 'w',
            TowelStripe::Blue  => 'u',
            TowelStripe::Black  => 'b',
            TowelStripe::Red  => 'r',
            TowelStripe::Green => 'g',
        }
    }  
}

#[derive(Debug, Clone)]
struct Design {
    design: Vec<TowelStripe>,
}

impl Design {

    fn to_word(&self) -> String {
        let mut s = String::new();
        for towel_stripe in &self.design {
            s.push(towel_stripe.to_str());
        }
        s
    } 

    fn matches(&self, against: &[TowelStripe]) -> Option<usize> {
        let num_tokens = against.len();
        let num_chars = self.design.len();
        let mut chars = self.design.clone().into_iter();
        let mut matching = Vec::new();

        if num_chars > against.len() {
            return None;
        }

        for idx in 0..num_chars.min(num_tokens) {
            match chars.next() {
                None => return None,
                Some(char_to_match) => {
                    if against[idx] == char_to_match {
                        matching.push(char_to_match.clone())
                    } else {
                        return None
                    }
                }
            }
        }
        return Some(matching.len());
    }
}

