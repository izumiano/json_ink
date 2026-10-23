use std::fmt::Debug;

use logging::*;

use crate::{
	json_reader::JsonReader,
	parsers::{IncString, JsonParsable, JsonValue},
	string_reader::{CharWithIndex, StringReader},
};

#[derive(PartialEq, Clone)]
pub struct JsonString(pub String);

#[derive(PartialEq, Debug, Clone)]
pub struct IncJsonString(pub IncString);

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
		if first_char.char != b'"' {
			return None;
		}

		trace!("is string");

		let val = IncJsonString(IncString { data: vec![] });

		Some(val.parse(sr))
	}
}

impl<'a> JsonParsable<'a> for IncJsonString {
	fn parse(mut self, sr: &mut StringReader) -> JsonValue<'a> {
		let start_index = sr.curr_index;

		let str = if let Some(string_end) = sr.find_quote() {
			let str = sr.get_range(start_index..string_end.index);
			self.0.combine_strings(str);
			self.finish()
		} else {
			let str = sr.get_range(start_index..sr.curr_index);
			self.0.combine_strings(str);
			self.into()
		};

		trace!("string.parse", str);

		str
	}

	fn finish(self) -> JsonValue<'a> {
		trace!("Finish string", self);

		let str = self.0.to_string().unwrap();

		JsonString(str.into()).into()
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
