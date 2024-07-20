# misskey.rs

A trial of [Misskey](https://misskey-hub.net) port into Rust

> **WARNING**: Since the 'autogen' in misskey.js is generated through OpenAPI  
and is depended by backend itself,  
non-dynamically porting it should be prohibited.
>
> **And it's what I'm doing right now.**

Part of work codes could be found at `docs/` directory: Still not implemented since of the limitations of Rust.

[Issues](https://github.com/HotoRas/misskey.rs/issues) and [PRs](https://github.com/HotoRas/misskey.rs/pulls) are always welcome!

## Installation
This project uses `standard type Never` (`!`)-  
You need to use Nightly build of Rust in order to use this code.

> This could change upon [this Rust issue](https://github.com/rust-lang/rust/issues/35121) being closed.
>
> Or, with closing [#2](https://github.com/HotoRas/misskey-rs/issues/2), the never type could be replaced to typescript::void (`()`).

## About
[about.md](./docs/about.md)

## Milestones
[phase.md](./docs/phase.md)

## Want to discuss?
Rustrize discussion: https://nekoplanet.xyz/notes/9utcdcp2os (Reply here at your language!)
