use std::fmt::Debug;

use logging::*;

use crate::{
	json_reader::JsonReader,
	parsers::{JsonParsable, JsonValue},
	string_reader::{CharWithIndex, StringReader},
};

#[derive(PartialEq, Clone)]
pub struct JsonString(pub String);

#[derive(PartialEq, Debug, Clone)]
pub struct IncJsonString(pub Option<Vec<u8>>);

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

		let val = IncJsonString(None);

		Some(val.parse(sr))
	}
}

impl IncJsonString {
	#[allow(unused)]
	pub fn get_string(&self) -> Option<&str> {
		match &self.0 {
			Some(data) => match str::from_utf8(data) {
				Ok(str) => Some(str),
				Err(err) => Some(str::from_utf8(&data[..err.valid_up_to()]).unwrap()),
			},
			None => None,
		}
	}

	fn combine_strings(mut self, other: &[u8]) -> Self {
		match &mut self.0 {
			Some(old_str) => old_str.extend_from_slice(other),
			None => self.0 = Some(other.to_vec()),
		};

		self
	}
}

impl<'a> JsonParsable<'a> for IncJsonString {
	fn parse(self, sr: &mut StringReader) -> JsonValue<'a> {
		let start_index = sr.curr_index;

		let str = if let Some(string_end) = sr.find_quote() {
			let str = sr.get_range(start_index..string_end.index);
			self.combine_strings(str).finish()
		} else {
			let str = sr.get_range(start_index..sr.curr_index);
			self.combine_strings(str).into()
		};

		trace!("string.parse", str);

		str
	}

	fn finish(self) -> JsonValue<'a> {
		trace!("Finish string", self);

		let data = self.0.unwrap();
		let str = str::from_utf8(&data).unwrap();

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
