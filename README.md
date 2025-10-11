# metafora

![version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Fcatdeal3r%2Fmetafora%2Frefs%2Fheads%2Fmain%2FCargo.toml&query=%24.package.version&label=version&style=flat-square)
![language](https://img.shields.io/badge/language-rust-red?style=flat-square)

## Abstract

Rust implementation of [DarkPrism](https://github.com/catdeal3r/darkprism).

Hosts files on the web using these steps:
1. Zip file and lightly compress
2. Encrypt with AES from user-defined password
3. Split into smaller chunks
4. Upload smaller chunks, while retaning the identifiers
5. Create a file listing the identifiers in order of the file
6. Upload that, then serve the identifier of that to the user

User can then input that identifier and encryption key back into metafora for it to download the file.

## Roadmap

- [x] Upload files, unzipped and unencrypted
- [x] Print the content of the identifier to stdout
- [ ] Zip file
- [ ] Unzip file
- [ ] Encrypt file
- [ ] Unencrypt file
- [ ] Split file after zipping and encryption
- [ ] Form file after decryption and unzip


## Build

### Linux/*nix

#### Nix/NixOS

If you are using Nix, you can run:

```bash
nix develop
```

to install and setup components.

#### Other Distributions

Install OpenSSL libraries:

##### Fedora:
```bash
sudo dnf install openssl-devel
```

##### Ubuntu:
```bash
sudo apt install libssl-dev
```

### Windows
No idea. Start with OpenSSL or similar libraries.

### Build

Finally, run this to build:
```bash
cargo build --release
```

The compiled binary will be here: `./target/release/metafora`
