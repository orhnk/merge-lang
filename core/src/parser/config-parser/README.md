# Merge Config Parser

Merge Configuration parser.

This module is responsible for parsing the configuration part of merge files. Or
the `Merge.toml` file.

## TOML

Merge uses an optional `Merge.toml` config file as build script configurations.

## In-line Configuration

Merge also supports in-line configuration. This is done by adding a `#![]`
section to your merge script.

```rust
#![
    rust = {
        dir = "rust",
        cmd = {
            build = "cargo build",
            run = "cargo run",
            test = "cargo test",
        },
        config = "Cargo.toml",
    },
    
    c = {
        dir = "c",
        cmd = {
            build = "make",
            run = "./main",
            test = "make test",
        },
        config = "Makefile",
    },
]

-> [took_from_rust]
rust! {
    println!("[RUST]: Started");
    let took_from_rust = "[RUST]: Hello World";
    println!("[RUST]: Ended");
}

[took_from_rust]
c! {
    printf("[C]: Started\n");
    printf("[C]: %s\n", took_from_rust);
    printf("[C]: Ended\n");
}
```

## Examples

```toml
[rust]
dir = "rust"
config = "Cargo.toml"
cmd = {
    build = "cargo build",
    run = "cargo run",
    test = "cargo test"
}

[c]
dir = "c"
config = "Makefile"
cmd = {
    build = "make",
    run = "./main",
    test = "make test"
}
```
