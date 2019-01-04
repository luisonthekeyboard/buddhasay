# `buddhasay`

         ______________________________________
        /                                      \
        | If you are swayed by things of the   |
        | world, how regrettable that is.      |
        | Things of the world are fleeting and |
        | impermanent.                         |
        \______________________________________/
                                 \
                                  \    ___
                                   \  (-_-)
                                      _) (_
                                     /     \
                                   _( \_ _/ )_
                                  (_____\_____)


Like [Cowsay](https://en.wikipedia.org/wiki/Cowsay), but with a Buddha and Buddhism related
sentences.

## How can install this?

If you're running Arch Linux, it is available in the [AUR](https://aur.archlinux.org/packages/buddhasay/).

You can also install it from [crates.io](https://crates.io/crates/buddhasay) with `cargo install buddhasay`.

If none of this works for you, then just checkout the source or one of the tags and build it with 
`cargo build --release`. The binary will be under `./target/release/buddhasay`.

## Why are you doing this?

1. To have some fun while learning Rust;
2. To have more inspiring messages whenever I open terminals;

## Hey, I have some Buddhism related sentences as well, can I get `buddhasay` to also say my sentences?

Probably, open a PR with your sentences and their sources and let's take it from there.

## What about other Linux distributions and other OSs?

Even though [Arch is the best](https://wiki.archlinux.org/index.php/Arch_is_the_best), I do plan to make `buddhasay` 
available in other distros and OSs.

## So what things are you planning next?

Mostly exercises for me to continue learning Rust and systems; in no particular order:

- `fortune` and `cowsay` compatibility mode. Right now `buddhasay` does the equivalent to `fortune | cowsay`. It should 
be possible to run `buddhasay | cowsay`, `fortune | buddhasay` or `buddhasay`.
- Packages for other Linux distros;
- Package for homebrew;
- Split sentences out of the code;
- Add tags to the sentences (zen, theravada, book, etc), authors and source;
- One release script to rule them all;
- Align naming conventions and naming style;
- Tests and separate modules;