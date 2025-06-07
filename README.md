# dotsec

`dotsec` is short for **dot**file **sec**urity. It's a lightweight tool meant to securely hold secrets. A *secret* is any bit of information that one might not wish to store in plaintext.

It's designed for terminal users who want a simple CLI-based secrets manager that they can self-host and build into their workflow.

## ✨ Features
- Securely store and retrieve secrets from the command line
- Local-only — no cloud storage or tracking
- Uses strong encryption ([ChaCha20-Poly1305](https://github.com/RustCrypto/AEADs/tree/master/chacha20poly1305))
- Supports listing and deleting secrets
- Config directory managed via [XDG](https://specifications.freedesktop.org/basedir-spec/latest/) base directories

## 💾 Installation

Currently `dotsec` must be cloned and built from source. One of the short-term goals is to publish the project so that it can be installed on platforms like `brew`, `apt`, and others.

### Prerequisites
You'll need [Rust](https://www.rust-lang.org/tools/install) installed.

### Install from source
```bash
git clone https://github.com/junhsonjb/dotsec.git
cd dotsec
cargo build --release
cargo run -- -V
```

### (Optional) Aliasing
Since `dotsec` currently has to be built from source, users will need to run the program in the following manner:
```bash
cargo run -- <command> <args>
```

This is a lot to type. The project is planned to be published in the short-term, but in the meantime we can use an alias as a workaround:
```bash
alias dotsec="cargo run --"
```
Run the above line, or add it to your `bashrc` (or the config for whichever shell you use) to make it persistent.

## 🔐 Security
`dotsec` uses [ChaCha20-Poly1305](https://github.com/RustCrypto/AEADs/tree/master/chacha20poly1305) to encrypt and decrypt secrets. Keys and values are stored locally on the user's machine using [sled](https://github.com/spacejam/sled).

⚠️ Encryption keys are stored at `$XDG_CONFIG_HOME/dotsec/private/dotsec.key` (`$XDG_CONFIG_HOME` resolves to `$HOME/.config` unless otherwise defined). **This file is not encrypted. Please do not publicize this file (on github or elsewhere)!** `dotsec`'s primary goal is to provide a safe place to store your CLI secrets. It is the user's responsibility to safely manage their encryption key.

> ⚠️ **Important: Encryption key is stored in plaintext!**
>
> The encryption key is saved to the following location:
> ```bash
> $XDG_CONFIG_HOME/dotsec/private/dotsec.key  # typically resolves to ~/.config/dotsec/private/dotsec.key
> ```
> This file is **not encrypted**, and it can decrypt all your stored secrets. If someone gets access to it, they can read your data. Be careful not to check it into version control or share it.

## ⌨️ Usage

`dotsec` has four primary functions:
- store secrets, along with an identifier (a "key")
```bash
cargo run -- put molly x12YtT4gL12y41$@a2bxy941GGLB12TOW$$@%17
```
- retrieve secrets, using the associated key
```bash
cargo run -- get molly
# stdout: `x12YtT4gL12y41$@a2bxy941GGLB12TOW$$@%17`
```
- list all existing keys
```bash
cargo run -- list
# stdout: `molly`
```
- delete secrets, using the associated key
```bash
cargo run -- delete molly -n # dry-run mode
# stdout: `would delete secret with name molly`

cargo run -- delete molly -f # forces deletion
```
because deletion is permanent, users are forced to run the command in either `dry-run` mode or `force` mode. Calling `delete` without either flag is an error.

## 🪪 License

This project is licensed under either:

- [MIT License](./LICENSE-MIT)
- [Apache License, Version 2.0](./LICENSE-APACHE)

You may choose either license to use this software.

## 🫱🏽‍🫲🏿 Contributing
Please send a PR or file an issue if you're interested in contributing. This project exists because I thought it could be helpful **and** because I love the Open Source community and wanted to give back to it. Don't be shy!

In the short-term, I plan on adding contributor guidelines and a PR template. But until then, just be civil 🙂

## 🗺️ Project Status / Roadmap
This project is in its early stages, but it works — the MVP is functional and ready for feedback!

For upcoming features and ideas, check out the [Issues](https://github.com/junhsonjb/dotsec/issues) tab. This is where we'll track planned improvements, bugs, and community requests.
