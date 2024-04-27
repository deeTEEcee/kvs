use clap::{Parser, Subcommand};

/*
kvs set <KEY> <VALUE>
Set the value of a string key to a string

kvs get <KEY>
Get the string value of a given string key

kvs rm <KEY>
Remove a given key

kvs -V
Print the version

*/


#[derive(Parser)]
// #[command(version, about, long_about = None)]
// #[command(propagate_version = true)]
struct Cli {

    #[arg(short='V', long, action)]
    version: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Set { key: Option<String>, value: Option<String> },
    Get { key: Option<String>},
    Rm { key: Option<String>},
}

fn main() {
    // Example call for dev: `cargo run --package kvs --bin kvs -- -v`
    let cli = Cli::parse();

    if cli.version {
        println!("{}", env!("CARGO_PKG_VERSION"))
    }
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    else if let Some(cmd) = &cli.command {
        match cmd {
            Commands::Set { key, value } => {
                eprintln!("unimplemented");
            }
            Commands::Get { key } => {
                eprintln!("unimplemented");

            }
            Commands::Rm { key } => {
                eprintln!("unimplemented");
            }
        }
        std::process::exit(1)
    }
    else {
        std::process::exit(1)
    }
}