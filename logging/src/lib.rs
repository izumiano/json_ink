#![allow(unused)]

use logging_proc::new_log;
use std::fmt::{Debug, Display};

macro_rules! _dbg_key_value {
	($str1:expr, $str2:expr) => {
		format!("{}: {:?}", $str1, $str2)
	};
}

macro_rules! _log_key_value {
	($str1:expr, $str2:expr) => {
		format!("{}: {}", $str1, $str2)
	};
}

#[inline(always)]
fn _log(str: &str) {
	#[cfg(feature = "logging")]
	println!("{str}");
}

#[inline(always)]
fn _log_info(str: &str) {
	println!("\x1b[1;37m[INFO]\x1b[0m: {str}");
}

#[inline(always)]
fn _log_debug(str: &str) {
	println!("\x1b[1;90m[DEBUG]\x1b[0;90m: {str}\x1b[0m");
}

#[inline(always)]
fn _trace(str: &str) {
	println!("\x1b[95m[TRACE]\x1b[0m: {str}");
}

#[inline(always)]
fn _log_error(str: &str) {
	println!("\x1b[1;31m[ERROR]\x1b[0m: {str}");
}

#[inline(always)]
fn _log_warn(str: &str) {
	println!("\x1b[1;33m[WARN]\x1b[0m: {str}");
}

#[macro_export]
macro_rules! log_format {
	($str:literal) => {{
		$str
	}};
	($title:literal, $var:expr) => {{
		let message = format!("{}: {:?}", stringify!($var), $var);
		format!("[{}] | {message}", $title)
	}};
	($title:literal, $($var:expr),*) => {{
		let mut message = String::from("");
		$(
			message += &format!("{}: {:?}, ", stringify!($var), $var);
		)*
		format!("[{}] | {message}", $title)
	}};
	($var:ident) => {
		format!("{}: {:?}", stringify!($var), $var)
	};
	($var:expr) => {{
		format!("{}", $var)
	}};
	($($var:expr),*) => {{
		let mut message = String::from("");
		$(
			message += &format!("{}: {:?}, ", stringify!($var), $var);
		)*
		message
	}};
}

#[macro_export]
macro_rules! log {
	($($x:tt)*) => {
		#[cfg(not(debug_assertions))]{{
			compile_error!("log!() can only be used in debug mode")
		}}

		#[allow(unused_imports)]
		use $crate::*;
		log_format!($($x)*).log();
	}
}

#[macro_export]
macro_rules! log_info {
	($($x:tt)*) => {
		#[allow(unused_imports)]
		use $crate::*;
		log_format!($($x)*).log_info();
	}
}

#[macro_export]
macro_rules! log_warn {
	($($x:tt)*) => {
		#[allow(unused_imports)]
		use $crate::*;
		log_format!($($x)*).log_warn();
	}
}

#[macro_export]
macro_rules! log_error {
	($($x:tt)*) => {
		#[allow(unused_imports)]
		use $crate::*;
		log_format!($($x)*).log_error();
	}
}

#[macro_export]
macro_rules! log_debug {
	($($x:tt)*) => {
		#[cfg(debug_assertions)]{{
			#[allow(unused_imports)]
			use $crate::*;
			log_format!($($x)*).log_debug();
		}}
	}
}

#[macro_export]
macro_rules! trace {
	($($x:tt)*) => {
		#[cfg(feature = "trace")]
		{
			{
				#[allow(unused_imports)]
				use $crate::*;
				use std::panic::Location;

				let caller = Location::caller();

				let formatted_log = $crate::log_format!($($x)*);

				format!("\x1b[1;90m[{}:{}:{}]\x1b[0m {}", caller.file(), caller.line(), caller.column(), formatted_log).trace();
			}
		}
	};
}

new_log!(Log);
new_log!(LogInfo);
new_log!(LogDebug);
new_log!(LogError);
new_log!(LogWarn);
new_log!(Trace);
