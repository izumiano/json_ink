use std::fmt::Debug;

use logging::*;

use crate::{
	parsers::{JsonParsable, JsonValue},
	string_reader::{CharWithIndex, StringReader},
};

#[derive(PartialEq)]
pub struct JsonNumber(pub f64);

#[derive(PartialEq, Debug)]
pub struct IncJsonNumber {
	pub val: f64,
	pub is_negative: bool,
	pub dot_index: Option<i32>,
}

impl Debug for JsonNumber {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl JsonNumber {
	pub fn is_valid_char(char: char) -> bool {
		((char as u8) >= '0' as u8 && (char as u8) <= '9' as u8) || char == '-' || char == '.'
	}

	pub fn try_start_parse<'a>(
		sr: &mut StringReader,
		first_char: &CharWithIndex,
	) -> Option<JsonValue<'a>> {
		let char = first_char.char as char;

		let mut negative = false;
		let mut dot_index: Option<i32> = None;

		trace!(char);

		if char == '-' {
			trace!("number is negative");
			negative = true;
		} else if char == '.' {
			trace!("first number char was dot");
			dot_index = Some(-1);
		} else if !JsonNumber::is_valid_char(char) {
			return None;
		}

		trace!("is number");

		if !negative && dot_index.is_none() {
			sr.curr_index -= 1;
		}

		let val = IncJsonNumber {
			val: 0.,
			is_negative: negative,
			dot_index,
		};

		Some(val.parse(sr))
	}
}

impl<'a> JsonParsable<'a> for IncJsonNumber {
	fn parse(mut self, sr: &mut StringReader) -> JsonValue<'a> {
		let mut val = self.val;
		let mut dot_index = self.dot_index;

		let mut invalid = false;

		for (index, c) in sr.enumerate() {
			let char = c.char as char;
			trace!(char, index);
			if !JsonNumber::is_valid_char(char) {
				match char {
					'}' | ']' | ',' => {
						sr.curr_index -= 1;
						self.val = val;
						return self.finish();
					}
					_ => invalid = true,
				}
				trace!("invalid digit", char);
				break;
			}

			if char == '-' {
				invalid = true;
				log_warn!("invalid digit", char);
				break;
			}

			if char == '.' {
				if dot_index.is_some() {
					invalid = true;
					log_warn!("invalid digit", char);
					break;
				}

				dot_index = Some(index as i32);
				continue;
			}

			let digit_val = c.char - '0' as u8;

			debug_assert!(digit_val < '9' as u8);

			if let Some(dot_index) = dot_index {
				val += digit_val as f64 / 10f64.powi(index as i32 - dot_index);
				trace!("after dot", val);
				continue;
			}

			val *= 10.;
			val += digit_val as f64;
			trace!("before dot", val);
		}

		if invalid {
			log_warn!("Failed parsing number");
			self.val = f64::NAN;
			return self.finish();
		}

		self.val = val;

		self.into()
	}

	fn finish(self) -> JsonValue<'a> {
		trace!("Finish number", self);

		JsonNumber(self.val * (self.is_negative as i64 as f64 * -2. + 1.)).into()
	}
}

impl<'a> From<JsonNumber> for JsonValue<'a> {
	fn from(value: JsonNumber) -> Self {
		JsonValue::Number(value)
	}
}

impl<'a> From<IncJsonNumber> for JsonValue<'a> {
	fn from(value: IncJsonNumber) -> Self {
		JsonValue::IncNumber(value)
	}
}
