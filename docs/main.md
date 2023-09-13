
# Table of Contents

1.  [Manifest](#org1a2c6b3)
    1.  [merge-lang](#org3e43cda)

New approach to programming languages with generic multilang communication system,
**allowing you code** with multiple programming languages in a project with a modern automated interface.


<a id="org1a2c6b3"></a>

# Manifest

Developing a project using multiple programming languages can easily become a complex task.
although there are some Simpler solutions to this problem. They generally lack relaibility,
performance, stability etc.

This is a common situation that happens to all of us. For example:

> X programming language has a library/framework that I cannot use with Y programming language.

There are some solutions to this problem like:

-   Transpilers: Automated X lang to Y lang converters. (just like .png to .jpeg but more complex)
-   Rewrite: Rewriting the program in X lang. (e.g fish shell was written in C++, now it is getting rewritten in rust)
-   FFI&rsquo;s (Foreign Function Interface)
-   ABI&rsquo;s (Application Binary interface)
-   Language Communication Protocols (e.g IPC, Shared Memory, Server)
-   Wrappers: Custom libraries that wrap around the native language using Language Communication Protocols.

One of the few solution&rsquo;s that I mentioned above are hard to implement in general. Others are:

-   Overall Transpilers: Could generate erroneous output (defective transpilers)
-   Rewrites: Consumes a lot of time & effort. Because of the human factor.

Because of these strenuous ways of implementing multi-lang interfaces We introdoce you merge-lang


<a id="org3e43cda"></a>

## merge-lang

> A generic meta programming language that automates the process of combining programming languages in a project

