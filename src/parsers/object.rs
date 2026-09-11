use std::fmt::Debug;

use indexmap::IndexMap;
use logging::*;

use crate::{
	parsers::{JsonParsable, JsonValue},
	string_reader::{CharWithIndex, StringReader},
};

#[derive(PartialEq)]
pub struct JsonObject<'a>(pub IndexMap<String, JsonValue<'a>>);

#[derive(PartialEq, Debug)]
pub struct IncJsonObject<'a> {
	map: IndexMap<String, JsonValue<'a>>,
	newest_property: Option<IncProperty<'a>>,
}

#[derive(PartialEq, Debug)]
enum Property<'a> {
	Complete(String, JsonValue<'a>),
	Incomplete(IncProperty<'a>),
}

#[derive(PartialEq, Debug)]
pub(crate) enum PropertyKey {
	Complete(String),
	Incomplete(IncPropertyKey),
}

#[derive(PartialEq, Debug)]
pub(crate) struct IncPropertyKey {
	pub name: String,
	pub quoted: bool,
}

#[derive(PartialEq, Debug)]
pub(crate) struct IncProperty<'a> {
	pub key: PropertyKey,
	pub value: Box<Option<JsonValue<'a>>>,
	pub found_colon: bool,
}

impl<'a> Debug for JsonObject<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_map().entries(self.0.iter()).finish()
	}
}

impl<'a> IncJsonObject<'a> {
	#[allow(unused)]
	pub(crate) fn new(
		vals: Vec<(&str, JsonValue<'a>)>,
		newest_property: Option<IncProperty<'a>>,
	) -> Self {
		let mut map: IndexMap<String, JsonValue<'a>> = IndexMap::new();
		for (key, val) in vals {
			map.insert(key.to_string(), val);
		}

		Self {
			map,
			newest_property,
		}
	}
}

impl<'a> JsonObject<'a> {
	#[allow(unused)]
	pub(crate) fn new(vals: Vec<(&str, JsonValue<'a>)>) -> Self {
		let mut map: IndexMap<String, JsonValue<'a>> = IndexMap::new();
		for (key, val) in vals {
			map.insert(key.to_string(), val);
		}

		Self(map)
	}

	pub fn try_start_parse(
		sr: &mut StringReader,
		first_char: &CharWithIndex,
	) -> Option<JsonValue<'a>> {
		if first_char.char != '{' as u8 {
			return None;
		}

		trace!("is object");

		let val = IncJsonObject {
			map: IndexMap::new(),
			newest_property: None,
		};

		let val = val.parse(sr);

		debug_assert!(matches!(
			val,
			JsonValue::Object(_) | JsonValue::IncObject(_)
		));

		return Some(val);
	}
}

impl<'a> JsonParsable<'a> for IncJsonObject<'a> {
	fn parse(mut self, sr: &mut StringReader) -> JsonValue<'a> {
		while let Some(c) = sr.peek() {
			trace!("object::parse", c);

			if self.newest_property.is_none() && c.char == '}' as u8 {
				sr.next();
				return self.finish();
			}

			let Some(property) = self.parse_property(sr) else {
				continue;
			};

			match property {
				Property::Complete(key, value) => {
					trace!(format!("new property | \"{key}\": {value:#?}"));

					#[cfg(debug_assertions)]
					if matches!(key.chars().nth(0), Some('"' | ',')) {
						panic!("first character in key was ','");
					}

					self.map.insert(key, value);
					sr.skip_whitespace();

					if let Some(c) = sr.peek()
						&& c.char == ',' as u8
					{
						sr.curr_index += 1;
						sr.skip_whitespace();
					}
				}
				Property::Incomplete(prop) => {
					trace!(format!("new inc property | {prop:#?}"));

					#[cfg(debug_assertions)]
					{
						let key = match &prop.key {
							PropertyKey::Complete(name) => name,
							PropertyKey::Incomplete(inc_property_key) => &inc_property_key.name,
						};

						if matches!(key.chars().nth(0), Some('"' | ',')) {
							panic!("first character in key was ','");
						}
					}

					self.newest_property = Some(prop);
				}
			}
		}

		self.into()
	}

	fn finish(self) -> JsonValue<'a> {
		trace!("Finish object", self);

		if let Some(property) = self.newest_property {
			log_warn!("Unfinished property", property);
		}

		JsonObject(self.map).into()
	}
}

