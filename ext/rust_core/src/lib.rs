use magnus::{define_module, function, prelude::*, Error};

fn hello(_x: usize) -> Vec<String> {
    vec![]
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("RustParser")?;
    module.define_singleton_method("get_unresolved_references", function!(hello, 1))?;
    Ok(())
}
