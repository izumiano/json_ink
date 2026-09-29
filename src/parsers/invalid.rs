use std::fmt::Debug;

use logging::*;

use crate::{
	json_reader::JsonReader,
	parsers::{JsonParsable, JsonValue},
	string_reader::StringReader,
};

#[derive(PartialEq, Clone)]
pub struct JsonInvalid(pub String);

#[derive(PartialEq, Debug, Clone, Default)]
pub struct IncJsonInvalid(pub String);

impl Debug for JsonInvalid {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl JsonInvalid {
	pub fn start_parse<'a>(sr: &mut StringReader, str: String) -> JsonValue<'a> {
		IncJsonInvalid(str).parse(sr)
	}
}

impl<'a> JsonParsable<'a> for IncJsonInvalid {
	fn parse(mut self, sr: &mut StringReader) -> JsonValue<'a> {
		let start_index = sr.curr_index;

		let found_safe = sr.goto_safe();

		let end_index = sr.curr_index;

		let str = sr.get_str(start_index..end_index);
		let str = match str {
			Ok(str) => str,
			Err(err) => &err.to_string(),
		};

		self.0 += str;

		if found_safe {
			self.finish()
		} else {
			self.into()
		}
	}

	fn finish(self) -> JsonValue<'a> {
		trace!("Finish invalid", self);

		JsonInvalid(self.0).into()
	}
}

impl<'a> From<JsonInvalid> for JsonValue<'a> {
	fn from(value: JsonInvalid) -> Self {
		JsonValue::Invalid(value)
	}
}

impl<'a> From<IncJsonInvalid> for JsonValue<'a> {
	fn from(value: IncJsonInvalid) -> Self {
		JsonValue::IncInvalid(value)
	}
}
