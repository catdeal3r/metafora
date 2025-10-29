# metafora

![version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Fcatdeal3r%2Fmetafora%2Frefs%2Fheads%2Fmain%2FCargo.toml&query=%24.package.version&label=version&style=flat-square)
![language](https://img.shields.io/badge/language-rust-red?style=flat-square)

## Abstract

Rust implementation of [DarkPrism](https://github.com/catdeal3r/darkprism).

Hosts files on the web using these steps:
1. Encrypt with AES from user-defined password
2. Split into smaller chunks
3. Upload smaller chunks, while retaning the identifiers
4. Create a file listing the identifiers in order of the file
5. Upload that, then serve the identifier of that to the user

User can then input that identifier and encryption key back into metafora for it to download the file.

## Roadmap

- [x] Upload files, unzipped and unencrypted
- [x] Print the content of the identifier's file to stdout
- [x] Change above to create a user-specified file and fill that with the content
- [x] Encrypt file
- [x] Decrypt file
- [ ] Split file after encryption
- [ ] Form file after decryption


## Build

Install rust and required libraries:

### Linux/*nix

First run this to install rust (if you haven't already):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then install libraries (dependant on OS).

#### Nix/NixOS:

```bash
nix develop
```

This will run `bash` and put you in the current directory with the required libraries already setup.

> [!Note]
> You will have to rerun this everytime you wish to compile.

#### Fedora:

```bash
sudo dnf install openssl-devel make automake gcc gcc-c++
```

#### Ubuntu:

```bash
sudo apt install libssl-dev build-essential
```

#### OpenSUSE:

```bash
sudo zypper in libopenssl-devel make automake gcc gcc-c++ 
```

#### Alpine:

```bash
sudo apk add openssl-libs-static build-base
```

### Windows
No idea. Start with OpenSSL or similar libraries.

### Compiling

First make sure you've cloned the repo:

```bash
git clone https://github.com/catdeal3r/metafora
```

Or, if you already have, run this inside of the cloned folder to update the codebase:

```bash
git reset --hard HEAD
git pull
```

#### Release

To build in release mode:
```bash
cargo build --release
```
The compiled binary will be located here: `./target/release/metafora`

#### Debug

To build in debug mode:
```bash
cargo build
```

The compiled binary will be located here: `./target/debug/metafora`
