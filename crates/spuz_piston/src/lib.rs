pub mod assets;
pub mod list;
mod manifest;
pub mod platform;
mod profiles;
pub mod rule;
pub mod shared;
#[cfg(test)]
mod test;

pub use assets::{AssetIndex, AssetObject};
pub use manifest::{Argument, Arguments, Artifact, AssetIndexRef, Library, ListOrValue, Manifest, Specifiers};
pub use platform::{Arch, NativeClassifier, Os, TARGET_ARCH, TARGET_OS};
pub use profiles::LauncherProfiles;
pub use rule::{
	ConditionalValue, Feature, FeatureSet, PlatformRequirement, Rule, RuleAction, RuleCompilance, RuleCondition,
};
pub use shared::{Arr, BoxPath, Size, Str};
