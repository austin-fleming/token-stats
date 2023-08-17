use hmac::{Hmac, Mac};
use sha2::Sha256;

pub type HmacSha256 = Hmac<Sha256>;

pub fn create_hmac264_signature(key: String, data: String) -> Result<String, String> {
    let mut mac =
        HmacSha256::new_from_slice(key.as_bytes()).expect("Failed to create mac from key");

    mac.update(data.as_bytes());

    let mac_output = mac.finalize();
    let mac_string = bytes_slice_to_string(mac_output.into_bytes().as_slice());

    Ok(mac_string)
}

fn bytes_slice_to_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("")
}
