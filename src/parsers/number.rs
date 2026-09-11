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
	pub(crate) integer_part: u64,
	pub(crate) decimal_part: DecimalPart,
	pub(crate) is_negative: bool,
	pub(crate) dot_index: Option<i32>,
	pub(crate) start_str_index: usize,
}

#[derive(PartialEq, Debug)]
pub(crate) struct DecimalPart {
	pub value: u64,
	pub digit_count: u64,
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

		let start_index = sr.curr_index - 1;

		if !negative && dot_index.is_none() {
			sr.curr_index -= 1;
		}

		let val = IncJsonNumber {
			integer_part: 0,
			decimal_part: DecimalPart {
				value: 0,
				digit_count: 0,
			},
			is_negative: negative,
			dot_index,
			start_str_index: start_index,
		};

		Some(val.parse(sr))
	}
}

impl<'a> JsonParsable<'a> for IncJsonNumber {
	fn parse(mut self, sr: &mut StringReader) -> JsonValue<'a> {
		let integer_part = &mut self.integer_part;
		let decimal_part = &mut self.decimal_part;
		let dot_index = &mut self.dot_index;

		let mut invalid = false;

		for (index, c) in sr.enumerate() {
			let char = c.char as char;
			trace!(char, index);
			if !JsonNumber::is_valid_char(char) {
				match char {
					'}' | ']' | ',' => {
						sr.curr_index -= 1;
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

				*dot_index = Some(index as i32);
				continue;
			}

			let digit_val = c.char - '0' as u8;

			debug_assert!(digit_val < '9' as u8);

			if let Some(new_dot_index) = dot_index {
				decimal_part.value *= 10;
				decimal_part.value += digit_val as u64;
				decimal_part.digit_count += 1;
				*dot_index = Some(*new_dot_index);
				trace!("after dot", decimal_part);
				continue;
			}

			*integer_part *= 10;
			*integer_part += digit_val as u64;
			trace!("before dot", integer_part);
		}

		if invalid {
			log_warn!("Failed parsing number");
			return JsonValue::Invalid(
				sr.get_string(self.start_str_index..sr.curr_index)
					.unwrap_or_else(|e| e.to_string()),
			);
		}

		self.into()
	}

	fn finish(self) -> JsonValue<'a> {
		trace!("Finish number", self);

		let val = self.integer_part as f64
			+ (self.decimal_part.value as f64 / 10u64.pow(self.decimal_part.digit_count as u32) as f64);

		let neg_multiplier = self.is_negative as i64 as f64 * -2. + 1.;

		JsonNumber(val as f64 * neg_multiplier).into()
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
