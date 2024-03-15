use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::{Arch, Os, Str};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
	pub action: RuleAction,
	#[serde(flatten)]
	pub condition: RuleCondition,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleAction {
	Allow,
	Disallow,
}

impl RuleAction {
	pub fn is_allowed(self) -> bool {
		matches!(self, Self::Allow)
	}

	pub fn is_disallowed(self) -> bool {
		!self.is_allowed()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleCondition {
	Os { name: Option<Os>, arch: Option<Arch> },
	Features(HashMap<Str, bool>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalValue<T> {
	pub rules: Vec<Rule>,
	pub value: T,
}
