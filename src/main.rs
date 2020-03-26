use clap::Clap;

mod cli;

fn main() {
    let opts = cli::Opts::parse();
    let _ = cli::send_message(&opts)
        .map_err(|e| {
            eprintln!("{}", e);
            std::process::exit(1)
        });
}

