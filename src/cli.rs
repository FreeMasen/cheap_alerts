use cheap_alerts::{Carrier, Destination, Error, Sender};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(version = "0.1", author = "FreeMasen")]
pub struct Opts {
    #[structopt(short = "c", long = "carrier")]
    pub carrier: Carrier,
    #[structopt(short = "n", long = "number")]
    pub number: String,
    #[structopt(short = "d", long = "domain")]
    pub domain: Option<String>,
    #[structopt(short = "f", long = "from")]
    pub from: String,
    pub message: String,
}

pub fn send_message(opts: &Opts) -> Result<(), Error> {
    let builder = Sender::builder().address(&opts.from);
    let mut sender = if let Some(domain) = &opts.domain {
        builder.smtp_simple(domain)?
    } else {
        builder.smtp_unencrypted_localhost()?
    };
    let dest = Destination::new(&opts.number, &opts.carrier);
    sender.send_to(&dest, &opts.message)?;
    Ok(())
}
