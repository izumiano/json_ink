use std::fmt::Debug;

use logging::*;

use crate::{
	parsers::JsonValue,
	string_reader::{CharWithIndex, StringReader},
};

#[derive(PartialEq)]
pub struct JsonString(pub String);

impl Debug for JsonString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\"{}\"", self.0)
	}
}

impl JsonString {
	pub fn try_start_parse<'a>(
		sr: &mut StringReader,
		first_char: &CharWithIndex,
	) -> Option<JsonValue<'a>> {
		if first_char.char != '"' as u8 {
			return None;
		}

		trace!("is string");

		let Some(string_end) = sr.find(|c| c.char == '"' as u8) else {
			return None;
		};

		return Some(JsonValue::String(JsonString(
			sr.get_str((first_char.index + 1)..(string_end.index))
				.unwrap(),
		)));
	}
}

impl<'a> From<JsonString> for JsonValue<'a> {
	fn from(value: JsonString) -> Self {
		JsonValue::String(value)
	}
}
