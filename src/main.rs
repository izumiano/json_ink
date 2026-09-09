mod parsers;
mod string_reader;

use logging::*;

use crate::parsers::Json;

fn main() {
	// let json_str = "{\"hello\": 10}";
	// let json_str = "{";
	// let json_str = "[{\"hello\": 10.5, \"second\": [\"two\", true, false, {\"thing\": null}]}]";

	let json_str = r#"
	false
	"#;
	json_str.log();

	match Json::parse(&[json_str]) {
		Some(val) => {
			val.dbg();
		}
		None => {
			"None".dbg();
		}
	}
}
