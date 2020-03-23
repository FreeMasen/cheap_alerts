//! A library for sending notifications
//! via email. If you need a quick and
//! dirty way to send out notifications
//! (and you have access to an email server),
//! this crate is for you!
use lettre::{EmailAddress, Transport, Envelope, SendableEmail};
use chrono::prelude::*;

#[derive(Debug)]
pub enum Error {
    Lettre(lettre::error::Error),
    LettreFile(lettre::file::error::Error),
    LettreSendMail(lettre::sendmail::error::Error),
    LettreSmtp(lettre::smtp::error::Error),
    MissingEmail,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Lettre(i) => i.fmt(f),
            Error::LettreFile(i) => i.fmt(f),
            Error::LettreSendMail(i) => i.fmt(f),
            Error::LettreSmtp(i) => i.fmt(f),
            Error::MissingEmail => write!(f, "Error, email address to build a Sender"),
        }
    }
}

impl std::error::Error for Error {}

impl From<lettre::error::Error> for Error {
    fn from(other: lettre::error::Error) -> Self {
        Self::Lettre(other)
    }
}

impl From<lettre::file::error::Error> for Error {
    fn from(other: lettre::file::error::Error) -> Self {
        Self::LettreFile(other)
    }
}

impl From<lettre::sendmail::error::Error> for Error {
    fn from(other: lettre::sendmail::error::Error) -> Self {
        Self::LettreSendMail(other)
    }
}

impl From<lettre::smtp::error::Error> for Error {
    fn from(other: lettre::smtp::error::Error) -> Self {
        Self::LettreSmtp(other)
    }
}

pub struct SenderBuilder {
    pub(crate) address: Option<EmailAddress>,
}

impl SenderBuilder {
    pub fn address(mut self, from: &str) -> Self {
        if let Ok(add) = EmailAddress::new(from.to_string()) {
            self.address = Some(add);
        }
        self
    }
    
    pub fn file<'a, P: AsRef<std::path::Path>>(self, p: P) -> Result<Sender<'a, lettre::file::error::FileResult>, Error> {
        let client = Box::new(lettre::FileTransport::new(p));
        if let Some(address) = self.address {
            Ok(Sender {
                address,
                client,
            })
        } else {
            Err(Error::MissingEmail)
        }
    }

    pub fn sendmail<'a>(self) -> Result<Sender<'a, lettre::sendmail::error::SendmailResult>, Error> {
        let client = Box::new(lettre::SendmailTransport::new());
        if let Some(address) = self.address {
            Ok(Sender {
                address,
                client,
            })
        } else {
            Err(Error::MissingEmail)
        }
    }

    pub fn smtp_unencrypted_localhost<'a>(self) -> Result<Sender<'a, lettre::smtp::error::SmtpResult>, Error> {
        let smtp = lettre::SmtpClient::new_unencrypted_localhost()?;
        self.smtp(smtp)
    }

    pub fn smtp_simple<'a>(self, domain: &str) -> Result<Sender<'a, lettre::smtp::error::SmtpResult>, Error> {
        let smtp = lettre::SmtpClient::new_simple(domain)?;
        self.smtp(smtp)
    }

    pub fn smtp_full<A: std::net::ToSocketAddrs>(self, addr: A, security: lettre::smtp::ClientSecurity) -> Result<Sender<'a, lettre::smtp::error::SmtpResult>, Error> {
        let smtp = lettre::SmtpClient::new(addr, security)?;
        self.smtp(smtp)
    }

    pub fn smtp<'a>(self, smtp: lettre::SmtpClient) -> Result<Sender<'a, lettre::smtp::error::SmtpResult>, Error> {
        let client = Box::new(lettre::SmtpTransport::new(
            smtp
        ));
        if let Some(address) = self.address {
            Ok(Sender {
                address,
                client,
            })
        } else {
            Err(Error::MissingEmail)
        }
    }
}

pub struct Sender<'a, R> {
    pub address: EmailAddress,
    pub client: Box<dyn Transport<'a, Result = R>>,
}

impl<'a> Sender<'a, ()> {
    pub fn builder() -> SenderBuilder {
        SenderBuilder {
            address: None,
        }
    }
}

impl<'a, E> Sender<'a, Result<(), E>> 
where E: Into<Error> {
    pub fn send_to(&mut self, dest: &Destination, msg: &str) -> Result<(), Error> {
        let to = EmailAddress::new(dest.address())?;
        let from = self.address.clone();
        let env = Envelope::new(Some(from), vec![to])?;
        let email = SendableEmail::new(env,
            Utc::now().to_rfc2822(),
            msg.as_bytes().to_vec()
        );
        match self.client.send(email) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}

/// A cell phone carrier
/// 
/// > Note: this is currently only US providers
/// > with support, we could include others as
/// > well. The `Other` case will allow for 
/// > you to extend this enum with anything
/// > not currently provided
pub enum Carrier {
    /// [number]@txt.att.net
    ATT,
    /// [number]@messaging.sprintpcs.com
    Sprint,
    /// [number]@tmomail.net
    TMobile,
    /// [number]@vtext.com
    Verizon,
    /// [number]@myboostmobile.com
    BoostMobile,
    /// [number]@sms.mycricket.com
    Cricket,
    /// [number]@mymetropcs.com
    MetroPCS,
    /// [number]@mmst5.tracfone.com
    Tracfone,
    /// [number]@email.uscc.net
    USCellular,
    /// [number]@vmobl.com
    VirginMobile,
    /// Other carrier, the string provided is
    /// the domain for this carrier
    Other { domain: String },
}

/// A phone number and 
/// mobile carrier pair
/// for sending a text
/// message
pub struct Destination {
    pub number: String,
    pub carrier: Carrier,
}

impl Destination {
    /// Creates a new destination with
    /// the provided phone number and
    /// carrier. The phone number provided
    /// will have all not decimal digits
    /// stripped from it (It is not validated in any way).
    pub fn new(number: &str, carrier: Carrier) -> Self {
        let number = number
            .chars()
            .filter(|c| c.is_digit(10))
            .collect();
        Self { number, carrier }
    }

    pub fn address(&self) -> String {
        format!("{}@{}", self.number, self.carrier.get_domain())
    }
}

impl Carrier {
    pub fn get_domain(&self) -> &str {
        match self {
            Carrier::ATT => "txt.att.net",
            Carrier::Sprint => "messaging.sprintpcs.com",
            Carrier::TMobile => "tmomail.net",
            Carrier::Verizon => "vtext.com",
            Carrier::BoostMobile => "myboostmobile.com",
            Carrier::Cricket => "sms.mycricket.com",
            Carrier::MetroPCS => "mymetropcs.com",
            Carrier::Tracfone => "mmst5.tracfone.com",
            Carrier::USCellular => "email.uscc.net",
            Carrier::VirginMobile => "vmobl.com",
            Carrier::Other { domain } => domain,
        }
    }
}
