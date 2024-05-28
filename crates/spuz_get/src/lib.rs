pub mod client;
mod err;
pub mod ext;
pub mod json_resource;
#[cfg(feature = "vanilla")]
pub mod vanilla;

pub use crate::{
	client::{Client, FetchError},
	err::Error,
	ext::FsExt,
	json_resource::JsonResource,
};
