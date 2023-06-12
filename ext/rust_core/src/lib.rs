use std::path::PathBuf;

use magnus::{define_module, function, prelude::*, Error};
// use pks::packs::get_unresolved_references;
use magnus::{IntoValue, Value};
use packs::packs::get_unresolved_references;
use serde::{Deserialize, Serialize};
use serde_magnus::serialize;

// class ProcessedFile < T::Struct
//   const :unresolved_references, T::Array[UnresolvedReference], default: []
//   const :offenses, T::Array[Offense], default: []
// end
//   UnresolvedReference = Struct.new(
//     :constant_name,
//     :namespace_path,
//     :relative_path,
//     :source_location,
//     keyword_init: true,
//   )

#[derive(Debug, Deserialize, Serialize)]
struct ProcessedFileBridge {
    pub unresolved_references: Vec<UnresolvedReferenceBridge>,
    pub offenses: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct UnresolvedReferenceBridge {
    pub constant_name: String,
    pub namespace_path: Vec<String>,
    pub relative_path: String,
    pub source_location: LocationBridge,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct LocationBridge {
    pub line: usize,
    pub column: usize,
}

// impl magnus::TryConvert for LocationBridge {
//     fn try_convert(value: magnus::Value) -> Result<Self, Error> {
//         let line = value.funcall("line", &[])?;
//         let line = line.try_convert_to::<usize>()?;
//         let column = value.funcall("column", &[])?;
//         let column = column.try_convert_to::<usize>()?;
//         Ok(LocationBridge { line, column })
//     }
// }
fn get_unresolved_references_bridge(
    absolute_root: PathBuf,
    cache_dir: PathBuf,
    relative_files: Vec<String>,
) -> Value {
    let ret = get_unresolved_references(&absolute_root, &cache_dir, relative_files);
    let ret: Vec<ProcessedFileBridge> = ret
        .into_iter()
        .map(|processed_file| {
            let relative_path = processed_file
                .absolute_path
                .strip_prefix(&absolute_root)
                .unwrap();
            let unresolved_references: Vec<UnresolvedReferenceBridge> = processed_file
                .unresolved_references
                .into_iter()
                .map(|unresolved_reference| {
                    let source_location = LocationBridge {
                        line: unresolved_reference.location.start_row,
                        column: unresolved_reference.location.start_col,
                    };
                    let unresolved_reference = UnresolvedReferenceBridge {
                        constant_name: unresolved_reference.name,
                        namespace_path: unresolved_reference.namespace_path,
                        relative_path: relative_path.to_str().unwrap().to_string(),
                        source_location,
                    };
                    unresolved_reference
                })
                .collect();
            let offenses: Vec<String> = vec![];

            let processed_file = ProcessedFileBridge {
                unresolved_references,
                offenses,
            };
            // processed_file
            // serialize(&processed_file).unwrap()
            processed_file
        })
        .collect();
    let processed_files: Value = serialize(&ret).expect("Could not serialize rust processed file");
    // ret
    // let new_ret: Value = serialize(&ret).unwrap();
    // new_ret
    processed_files
}

// impl std::convert::From<ProcessedFileBridge> for magnus::Value {
//     fn from(processed_file: ProcessedFileBridge) -> Self {
//         // Convert the ProcessedFileBridge to magnus::Value
//         // Implementation logic goes here
//         // ...
//         let ret: Value = Value::from(serialize(&processed_file).unwrap());
//         ret
//     }
// }

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("RustParser")?;
    module.define_singleton_method(
        "get_unresolved_references",
        function!(get_unresolved_references_bridge, 3),
    )?;
    Ok(())
}
