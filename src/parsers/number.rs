use std::fmt::Debug;

use logging::*;

use crate::{
	parsers::JsonValue,
	string_reader::{CharWithIndex, StringReader},
};

#[derive(PartialEq)]
pub struct JsonNumber(pub f64);

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
			trace!("is number");
			trace!("number is negative");
			negative = true;
		} else if char == '.' {
			trace!("is number");
			trace!("first number char was dot");
			dot_index = Some(-1);
		} else if !JsonNumber::is_valid_char(char) {
			return None;
		}

		let mut val = 0.;

		let mut invalid = false;

		if !negative && dot_index.is_none() {
			sr.curr_index -= 1;
		}

		for (index, c) in sr.enumerate() {
			let char = c.char as char;
			trace!(char, index);
			if !JsonNumber::is_valid_char(char) {
				match char {
					'}' | ']' | ',' => sr.curr_index -= 1,
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
			return None;
		}

		Some(JsonValue::Number(JsonNumber(
			val * (negative as i64 as f64 * -2. + 1.),
		)))
	}
}

impl<'a> From<JsonNumber> for JsonValue<'a> {
	fn from(value: JsonNumber) -> Self {
		JsonValue::Number(value)
	}
}
