use std::str::FromStr;

use crate::{runtime::Component, RuntimeComponents, RuntimeManifest};

#[test]
fn parse_runtimes_json() {
	let str = include_str!("all.json");
	let parsed = RuntimeComponents::from_str(str);
	if let Err(err) = &parsed {
		eprintln!("{err:?}");
	}
	assert!(parsed.is_ok());
}

#[test]
fn parse_component() {
	assert_eq!("java-runtime-alpha".parse(), Ok(Component::Alpha));
	assert_eq!("java-runtime-beta".parse(), Ok(Component::Beta));
	assert_eq!("java-runtime-gamma".parse(), Ok(Component::Gamma));
	assert_eq!("java-runtime-gamma-snapshot".parse(), Ok(Component::GammaSnapshot));
	assert_eq!("java-runtime-delta".parse(), Ok(Component::Delta));
	assert_eq!("jre-legacy".parse(), Ok(Component::Legacy));
	assert_eq!("minecraft-java-exe".parse(), Ok(Component::Exe));
}

#[test]
fn parse_manifest() {
	let str = include_str!("manifest.json");
	let parsed = RuntimeManifest::from_str(str);
	if let Err(err) = &parsed {
		eprintln!("{err:?}");
	}
	assert!(parsed.is_ok());
}
