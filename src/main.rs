mod json_reader;
mod parsers;
mod string_reader;

use std::io::{self, Error};

use logging::*;

use crate::parsers::JsonInk;

fn main() -> Result<(), Error> {
	let mut parser = JsonInk::new();

	let stdin = io::stdin();
	loop {
		let mut buffer = String::new();
		stdin.read_line(&mut buffer)?;

		parser.parse_part(&buffer.replace("\r\n", "").replace("\n", ""));

		match parser.get() {
			Some(val) => {
				val.dbg();
			}
			None => {
				"None".dbg();
			}
		}
	}
}
