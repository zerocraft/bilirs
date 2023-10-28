pub const HK_ACCEPT: &str = "Accept";
pub const HV_ACCEPT: &str = "application/json";
pub const HK_TYPE: &str = "Content-Type";
pub const HV_TYPE: &str = "application/json";
pub const HK_AUTHORIZATION: &str = "Authorization";
pub const HK_BILI_CONTENT_MD5: &str = "x-bili-content-md5";
pub const HK_BILI_TIMESTAMP: &str = "x-bili-timestamp";
pub const HK_BILI_SIGNATURE_METHOD: &str = "x-bili-signature-method";
pub const HV_BILI_SIGNATURE_METHOD: &str = "HMAC-SHA256";
pub const HK_BILI_SIGNATURE_NONCE: &str = "x-bili-signature-nonce";
pub const HK_BILI_ACCESSKEYID: &str = "x-bili-accesskeyid";
pub const HK_BILI_SIGNATURE_VERSION: &str = "x-bili-signature-version";
pub const HV_BILI_SIGNATURE_VERSION: &str = "1.0";

use data_encoding::HEXLOWER;
use rand::prelude::*;
use reqwest::header::HeaderMap;
use ring::hmac;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Auth {
    pub accesskey_id: String,
    pub accesskey_secret: String,
}

impl Auth {
    pub fn new(key_id: impl Into<String>, key_secret: impl Into<String>) -> Self {
        Self {
            accesskey_id: key_id.into().clone(),
            accesskey_secret: key_secret.into().clone(),
        }
    }

    pub fn build_headers(&self, content_md5: String, headers: &mut HeaderMap) {
        headers.append(HK_BILI_CONTENT_MD5, content_md5.parse().unwrap());
        let timestamp = timestamp();
        headers.append(HK_BILI_TIMESTAMP, timestamp.parse().unwrap());
        headers.append(
            HK_BILI_SIGNATURE_METHOD,
            HV_BILI_SIGNATURE_METHOD.parse().unwrap(),
        );
        let nonce = nonce();
        headers.append(HK_BILI_SIGNATURE_NONCE, nonce.parse().unwrap());
        headers.append(HK_BILI_ACCESSKEYID, self.accesskey_id.parse().unwrap());
        headers.append(
            HK_BILI_SIGNATURE_VERSION,
            HV_BILI_SIGNATURE_VERSION.parse().unwrap(),
        );
        let sign_str = builder_sign_str(self.accesskey_id.clone(), content_md5, nonce, timestamp);
        let sign_str = sign(sign_str, self.accesskey_secret.clone());
        headers.append(HK_AUTHORIZATION, sign_str.parse().unwrap());
    }
}

pub fn md5(content: String) -> String {
    let hash = md5::compute(content);
    format!("{:x}", hash)
}

fn nonce() -> String {
    let mut rng = rand::thread_rng();
    rng.gen::<usize>().to_string()
}

fn timestamp() -> String {
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).unwrap();
    duration.as_secs().to_string()
}

fn builder_sign_str(
    key_id: impl Into<String>,
    content_md5: impl Into<String>,
    nonce: impl Into<String>,
    timestamp: impl Into<String>,
) -> String {
    format!(
        "{}:{}\n{}:{}\n{}:{}\n{}:{}\n{}:{}\n{}:{}",
        HK_BILI_ACCESSKEYID,
        key_id.into(),
        HK_BILI_CONTENT_MD5,
        content_md5.into(),
        HK_BILI_SIGNATURE_METHOD,
        HV_BILI_SIGNATURE_METHOD,
        HK_BILI_SIGNATURE_NONCE,
        nonce.into(),
        HK_BILI_SIGNATURE_VERSION,
        HV_BILI_SIGNATURE_VERSION,
        HK_BILI_TIMESTAMP,
        timestamp.into()
    )
}

pub fn sign(sign_str: String, secret: impl Into<String>) -> String {
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.into().as_bytes());
    let tag = hmac::sign(&key, sign_str.as_bytes());
    HEXLOWER.encode(tag.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_time_now() {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let time = duration.as_secs().to_string();
        println!("{}", time);
    }

    #[test]
    fn test_rand() {
        let mut rng = rand::thread_rng();
        let r: u64 = rng.gen();
        println!("{}", r);
    }

    #[test]
    fn test_md5() {
        let content = "1234567890";
        let hash = md5::compute(content);
        let content_md5 = format!("{:x}", hash);
        println!("{}", content_md5);
    }

    #[test]
    fn test_signature_data() {
        let str = builder_sign_str(
            "xxxx",
            "fa6837e35b2f591865b288dfd859ce9d",
            "ad184c09-095f-91c3-0849-230dd3744045",
            "1624594467",
        );
        println!("{}", str);
        let sign = sign(str, "JzOzZfSHeYYnAMZ");
        println!("{}", sign);
        assert_eq!(
            sign,
            "a81c50234b6bbf15bc56e387ee4f19c6f871af2f70b837dc56db16517d4a341f"
        );
    }
}
