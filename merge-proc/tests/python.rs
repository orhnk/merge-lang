use merge_proc::python;

// Check out for more metavariable types:
// https://doc.rust-lang.org/reference/macros-by-example.html#metavariables
macro_rules! using {              // single use statement
    ($e:path) => {
        use $e;
    };
    ($e:path, $($es:path),+) => { // no trailing comma
        using!($e);
        using!($($es),+);
    };
    ($e:path, $($es:path),+ ,) => { // trailing comma
        using!($e);
        using!($($es),+);
    };
    ($e:path, ) => { // trailing comma
        using!($e);
        using!($($es),+);
    };

    /************************************************/
    /*                    ERRORS                    */
    /************************************************/
    // Two trailing non capturing commas
    () => {
        ::std::compile_error!("Redundant using statement")
    };
}

#[test]
fn use_python() {
    using! {
        std::io::copy,
        std::io,
        std::iostream,
        std::io
    };
}
