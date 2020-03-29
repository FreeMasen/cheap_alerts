use structopt::StructOpt;

mod cli;

fn main() {
    let opts = cli::Opts::from_args();
    let _ = cli::send_message(&opts).map_err(|e| {
        eprintln!("{}", e);
        std::process::exit(1)
    });
}