impl<'a> IncJsonObject<'a> {
	fn parse_key(
		&self,
		sr: &mut StringReader,
		first: usize,
		first_off: usize,
		quoted: bool,
	) -> Option<(PropertyKey, bool)> {
		let prop_end = if quoted {
			let Some(property_name_end) = sr.find(|c| c.char == '"' as u8) else {
				trace!("Failed finding '\"'");
				if let Ok(name) = sr.get_string((first + first_off)..sr.curr_index) {
					return Some((
						PropertyKey::Incomplete(IncPropertyKey { name, quoted }),
						false,
					));
				}

				return None;
			};

			property_name_end.index
		} else {
			let Some(property_name_end) = sr.find(|c| c.char == ':' as u8) else {
				trace!("Failed finding ':'");
				if let Ok(name) = sr.get_string((first + first_off)..sr.curr_index) {
					return Some((
						PropertyKey::Incomplete(IncPropertyKey { name, quoted }),
						false,
					));
				}

				return None;
			};

			sr.curr_index -= 1;
			property_name_end.index
		};

		let Ok(name) = sr.get_string((first + first_off)..prop_end) else {
			return None;
		};

		if let Some(_) = sr.find(|c| c.char == ':' as u8) {
			return Some((PropertyKey::Complete(name), true));
		};

		trace!("Failed finding ':'");

		Some((PropertyKey::Complete(name), false))
	}

	fn parse_property(&mut self, sr: &mut StringReader) -> Option<Property<'a>> {
		trace!("parse property");

		if self.newest_property.is_none()
			&& let Some(c) = sr.peek()
			&& c.char == ',' as u8
		{
			sr.next();
		}

		let property_key;
		let mut property_value = None;
		let found_colon;
		if let Some(mut property) = self.newest_property.take() {
			trace!("parse_property -> continuation", property);

			match property.key {
				PropertyKey::Incomplete(mut orig_prop) => {
					let start_index = sr.curr_index;

					let Some(key) = self.parse_key(sr, start_index, 0, orig_prop.quoted) else {
						return Some(Property::Incomplete(IncProperty {
							key: PropertyKey::Incomplete(orig_prop),
							value: property.value,
							found_colon: property.found_colon,
						}));
					};

					property_key = match key {
						(PropertyKey::Complete(name), _found_colon) => {
							found_colon = _found_colon;
							PropertyKey::Complete(orig_prop.name + &name)
						}
						(PropertyKey::Incomplete(name), _found_colon) => {
							found_colon = _found_colon;
							orig_prop.name += &name.name;
							PropertyKey::Incomplete(orig_prop)
						}
					};
					property_value = property.value.take();
				}
				PropertyKey::Complete(name) => {
					if !property.found_colon {
						let Some(_) = sr.find(|c| c.char == ':' as u8) else {
							trace!("Failed finding ':'");
							return Some(Property::Incomplete(IncProperty {
								key: PropertyKey::Complete(name),
								value: property.value,
								found_colon: false,
							}));
						};
					}

					property_key = PropertyKey::Complete(name);
					property_value = property.value.take();
					found_colon = true;
				}
			}
		} else {
			trace!("parse_property -> normal");
			sr.skip_whitespace();

			let Some(first) = sr.next() else {
				log_warn!("Empty string");
				return None;
			};

			let quoted = if first.char == '"' as u8 {
				trace!("-> quoted");
				true
			} else {
				trace!("-> unquoted");
				false
			};

			let Some((key, _found_colon)) = self.parse_key(sr, first.index, quoted as usize, quoted)
			else {
				trace!("failed parsing key");
				return None;
			};

			property_key = key;
			found_colon = _found_colon;
		}

		sr.skip_whitespace();

		trace!(property_key, found_colon);

		let property_value = JsonValue::parse(sr, property_value);

		match property_value {
			Some(
				JsonValue::IncArray(_)
				| JsonValue::IncBool(_)
				| JsonValue::IncNull(_)
				| JsonValue::IncNumber(_)
				| JsonValue::IncObject(_)
				| JsonValue::IncString(_),
			)
			| None => Some(Property::Incomplete(IncProperty {
				key: property_key,
				value: Box::new(property_value),
				found_colon,
			})),
			Some(property_value) => Some(Property::Complete(
				match property_key {
					PropertyKey::Complete(name) => name,
					PropertyKey::Incomplete(prop) => prop.name,
				},
				property_value,
			)),
		}
	}
}

impl<'a> From<JsonObject<'a>> for JsonValue<'a> {
	fn from(value: JsonObject<'a>) -> Self {
		JsonValue::Object(value)
	}
}

impl<'a> From<IncJsonObject<'a>> for JsonValue<'a> {
	fn from(value: IncJsonObject<'a>) -> Self {
		JsonValue::IncObject(value)
	}
}
