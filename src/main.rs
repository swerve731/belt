use file_ops::MyTemplate;

pub mod file_ops;

#[tokio::main]
async fn main() {

    file_ops::create_file_from_template(MyTemplate {
        name: "test".to_string(),
        should_show:true
    }, "something/something/another/test.txt".into()).await.unwrap();

}