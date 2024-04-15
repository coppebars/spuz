pub mod assets;
pub mod err;
pub mod libs;
pub mod list;
pub mod package;
pub mod platform;
pub mod rule;
pub mod shared;

pub use assets::{AssetIndex, AssetObject};
pub use err::{Error, Result};
pub use libs::{Artifact, PackageName};
pub use package::{Argument, Arguments, AssetIndexResource, Library, LibrarySpecifiers, PistonPackage};
pub use platform::{Arch, Os, TARGET_ARCH, TARGET_OS};
pub use rule::{
	ConditionalValue, Feature, FeatureSet, PlatformRequirement, Rule, RuleAction, RuleCompilance, RuleCondition,
};
pub use shared::{ListOrValue, SemanticVersion, Size, UrlStr, VersionId, VersionStability};

// TODO: tests
