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
