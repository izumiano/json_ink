#![allow(unused)]

pub trait JsonReader {
	fn goto_safe(&mut self);
	fn next_is_separator(&self) -> bool;
	fn find_quote(&mut self) -> Option<CharWithIndex>;
}

impl<'a> JsonReader for StringReader<'a> {
	fn goto_safe(&mut self) {
		self.skip_whitespace();

		while let Some(c) = self.next() {
			let char = c.char as char;

			trace!("goto_safe", c);

			match char {
				'}' | ']' => {
					self.curr_index -= 1;
					break;
				}
				',' => {
					self.skip_whitespace();
					break;
				}
				_ => {
					self.skip_whitespace();
				}
			}
		}
	}

	fn next_is_separator(&self) -> bool {
		trace!("next_is_separator()");
		let Some(c) = self.peek() else {
			trace!("-> false");
			return false;
		};

		let ret = matches!(c.char as char, '}' | ']' | ',' | '\n');

		trace!(format!("-> {ret}"));

		ret
	}

	fn find_quote(&mut self) -> Option<CharWithIndex> {
		let mut quote = None;
		while let Some(c) = self.find(|c| c.char == '"' as u8) {
			if let Some(prev_c) = self.previous() {
				if prev_c.char == '\\' as u8 {
					trace!("found escaped quote");
					continue;
				}
			}

			quote = Some(c);
			break;
		}

		quote
	}
}

macro_rules! finish_if_complete {
	($self:ident, $ret:expr, $sr:ident, $cmp:expr, $count:expr) => {
		trace!(format!("check if {} complete", stringify!($ret)));
		if $count >= $cmp.len() {
			if $sr.next_is_separator() {
				trace!("-> complete");
				return $self.finish();
			}
		}
		trace!("-> incomplete");
	};
}

pub(crate) use finish_if_complete;

macro_rules! thing {
	($self:ident, $ret:expr, $sr:ident, $cmp:expr, $orig_count:expr) => {{
		use $crate::json_reader::*;

		finish_if_complete!($self, $ret, $sr, $cmp, $orig_count);

		match $sr.str_compare(&$cmp[$orig_count..]) {
			crate::string_reader::StrCompareIsMatch::True(count) => {
				$sr.curr_index += count;

				let total_count = $orig_count + count;
				let val = $ret(total_count);
				if total_count >= $cmp.len() {
					finish_if_complete!(val, $ret, $sr, $cmp, total_count);
				}

				return val.into();
			}
			crate::string_reader::StrCompareIsMatch::False => {
				log_warn!(format!("Invalid {}", stringify!($ret)));
				return JsonValue::Invalid(
					$sr
						.get_string($sr.curr_index..$sr.bytes.len())
						.unwrap_or_else(|e| e.to_string()),
				);
			}
		}
	}};
}

use logging::trace;
pub(crate) use thing;

use crate::string_reader::{CharWithIndex, StringReader};
