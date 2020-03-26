

use clap::Clap;
use cheap_alerts::{Carrier, Sender, Error, Destination};

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "FreeMasen")]
pub struct Opts {
    #[clap(short = "c", long = "carrier")]
    pub carrier: Carrier,
    #[clap(short = "n", long = "number")]
    pub number: String,
    #[clap(short = "d", long = "domain")]
    pub domain: Option<String>,
    #[clap(short = "f", long = "from")]
    pub from: String,
    pub message: String,
}

pub fn send_message(opts: &Opts) -> Result<(), Error> {
    let builder = Sender::builder()
        .address(&opts.from);
    let mut sender = if let Some(domain) = &opts.domain {
        builder.smtp_simple(domain)?
    } else {
        builder.smtp_unencrypted_localhost()?
    };
    let dest = Destination::new(&opts.number, &opts.carrier);
    sender.send_to(&dest, &opts.message)?;
    Ok(())
}