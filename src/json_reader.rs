#![allow(unused)]

#[derive(Debug)]
pub struct FindQuoteResult {
	pub end_quote: Option<CharWithIndex>,
	pub escape_next: bool,
}

pub trait JsonReader {
	fn goto_safe(&mut self) -> bool;
	fn next_is_separator(&self) -> bool;
	fn find_quote(&mut self, escape_next: bool) -> FindQuoteResult;
}

impl<'a> JsonReader for StringReader<'a> {
	fn goto_safe(&mut self) -> bool {
		self.skip_whitespace();

		while let Some(c) = self.next() {
			let char = c.char as char;

			trace!("goto_safe", c);

			match char {
				'}' | ']' => {
					self.curr_index -= 1;
					return true;
				}
				',' => {
					self.skip_whitespace();
					return true;
				}
				_ => {
					self.skip_whitespace();
				}
			}
		}

		false
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

	fn find_quote(&mut self, escape_next: bool) -> FindQuoteResult {
		if escape_next
			&& let Some(c) = self.peek()
			&& c.char == b'"'
		{
			self.next();
			trace!("found escaped quote");
		}

		let mut quote = None;
		while let Some(c) = self.find(|c| c.char == b'"') {
			if let Some(prev_c) = self.previous()
				&& prev_c.char == b'\\'
			{
				trace!("found escaped quote");
				continue;
			}

			quote = Some(c);
			break;
		}

		trace!("test", quote, self.current());

		let mut escape_next = false;
		if quote.is_none()
			&& let Some(prev_c) = self.current()
			&& prev_c.char == b'\\'
		{
			trace!("found start of escape character");
			escape_next = true;
		}

		FindQuoteResult {
			end_quote: quote,
			escape_next,
		}
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

macro_rules! string_parse {
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
				return JsonValue::Invalid($crate::parsers::invalid::JsonInvalid(
					$sr
						.get_string($sr.curr_index..$sr.bytes.len())
						.unwrap_or_else(|e| e.to_string()),
				));
			}
		}
	}};
}

use logging::trace;
pub(crate) use string_parse;

use crate::string_reader::{CharWithIndex, StringReader};
