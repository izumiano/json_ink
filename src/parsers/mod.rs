use std::fmt::Debug;

use logging::*;

use crate::{
	parsers::{
		array::{IncJsonArray, JsonArray},
		bool::{IncJsonBool, JsonBool},
		null::{IncJsonNull, JsonNull},
		number::{IncJsonNumber, JsonNumber},
		object::{IncJsonObject, JsonObject},
		string::{IncJsonString, JsonString},
	},
	string_reader::StringReader,
};

pub mod array;
pub mod bool;
pub mod null;
pub mod number;
pub mod object;
pub mod string;

pub struct JsonInk<'a>(Option<JsonValue<'a>>);

impl<'a> JsonInk<'a> {
	pub fn new() -> Self {
		Self(None)
	}

	#[allow(unused)]
	pub fn parse(str: &'a str) -> Option<JsonValue<'a>> {
		let mut instance = Self::new();
		instance.parse_part(str);
		instance.0.take()
	}

	pub fn parse_part<'b>(&mut self, str: &'b str) -> &Option<JsonValue<'a>> {
		trace!("parse");
		let mut sr = StringReader::new(str);

		let val = JsonValue::parse(&mut sr, self.0.take());

		self.0 = val;

		&self.0
	}

	pub fn get(&self) -> &Option<JsonValue<'a>> {
		&self.0
	}

	#[allow(unused)]
	pub fn take(self) -> Option<JsonValue<'a>> {
		self.0
	}
}

pub trait JsonParsable<'a> {
	fn parse(self, sr: &mut StringReader) -> JsonValue<'a>;
	fn finish(self) -> JsonValue<'a>;
}

#[derive(PartialEq)]
pub enum JsonValue<'a> {
	IncObject(IncJsonObject<'a>),
	IncArray(IncJsonArray<'a>),
	IncString(IncJsonString),
	IncNumber(IncJsonNumber),
	IncBool(IncJsonBool),
	IncNull(IncJsonNull),

	Object(JsonObject<'a>),
	Array(JsonArray<'a>),
	String(JsonString),
	Number(JsonNumber),
	Bool(JsonBool),
	Null(JsonNull),

	Invalid(String),
}

impl<'a> JsonValue<'a> {
	fn continue_parse(sr: &mut StringReader, value: Self) -> Option<Self> {
		trace!("continue_parse");
		match value {
			JsonValue::IncObject(object) => {
				trace!(" -> object");
				return Some(object.parse(sr));
			}
			JsonValue::IncArray(array) => {
				trace!(" -> array");
				return Some(array.parse(sr));
			}
			JsonValue::IncString(string) => {
				trace!(" -> string");
				return Some(string.parse(sr));
			}
			JsonValue::IncNumber(number) => {
				trace!(" -> number");
				return Some(number.parse(sr));
			}
			JsonValue::IncBool(bool) => {
				trace!(" -> bool");
				return Some(bool.parse(sr));
			}
			JsonValue::IncNull(null) => {
				trace!(" -> null");
				return Some(null.parse(sr));
			}
			_ => {
				trace!(" -> none");
				return Some(value);
			}
		}
	}

	fn parse(sr: &mut StringReader, value: Option<Self>) -> Option<Self> {
		macro_rules! try_start_parse {
			($name:ident, $curr_value:ident, $sr:expr, $first_char:expr) => {
				if let Some(val) = $name::try_start_parse($sr, &$first_char) {
					trace!(val);
					return Some(val);
				}
			};
		}

		if let Some(value) = value
			&& let Some(value) = Self::continue_parse(sr, value)
		{
			return Some(value);
		}

		sr.skip_whitespace();

		let Some(first_char) = sr.next() else {
			trace!("String was empty.");
			return None;
		};

		trace!("normal parse");

		try_start_parse!(JsonObject, curr_value, sr, first_char);
		try_start_parse!(JsonArray, curr_value, sr, first_char);
		try_start_parse!(JsonString, curr_value, sr, first_char);
		try_start_parse!(JsonNumber, curr_value, sr, first_char);
		try_start_parse!(JsonBool, curr_value, sr, first_char);
		try_start_parse!(JsonNull, curr_value, sr, first_char);

		log_warn!("could not parse to any type");

		None
	}
}

impl<'a> Debug for JsonValue<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::IncObject(arg0) => f.debug_tuple("IncObject").field(arg0).finish(),
			Self::IncArray(arg0) => f.debug_tuple("IncArray").field(arg0).finish(),
			Self::IncString(arg0) => f.debug_tuple("IncString").field(arg0).finish(),
			Self::IncNumber(arg0) => f.debug_tuple("IncNumber").field(arg0).finish(),
			Self::IncBool(arg0) => f.debug_tuple("IncBool").field(arg0).finish(),
			Self::IncNull(arg0) => f.debug_tuple("IncNull").field(arg0).finish(),

			Self::Object(arg0) => write!(f, "{:#?}", arg0.0),
			Self::Array(arg0) => write!(f, "{:#?}", arg0.0),
			Self::String(arg0) => write!(f, "{:#?}", arg0.0),
			Self::Number(arg0) => write!(f, "{:#?}", arg0.0),
			Self::Bool(arg0) => write!(f, "{:#?}", arg0.0),
			Self::Null(arg0) => write!(f, "{:#?}", arg0),

			Self::Invalid(arg0) => f.debug_tuple("Invalid").field(arg0).finish(),
		}
	}
}

