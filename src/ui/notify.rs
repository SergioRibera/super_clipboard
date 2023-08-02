use crate::sync::MDnsDevice;
use notify_rust::Notification;

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn accept_link_request(
    _device: &MDnsDevice,
    _msg: &str,
    _acept: impl FnOnce(),
    _decline: impl FnOnce(&str),
) {
}

#[cfg(all(unix, not(target_os = "macos")))]
pub fn accept_link_request(
    device: &MDnsDevice,
    msg: &str,
    acept: impl FnOnce(),
    decline: impl FnOnce(&str),
) {
    use notify_rust::Timeout;
    use std::time::Duration;

    Notification::new()
        .appname("SuperClipboard")
        .summary(&format!("Device binding request \"{}\"", device.name))
        .body(msg)
        .action("accept", "Accept")
        .action("reject", "Cancel")
        .timeout(Timeout::from(Duration::from_secs(5)))
        .show()
        .unwrap()
        .wait_for_action(|action| match action {
            "accept" => acept(),
            x => decline(x),
        });
}
