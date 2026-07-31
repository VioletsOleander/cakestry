use anyhow::Result;
use clap::Parser;

use cakestry::App;

/// AI agent in the command line.
#[derive(Parser)]
struct Args {}

fn main() -> Result<()> {
    let _ = Args::parse();
    let mut app = App::default();

    app.run()
}
