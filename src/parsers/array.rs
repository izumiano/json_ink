use std::{fmt::Debug, marker::PhantomData};

use logging::*;

use crate::{
	parsers::JsonValue,
	string_reader::{CharWithIndex, StringReader},
};

pub struct JsonArray<'a>(pub Vec<JsonValue<'a>>, PhantomData<&'a u8>);

impl<'a> Debug for JsonArray<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.0.iter()).finish()
	}
}

impl<'a> JsonArray<'a> {
	pub fn try_start_parse(
		sr: &mut StringReader,
		first_char: &CharWithIndex,
	) -> Option<JsonValue<'a>> {
		if first_char.char != '[' as u8 {
			return None;
		}

		trace!("is array");

		let mut array = Vec::new();
		loop {
			sr.skip_whitespace();
			let Some(c) = sr.peek() else {
				break;
			};

			if c.char == ']' as u8 {
				trace!("Finish array");
				sr.next();
				break;
			}

			if c.char == ',' as u8 {
				sr.next();
				continue;
			}

			// let peek = sr.peek();
			// trace!("Parsing for item in array", peek);
			array.push(JsonValue::parse(sr)?);
		}

		return Some(JsonValue::Array(JsonArray(array, PhantomData::default())));
	}
}
