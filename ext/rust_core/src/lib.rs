use std::path::PathBuf;

use magnus::{define_module, function, prelude::*, Error};
// use pks::packs::get_unresolved_references;
use packs::packs::get_unresolved_references;

fn get_unresolved_references_bridge(
    absolute_root: PathBuf,
    cache_dir: PathBuf,
    relative_files: Vec<String>,
) -> Vec<String> {
    dbg!(&absolute_root);
    dbg!(&cache_dir);
    dbg!(&relative_files);

    get_unresolved_references(&absolute_root, &cache_dir, relative_files);

    vec![]
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("RustParser")?;
    module.define_singleton_method(
        "get_unresolved_references",
        function!(get_unresolved_references_bridge, 3),
    )?;
    Ok(())
}
