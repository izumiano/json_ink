use std::fmt::Debug;

use logging::*;

use crate::{
	json_reader::JsonReader,
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

impl IncJsonString {
	fn combine_strings(mut self, other: &str) -> Self {
		let str = match self.0 {
			Some(old_str) => old_str + other,
			None => other.to_string(),
		};

		self.0 = Some(str);

		self
	}
}

impl<'a> JsonParsable<'a> for IncJsonString {
	fn parse(self, sr: &mut StringReader) -> JsonValue<'a> {
		let start_index = sr.curr_index;

		let str = if let Some(string_end) = sr.find_quote() {
			let str = sr.get_str(start_index..string_end.index).unwrap();
			self.combine_strings(str).finish()
		} else {
			let str = sr.get_str(start_index..sr.curr_index).unwrap();
			self.combine_strings(str).into()
		};

		trace!("string.parse", str);

		str
	}

	fn finish(self) -> JsonValue<'a> {
		trace!("Finish string", self);
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
