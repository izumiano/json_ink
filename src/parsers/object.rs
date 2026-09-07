use std::{collections::HashMap, fmt::Debug};

use logging::*;

use crate::{
	parsers::JsonValue,
	string_reader::{CharWithIndex, StringReader},
};

// #[derive(Debug)]
pub struct JsonObject<'a>(pub HashMap<String, JsonValue<'a>>);

impl<'a> Debug for JsonObject<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_map().entries(self.0.iter()).finish()
	}
}

impl<'a> JsonObject<'a> {
	pub fn try_start_parse(
		sr: &mut StringReader,
		first_char: &CharWithIndex,
	) -> Option<JsonValue<'a>> {
		if first_char.char != '{' as u8 {
			return None;
		}

		trace!("is object");

		let mut val = JsonObject(HashMap::new());

		val.parse(sr);

		return Some(JsonValue::Object(val));
	}
}

impl<'a> JsonObject<'a> {
	fn parse(&mut self, sr: &mut StringReader) {
		while let Some(c) = sr.peek() {
			if c.char == '}' as u8 {
				trace!("Finish object");
				sr.next();
				return;
			}

			let Some((key, value)) = self.parse_property(sr) else {
				sr.goto_safe(); // TODO
				continue;
			};

			self.0.insert(key, value);
			sr.goto_safe();
		}
	}

	fn parse_property(&self, sr: &mut StringReader) -> Option<(String, JsonValue<'a>)> {
		trace!("parse property");
		sr.skip_whitespace();

		let Some(first) = sr.next() else {
			log_warn!("Empty string");
			return None;
		};

		if first.char != '"' as u8 {
			log_warn!(format!(
				"First character of property was not '\"', but instead '{}'",
				first.char as char
			));
			return None;
		}

		let Some(property_name_end) = sr.find(|c| c.char == '"' as u8) else {
			log_warn!("Failed finding '\"'");
			return None;
		};

		let Some(_) = sr.find(|c| c.char == ':' as u8) else {
			log_warn!("Failed finding ':'");
			return None;
		};

		sr.skip_whitespace();

		let property_name = sr
			.get_str((first.index + 1)..property_name_end.index)
			.unwrap();

		trace!(property_name);

		let property_value = JsonValue::parse(sr);

		Some((property_name, property_value.unwrap_or(JsonValue::Unset)))
	}
}
