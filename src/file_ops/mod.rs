use std::{env, path::{Path, PathBuf}};
use tokio::{fs::File, io::AsyncWriteExt};


#[derive(askama::Template)]
#[template(path = "test.txt")]
pub struct MyTemplate {
    pub name: String,
    pub should_show: bool
}




pub async fn create_file_from_template(template: impl askama::Template, path: PathBuf) -> tokio::io::Result<()> {

    /// * `template` - An Askama template that implements the `askama::Template` trait.
    /// * `path` - The path to the file where the rendered template should be written.

    // the path passed in is relative to the directory where the program is run
    // so we need to get the current working directory
    let current_dir = env::current_dir().unwrap();
    let full_path = current_dir.join(path);
    // the path can contain multiple directories that dont yet exist ie.(templates/dashboard/components/index.html)
    // so we need to create the directories first if they dont exist 
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





#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use tempfile::tempdir;
    use tokio::fs;

    #[tokio::test]
    async fn test_create_file_from_template() {
        // Create a temporary directory for testing.
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("output.txt");

        // Create a template instance.
        let template = MyTemplate {
            name: "Test Name".to_string(),
            should_show: false,
        };

        // Call the function to create the file from the template.
        let result = create_file_from_template(template, file_path.clone()).await;
        assert!(result.is_ok());

        // Check if the file exists.
        assert!(file_path.exists());

        // Read the file content and check if it matches the rendered template.
        let content = fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(content, "Hello, Test Name");

        // Clean up the temporary directory.
        temp_dir.close().unwrap();
    }

    #[tokio::test]
    async fn test_create_file_from_template_error() {
        // Create a template instance.
        let template = MyTemplate {
            name: "Test Name".to_string(),
            should_show: false
        };

        // Call the function to create the file from the template with an invalid path.
        let result = create_file_from_template(template, PathBuf::from("/invalid/path/output.txt")).await;
        assert!(result.is_err());
    }
}

