use rhai::{Dynamic, Engine, EvalAltResult, ImmutableString};
use std::fs;
mod containers;

// Normal function that returns a standard type
// Remember to use 'ImmutableString' and not 'String'
fn add_len(x: i64, s: ImmutableString) -> i64 {
    x + s.len() as i64
}
// Alternatively, '&str' maps directly to 'ImmutableString'
fn add_len_count(x: i64, s: &str, c: i64) -> i64 {
    x + s.len() as i64 * c
}
// Function that returns a 'Dynamic' value
fn get_any_value() -> Dynamic {
    42_i64.into()                       // standard types can use '.into()'
}

pub fn main() -> Result<(), Box<EvalAltResult>>
//                          ^^^^^^^^^^^^^^^^^^
//                          Rhai API error type
{
    // let args: Vec<String> = env::args().collect();
    // let file_path = &args[1];
    let file_path = "./examples/example.rhai";
    println!("In file {file_path}");
    // Create an 'Engine'
    let mut engine = Engine::new();
    engine.register_fn("add", add_len)
        .register_fn("add", add_len_count)
        .register_fn("add", get_any_value)
        .register_fn("hi", containers::pull::pull)
        .register_fn("inc", |x: i64| {    // closure is also OK!
            x + 1
        })
        .register_fn("log", |label: &str, x: i64| {
            println!("{label} = {x}");
        });

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
    // Your first Rhai Script
    // let script = "print(40 + 2);";
    // Run the script - prints "42"
    engine.run(contents.as_str())?;
    // Done!
    Ok(())
}