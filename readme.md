![spuz_banner](https://github.com/coppebars/spuz/assets/81296950/0002ebbf-35a5-4878-bdbb-d3b9b3339a85)
###### Tools to manage and launch minecraft programmatically

> [!Warning]
> #### Under Development
> ###### Some parts are still in the design stage and their API will constantly change and contain `todo!()`s or incompleted features with empty or `panic!()` functions.

## Stable crates
* **spuz_piston** - https://crates.io/crates/spuz_piston
* **spuz_spawner** - _unpublished yet_
* **spuz_wrench** - _unpublished yet_

## Milestones
- `spuz_piston` - Description of a variety of minecraft json specifications
    - [x] Version manifest `>1.12`
    - [x] Asset index
    - [x] Listing
    - [x] Launcher profile
    - [x] Java runtimes manifest
    - [ ] Support for older versions `<=1.12`
    - [ ] Docs
- `spuz_spawner` - Helpers for spawning java runtime
    - [x] Spawning java
    - [x] Easy way to apply argument changes
    - [x] Crossplatform
    - [ ] Docs
- `spuz_wrench` - Layers for `spuz_spawner` to configure java runtime command to launch game from version manifest
    - [x] Support variables `>1.12`
    - [x] Conditional arguments and libraries (depending on target os and arch) `1.19+`
    - [x] Friendly and typed builder
    - [ ] Docs
- `spuz_get` - Pack of apis to get any versions of the game, even modded, such as fabric, quilt, forge, etc.
    - [ ] Vanilla
    - [ ] Fabric
    - [ ] Forge
    - [ ] Docs
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
  let wrench = LauncherWrench::builder()
    .manifest(&manifest)
    .current_dir(&root)
    .game_dir(&game_dir)
    .build();

  // Apply wrench
  builder.apply(wrench);
  // Apply player settings
  builder.apply(Player::new("LIMPIX31", "268903ca-7946-400a-8984-1fdc0b8baf71"));
  // Window size
  builder.apply(WindowSize::new(1280, 720));
  // Set if you want start in fullscreen mode
  // builder.apply(Fullscreen);

  // Build and spawn process
  let process = builder.build().spawn()?;

  // Watch for logs
  while let Ok(log) = process.logs.recv().await {
    print!("{log}");
  }

  Ok(())
}
```
