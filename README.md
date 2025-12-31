# Meow


Print ASCII cats to your terminal!

This is a simple command-line tool to display cute little kitties :D

I love cats

[![Number of repositories](https://repology.org/badge/tiny-repos/meow-ascii-cats.svg)](https://repology.org/project/meow-ascii-cats/versions)
<br />
[![latest packaged version(s)](https://repology.org/badge/latest-versions/meow-ascii-cats.svg)](https://repology.org/project/meow-ascii-cats/versions)
<br />
[![Packaging status](https://repology.org/badge/vertical-allrepos/meow-ascii-cats.svg)](https://repology.org/project/meow-ascii-cats/versions)


## Usage

```
Usage: meow [OPTIONS]

Options:
  -c, --count <COUNT>  How many cats to print [default: 1]
  -l, --literally      Are you literally this cat?
  -h, --help           Print help
  -V, --version        Print version
  multiple meows to print multiple cats eg: meow meow -> 2 cats and meow meow meow meow -> 4 cats
```

## Installation
## Building from source on any OS

1. Install Rust
1. Clone this repository
1. Build and run with `cargo run` or `cargo run -- [OPTIONS]`


If rust isn't already installed install it with:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

git clone https://github.com/Starwaves1/meow-meow-meow.git
cd meow-meow-meow
cargo install --path .

## Packaging shenanigans

The goal of this package is to eventually be in every Linux package repository and more (who doesn't like cats?)
Current status:

- [x] NixOS
- [x] Debian Stable
- [x] Ubuntu
- [ ] AUR
- [ ] Fedora
- [ ] EPEL
- [ ] Homebrew
- [ ] WinGet
- [ ] Extras
  - [ ] Gentoo
  - [ ] Alpine
  - [ ] OpenBSD
  - [ ] FreeBSD
  - [ ] openSUSE

## Credits

- Cats
- Meowing
- [How to exit vim](https://stackoverflow.com/questions/11828270/how-do-i-exit-the-vim-editor)
