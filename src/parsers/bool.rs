use std::fmt::Debug;

use logging::*;

use crate::{
	parsers::{JsonParsable, JsonValue},
	string_reader::{CharWithIndex, StrCompareIsMatch, StringReader},
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
			't' => IncJsonBool::True(0),
			'f' => IncJsonBool::False(0),
			_ => {
				return None;
			}
		};

		trace!("is bool");
		sr.curr_index -= 1;

		Some(val.parse(sr))
	}
}

impl<'a> JsonParsable<'a> for IncJsonBool {
	fn parse(self, sr: &mut StringReader) -> JsonValue<'a> {
		match self {
			IncJsonBool::True(orig_count) => {
				match sr.str_compare("true") {
					StrCompareIsMatch::True(count) => {
						sr.curr_index += count;

						let total_count = orig_count + count;
						let val = IncJsonBool::True(total_count);
						if total_count >= "true".len() {
							return val.finish();
						}

						return val.into();
					}
					StrCompareIsMatch::False => {
						log_warn!("Invalid bool");
						return JsonValue::Unset;
					}
				};
			}
			IncJsonBool::False(orig_count) => {
				match sr.str_compare("false") {
					StrCompareIsMatch::True(count) => {
						sr.curr_index += count;

						let total_count = orig_count + count;
						let val = IncJsonBool::False(total_count);
						if total_count >= "false".len() {
							return val.finish();
						}

						return val.into();
					}
					StrCompareIsMatch::False => {
						log_warn!("Invalid bool");
						return JsonValue::Unset;
					}
				};
			}
		}
	}

	fn finish(self) -> JsonValue<'a> {
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
