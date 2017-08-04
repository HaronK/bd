# TODO

## Fixes

* [ ] Rename the repository to **bd**.
* [ ] Reorganize folders.
* [ ] Change DataSource trait to Read+Seek traits combination?

## Releases and features

* [ ] Version 0.1.0: Basic structures
  * [ ] generic template - flexible structure
  * [ ] generic structure - fixed structure (custom derived macros (1.1) for a fixed Rust structures and types)
  * [ ] POD - char, short, int, float, string with counter
  * [ ] array
  * [ ] 0-terminated string

* [ ] Version 0.2.0: Attributes
  * [ ] decimal system (hex, dec, ...)
  * [ ] endian (little, big)
  * [ ] to_string
  * [ ] block/block value validation

* [ ] Version 0.3.0: CI support/Testing/Documentation
  * [ ] Add tests
  * [ ] Add [rust-skeptic](https://github.com/brson/rust-skeptic) support
  * [ ] Travis support
  * [ ] AppVeyor support

* [ ] Version 0.4.0: Dynamic templates
  * [ ] Data template declaration (based on macros).
  * [ ] Dynamic templates (compiled into dynamically linked libraries).
  * [ ] Containers - sized block of data that can be initialized/parsed by other template.

* [ ] Version 0.5.0: CLI
 *  [ ] Command line interface (separate repository: **bd_cli**).

* [ ] Version 0.6.0
 *  [ ] Lua templates.

* [ ] Version 0.7.0: GUI (Qt - C++)
  * [ ] GUI (separate repository: **bd_qt**): reuse existing Qt implementation from the C++ code. Generate FFI interface to access it from the C++ side.