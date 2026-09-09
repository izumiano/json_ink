use std::fmt::Debug;

use logging::*;

use crate::{
	parsers::JsonValue,
	string_reader::{CharWithIndex, StringReader},
};

#[derive(PartialEq)]
pub struct JsonBool(pub bool);

impl Debug for JsonBool {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl JsonBool {
	pub fn try_start_parse<'a>(
		sr: &mut StringReader,
		first_char: &CharWithIndex,
	) -> Option<JsonValue<'a>> {
		if first_char.char != 't' as u8 && first_char.char != 'f' as u8 {
			return None;
		}

		trace!("is bool");

		sr.curr_index -= 1;

		match sr.str_compare("true") {
			(true, count) => {
				sr.curr_index += count;
				return Some(JsonValue::Bool(JsonBool(true)));
			}
			(false, _) => {}
		};

		match sr.str_compare("false") {
			(true, count) => {
				sr.curr_index += count;
				return Some(JsonValue::Bool(JsonBool(false)));
			}
			(false, _) => {}
		};

		None
	}
}

impl<'a> From<JsonBool> for JsonValue<'a> {
	fn from(value: JsonBool) -> Self {
		JsonValue::Bool(value)
	}
}