#[macro_export]
macro_rules! json_parse {
	[$($vals:literal),+ $(,)?] => {{
		let mut parser = JsonInk::new();
		let arr = [$($vals),+];
		for part in arr {
			parser.parse_part(part);

			#[cfg(feature = "logging")]{
				println!("\n----------");

				let current = parser.get();
				log_info!(part, current);

				println!("----------\n");
			}
		}

		parser.take()
	}};
	($val:literal) => {
		JsonInk::parse($val)
	}
}

#[cfg(test)]
mod tests {
	use crate::parsers::{
		number::DecimalPart,
		object::{IncProperty, PropertyKey},
	};

	use super::*;

	#[test]
	fn empty() {
		assert_eq!(JsonInk::parse(r#""#), None);
		assert_eq!(JsonInk::parse(r#"      "#), None);
		assert_eq!(
			JsonInk::parse(
				r#"   				
		
			,}
		
			 "#
			),
			None
		);
	}

	#[test]
	fn object_equal() {
		assert_eq!(
			JsonInk::parse(r#"{}"#),
			Some(JsonObject::new(vec![]).into())
		);

		assert_eq!(
			JsonInk::parse(
				r#"
				{
					"prop": true
				}
			"#
			),
			Some(JsonObject::new(vec![("prop", JsonBool(true).into())]).into())
		);

		assert_eq!(
			JsonInk::parse(
				r#"
				{
					"prop": true,
				}
			"#
			),
			Some(JsonObject::new(vec![("prop", JsonBool(true).into())]).into())
		);

		assert_eq!(
			JsonInk::parse(
				r#"
				{
					"prop": false
				}
			"#
			),
			Some(JsonObject::new(vec![("prop", JsonBool(false).into())]).into())
		);

		assert_eq!(
			JsonInk::parse(
				r#"
				{
					"prop": false,
				}
			"#
			),
			Some(JsonObject::new(vec![("prop", JsonBool(false).into())]).into())
		);

		assert_eq!(
			JsonInk::parse(
				r#"
				{
					"❤️": false,
				}
			"#
			),
			Some(JsonObject::new(vec![("❤️", JsonBool(false).into())]).into())
		);

		assert_eq!(
			JsonInk::parse(
				r#"
				{
					"prop": "❤️",
					"prop2": null
				}
			"#
			),
			Some(
				JsonObject::new(vec![
					("prop", JsonString("❤️".into()).into()),
					("prop2", JsonNull.into())
				])
				.into()
			)
		);
	}

	#[test]
	fn object_not_equal() {
		assert_ne!(
			JsonInk::parse(r#"{"prop": true}"#),
			Some(JsonObject::new(vec![("prop2", JsonBool(true).into())]).into())
		);
	}

	#[test]
	fn array_equal() {
		assert_eq!(
			JsonInk::parse(
				r#"
			[]
			"#
			),
			Some(JsonArray::new(vec![]).into())
		);

		assert_eq!(
			JsonInk::parse(
				r#"
			["hello"]
			"#
			),
			Some(JsonArray::new(vec![JsonValue::String(JsonString("hello".to_string()))]).into())
		);

		assert_eq!(
			JsonInk::parse(
				r#"
			[
				{}
			]
			"#
			),
			Some(JsonArray::new(vec![JsonObject::new(vec![]).into()]).into())
		);

		assert_eq!(
			JsonInk::parse(
				r#"
			[
				{
					"prop": "str",
					"val": 10,
				},
				-5.3,
				-.2,
				.9,
				[
					{
						"val": null
					}
				],
				15.3,
			]
			"#
			),
			Some(
				JsonArray::new(vec![
					JsonObject::new(vec![
						(&"prop", JsonString("str".to_string()).into()),
						(&"val", JsonNumber(10.).into())
					])
					.into(),
					JsonNumber(-5.3).into(),
					JsonNumber(-0.2).into(),
					JsonNumber(0.9).into(),
					JsonArray::new(vec![JsonObject::new(vec![("val", JsonNull.into())]).into()]).into(),
					JsonNumber(15.3).into(),
				])
				.into()
			)
		);

		assert_eq!(
			JsonInk::parse(r#"["❤️"]"#),
			Some(JsonArray::new(vec![JsonString("❤️".into()).into()]).into())
		)
	}

	#[test]
	fn string_equal() {
		assert_eq!(
			JsonInk::parse(r#""str""#),
			Some(JsonString("str".into()).into())
		);

		assert_eq!(
			JsonInk::parse(r#""❤️""#),
			Some(JsonString("❤️".into()).into())
		);
	}

	#[test]
	fn string_not_equal() {
		assert_ne!(
			JsonInk::parse(r#""str""#),
			Some(JsonString("str2".into()).into())
		)
	}

	#[test]
	fn number_equal() {
		assert_eq!(
			JsonInk::parse(r#"10.5"#),
			Some(
				IncJsonNumber {
					integer_part: 10,
					decimal_part: DecimalPart {
						value: 5,
						digit_count: 1
					},
					is_negative: false,
					dot_index: Some(2),
					start_str_index: 0,
				}
				.into()
			)
		);
		assert_eq!(
			JsonInk::parse(r#".5"#),
			Some(
				IncJsonNumber {
					integer_part: 0,
					decimal_part: DecimalPart {
						value: 5,
						digit_count: 1
					},
					is_negative: false,
					dot_index: Some(-1),
					start_str_index: 0,
				}
				.into()
			)
		);
		assert_eq!(JsonInk::parse(r#"10.5,"#), Some(JsonNumber(10.5).into()));
		assert_eq!(JsonInk::parse(r#".5}"#), Some(JsonNumber(0.5).into()));
		assert_eq!(JsonInk::parse(r#"-.5]"#), Some(JsonNumber(-0.5).into()));
		assert_eq!(JsonInk::parse(r#"0.3,"#), Some(JsonNumber(0.3).into()));
		assert_eq!(JsonInk::parse(r#"-0.7}"#), Some(JsonNumber(-0.7).into()));
		assert_eq!(JsonInk::parse(r#"100]"#), Some(JsonNumber(100.).into()));
	}

	#[test]
	fn number_not_equal() {
		assert_ne!(JsonInk::parse(r#"hello"#), Some(JsonNumber(10.5).into()));
		assert_ne!(JsonInk::parse(r#"10g"#), Some(JsonNumber(10.).into()));
	}

	#[test]
	fn number_invalid() {
		assert_eq!(
			JsonInk::parse(r#"10g"#),
			Some(JsonValue::Invalid("10g".into()))
		);
		assert_eq!(
			JsonInk::parse(r#"-a"#),
			Some(JsonValue::Invalid("-a".into()))
		);
		assert_eq!(
			JsonInk::parse(r#".btasf"#),
			Some(JsonValue::Invalid(".b".into()))
		);
		assert_eq!(
			JsonInk::parse(r#"10.52m"#),
			Some(JsonValue::Invalid("10.52m".into()))
		);
	}

	#[test]
	fn bool_equal() {
		assert_eq!(JsonInk::parse(r#"true,"#), Some(JsonBool(true).into()));
		assert_eq!(JsonInk::parse(r#"false,"#), Some(JsonBool(false).into()));
	}

	#[test]
	fn bool_not_equal() {
		assert_ne!(JsonInk::parse(r#"true"#), Some(JsonBool(true).into()));
		assert_ne!(JsonInk::parse(r#"false"#), Some(JsonBool(false).into()));
	}

	#[test]
	fn null_equal() {
		assert_eq!(JsonInk::parse(r#"null,"#), Some(JsonNull.into()));

		assert_eq!(
			JsonInk::parse(r#"null"#),
			Some(JsonValue::IncNull(IncJsonNull(4)))
		);
	}

	macro_rules! assert_split_eq {
		[$($vals:literal),+ $(,)?] => {
			let mut parser = JsonInk::new();
			let arr = [$($vals),+];
			for part in arr {
				parser.parse_part(part);
			}

			assert_eq!(parser.take(), JsonInk::parse(&arr.join("")), "split == single");
		};
		([$($vals:literal),+ $(,)?], None) => {
			let mut parser = JsonInk::new();
			let arr = [$($vals),+];
			for part in arr {
				parser.parse_part(part);
			}

			let joined = &arr.join("");
			let val = parser.take();
			let val2 = JsonInk::parse(joined);
			assert_eq!(val2, None, "normal == intended");
			assert_eq!(val, None, "split == intended");
			assert_eq!(val, val2, "split == normal");
		};
		([$($vals:literal),* $(,)?], $other:expr) => {
			let mut parser = JsonInk::new();
			let arr = [$($vals),*];
			for part in arr {
				parser.parse_part(part);
			}

			let joined = &arr.join("");
			let val = parser.take();
			let val2 = JsonInk::parse(joined);
			assert_eq!(val2, Some($other), "normal == intended");
			assert_eq!(val, Some($other), "split == intended");
			assert_eq!(val, val2, "split == normal");
		};
	}

	#[test]
	fn split_object() {
		assert_split_eq!(
			["{", r#""prop""#, ":", "\t10", ".5,", "}"],
			JsonObject::new(vec![("prop", JsonNumber(10.5).into())]).into()
		);

		assert_split_eq!(
			[r#"{"prop"#, r#"":"#, "[", ".5,", "]}"],
			JsonObject::new(vec![(
				"prop",
				JsonArray::new(vec![JsonNumber(0.5).into()]).into()
			)])
			.into()
		);

		assert_split_eq!(
			[r#"{"prop"#, r#" two"}"#],
			IncJsonObject::new(
				vec![],
				Some(IncProperty {
					key: PropertyKey::Complete("prop two".to_string()),
					value: Box::new(None),
					found_colon: false
				})
			)
			.into()
		);

		assert_split_eq!(
			["{", r#""hello"#, r#"": true}"#],
			JsonObject::new(vec![("hello", JsonBool(true).into())]).into()
		);

		assert_split_eq!(
			[
				r#"{"so"#,
				"me",
				"\"",
				": {",
				r#""arr": [-.33, "str"]"#,
				"}}",
			],
			JsonObject::new(vec![(
				"some",
				JsonObject::new(vec![(
					"arr",
					JsonArray::new(vec![
						JsonNumber(-0.33).into(),
						JsonString("str".into()).into()
					])
					.into()
				)])
				.into()
			)])
			.into()
		);
	}

	#[test]
	fn split_array() {
		assert_split_eq!["[", "10.5,", r#""str"]"#];
		assert_split_eq!(
			["[", r#""he"#, "lllllloooo", r#"oo"]"#],
			JsonArray::new(vec![JsonString("helllllloooooo".to_string()).into()]).into()
		);
	}

	#[test]
	fn split_string() {
		assert_split_eq![r#""  hello "#, " wo", r#"rld  ","#];
	}

	#[test]
	fn split_number() {
		assert_split_eq!["", "-", "10", ".3,"];
	}

	#[test]
	fn split_bool() {
		assert_split_eq!["tr", "ue,"];
	}

	#[test]
	fn split_null() {
		assert_split_eq!["nu", "ll"];
	}
}
