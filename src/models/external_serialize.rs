use serde::Serialize;

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

pub trait SerializeExternal {
	type Item: Serialize;
    fn serialize_with_(&self) -> Self::Item;
}
