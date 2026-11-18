mod app;
mod model;
mod storage;

use clap::Parser;

#[derive(Parser)]
#[command(name = "memo", version, about = "Un carnet de notes interactif")]
struct Cli {}

fn main() {
    let _cli = Cli::parse();

    if let Err(error) = app::run() {
        eprintln!("memo: {error}");
        std::process::exit(1);
    }
}
