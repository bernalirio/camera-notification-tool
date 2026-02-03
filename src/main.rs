use notify_rust::{Hint, Notification, Timeout, Urgency};
use std::error::Error;
use std::thread;
use std::time::Duration;
use udev::MonitorBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let monitor = MonitorBuilder::new()?
        .match_subsystem("video4linux")?
        .listen()?;

    let mut is_active = false;
    let mut last_notification_handle: u32 = 0;

    loop {
        for event in monitor.iter() {
            let (icon, label) = match event.action() {
                Some(action) if action == "add" && !is_active => {
                    is_active = true;
                    ("camera-web-symbolic", "Camera On")
                }
                Some(action) if action == "remove" && is_active => {
                    is_active = false;
                    ("camera-web-disabled-symbolic", "Camera Off")
                }
                _ => continue,
            };

            let mut notification = Notification::new();
            notification
                .summary(label)
                .icon(icon)
                .urgency(Urgency::Critical)
                .hint(Hint::Transient(true))
                .hint(Hint::Category("device".to_string()))
                .hint(Hint::Custom(
                    "x-canonical-private-synchronous".to_string(),
                    "camera-monitor".to_string(),
                ))
                .timeout(Timeout::Milliseconds(1500));

            if last_notification_handle != 0 {
                notification.id(last_notification_handle);
            }

            match notification.show() {
                Ok(handle) => last_notification_handle = handle.id(),
                Err(e) => eprintln!("Failed to show notification: {}", e),
            }
        }

        thread::sleep(Duration::from_millis(300));
    }
}
