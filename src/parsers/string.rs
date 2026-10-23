use std::{fmt::Debug, str::Utf8Error};

use logging::*;

use crate::{
	json_reader::JsonReader,
	parsers::{IncString, JsonParsable, JsonValue},
	string_reader::{CharWithIndex, StringReader},
};

#[derive(PartialEq, Clone)]
pub struct JsonString(pub String);

#[derive(PartialEq, Debug, Clone)]
pub struct IncJsonString {
	string: IncString,
	escape_next: bool,
}

impl IncJsonString {
	#[allow(unused)]
	pub fn to_valid_str(&self) -> &str {
		self.string.to_valid_str()
	}

	#[allow(unused)]
	pub fn to_str(&self) -> Result<&str, Utf8Error> {
		self.string.to_str()
	}
}

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

		let val = IncJsonString {
			string: IncString { data: vec![] },
			escape_next: false,
		};

		Some(val.parse(sr))
	}
}

impl<'a> JsonParsable<'a> for IncJsonString {
	fn parse(mut self, sr: &mut StringReader) -> JsonValue<'a> {
		let start_index = sr.curr_index;

		let quote_result = sr.find_quote(self.escape_next);
		trace!("string.parse early", quote_result, self.escape_next);

		let str = if let Some(string_end) = quote_result.end_quote {
			let str = sr.get_range(start_index..string_end.index);
			self.string.combine_strings(&str);
			self.finish()
		} else {
			let str = sr.get_range(start_index..sr.curr_index);
			self.string.combine_strings(&str);
			self.escape_next = quote_result.escape_next;
			self.into()
		};

		trace!("string.parse", str);

		str
	}

	fn finish(self) -> JsonValue<'a> {
		trace!("Finish string", self);

		let str = self.string.to_str().unwrap();

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
