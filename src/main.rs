use std::{path::PathBuf, str::FromStr};

use file_ops::utils::{clone_static_file, Scaffolds};

pub mod cli;
pub mod file_ops;

#[tokio::main]
async fn main() {
    // cli::run().await.unwrap();

    clone_static_file("initial/static/src/main.rs", &PathBuf::from_str("output").unwrap()).await.unwrap();
    
    // let current_dir = std::env::current_dir().unwrap();
    // let input_path = current_dir.join("scaffolds/initial/static");
    // let output_path = &current_dir.join("test_clone");

    // file_ops::initialize::initialize("myapp").await.unwrap();

    // file_ops::create_file_from_template(MyTemplate {
    //     name: "test6000".to_string(),
    //     should_show:true
    // }, "test.txt".into(), true).await.unwrap();

}