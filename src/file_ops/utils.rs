use std::{env, path::PathBuf};
use rust_embed::{Embed, EmbeddedFile};
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(askama::Template)]
#[template(path = "test.txt")]
pub struct MyTemplate {
    pub name: String,
    pub should_show: bool
}

#[derive(Embed)]
#[folder = "scaffolds"]
pub struct Scaffolds;

// #[derive(Embed)]
// #[folder = "scaffolds/initial/static"]
// pub struct InitialStatic;

pub async fn create_file_from_template(
    template: impl askama::Template,
    path: PathBuf,
    overwrite: bool,
) -> tokio::io::Result<()> {
    // the path passed in is relative to the directory where the program is run
    // so we need to get the current working directory
    let current_dir = env::current_dir().unwrap();
    let full_path = current_dir.join(path);

    // Check if the file already exists
    if full_path.exists() && !overwrite {
        return Err(tokio::io::Error::new(
            tokio::io::ErrorKind::AlreadyExists,
            "File already exists and overwrite is set to false",
        ));
    }

    // the path can contain multiple directories that don't yet exist
    // so we need to create the directories first if they don't exist
    if let Some(parent) = full_path.parent() {
        if !parent.exists() {
            tokio::fs::create_dir_all(parent).await?;
        }
    }

    // create the file
    let mut file = File::create(&full_path).await?;
    // render the template
    let rendered = template.render().unwrap();

    // write the rendered template to the file
    file.write_all(rendered.as_bytes()).await?;
    // flush the file to ensure all data is written
    file.flush().await?;
    // close the file
    drop(file);
    // return the path to the file
    Ok(())
}

pub async fn clone_static_file(embed_path: &str, output_path: &PathBuf) -> tokio::io::Result<()> {
    //clone all the files and dirs from the input path and write them to the output path, only clone as much as the limit
    let file = Scaffolds::get(embed_path).ok_or_else(|| {
            tokio::io::Error::new(
                tokio::io::ErrorKind::NotFound,
                format!("File not found in embedded directory: {}", embed_path),
            )
        })?;
    let file_data = std::str::from_utf8(file.data.as_ref())
        .map_err(|_| tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, "Invalid UTF-8 data"))?;

    // create the file
    let mut out_file = File::create(output_path).await?;
    out_file.write_all(file_data.as_bytes()).await?;

    out_file.flush().await?;
    
    Ok(())
}




