use std::{
	collections::HashSet,
	fmt::Formatter,
	ops::{Deref, DerefMut},
};

use serde::{
	de::{MapAccess, Visitor},
	ser::SerializeMap,
	Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{Arch, Os, Str};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Feature {
	#[serde(rename = "is_demo_user")]
	DemoUser,
	#[serde(rename = "has_custom_resolution")]
	CustomResolution,
	#[serde(rename = "has_quick_plays_support")]
	QuickPlays,
	#[serde(rename = "is_quick_play_singleplayer")]
	QuickPlaySinglePlayer,
	#[serde(rename = "is_quick_play_multiplayer")]
	QuickPlayMultiplayer,
	#[serde(rename = "is_quick_play_realms")]
	QuickPlayRealms,
	#[serde(other)]
	Unknown,
}

#[derive(Debug, Clone)]
pub struct FeatureSet(HashSet<Feature>);

impl Deref for FeatureSet {
	type Target = HashSet<Feature>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for FeatureSet {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<'de> Deserialize<'de> for FeatureSet {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct FeaturesMapVisitor;

		impl<'de> Visitor<'de> for FeaturesMapVisitor {
			type Value = FeatureSet;

			fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
				f.write_str("Expected a feature set")
			}

			fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
			where
				A: MapAccess<'de>,
			{
				let mut set = HashSet::<Feature>::with_capacity(map.size_hint().unwrap_or(0));

				while let Some(key) = map.next_key()? {
					set.insert(key);
				}

				Ok(FeatureSet(set))
			}
		}

		deserializer.deserialize_map(FeaturesMapVisitor)
	}
}

impl Serialize for FeatureSet {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut map = serializer.serialize_map(Some(self.0.len()))?;
		for feature in &self.0 {
			map.serialize_entry(&feature, &true)?;
		}
		map.end()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformRequirement {
	pub name: Option<Os>,
	pub arch: Option<Arch>,
	pub version: Option<Str>,
}

impl PlatformRequirement {
	pub fn is_met(&self) -> bool {
		let mut met = true;

		if let Some(os) = &self.name {
			met = met && os.is_target();
		}

		if let Some(arch) = &self.arch {
			met = met && arch.is_target();
		}

		if let Some(_version) = &self.version {
			// TODO: Do os version match
		}

		met
	}
}

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
	#[serde(rename = "os")]
	Platform(PlatformRequirement),
	Features(FeatureSet),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalValue<T> {
	pub rules: Vec<Rule>,
	pub value: T,
}

#[derive(Debug, Clone)]
pub struct RuleCompilance {
	features: HashSet<Feature>,
}

impl RuleCompilance {
	pub fn new(features: HashSet<Feature>) -> Self {
		Self { features }
	}

	pub fn is_met(&self, rule: &Rule) -> bool {
		let compilance = match &rule.condition {
			RuleCondition::Platform(req) => req.is_met(),
			RuleCondition::Features(features) => features.iter().all(|it| self.features.contains(it)),
		};

		match rule.action {
			RuleAction::Allow => compilance,
			RuleAction::Disallow => !compilance,
		}
	}

	pub fn unpack<T>(&self, container: ConditionalValue<T>) -> Option<T> {
		container
			.rules
			.iter()
			.all(|rule| self.is_met(rule))
			.then_some(container.value)
	}
}
