# Fork
This is the fork from wonderfull [joshuto](https://github.com/kamiyaa/joshuto/)repository.
## Why I forked?
* I work solo for so many years, that pick bad habbits, like leave mess in the code
(comments usually).
* I move quick and dirty and  may have some funny ideas.
* I was searching for tool that will simply work. Since I'm on my fork I can do
whichever feel is more workable for me. Like for example I restrict number of tabs to
five or implemented closing with Ctrl-C even before I made an issue in joshuto repository.      
I stil hope that I will make useful contribution to original repository, but if not at least I will nave good tool in my belt.

# joshuto

[ranger](https://github.com/ranger/ranger)-like terminal file manager written in Rust.

![Alt text](screenshot.png?raw=true "joshuto")

## Dependencies

- [cargo](https://github.com/rust-lang/cargo/) >= 1.55
- [rustc](https://www.rust-lang.org/) >= 1.55
- xsel/xclip/wl-clipboard (optional, for clipboard support)
- fzf (optional)
- zoxide (optional)

Also see [Cargo.toml](Cargo.toml)

## Building

```
~$ cargo build
```

## Installation

#### For single user

```
~$ cargo install --path=. --force
```

#### System wide

```
~# cargo install --path=. --force --root=/usr/local     # /usr also works
```

#### Packaging status

##### Fedora ([COPR](https://copr.fedorainfracloud.org/coprs/atim/joshuto/))

```
sudo dnf copr enable atim/joshuto -y
sudo dnf install joshuto
```

##### Arch ([AUR](https://aur.archlinux.org))

* [release](https://aur.archlinux.org/packages/joshuto)
```
[yay/paru] -S joshuto
```

* [build from source](https://aur.archlinux.org/packages/joshuto-git)
```
[yay/paru] -S joshuto-git
```

##### MacOS ([MacPorts](https://ports.macports.org/port/joshuto/details/))

```
sudo port install joshuto
```

## Usage
```
~ $ joshuto
```
See [docs#quit](/docs/configuration/keymap.toml.md#quit-quit-joshuto) for exiting into current directory and other usages

## Configuration

Check out [docs](/docs) for details and [config](/config) for examples

#### [joshuto.toml](/config/joshuto.toml)
- general configurations

#### [keymap.toml](/config/keymap.toml)
- for keybindings

#### [mimetype.toml](/config/mimetype.toml)
- for opening files with applications

#### [theme.toml](/config/theme.toml)
- color customizations

#### [bookmarks.toml](/config/bookmarks.toml)
- bookmarks

## Contributing
See [docs](/docs)

## Bugs/Feature Request

Please create an issue :)

## Features
- Tabs
- Devicons
- Fuzzy search via [fzf](https://github.com/junegunn/fzf)
- Ctrl/Shift/Alt support
- Bulk rename
- File previews
  - See [Image previews](/docs/image_previews.md) for more details
- Exit to current directory
- Asynch File IO (cut/copy/paste)
- Custom colors/theme
- Line numbers
  - Jump to number
- File chooser
- Trash support

## TODOs

- [x] Built-in command line
  - Mostly working
  - Currently implementation is kind of janky
  - [ ] Tab autocomplete (in progress)
