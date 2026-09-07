use std::fmt::Debug;

use logging::*;

use crate::{
	parsers::{
		array::JsonArray, bool::JsonBool, null::JsonNull, number::JsonNumber, object::JsonObject,
		string::JsonString,
	},
	string_reader::StringReader,
};

pub mod array;
pub mod bool;
pub mod null;
pub mod number;
pub mod object;
pub mod string;

pub struct Json;

impl Json {
	pub fn parse<'a>(strs: &'a [&str]) -> Option<JsonValue<'a>> {
		trace!("parse");
		let mut curr_value: Option<JsonValue<'a>> = None;
		for str in strs {
			let mut sr = StringReader::new(str);
			curr_value = if curr_value.is_none() {
				JsonValue::parse(&mut sr)
			} else {
				curr_value
			};
		}

		println!("\n");

		curr_value
	}
}

pub enum JsonValue<'a> {
	Object(JsonObject<'a>),
	Array(JsonArray<'a>),
	String(JsonString),
	Number(JsonNumber),
	Bool(JsonBool),
	Null(JsonNull),
	Unset,
}

impl<'a> JsonValue<'a> {
	fn parse(sr: &mut StringReader) -> Option<Self> {
		macro_rules! try_start_parse {
			($name:ident, $curr_value:ident, $sr:expr, $first_char:expr) => {
				if let Some(val) = $name::try_start_parse($sr, &$first_char) {
					trace!(val);
					return Some(val);
				}
			};
		}

		sr.skip_whitespace();

		let Some(first_char) = sr.next() else {
			log_warn!("String was empty.");
			return None;
		};

		sr.skip_whitespace();

		try_start_parse!(JsonObject, curr_value, sr, first_char);
		try_start_parse!(JsonArray, curr_value, sr, first_char);
		try_start_parse!(JsonString, curr_value, sr, first_char);
		try_start_parse!(JsonNumber, curr_value, sr, first_char);
		try_start_parse!(JsonBool, curr_value, sr, first_char);
		try_start_parse!(JsonNull, curr_value, sr, first_char);

		None
	}
}

impl<'a> Debug for JsonValue<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Object(arg0) => write!(f, "{:#?}", arg0.0),
			Self::Array(arg0) => write!(f, "{:#?}", arg0.0),
			Self::String(arg0) => write!(f, "{:#?}", arg0.0),
			Self::Number(arg0) => write!(f, "{:#?}", arg0.0),
			Self::Bool(arg0) => write!(f, "{:#?}", arg0.0),
			Self::Null(arg0) => write!(f, "{:#?}", arg0),
			Self::Unset => write!(f, "Unset"),
		}
	}
}
