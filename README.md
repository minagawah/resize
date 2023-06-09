# resize

A very simple CLI program written in Rust for resizing an image with specified width.

## Install

```shell
# build
cargo build ---release

# copy the binary to desired directory
cp -p target/release/resize ~/
cd ~

# run
./resize --path ~/Pictures/test.png --width 300
# it will emit the new file 'test.new.png' in the same folder as the original image file resides.
```

## Notes

### (a) cargo fmt

Looking into [rustfmt.toml](rustfmt.toml),
you see the setting:

```toml
format_strings = true
```

which works only for the Nightly build.  
So, you need:

```bash
cargo +nightly fmt
```

## License

Dual-licensed under either of the followings.  
Choose at your option.

- The UNLICENSE ([LICENSE.UNLICENSE](LICENSE.UNLICENSE))
- MIT license ([LICENSE.MIT](LICENSE.MIT))
