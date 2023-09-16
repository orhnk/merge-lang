# FFI

Acronym for "Foreign Function Interface"

> _Performant way of interlanguage communication_

## Pros

- Fast!
- Easier to implement
- Well supported & Used by the community

## Cons

- Requires extra build scripts that can get incredibly difficult to write (e.g
  c)
  - merge will have options configuration files (e.g `.rustfmt.toml`) And for
    languages that makes it hard to implement a build script automatically
    (C/C++ etc.) a cmake file will be required
