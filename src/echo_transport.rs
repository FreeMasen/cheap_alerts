use lettre::{SendableEmail, Transport};

pub struct EchoTransport;

impl<'a> Transport<'a> for EchoTransport {
    type Result = std::io::Result<()>;
    fn send(&mut self, email: SendableEmail) -> Self::Result {
        use std::io::{Read, Write};
        let out = std::io::stdout();
        let mut out = std::io::BufWriter::new(out);
        out.write_all(format!("Message: {}\n", email.message_id()).as_bytes())?;
        out.write_all(b"-----------\n")?;
        let env = email.envelope();
        if let Some(from) = env.from() {
            out.write_all(format!("From: {}", from).as_bytes())?;
        } else {
            out.write_all(b"From: no-one")?;
        }
        out.write_all(b"\n")?;
        let to = env.to();

        if to.is_empty() {
            out.write_all(b"To: no-one")?;
        } else {
            out.write_all(b"To: ")?;
            for address in to {
                out.write_all(format!("{}\n", address).as_bytes())?;
            }
        }
        out.write_all(b"-----------\nMessage\n----------\n")?;
        let mut msg = email.message();
        let mut bytes = Vec::new();
        let _ = msg.read_to_end(&mut bytes)?;
        out.write_all(&bytes)?;
        Ok(())
    }
}
