use reqwest::Client;

#[derive(Debug, Default)]
pub struct Fetch {
	pub client: Client
}

impl Fetch {
	pub fn new(client: Client) -> Self {
		Self { client }
	}
}

impl From<Client> for Fetch {
	fn from(client: Client) -> Self {
		Self { client }
	}
}
