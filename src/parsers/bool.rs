use std::fmt::Debug;

use logging::*;

use crate::{
	json_reader::thing,
	parsers::{JsonParsable, JsonValue},
	string_reader::{CharWithIndex, StringReader},
};

#[derive(PartialEq)]
pub struct JsonBool(pub bool);

#[derive(PartialEq, Debug)]
pub enum IncJsonBool {
	True(usize),
	False(usize),
}

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
		let val = match first_char.char as char {
			't' => IncJsonBool::True(1),
			'f' => IncJsonBool::False(1),
			_ => {
				return None;
			}
		};

		trace!("is bool");

		Some(val.parse(sr))
	}
}

impl<'a> JsonParsable<'a> for IncJsonBool {
	fn parse(self, sr: &mut StringReader) -> JsonValue<'a> {
		match self {
			IncJsonBool::True(orig_count) => {
				// finish_if_complete!(self, sr, "true", orig_count);
				thing!(self, IncJsonBool::True, sr, "true", orig_count)
			}
			IncJsonBool::False(orig_count) => {
				// finish_if_complete!(self, sr, "false", orig_count);
				thing!(self, IncJsonBool::False, sr, "false", orig_count)
			}
		}
	}

	fn finish(self) -> JsonValue<'a> {
		trace!("Finish bool", self);

		match self {
			IncJsonBool::True(_) => JsonBool(true).into(),
			IncJsonBool::False(_) => JsonBool(false).into(),
		}
	}
}

impl<'a> From<JsonBool> for JsonValue<'a> {
	fn from(value: JsonBool) -> Self {
		JsonValue::Bool(value)
	}
}

impl<'a> From<IncJsonBool> for JsonValue<'a> {
	fn from(value: IncJsonBool) -> Self {
		JsonValue::IncBool(value)
	}
}
