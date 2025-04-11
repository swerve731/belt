use std::fs;
use std::path::{Path, PathBuf};

pub mod utils;
pub mod initialize;

// pub struct IncludeFile {
//     pub content: &'static [u8], // Compile-time included content
//     pub name: String,
// }

// pub struct IncludeDir {
//     pub parent_dir: Option<*mut IncludeDir>,
//     pub files: Vec<IncludeFile>,
//     pub dirs: Vec<IncludeDir>,
// }

// impl IncludeFile {
//     pub fn from_file(path: &Path) -> Self {
//         let name = path.file_name().unwrap().to_string_lossy().to_string();
//         let content = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", path.to_str().unwrap()));
//         Self { name, content }
//     }
// }

// impl IncludeDir {
//     pub fn new() -> Self {
//         Self {
//             parent_dir: None,
//             files: Vec::new(),
//             dirs: Vec::new(),
//         }
//     }

//     pub fn from_dir(path: &Path) -> Self {
//         let mut dir = IncludeDir::new();
//         let entries = fs::read_dir(path).expect("Failed to read directory");

//         for entry in entries {
//             let entry = entry.expect("Failed to read directory entry");
//             let entry_path = entry.path();

//             if entry_path.is_dir() {
//                 let sub_dir = IncludeDir::from_dir(&entry_path);
//                 dir.dirs.push(sub_dir);
//             } else if entry_path.is_file() {
//                 let file = IncludeFile::from_file(&entry_path);
//                 dir.files.push(file);
//             }
//         }

//         dir
//     }
// }