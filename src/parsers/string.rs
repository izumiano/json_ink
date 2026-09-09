use std::fmt::Debug;

use logging::*;

use crate::{
	parsers::{JsonParsable, JsonValue},
	string_reader::{CharWithIndex, StringReader},
};

#[derive(PartialEq)]
pub struct JsonString(pub String);

#[derive(PartialEq, Debug)]
pub struct IncJsonString(pub Option<String>);

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

		let val = IncJsonString(None);

		Some(val.parse(sr))
	}
}

impl<'a> JsonParsable<'a> for IncJsonString {
	fn parse(self, sr: &mut StringReader) -> JsonValue<'a> {
		let start_index = sr.curr_index;

		if let Some(string_end) = sr.find(|c| c.char == '"' as u8) {
			let str = sr.get_str(start_index..string_end.index).unwrap();
			JsonString(str).into()
		} else {
			let str = sr.get_str(start_index..sr.curr_index).unwrap();
			IncJsonString(Some(str)).into()
		}
	}

	fn finish(self) -> JsonValue<'a> {
		trace!("Finish string");
		JsonString(self.0.unwrap()).into()
	}
}

impl<'a> From<JsonString> for JsonValue<'a> {
	fn from(value: JsonString) -> Self {
		JsonValue::String(value)
	}
}

impl<'a> From<IncJsonString> for JsonValue<'a> {
	fn from(value: IncJsonString) -> Self {
		JsonValue::IncString(value)
	}
}
