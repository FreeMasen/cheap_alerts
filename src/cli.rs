

use clap::Clap;
use cheap_alerts::{Carrier, Sender, Error};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Clap)]
#[clap(version = "0.1", author = "FreeMasen")]
pub struct Opts {
    #[clap(short = "c", long = "carrier")]
    pub carrier: Carrier,
    #[clap(short = "n", long = "number")]
    pub number: String,
    #[clap(short = "m", long = "message")]
    pub message: String,
    pub config_path: Option<PathBuf>,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub from: String,
    pub transport: Transport,
}

#[derive(Deserialize, Serialize)]
pub enum Transport {
    File { path: PathBuf },
    SendMail,
    StmpLocalHost,
    StmpSimple { domain: String },
}

impl Config {
    fn into_sender<'a, R>(self) -> Result<Sender<'a, R>, Error> {
        let b = Sender::builder().address(&self.from);
        let ret = match  self.transport {
            Transport::File { path } => {
                b.file(path)
            },
            Transport::SendMail => {
                b.sendmail()
            },
            Transport::StmpLocalHost => {
                b.smtp_unencrypted_localhost()
            },
            Transport::StmpSimple { domain } => {
                let ret = b.smtp_simple(&domain)
            },
        };

        Ok(ret)
    }
}