use std::path::PathBuf;

use magnus::{define_module, function, prelude::*, Error};

fn get_unresolved_references(
    absolute_root: PathBuf,
    cache_dir: PathBuf,
    absolute_files: Vec<PathBuf>,
) -> Vec<String> {
    dbg!(absolute_root);
    dbg!(cache_dir);
    dbg!(absolute_files);
    vec![]
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("RustParser")?;
    module.define_singleton_method(
        "get_unresolved_references",
        function!(get_unresolved_references, 3),
    )?;
    Ok(())
}
