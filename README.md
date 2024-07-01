[![Crates.io](https://img.shields.io/crates/v/optional-future)](https://crates.io/crates/optional-future)
[![Documentation](https://docs.rs/optional-future/badge.svg)](https://docs.rs/optional-future)

`optional-future` is a library that allows a future that is only sometimes valid
to be used in a select context. When it is `None` it will always be pending and
thus never resolve, but when it is `Some` then then it will resolve when the
inner future resolves.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
optional-future = "0.1"
```

## License

Licensed under the [MIT License](LICENSE).
