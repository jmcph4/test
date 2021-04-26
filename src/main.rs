use clap::{crate_version, Clap};

#[derive(Clap)]
#[clap(version = crate_version!())]
pub struct Opts {/* TODO: add command-line options here! */}

fn main() {
    let _opts: Opts = Opts::parse(); /* TODO: remove underscore */

    println!("Hello, world!");
}
