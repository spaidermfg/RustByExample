/// clap test
fn main() {
    command();
}

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    name: Option<String>,

    #[arg(short, long, value_name="FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}


#[derive(Subcommand)]
enum Commands {
    Test {
        #[arg(short, long)]
        list: bool,
    },  
}

fn command() {
    let args = Args::parse;

    //name
    if let Some(name) = args().name.as_deref() {
        println!("Value for name: {}", name);
    }

    //config
    if let Some(config_path) = args().config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    //debug
    match args().debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    //command
    match &args().command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }

}
