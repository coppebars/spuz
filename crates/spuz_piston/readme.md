# spuz_piston <sub><sub>_*by [coppebars](https://github.com/coppebars)*_<sub/><sub/>

Json specification of the minecraft json things, such as version manifests, version lists, jre components

## Supported documents

* Version
  manifest ([example](https://piston-meta.mojang.com/v1/packages/111890b5a8c2fee9b77036f9f377b33df42c718a/1.20.6.json))

  Contains information about the game files and how else to run the game
* Version list ([example](https://piston-meta.mojang.com/mc/game/version_manifest_v2.json))

  Information about versions over time
* Asset Index ([example](https://piston-meta.mojang.com/v1/packages/70b356f90765d4b8da2ae93737d0f384e2343c4a/16.json))

  Game assets files metadata
* Java
  Runtimes ([example](https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json))

  Java runtime components that minecraft runs on
* Java Runtime
  Manifest ([example](https://piston-meta.mojang.com/v1/packages/8adc688f802f47a4e5e8f5c15c459448a8591a23/manifest.json))

  All files you can download to get jre for target component

## Terminology

* **Manifest** - json file required by the game
* **Java Component** - Different versions of Minecraft require different versions of Java, this is expressed by
  specifying the component in the version manifest. It doesn't make sense outside of the context of Minecraft. This is
  an internal value that maps to the required version of jre. Usually the name of the component is: `java-runtime-gamma` or `java-runtime-delta`
* **Rule** - This is a simple condition that configures the version manifest (to launch or download files) for a particular OS version to not download unnecessary files and apply OS-specific optimizations.

## How to launch from any manifest
You can use **spuz_piston** along with [spuz_spawner](https://lib.rs/crates/spuz_spawner) and [spuz_wrench](https://lib.rs/crates/spuz_wrench) to configure and run the game process

## Example

```rust
// Read manifest from filesystem
let manifest_str = fs::read_to_string("./1.20.6.json")?;
let manifest = Manifest::from_str(&manifest_str)?;
```
