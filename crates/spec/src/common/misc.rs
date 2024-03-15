use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sha(Box<str>);

impl Deref for Sha {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOrValue<T> {
	List(Vec<T>),
	Value(T),
}

impl<T> ListOrValue<T> {
	pub fn is_list(&self) -> bool {
		matches!(self, Self::List(_))
	}
	
	pub fn is_value(&self) -> bool {
		!self.is_list()
	}
}
