use std::fmt::Debug;

use logging::*;

use crate::{
	json_reader::thing,
	parsers::{JsonParsable, JsonValue},
	string_reader::{CharWithIndex, StringReader},
};

#[derive(PartialEq)]
pub struct JsonNull;

#[derive(PartialEq, Debug)]
pub struct IncJsonNull(pub(crate) usize);

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
			return None;
		}

		trace!("is null");

		Some(IncJsonNull(1).parse(sr))
	}
}

impl<'a> JsonParsable<'a> for IncJsonNull {
	fn parse(self, sr: &mut StringReader) -> JsonValue<'a> {
		thing!(self, IncJsonNull, sr, "null", self.0);
	}

	fn finish(self) -> JsonValue<'a> {
		trace!("Finish null", self);

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
