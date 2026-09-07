use std::{
	fmt::{Debug, Display},
	ops::Range,
};

pub struct CharWithIndex {
	pub index: usize,
	pub char: u8,
}

impl Display for CharWithIndex {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{{ [{}] => '{}' }}", self.index, self.char as char)
	}
}

impl Debug for CharWithIndex {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{{ [{}] => '{}' }}", self.index, self.char as char)
	}
}

#[derive(Debug)]
pub struct StringReader<'a> {
	pub bytes: &'a [u8],
	pub curr_index: usize,
}

impl<'a> StringReader<'a> {
	pub fn new(str: &'a str) -> Self {
		Self {
			bytes: str.as_bytes(),
			curr_index: 0,
		}
	}

	pub fn next(&mut self) -> Option<CharWithIndex> {
		let length = self.bytes.len();

		let curr_index = self.curr_index;

		if curr_index >= length {
			return None;
		}

		self.curr_index += 1;

		// trace!(format!("Next = '{}'", self.bytes[curr_index] as char));

		Some(CharWithIndex {
			index: curr_index,
			char: self.bytes[curr_index],
		})
	}

	pub fn peek(&self) -> Option<CharWithIndex> {
		let length = self.bytes.len();

		let curr_index = self.curr_index;

		if curr_index >= length {
			return None;
		}

		Some(CharWithIndex {
			index: curr_index,
			char: self.bytes[curr_index],
		})
	}

	pub fn str_compare(&self, str: &str) -> (bool, usize) {
		let len = self.bytes.len();

		let str = str.as_bytes();
		if self.curr_index + str.len() >= len {
			return (false, 0);
		}

		for (str_index, str_c) in str.iter().enumerate() {
			if self.bytes[self.curr_index + str_index] != *str_c {
				return (false, 0);
			}
		}

		(true, str.len())
	}

	pub fn skip_whitespace(&mut self) {
		while self.curr_index < self.bytes.len()
			&& (self.bytes[self.curr_index] as char).is_whitespace()
		{
			self.curr_index += 1;
		}
	}

	pub fn goto_safe(&mut self) {
		while let Some(c) = self.next() {
			let char = c.char as char;

			self.skip_whitespace();

			match char {
				'}' | ']' => {
					self.curr_index -= 1;
					break;
				}
				',' => {
					break;
				}
				_ => {}
			}
		}
	}

	pub fn get_str(&self, range: Range<usize>) -> Result<String, std::str::Utf8Error> {
		Ok(str::from_utf8(&self.bytes[range])?.to_string())
	}
}

impl<'a> Iterator for StringReader<'a> {
	type Item = CharWithIndex;

	fn next(&mut self) -> Option<Self::Item> {
		self.next()
	}
}
