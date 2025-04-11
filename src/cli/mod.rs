use clap_v3::{Arg, App}
;
pub async fn run() {
    let app = App::new("Belt")
        .version("0.1")
        .author("Eoin Mitchell <eoinmitchell39@proton.me>")
        .about("A CLI scaffolding for rust and htmx web apps")
        .subcommand(
            App::new("new")
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
        ("new", Some(sub_matches)) => {
            println!("Creating a new project...");
        }
        _ => {
            println!("No subcommand");
        }
    }
}