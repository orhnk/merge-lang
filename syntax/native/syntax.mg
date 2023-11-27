![
    rust = {
        build = "hello world",
    },
]

# hello

![main]
-> [scooped]
rust! {
    println!("It's A me! mArio!");
    let scooped = "RUST STRING";
    {
        println!("inside block");
    }
}

[scooped]
c! {
    printf("Hello, World!");
    {},
    printf("%s", scooped);
    char* scooped = "C STRING";
}
