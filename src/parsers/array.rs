use std::{fmt::Debug, marker::PhantomData};

use logging::*;

use crate::{
	parsers::{JsonParsable, JsonValue},
	string_reader::{CharWithIndex, StringReader},
};

#[derive(PartialEq)]
pub struct JsonArray<'a>(pub Vec<JsonValue<'a>>, PhantomData<&'a u8>);

#[derive(PartialEq, Debug)]
pub struct IncJsonArray<'a>(pub Vec<JsonValue<'a>>, PhantomData<&'a u8>);

impl<'a> Debug for JsonArray<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.0.iter()).finish()
	}
}

impl<'a> JsonArray<'a> {
	#[allow(unused)]
	pub fn new(arr: Vec<JsonValue<'a>>) -> Self {
		Self(arr, PhantomData::default())
	}

	pub fn try_start_parse(
		sr: &mut StringReader,
		first_char: &CharWithIndex,
	) -> Option<JsonValue<'a>> {
		if first_char.char != '[' as u8 {
			return None;
		}

		trace!("is array");

		let val = IncJsonArray(vec![], PhantomData::default());

		let val = val.parse(sr);

		return Some(val);
	}

	pub fn take_children(self) -> Vec<JsonValue<'a>> {
		self.0
	}
}

impl<'a> JsonParsable<'a> for IncJsonArray<'a> {
	fn parse(mut self, sr: &mut StringReader) -> JsonValue<'a> {
		loop {
			trace!("array.parse");
			sr.skip_whitespace();
			let Some(mut c) = sr.peek() else {
				trace!("Array unfinished");
				return self.into();
			};

			if self.0.len() > 0 {
				trace!("parse array child");
				let child = self.0.swap_remove(self.0.len() - 1);
				if let Some(new_child) = JsonValue::continue_parse(sr, child) {
					self.0.push(new_child);

					sr.skip_whitespace();

					let Some(_c) = sr.peek() else {
						trace!("Array unfinished");
						return self.into();
					};

					c = _c;
				}
				trace!("after parse array child", c);
			}

			if c.char == ']' as u8 {
				sr.next();
				return self.finish();
			}

			if c.char == ',' as u8 {
				sr.next();
				continue;
			}

			let child = JsonValue::parse(sr, None);

			match child {
				Some(child) => self.0.push(child),
				None => {}
			}
		}
	}

	fn finish(self) -> JsonValue<'a> {
		trace!("Finish array", self.0);

		JsonArray(self.0, self.1).into()
	}
}

impl<'a> From<JsonArray<'a>> for JsonValue<'a> {
	fn from(value: JsonArray<'a>) -> Self {
		JsonValue::Array(value)
	}
}

impl<'a> From<IncJsonArray<'a>> for JsonValue<'a> {
	fn from(value: IncJsonArray<'a>) -> Self {
		JsonValue::IncArray(value)
	}
}
