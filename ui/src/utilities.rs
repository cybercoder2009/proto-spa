use serde::de::DeserializeOwned;

pub fn from_jwt<T: DeserializeOwned>(jwt: &str) -> Option<T> {
    let parts: Vec<&str> = jwt.split('.').collect();
    if parts.len() < 2 {
        return None;
    }

    let mut payload_b64 = parts[1].replace('-', "+").replace('_', "/");
    while payload_b64.len() % 4 != 0 {
        payload_b64.push('=');
    }

    let window = web_sys::window()?;
    let latin1_str = window.atob(&payload_b64).ok()?;
    let bytes: Vec<u8> = latin1_str.chars().map(|c| c as u8).collect();
    let utf8_str = String::from_utf8(bytes).ok()?;
    serde_json::from_str::<T>(&utf8_str).ok()
}
