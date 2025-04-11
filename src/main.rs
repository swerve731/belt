
pub mod file_ops;

#[tokio::main]
async fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let input_path = current_dir.join("scaffolds/initial/static");
    let output_path = &current_dir.join("test_clone");

    file_ops::initialize::initialize("myapp").await.unwrap();

    // file_ops::create_file_from_template(MyTemplate {
    //     name: "test6000".to_string(),
    //     should_show:true
    // }, "test.txt".into(), true).await.unwrap();

}