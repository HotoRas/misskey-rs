# What is misskey.rs?
`misskey.rs` is a project to *translate* the live version of `misskey.js` into Rust.

This is different from those in Cargo: that is for frontend but this is full-translation.

## I feel dizzy..
Sorry about that. But it's misskey's milestone.

## What's happening?
I decided to seperate the progress into 3 parts:
- Translate types and predefined actions into Rust
- Translate static&streaming API actions into Rust
- Translate others

For now, it's at the first phase. This will take time to be done..

## Difficulties
Since of the natives of Typescript and Rust, these are difficult to solve.

### Typescript can use AND or ORs in Type
Translators should fully understand the TypeAnd and TypeOrs to translate into fully-working code.

### Rust don't know how to have their structs' parents
This results the values be defined one by one for each types.

### Rust don't have Null or Undefined but None
Every Nullables and Undefinedables should become None.

For example, `name?: string | null` will become `pub mut name: Option<String>`.

> Resolving `Some(T)` is `T` but `None` is `panic!`, so it should be careful.

### Typescript allows types be Function template but not for Rust
This comes into issue on API creation. Check https://github.com/HotoRas/misskey.rs/issues/1 for details.
