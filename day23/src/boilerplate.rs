use std::{
    env,
    fs,
    collections:: {
        HashMap
    },
    hash::Hash
};

fn get_filename_from_args() -> Option<String> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    if arguments.is_empty() {
        return None;
    }
    let mut arguments = arguments.iter();
    arguments.next().cloned()
}

pub fn get_sample_if_no_input() -> Result<String, std::io::Error> {
    match get_filename_from_args() {
        None => fs::read_to_string("sample.txt"),
        Some(filename) => fs::read_to_string(filename)
    }
}

#[derive(Debug)]
pub struct GenericTrie<T: Eq + Hash + Clone> {
    pub root: GenericTrieNode<T>
}

impl<T: Eq + Hash + Clone> GenericTrie<T> {
    pub fn new() -> GenericTrie<T> {
        GenericTrie {
            root: GenericTrieNode::new()
        }
    }

    pub fn insert(&mut self, list: &[T]) {
        let mut node = &mut self.root;
        for item in list {
            node = node.children.entry(item.clone()).or_default()
        }
        node.ends_a_word = true;
    }

    pub fn count_combos(&self, target: &[T]) -> u64 {
        let len = target.len() + 1;
        let mut combos_for_index = vec![0u64; len];
        combos_for_index[0] = 1;
        for i in 0..target.len() {
            if combos_for_index[i] == 0 {
                continue;
            }

            let mut node = &self.root;
            let mut j = i;
            while j < target.len() {
                if let Some(next) = node.children.get(&target[j]) {
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

        combos_for_index[target.len()]
    }
}

impl<T: Eq + Hash + Clone> Default for GenericTrie<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct GenericTrieNode<T> {
    pub ends_a_word: bool,
    pub children: HashMap<T, GenericTrieNode<T>>
}

impl<T> GenericTrieNode<T> {
    pub fn new() -> Self {
        GenericTrieNode {
            children: HashMap::new(),
            ends_a_word: false,
        }
    }
}

impl<T> Default for GenericTrieNode<T> {
    fn default() -> Self {
        Self::new()
    }
}