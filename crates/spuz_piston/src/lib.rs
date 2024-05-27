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

pub use assets::AssetIndex;
pub use manifest::Manifest;
pub use profiles::LauncherProfiles;
pub use runtime::{RuntimeComponents, RuntimeManifest};
pub(crate) use shared::{Arr, BoxPath, Size, Str};
