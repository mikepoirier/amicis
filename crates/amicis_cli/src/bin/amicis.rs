use amicis_cli::args::HelloArgs;
use clap::Parser as _;

fn main() {
    let args = HelloArgs::parse();

    amicis_cli::run(args)
}
