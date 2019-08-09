# Rust Warrior

[![Build Status][travis-badge]][travis] [![Crates.io][crates-badge]][crates] [![Docs.rs][docs-badge]][docs] [![Deps.rs][deps-badge]][deps]

[travis-badge]: https://travis-ci.org/miller-time/rust-warrior.svg
[travis]: https://travis-ci.org/miller-time/rust-warrior
[crates-badge]: https://img.shields.io/crates/v/rust-warrior
[crates]: https://crates.io/crates/rust-warrior
[docs-badge]: https://docs.rs/rust-warrior/badge.svg
[docs]: https://docs.rs/rust-warrior/
[deps-badge]: https://deps.rs/repo/github/miller-time/rust-warrior/status.svg
[deps]: https://deps.rs/repo/github/miller-time/rust-warrior

This game is inspired by [Ruby Warrior][ruby-warrior], which I played many
years ago. This project started as a direct port, but because Ruby is an
interpreted language and allows (encourages?) meta programming... I've
designed Rust Warrior partly from scratch.

[ruby-warrior]: https://github.com/ryanb/ruby-warrior

## Set Up

Rust Warrior ships with two major components: a binary that generates a
new game directory, and a library which contains the main game engine.

To get started, install `rust-warrior`.

```sh
$ cargo install rust-warrior
```

Then run `rust-warrior` and follow the prompts to pick a player name and
set up all the necessary files in a new game directory.

```sh
$ rust-warrior
```

Then navigate to `rustwarrior/<name>` (the name you provided). You'll find
a `README.md` with instructions for level 1 and a `src/main.rs` with the
scaffolding for a Rust Warrior player project (with some `rust_warrior`
imports).

After reviewing the instructions, start playing level 1 by running the project:

```sh
$ cargo run
```
