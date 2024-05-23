pub mod assets;
pub mod list;
pub mod manifest;
pub mod platform;
pub mod profiles;
pub mod rule;
pub mod runtime;
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
pub use runtime::{RuntimeComponents, RuntimeManifest};
pub(crate) use shared::{Arr, BoxPath, Size, Str};
