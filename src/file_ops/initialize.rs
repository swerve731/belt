use super::utils;
use include_dir::{include_dir, Dir};
use rust_embed::Embed;


#[derive(askama::Template)]
#[template(path = "initial/scaffolds/Cargo.toml.jinja")]
pub struct InitialCargoToml {
    project_name: String
}




pub async fn initialize(project_name: &str) -> tokio::io::Result<()> {
    let current_dir =  std::env::current_dir()?;
    let project_path = current_dir.join(project_name);
    
    if project_path.exists() {
        return Err(tokio::io::Error::new(tokio::io::ErrorKind::AlreadyExists, "Project already exists"));
    }

    tokio::fs::create_dir(&project_path).await?;

    
    // clone the files and dirs from scaffolds/initial/static 
    // utils::clone_dir(&"scaffolds/initial/static".into(), &project_path, 50).await?;


    Ok(())
}




// returns how many files were moved
