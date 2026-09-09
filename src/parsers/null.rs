use std::fmt::Debug;

use logging::*;

use crate::{
	parsers::{JsonParsable, JsonValue},
	string_reader::{CharWithIndex, StrCompareIsMatch, StringReader},
};

#[derive(PartialEq)]
pub struct JsonNull;

#[derive(PartialEq, Debug)]
pub struct IncJsonNull(usize);

impl Debug for JsonNull {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "null")
	}
}

impl JsonNull {
	pub fn try_start_parse<'a>(
		sr: &mut StringReader,
		first_char: &CharWithIndex,
	) -> Option<JsonValue<'a>> {
		if first_char.char != 'n' as u8 {
			trace!("not null", first_char);
			return None;
		}

		trace!("is null");

		sr.curr_index -= 1;

		Some(IncJsonNull(0).parse(sr))
	}
}

impl<'a> JsonParsable<'a> for IncJsonNull {
	fn parse(self, sr: &mut StringReader) -> JsonValue<'a> {
		match sr.str_compare("null") {
			StrCompareIsMatch::True(count) => {
				sr.curr_index += count;

				let total_count = self.0 + count;
				let val = IncJsonNull(total_count);
				if total_count >= "null".len() {
					return val.finish();
				}

				return val.into();
			}
			StrCompareIsMatch::False => todo!(),
		};
	}

	fn finish(self) -> JsonValue<'a> {
		JsonNull.into()
	}
}

impl<'a> From<JsonNull> for JsonValue<'a> {
	fn from(value: JsonNull) -> Self {
		JsonValue::Null(value)
	}
}

impl<'a> From<IncJsonNull> for JsonValue<'a> {
	fn from(value: IncJsonNull) -> Self {
		JsonValue::IncNull(value)
	}
}
