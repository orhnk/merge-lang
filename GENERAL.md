# Merge Programming Language Plan

## File Extention

Plain Merge files will have the extension `.mg`

But there is a pending merge feature that allows you write language specific
code in a merge file. This will be done by adding a language extension to the
file name. For example, a merge file with python code in it will have the
extension `.mg.py`

## Syntax

<!-- deno-fmt-ignore -->
> [!NOTE]
> This is a work in progress and is subject to change
> Following snippet is using rust syntax highlighting
> which is not native but the closest to merge syntax

```rust
// A comment (MAY CHANGE TO A #)

#[
    rust = {
        main_only = true,
    }
]

/// Following syntax may change in the future
/// The full syntax is:
///
/// [to_yeet] -> [to_suck]
/// lang! {
///     ...
/// }
-> [rust_result] // Means: suck the rust_result out!
rust! {
    println!("[RUST]: START");
    let rust_result = 0;
    println!("[RUST]: END");
}

[rust_result] -> [c_suck] // Means: Yeet rust_result to the following lang! and suck the result to c_suck!

c! {
    printf("[C]: START\n");
    printf("[C]: %d\n", rust_result);
    int c_result = 0;
    printf("[C]: END\n");
}
```

## Challenges

### Type Checking

Main challenge is type checking. Merge is a multi-language programming language
and each language has its own type system. This means that the type checker
needs to be able to type check multiple languages at the same time.

I'm planning to leave this to the user for now. And provide the utilities to
make it easier to do so. (Which is probably going to get automated by the time)

### Error Handling

Error handling is another challenge. Each language has its own error handling
system. And the error handling system of the language that is being used to
write the merge code is not necessarily the same as the error handling system of
the language that is being used to write the code that is being merged.

I'm planning to use file-tree in a way that makes it easy to handle errors. (e.g
putting every language call in a separate file and using the file name to) I can
offset the merge block according to the file and block that caused the error.

### Syntax Highlighting

Will inspire a lot from markdown syntax highlighting.

## Compiling

Firstly merge code will get seperated to a file tree. Then the yeets and sucks
will get resolved. Then the code will get compiled to the target language. (with
the build script if it's necessary)

## Build Scripts

Build scripts will get written inline (but for quick testing purposes, it's
better to write them in a separate file). They will get compiled to the target
language and then executed.

```rust
#[
    rust = {
        build = "rust/Cargo.toml",
    }
    c = {
        build = "c/Makefile",
    }
]
```

## Debugging

Saved for later (not essential for now)

## Testing

Some languages are not standerdized. So I'm planning to use the build scripts to
test the code. (if essential)

```rust
#[
    c = {
        test = {
            dir  = "c/tests",
            test = "make test",
        },
    }
]
```

## Documentation

Same with testing (uses default if possible, else uses build script)

```rust
#[
    c = {
        doc = "make doc",
    }
]
```

## Examples

Saved for later (not essential for now)

## Benchmarks

Same as testing and documentation

```rust
#[
    c = {
        bench = "make bench",
    }
]
```
