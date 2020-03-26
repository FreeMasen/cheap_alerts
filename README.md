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

fn main() {
  let mut current = None;
  loop {
    match check_for_pizza_status() {
      Some(status) => {
        if let Some(prev) = current {
          if prev != status {
            send_update(&status, &prev);
            current = Some(status);
          }
        } else {
          send_update(&status, "nothing");
          current = Some(status);
        }
      },
      None => {
        current = None;
      }
    }
    std::thread::sleep(std::time::Duration::from_secs(60));
  }
}

fn check_for_pizza_status() -> Option<String> {
  // Look at me, I'm web scraping!
}

fn send_update(new: &str, old: &str) {
  let mut sender = Sender::builder()
                .address("junk@example.com")
                .smtp_unencrypted_localhost()
                .expect("failed to create sender");
  
  sender.send_to("6125550111@vtext.com", &format!("🍕🍕🍕🍕🍕🍕🍕🍕\nPizza Update: {} -> {}\n🍕🍕🍕🍕🍕🍕🍕🍕", old, new)
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