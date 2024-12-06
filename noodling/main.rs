
fn main() {
	test_it();
}

fn test_it() {
	let mut test_v = Vec::new();
	test_v.push(SingleToken::Meaningless);
	test_v.push(SingleToken::new('d'));
	test_v.push(SingleToken::new('o'));
	test_v.push(SingleToken::new('('));
	test_v.push(SingleToken::new(')'));
	let failed = CompoundToken::matches(test_v.as_slice(), "do()");
	println!("{:?}", failed);
	test_v.remove(0);
	let success = CompoundToken::matches(test_v.as_slice(), "do()");
	println!("{:?}", success);
	test_v.clear();
	test_v.push(SingleToken::new('1'));
	test_v.push(SingleToken::new('2'));
	test_v.push(SingleToken::new('3'));
	let ct = CompoundToken::matches_func_while(test_v.as_slice(), is_digit_radix10);
	match ct {
		CompoundToken::Null => {}
		CompoundToken::Token(t) => println!("{:?}", t),
	}
	test_v.clear();
	test_v.push(SingleToken::new('m'));
	test_v.push(SingleToken::new('u'));
	test_v.push(SingleToken::new('l'));
	test_v.push(SingleToken::new('('));
	test_v.push(SingleToken::new('1'));
	test_v.push(SingleToken::new(','));
	test_v.push(SingleToken::new('2'));
	test_v.push(SingleToken::new('3'));
	test_v.push(SingleToken::new(')'));
	let result = parse_m(test_v.as_slice());
	match result {
		Some(Command::Do) => {},
		Some(Command::Dont) => {},
		Some(Command::Mul(f,s)) => println!("{},{}", f, s),
		None => {},
	}
	println!("{:?}", result);
}

fn is_digit_radix10(c: char) -> bool {
	c.is_digit(10)
}


fn parse_m(tokens: &[SingleToken]) -> Option<Command> {
	CompoundToken::matches(tokens, "mul(").lift().and_then(|_| {
		let tokens = &tokens[4..];
		CompoundToken::matches_func_while(tokens, is_digit_radix10).lift().and_then(|digits| {
			let characters = digits.len();
			if characters <= 0 || characters > 4 {
				return None;
			}
			let first_digits: i32 = digits.iter().collect::<String>().parse().unwrap();
			let tokens = &tokens[characters..];
			CompoundToken::matches(tokens, ",").lift().and_then(|_| {
				let tokens = &tokens[1..];
				CompoundToken::matches_func_while(tokens, is_digit_radix10).lift().and_then(|digits| {
					let characters = digits.len();
					if characters <= 0 || characters > 4 {
						return None;
					}
					let second_digit: i32 = digits.iter().collect::<String>().parse().unwrap();
					let skip = characters as usize;
					let tokens = &tokens[skip..];
					CompoundToken::matches(tokens, ")").lift().and_then(|_| {
						Some(Command::Mul(first_digits, second_digit))
					})
				})
			})
		})
	})
}

#[derive(Debug)]
enum Command {
	Mul(i32, i32),
	Do,
	Dont,
}

#[derive(Debug)]
enum CompoundToken {
	Null,
	Token(Vec<char>)
}

impl CompoundToken {
	fn lift(&self) -> Option<Vec<char>> {
		match self {
			CompoundToken::Null => None,
			CompoundToken::Token(chars) => Some(chars.to_vec())
		}
	}

	fn matches(token: &[SingleToken], string: &str) -> CompoundToken {
		let num_tokens = token.len();
		let num_chars = string.chars().count();
		let mut chars = string.chars();
		let mut matching = Vec::new();
		for idx in 0..num_chars.min(num_tokens) {
			match chars.next() {
				None => return CompoundToken::Null,
				Some(char_to_match) => {
					if token[idx].is_value(&char_to_match) {
						matching.push(char_to_match)
					} else {
						return CompoundToken::Null
					}
				}
			}
		}
		return CompoundToken::Token(matching);
	}

	fn matches_func_while(tokens: &[SingleToken], f: fn(char) -> bool) -> CompoundToken {
		let mut matching = Vec::new();
		for token in tokens.iter() {
			if token.has_meaning() {
				if f(token.value()) {
					matching.push(token.value());	
				} else {
					break;
				}
			} else {
				break;
			}
		}

		if matching.is_empty() {
			CompoundToken::Null
		} else {	
			CompoundToken::Token(matching)
		}
	}
}

#[derive(Debug)]
enum SingleToken {
	Meaningless,
	HasMeaning(char)
}

impl SingleToken {
	fn new(c: char) -> SingleToken {
		match c {
			'm' | 'u' | 'l' | '(' | ',' | ')' | '0'..='9' => SingleToken::HasMeaning(c),
			'd' | 'o' | 'n' | '\''| 't' => SingleToken::HasMeaning(c),
			_ => SingleToken::Meaningless
		}
	}

	fn has_meaning(&self) -> bool {
		match self {
			SingleToken::Meaningless => false,
			_ => true
		}
	}

	fn value(&self) -> char {
		match self {
			SingleToken::HasMeaning(c) => *c,
			SingleToken::Meaningless => '\0'
		}
	}

	fn is_value(&self, value: &char) -> bool {
		match self {
			SingleToken::HasMeaning(c) => c == value,
			_ => false
		}
	}

}
