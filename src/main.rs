use anyhow::Result;
use cakestry::App;
use clap::Parser;

/// AI agent in the command line
#[derive(Parser)]
struct Args {}

fn main() -> Result<()> {
    let _ = Args::parse();
    let mut app = App::default();

    app.run()
}
