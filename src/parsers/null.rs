use std::fmt::Debug;

use logging::*;

use crate::{
	parsers::JsonValue,
	string_reader::{CharWithIndex, StringReader},
};

pub struct JsonNull;

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

		match sr.str_compare("null") {
			(true, count) => {
				sr.curr_index += count;
				return Some(JsonValue::Null(JsonNull));
			}
			(false, _) => {}
		};

		None
	}
}
