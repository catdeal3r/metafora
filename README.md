# metafora

| Version |
| --- |
| **0.1.2** |

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
