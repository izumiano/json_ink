extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;

fn pascal_to_snake(input: &str) -> String {
	let mut snake = String::new();
	for (i, ch) in input.chars().enumerate() {
		if ch.is_uppercase() {
			if i > 0 {
				snake.push('_');
			}
			snake.push(ch.to_ascii_lowercase());
		} else {
			snake.push(ch);
		}
	}
	snake
}

trait StringUtils {
	fn replace_or_prepend<'a>(self, from: &str, to: &str) -> String;
}

impl StringUtils for &str {
	fn replace_or_prepend<'a>(self, from: &str, to: &str) -> String {
		let orig = String::from(self);

		let new = orig.replace(from, to);

		if orig != new {
			return new;
		}

		String::from(to) + "_" + &orig
	}
}

#[proc_macro]
pub fn new_log(input: TokenStream) -> TokenStream {
	let mut input_iter = input.into_iter();

	let arg1 = input_iter.next();

	let trait_name = match arg1 {
		Some(proc_macro::TokenTree::Ident(ident)) => ident,
		None => panic!("Missing first argument"),
		_ => panic!("First argument is wrong type"),
	};

	let trait_str = trait_name.to_string();

	let log_trait_name: Ident = syn::parse_str(&trait_str).expect("Failed parsing trait name");
	let dbg_trait_name: Ident =
		syn::parse_str(&format!("{}", trait_str.replace_or_prepend("Log", "Dbg")))
			.expect("Failed parsing trait name");

	let log_func_ident: Ident = syn::parse_str(&format!("{}", pascal_to_snake(&trait_str)))
		.expect("Failed parsing log function");
	let log_label_func: Ident = syn::parse_str(&format!("{}_label", pascal_to_snake(&trait_str)))
		.expect("Failed parsing log label function");

	let dbg_func_ident: Ident = syn::parse_str(&format!(
		"{}",
		pascal_to_snake(&trait_str).replace_or_prepend("log", "dbg")
	))
	.expect("Failed parsing log function");
	let dbg_label_func: Ident = syn::parse_str(&format!(
		"{}_label",
		pascal_to_snake(&trait_str).replace_or_prepend("log", "dbg")
	))
	.expect("Failed parsing log label function");

	let log_func_impl: Ident = syn::parse_str(&format!("_{}", pascal_to_snake(&trait_str)))
		.expect("Failed parsing log function impl");

	quote!(
		pub trait #dbg_trait_name
		{
			fn #dbg_func_ident(&self) -> &Self;
			fn #dbg_label_func(&self, label: &str) -> &Self;
		}

		impl<T> #dbg_trait_name for T
		where
			T: Debug,
		{
			fn #dbg_func_ident(&self) -> &Self {
				#[cfg(feature = "logging")]
				#log_func_impl(&format!("{:?}", self));
				self
			}
			fn #dbg_label_func(&self, label: &str) -> &Self {
				#[cfg(feature = "logging")]
				#log_func_impl(&_dbg_key_value!(label, self));
				self
			}
		}

		pub trait #log_trait_name
		{
			fn #log_func_ident(&self) -> &Self;
			fn #log_label_func(&self, label: &str) -> &Self;
		}

		impl<T> #log_trait_name for T
		where
			T: Display,
		{
			fn #log_func_ident(&self) -> &Self {
				#[cfg(feature = "logging")]
				#log_func_impl(&format!("{}", self));
				self
			}
			fn #log_label_func(&self, label: &str) -> &Self {
				#[cfg(feature = "logging")]
				#log_func_impl(&_log_key_value!(label, self));
				self
			}
		}
	)
	.into()
}
