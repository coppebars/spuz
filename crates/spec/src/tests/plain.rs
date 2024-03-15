use serde_json::from_str;

use crate::client::latest::ClientVersion;

#[test]
pub fn client_1_20_4() {
	const CLIENT_JSON: &str = include_str!("./jsons/1.20.4.json");
	let ret: ClientVersion = from_str(CLIENT_JSON).unwrap();
	println!("{ret:#?}");
}
