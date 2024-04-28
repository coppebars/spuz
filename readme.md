# Spuz
###### Pronounced as `Spo͞ozh`
###### Tools to manage and launch minecraft programmatically

> [!Warning]
> #### Under Development
> ###### Some parts are still in the design stage and their API will constantly change and contain `todo!()`s or incompleted features with empty or `panic!()` functions.

## Milestones
- `spuz_piston` - Description of a variety of minecraft json specifications
    - [x] Version manifest `1.19+`
    - [x] Asset index
    - [x] Listing
    - [ ] Support for older versions `1.16+`
- `spuz_spawner` - Helpers for spawning java runtime
    - [x] Spawning java
    - [x] Easy way to apply argument changes
    - [x] Crossplatform
- `spuz_wrench` - Layers for `spuz_spawner` to configure java runtime command to launch game from version manifest
    - [x] Support variables `1.16+`
    - [x] Conditional arguments and libraries (depending on target os and arch) `1.19+`
    - [ ] Support for older versions `1.16+`
- `spuz_get` - Concurrent file downloader. Will download minecraft for you.
    - [x] Download files concurrently
    - [x] Progress tracking
    - [x] Support lzma decompression
    - [ ] Verify hash?
    - [ ] Retry on error
- `spuz_folder` - To manage minecraft installations, versions, instances, mods, etc...
    - ###### under design

# Examples
```rust
#[tokio::main]
async fn main() -> Result<()> {
  // Lookup for manifest and load it
  // We will later use `spuz_folder` for this once it is ready
  let manifest = todo!();

  // Use global java
  // Later we will use a local installation with right version
  let mut builder = CommandBuilder::new("java");
  // Apply allocation arguments: -Xms1024m -Xmx4096m
  builder.apply(AllocRange(1024..4096));

  // Setup launcher wrench
  let wrench = LauncherWrench {
    manifest: manifest.deref().clone(),
    libraries_dir: root.join("libraries").into(),
    assets_dir: root.join("assets").into(),
    natives_dir: client_dir.join("natives").into(),
    client_jar: client_dir.join(format!("1.20.4.jar")).into(),
    game_dir: root.join("instances").join("test").into(),
    features: HashSet::from([Feature::CustomResolution]),
  };

  // Apply wrench
  builder.apply(wrench);
  // Apply player settings
  builder.apply(Player::new("LIMPIX31", "268903ca-7946-400a-8984-1fdc0b8baf71"));
  // Window size
  builder.apply(WindowSize::new(1280, 720));
  // Set if you want start in fullscreen mode
  // builder.apply(Fullscreen);

  // Build and spawn process
  let mut process = builder.build().spawn()?;

  // Watch for logs
  while let Some(log) = process.logs.recv().await {
    print!("{log}");
  }

  Ok(())
}
```
