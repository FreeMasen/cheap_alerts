# cheap_alerts
A utility for sending text alerts via email

## Basic Idea
All US cellular providers allow someone to send and recieve sms messages via email. 
Leveraging this, you could send "push-like" alerts out, if you have access to an email server.


## An Example
Let's say your phone number is (612)-555-0111 and your carrier is Verizon,
your number would have an email address of 6125550111@vtext.com.

Let's also say you want to send yourself an alert when your pizza order changes status at your favorite local pizza place.
You could build yourself a little web scraper that would keep track of your current orders and when the status changed
you could send the updated status to your phone by emailing the above.

```rust
use cheap_alerts::{Carrier, Destination, Sender};
use std::sync::atomic::{AtomicU8, Ordering};

static STATUS: AtomicU8 = AtomicU8::new(0);

fn main() {
    let mut current: Option<String> = None;
    loop {
        match check_for_pizza_status() {
            Some(status) => {
                if let Some(prev) = &current {
                    if prev != &status {
                        send_update(&status, &prev);
                        current = Some(status.to_string());
                    }
                } else {
                    send_update(&status, "nothing");
                    current = Some(status.to_string());
                }
            }
            None => {
                current = None;
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
/// Fake web scraping
fn check_for_pizza_status() -> Option<&'static str> {
    let old = STATUS.fetch_add(1, Ordering::Relaxed);
    if old > 4 {
        STATUS.store(0, Ordering::Relaxed);
        None
    } else {
        determine_status(&STATUS)
    }
}

fn determine_status(val: &AtomicU8) -> Option<&'static str> {
    match val.load(Ordering::Relaxed) {
        1 => Some("Pending"),
        2 => Some("Cooking"),
        3 => Some("On the way"),
        4 => Some("Complete"),
        _ => None,
    }
}

fn send_update(new: &str, old: &str) {
    let mut sender = Sender::builder()
        .address("junk@example.com")
        .stdout()
        .expect("failed to create sender");
    let dest = Destination::new("6125550111", &Carrier::Verizon);
    sender
        .send_to(
            &dest,
            &format!(
                "🍕🍕🍕🍕🍕🍕🍕🍕\nPizza Update: {} -> {}\n🍕🍕🍕🍕🍕🍕🍕🍕\n===========\n",
                old, new
            ),
        )
        .expect("failed to send notification");
}

```
# Some Details
The email portion is built on top of [`lettre`](https://crates.io/crates/lettre) and there are 3
options for the sender

- File: This takes a file path and will write the email to a file on the file system as json using the message ID (the RFC 2822 time stamp) as the file name
- Sendmail: This uses the `sendmail` cli tool for sending an email
- SMTP: This uses the Simple Mail Transport Protocol there are 3 methods for this
  - Unencrypted Localhost, this is by far the simplest, but least secure
  - Simple, this you provide a domain (as an `&str`) and it will use TLS to send the message
  - Full, this 