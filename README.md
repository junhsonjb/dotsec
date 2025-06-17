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

You can install `dotsec` using one of the following methods:
- [build from source](#build-from-source)
- [install via `crates.io`](#install-via-cratesio)
- [install via homebrew](#install-via-homebrew)
- [install via script (Linux Only)](#install-via-script)

🥅 Short-term goal: Publish on more package managers!

### Build from source

#### 📋 Prerequisites
You'll need [Rust](https://www.rust-lang.org/tools/install) installed.

```sh
git clone https://github.com/junhsonjb/dotsec.git
cd dotsec
cargo build --release
cargo run -- -V         # optional: check that it works
cargo install --path .  # optional: make `ds` globally available
```

### Install via crates.io
```sh
cargo install dotsec
```

### Install via homebrew
```sh
brew tap junhsonjb/dotsec
brew install dotsec
```

### Install via script
Use `curl` to download the installation script and execute it with `sh`:
```sh
curl -sSL https://raw.githubusercontent.com/junhsonjb/dotsec/main/distribution/install.sh | sh
```
> This script downloads the latest binary and installs it to /usr/local/bin/ds

It's always recommended to inspect scripts that you download off the internet before running them. You can do so by running the following:
```sh
curl -sSL https://raw.githubusercontent.com/junhsonjb/dotsec/main/distribution/install.sh | less
```
> Alternatively, [download the script directly](./distribution/install.sh)

## 🧠 Heads up
The CLI is in early development (`v0.1.x`) — expect rapid iteration and the occasional breaking change. Feedback welcome!

## 🔐 Security
`dotsec` uses [ChaCha20-Poly1305](https://github.com/RustCrypto/AEADs/tree/master/chacha20poly1305) to encrypt and decrypt secrets. Keys and values are stored locally on the user's machine using [sled](https://github.com/spacejam/sled).

> ⚠️ **Important: Encryption key is stored in plaintext!**
>
> The encryption key is saved to the following location:
> ```sh
> $XDG_CONFIG_HOME/dotsec/private/dotsec.key  # typically resolves to ~/.config/dotsec/private/dotsec.key
> ```
> This file is **not encrypted**, and it can decrypt all your stored secrets. If someone gets access to it, they can read your data. Be careful not to check it into version control or share it.

## ⌨️ Usage

`dotsec` has four primary functions:
- store secrets, along with an identifier (a "key")
```sh
cargo run -- put molly weasley-family-password@#$%
```
- retrieve secrets, using the associated key
```sh
cargo run -- get molly
# stdout: `weasley-family-password@#$%`
```
- list all existing keys
```sh
cargo run -- list
# stdout: `molly`
```
- delete secrets, using the associated key
```sh
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
