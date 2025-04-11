use std::{env, path::PathBuf};
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(askama::Template)]
#[template(path = "test.txt")]
pub struct MyTemplate {
    pub name: String,
    pub should_show: bool
}

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

pub async fn clone_dir(input_path: &PathBuf, output_path: &PathBuf, limit: u8) -> tokio::io::Result<u8> {
    //clone all the files and dirs from the input path and write them to the output path, only clone as much as the limit
    
    let mut count = 0;
    let mut entries = tokio::fs::read_dir(input_path).await?;
    while let Some(entry) = entries.next_entry().await? {
        if count >= limit {
            break;
        }
        let entry_path = entry.path();
        let file_name = entry_path.file_name().unwrap();
        let new_path = output_path.join(file_name);
        if entry_path.is_dir() {
            tokio::fs::create_dir(&new_path).await?;
            count += Box::pin(clone_dir(&entry_path, &new_path, limit-count)).await?;
        } else {
            count += 1;
            tokio::fs::copy(&entry_path, &new_path).await?;
        }
    }

    Ok(count)
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
        let result = create_file_from_template(template, file_path.clone(), true).await;
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
        let result = create_file_from_template(template, PathBuf::from("/invalid/path/output.txt"), true).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_file_from_template_overwrite() {
        // Create a temporary directory for testing.
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("output.txt");

        // Create a template instance.
        let template = MyTemplate {
            name: "Test Name".to_string(),
            should_show: false,
        };

        // Call the function to create the file from the template.
        let result = create_file_from_template(template, file_path.clone(), true).await;
        assert!(result.is_ok());

        // Check if the file exists.
        assert!(file_path.exists());

        // Read the file content and check if it matches the rendered template.
        let content = fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(content, "Hello, Test Name");

        // Create a new template instance.
        let template2 = MyTemplate {
            name: "Test Name 2".to_string(),
            should_show: true,
        };

        // Call the function to create the file from the template again, this time it should overwrite.
        let result = create_file_from_template(template2, file_path.clone(), true).await;
        assert!(result.is_ok());

        // Read the file content and check if it matches the new rendered template.
        let content = fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(content, "Hello, Test Name 2");

        // Clean up the temporary directory.
        temp_dir.close().unwrap();
    }

    #[tokio::test]
    async fn test_create_file_from_template_no_overwrite() {
        // Create a temporary directory for testing.
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("output.txt");

        // Create a template instance.
        let template = MyTemplate {
            name: "Test Name".to_string(),
            should_show: false,
        };

        // Call the function to create the file from the template.
        // Call the function to create the file from the template.
        let result = create_file_from_template(template, file_path.clone(), true).await;
        assert!(result.is_ok());
        
        // Check if the file exists.
        assert!(file_path.exists());

        let template2 = MyTemplate {
            name: "Test No Overwrite".to_string(),
            should_show: false,
        };

        // Attempt to create the file again without overwriting.
        let result = create_file_from_template(template2, file_path.clone(), false).await;
        assert!(result.is_err());

        // Clean up the temporary directory.
        temp_dir.close().unwrap();
    }
}

