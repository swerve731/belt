use file_ops::MyTemplate;

pub mod file_ops;

#[tokio::main]
async fn main() {

    file_ops::create_file_from_template(MyTemplate {
        name: "test6000".to_string(),
        should_show:true
    }, "test.txt".into(), true).await.unwrap();

}