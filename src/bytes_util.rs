use std::{borrow::{Borrow, Cow}, collections::HashMap};
use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct ByteSerializeOptions {
	bytes: bool,
	utf8: bool,
	size: bool
}

impl ByteSerializeOptions {
	pub fn new(bytes: bool, utf8: bool, size: bool) -> Self {
		ByteSerializeOptions { bytes, utf8, size }
	}
	pub fn all() -> Self {
		ByteSerializeOptions { bytes: true, utf8: true, size: true }
	}
	pub fn nothing() -> Self {
		ByteSerializeOptions { bytes: false, utf8: false, size: false }
	}
	pub fn include_size(&mut self, include: bool) {
		self.size = include;
	}
	pub fn include_bytes(&mut self, include: bool) {
		self.bytes = include;
	}
	pub fn include_utf8(&mut self, include: bool) {
		self.utf8 = include;
	}
}


pub trait Utf8Opt {
	fn to_utf8_string(&self) -> Option<String>;
	fn to_utf8_string_lossy(&self) -> String;
	fn serialize_with_options(&self, options: ByteSerializeOptions) -> HashMap<String, serde_json::Value>;
}

impl Utf8Opt for bytes::Bytes {
	fn to_utf8_string(&self) -> Option<String> {
        let x_body = self.clone();
		let a = x_body.to_vec();
        std::string::String::from_utf8(a).ok()
	}
	fn to_utf8_string_lossy(&self) -> String {
        let x_body = self.clone();
        std::string::String::from_utf8_lossy(x_body.borrow()).to_string()
	}
	fn serialize_with_options(&self, options: ByteSerializeOptions) -> HashMap<String, serde_json::Value> {
		let mut rv = HashMap::new();
		if options.size {
			rv.insert("size".to_string(), json!(self.len()));
		}
		if options.bytes {
			rv.insert("bytes".to_string(), json!(self));
		}
		if options.utf8 {
			rv.insert("body".to_string(), json!(self));
		}

		rv

	}
}