# NSFW?

[![crates.io]](https://crates.io/crates/nsfw)
[![Documentation]](https://docs.rs/nsfw)
[![Build Status]](https://github.com/Fyko/nsfw/actions/workflows/test.yml)
[![Discord]](https://discord.gg/RU3FhmX3Ja)

`nsfw` is a library for determining the not-safe-for-work-ness of images.
It is based off of [GantMan's model].

## Prerequisites

Because Cargo has a size limit of 10mb, the pre-trained model cannot be included in the crate.
You will need to download it from [the release page] or download the model yourself and convert it into [ONNX]

### Downloading from [the release page] (easy)

```sh
$ gh release download -R Fyko/nsfw --pattern "model.onnx"
# or naviate to the release page and download it manually
```

### Convering from [GantMan's model]

See our [GitHub Workflow]

## Example: Static Images

```toml
[dependencies]
image = { version = "0.24.7", default-features = false, features = ["jpeg"] }
nsfw = { version = "0.1.0", default-features = false, features = ["jpeg"] }
```

```rust,ignore
use nsfw::{create_model, examine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let model = create_model(
		include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/model.onnx"))
	);

	let image = image::open(concat!(env!("CARGO_MANIFEST_DIR"), "/might_be_porn.jpg"))?;
	let result = examine(&model, &image)?;
	println!("{:#?}", result);

	Ok(())
}
```

## Example: GIF

```toml
[dependencies]
image = { version = "0.24.7", default-features = false, features = ["gif"] }
nsfw = { version = "0.1.0", default-features = false, features = ["gif"] }
```

```rust,ignore
use nsfw::{examine, MODEL, GifParser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let model = create_model(
		include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/model.onnx"))
	);

	let path = concat!(env!("CARGO_MANIFEST_DIR"), "/might_be_porn.gif");
	let file = BufReader::new(File::open(gif_path)?);
	let frames = GifParser::new(GifDecoder::new(file)?, &model);

	for frame in frames {
		println!("{frame:#?}")
	}

	Ok(())
}
```

## Feature Flags

- `default` - `jpeg` and `png`
- `serde` - Enables serialization and deserialization of the model using [`serde`].
- `gif` - Enables GIF support for [`image`].
- `jpeg` - Enables JPEG support for [`image`].
- `png` - Enables PNG support for [`image`].
- `webp` - Enables WEBP support for [`image`].

## Benchmarking

| Name          | Size                          | Language                                | Time             |
| ------------- | ----------------------------- | --------------------------------------- | ---------------- |
| test_porn.gif | `50495726` bytes (`50.49` MB) | [Rust](./src/gif.rs#L61)                | `22.719` seconds |
| test_porn.gif | `50495726` bytes (`50.49` MB) | [JavaScript](./test/node/nsfw_bench.js) | `219.96` seconds |

[crates.io]: https://img.shields.io/crates/v/nsfw.svg
[Documentation]: https://docs.rs/nsfw/badge.svg
[Build Status]: https://github.com/Fyko/nsfw/actions/workflows/ci.yml/badge.svg
[Discord]: https://img.shields.io/discord/1041931589631881257?color=5865F2&logo=discord&logoColor=white
[GantMan's model]: https://github.com/GantMan/nsfw_model
[the release page]: https://github.com/Fyko/nsfw/releases/latest
[ONNX]: https://onnx.ai/
[GitHub Workflow]: https://github.com/Fyko/nsfw/actions/workflows/create_model.yml
[`serde`]: https://crates.pm/serde
[`image`]: https://crates.pm/image
