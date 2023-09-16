use std::{
    fs::File,
    io::{self, Write},
    process::Command,
};

#[derive(Debug)]
struct Pipe<'a> {
    data: &'a str,
    ptr: File,
}

impl<'a> Pipe<'a> {
    fn new(data: &'a str) -> std::io::Result<Self> {
        Ok(Pipe {
            data,
            ptr: File::create("examples/demo/src/pipe.json")?,
        })
    }

    fn send(&mut self) -> io::Result<usize> {
        Ok(self.ptr.write(self.data.as_bytes())?)
    }
}

fn main() -> io::Result<()> {
    println!("--------------- RUST ---------------");
    let mut data = Pipe::new(
        r#"
{
    "data": "(main.rs) Sent from rust."
}
"#,
    )?;

    println!("From Rust: {:#?}", data);
    println!("Sending...");
    data.send()?;
    println!("Sent!");

    Command::new("python")
        .arg("examples/demo/src/pipe.py")
        .spawn()
        .expect("Couldn't run python. Make Sure you've installed it.");

    println!("------------- END RUST -------------");
    println!();
    Ok(())
}
