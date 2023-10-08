
# Table of Contents

1.  [Merge](#orge44d3ea)
    1.  [TLDR](#org7f8a8f8)
    2.  [Manifest](#org6a9c200)
        1.  [merge-lang](#org142af60)
        2.  [Inference](#org16ce583)
        3.  [File Structure](#org25153e1)
        4.  [Compile](#org6134981)
        5.  [Scheduling Execution](#org5d00ada)
        6.  [Runtime](#org2d62681)
    3.  [Package Manager](#org7eb36e9)
2.  [API](#orgbad0db3)



<a id="orge44d3ea"></a>

# Merge

NOTE: Any of the features are not implemented yet. It&rsquo;s planned to do so.


<a id="org7f8a8f8"></a>

## TLDR

New approach to programming languages with generic multilang communication system,
**allowing you code** with multiple programming languages in a project with a modern automated interface.


<a id="org6a9c200"></a>

## Manifest

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


<a id="org142af60"></a>

### merge-lang

> A generic meta programming language that automates the process of combining programming languages in a project

Merge takes another approach than the other solutions I mentioned.

1.  Initial Algorithm

    -   Inference (understands the data communication points: more on that in the next section)
    -   Constructing the file structure (splitting code to it&rsquo;s pieces by the fewest whispers<sup><a id="fnr.1" class="footref" href="#fn.1" role="doc-backlink">1</a></sup>)
    -   Compiling
    -   Scheduling Execution
    -   Runtime


<a id="org16ce583"></a>

### Inference

This is probably the most complex part of merge-lang. (Also the most innovative way)
Look at the following example:

```rust
    #[python]
    use *;
    
    fn main() {
        python::print("hello, world");
    
        #[python]
        {
            a = "world"
            print(f"hello {a}")
        }
    
        python!{
            a = "hello"
            print(f"{a} world!")
            import a from b
            a.hello()
    
            def python_fn():
                return 5
        };
    
        let val_py = python!(python_fn());
        let val_py = python::python_fn();
    
        println!("from rust: {val_py}");
    }
```

<a id="org25153e1"></a>

### File Structure

Writing our own compiler for all languages that we support would be imposible. Because of this merge compiler will split your code into pieces that external languages can understand.

With this tecnique you can integrate any programming language to merge-lang using [It&rsquo;s API](#orgbad0db3)


<a id="org6134981"></a>

### Compile

Merge language compiles to rust code, then LLVM IR and ASM.


<a id="org5d00ada"></a>

### Scheduling Execution

The dependencies need to evaluate in a way that every language can get the value they need at the runtime in a linear way. (Just like single vs multi-thread apps)

Think this as a major surgery with you (the doctor) and Mr. Clumsy (the nurse)

1.  If Mr. Clumsy gives you a cleaver instead of a lancet, the patient&rsquo;d probably die. So Mr. Clumsy must give you the right tool to do the surgery. But It doesn&rsquo;t end here.
2.  If Mr. Clumsy&rsquo;d give you a lancet one minute later (or before) than the time you need it. The patient&rsquo;d die again because of haemorrhage. So timing is a must too!

And don&rsquo;t forget that we made a preconception by saying that Mr. Clumsy will give us a thing.


<a id="org2d62681"></a>

### Runtime

It&rsquo;s wanted to see a nice execution sequience between languages that you use.


<a id="org7eb36e9"></a>

## Package Manager

merge package manager is pending right now.


<a id="orgbad0db3"></a>

# API

Merge-lang introduces an API to be able to use more and more languages with it.


# Footnotes

<sup><a id="fn.1" href="#fnr.1">1</a></sup> data transmissions done between programming languages.
