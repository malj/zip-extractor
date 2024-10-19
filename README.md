# ZIP Extractor

A small ZIP file extraction utility written in [Rust](https://www.rust-lang.org/). The crate exposes both a [library API](src/lib.rs) and a [CLI program](src/main.rs).

## Build

To build the executable use [Cargo](https://doc.rust-lang.org/cargo/):

```sh
cargo build --release
```

The executable output path is `/target/release/zip-extractor`

## Usage

Executing the CLI program with the `help` flag will print instructions how to use it:

```sh
zip-extractor --help
```

For example to extract file `my_file.txt` from `my_archive.zip` and save its contents to `my_extracted_file.txt`:

```
zip-extractor -a my_archive.zip -f my_file.txt > my_extracted_file.txt
```

## Test

Run the test suite with Cargo:

```sh
cargo test
```

## License

MIT License ([LICENSE-MIT](/LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
