use std::{
	fmt::{Debug, Display},
	ops::Range,
};

use logging::trace;

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

pub enum StrCompareIsMatch {
	True(usize),
	False,
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

		trace!(format!("Next = '{}'", self.bytes[curr_index] as char));

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

	pub fn previous(&self) -> Option<CharWithIndex> {
		if self.curr_index < 2 || self.curr_index - 2 >= self.bytes.len() {
			return None;
		}

		Some(CharWithIndex {
			index: self.curr_index - 2,
			char: { self.bytes[self.curr_index - 2] },
		})
	}

	pub fn str_compare(&self, str: &str) -> StrCompareIsMatch {
		trace!(format!("str_compare [{}]", str));

		let bytes_len = self.bytes.len();

		let str = str.as_bytes();
		let str_len = str.len().min(bytes_len - self.curr_index);

		for i in 0..str_len {
			if self.bytes[self.curr_index + i] != str[i] {
				trace!("str_compare -> false");
				return StrCompareIsMatch::False;
			}
		}

		trace!(format!("str_compare -> true ({})", str_len));
		StrCompareIsMatch::True(str_len)
	}

	pub fn skip_whitespace(&mut self) {
		while self.curr_index < self.bytes.len()
			&& (self.bytes[self.curr_index] as char).is_whitespace()
		{
			self.curr_index += 1;
		}
	}

	#[allow(unused)]
	pub fn goto_after(&mut self, char: char) {
		while let Some(c) = self.next()
			&& c.char != char as u8
		{}
	}

	pub fn get_str(&self, range: Range<usize>) -> Result<&str, std::str::Utf8Error> {
		Ok(str::from_utf8(&self.bytes[range])?)
	}

	pub fn get_string(&self, range: Range<usize>) -> Result<String, std::str::Utf8Error> {
		Ok(self.get_str(range)?.to_string())
	}
}

impl<'a> Iterator for StringReader<'a> {
	type Item = CharWithIndex;

	fn next(&mut self) -> Option<Self::Item> {
		self.next()
	}
}
