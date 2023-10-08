use merge_proc::rust;

#[rust]
use std::{
    collections::HashMap,
    io::{copy, sink},
    mem::zeroed,
};

#[test]
fn use_rust() {
    let mut map = HashMap::new();
    map.insert("hello", "world");
    println!("{:?}", map);
}
