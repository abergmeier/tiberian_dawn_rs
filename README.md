
# Tiberian Dawn in Rust

This repository includes source code for Command & Conquer. This release provides support to the [Steam Workshop](https://steamcommunity.com/workshop/browse/?appid=2229830) for the game.

## Porting

Most of the game configuration is written in static - depending on the C++ compiler to - in the best way - compile it to e.g. *.rodata*-section of ELF. With Rust the aim is to explicitly define most game configurations at compile-time - thus making it way more likely to get compiled into the **rodata**-section.

### Code style

The main goal of porting is to keep the Rust port comparable to C++ code. This in turn means that ordering of definitions
will mostly be kept. Also symbols directly ported from C++ to Rust will keep their C++ naming.
New code will adhere to Rust code style. This difference should make it easy to distinguish between straight C++ porting and new Rust engine code.

### Compile-time Limitations in Rust

While Rust provides some compile-time support, there are still certain areas in the Rust ecosystem that lack this feature.
Typical examples are `map`, `unwrap`.

For most of these it can be tried to replace this `match` - which supports compile-time evaluation.

https://github.com/rust-lang/rust/issues/67792

### Idiomatic Enums

While Rust has a different enum concept - the goal is to use Rusts enum variants over C++ enum values.
For compatibility most enums must in turn be expressed with `#[repr(u8)]` or similar to still express the values.
`NONE` values for indexes should be represented by wrapping the enum in `Option<>`. `NONE` values only make sense in context of Flags.
`COUNT` values should be expressed by using `EnumCount` crate, which gives the Enum a `COUNT` member value.

## License

This repository and its contents are licensed under the GPL v3 license, with additional terms (from EA) applied. Please see [LICENSE.md](LICENSE.md) for details.
