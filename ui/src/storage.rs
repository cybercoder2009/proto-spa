pub fn read(key: &str) -> Option<String> {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item(key).ok().flatten())
}

pub fn write(key: &str, value: Option<&str>) {
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        if let Some(val) = value {
            let _ = storage.set_item(key, val);
        } else {
            let _ = storage.remove_item(key);
        }
    }
}
