# rust-warrior

This game is inspired by [Ruby Warrior][ruby-warrior], which I played many
years ago. This project started as a direct port, but because Ruby is an
interpreted language and allows (encourages?) meta programming... I've
designed `rust-warrior` partly from scratch.

# Set Up

`rust-warrior` ships with two major components: a binary that generates a
new game directory, and a library which contains the main game engine.

To get started, install `rust-warrior`.

```sh
$ cargo install rust-warrior
```

Then run `rust-warrior` and follow the prompts to set up all the
necessary files in a new game directory.

```sh
$ rust-warrior
```

Then navigate to `rustwarrior/<name>` and run the program.

```sh
$ cd rustwarrior/<name>
$ cargo run
```

[ruby-warrior]: https://github.com/ryanb/ruby-warrior
