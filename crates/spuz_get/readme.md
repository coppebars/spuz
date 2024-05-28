# spuz_get <sub><sub>*by [coppebars](https://github.com/coppebars)*<sub/><sub/>
All you need to install minecraft

# Example
```rust
use reqwest::Client;
use spuz_get::{vanilla::package, Error};
use spuz_piston::Manifest;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let client = Client::default();

  let package =
    package::<Manifest, _>(&client, "d585c8e981e58326237746ca1253dea15c9e4aaa", "24w21b")
    .await
    .map_err(Error::from_fetch)?
    .json()
    .await?;

  println!("{package:#?}");

  Ok(())
}

```
