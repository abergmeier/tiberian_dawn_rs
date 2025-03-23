# Porting

In this document we try to describe how porting is to be done.
A major goal of porting is to keep the resulting Rust port comparable to C++ code.

## Symbols

Rust has some similarity to C++98. For symbols the goal is to either keep the C++ names or add a comment what the previous C++ name was.

## Statics

Most of the game configuration is written in static - depending on the C++ compiler to - in the best way - compile it to e.g. *.rodata*-section of ELF. With Rust the aim is to explicitly define most game configurations at compile-time - thus making it way more likely to get compiled into the **rodata**-section.

## Code style

For all Rust code default Rust fmt is applied to get an uniform code base. Note that this is a tradeoff and violates comparability focus mentioned earlier.
Ordering of definitions will mostly remain the same.
New code will adhere to Rust code style.
This difference should make it easy to distinguish between straight C++ porting and new Rust engine code.

## Compile-time Limitations in Rust

While Rust provides some compile-time support, there are still certain areas in the Rust ecosystem that lack this feature.
Typical examples are `map`, `unwrap`.

For most of these it can be tried to replace this `match` - which supports compile-time evaluation.

https://github.com/rust-lang/rust/issues/67792

## Idiomatic Enums

While Rust has a different enum concept - the goal is to use Rusts enum variants over C++ enum values.
For compatibility most enums must in turn be expressed with `#[repr(u8)]` or similar to still express the values.
`NONE` values for indexes should be represented by wrapping the enum in `Option<>`. `NONE` values only make sense in context of Flags.
`COUNT` values should be expressed by using `EnumCount` crate, which gives the Enum a `COUNT` member value.
