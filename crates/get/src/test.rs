use crate::{vanilla::VanillaFetch, Fetch};

#[tokio::test]
pub async fn test_1() {
	let fetch = Fetch::default();
	let listing = fetch.vanilla_listing().await.unwrap();
	println!("{listing:#?}");
}