use crate::prelude::*;

pub trait ObjectSerialization {
	fn serialize_obj(&self) -> serde_json::Map<String, Value>;
}