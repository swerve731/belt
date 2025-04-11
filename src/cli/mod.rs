use clap::Arg;
use spinners::{Spinner, Spinners};

use crate::file_ops;
pub async fn run() -> tokio::io::Result<()> {
    let app = clap::Command::new("belt")
        .version("0.1")
        .author("Eoin Mitchell <eoinmitchell39@proton.me>")
        .about("A CLI scaffolding for rust and htmx web apps")
        .subcommand(
            clap::Command::new("new")
                .about("Create a new project")
                .arg(
                    Arg::new("name")
                        .help("The name of the project")
                        .required(true)
                        .index(1),
                )
        );
    
    let matches = app.get_matches();
    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let name: &String = sub_matches.get_one::<String>("name").unwrap();
            let mut sp = Spinner::new(Spinners::FingerDance, "Cloning Template...".into());
            file_ops::initialize::initialize(name).await.unwrap();
            sp.stop_with_message(format!("Project ({}) Successfully Created", name));
            Ok(())
        }
        _ => {
            println!("No subcommand");
            Ok(())
        }
    }
}