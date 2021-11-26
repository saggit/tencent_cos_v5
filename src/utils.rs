use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::env;
use std::str;

use ring::hmac;
use hex::encode as hex_encode;

// https://cloud.tencent.com/document/product/436/7778
// TODO move expire to config object
const SIGN_EXPIRE: u64 = 60 * 60 * 24;

pub fn gen_keytime() -> String {
    let sys_time = SystemTime::now();
    let some_duration = Duration::from_secs(SIGN_EXPIRE);

    match sys_time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs().to_string() + ";" + &(n.as_secs() + SIGN_EXPIRE).to_string(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub fn gen_signkey(keytime: String) -> String {
    let secret = match env::var("SECRET_KEY") {
            Ok(val) => val,
            Err(err) => {
                panic!("SECRET_KEY not found in env vars!");
            },
        };
    let key = hmac::Key::new(hmac::HMAC_SHA256, &secret.as_bytes());

    return hex_encode(hmac::sign(&key, keytime.as_bytes()).as_ref());
}

#[cfg(test)]
mod tests {
    use crate::utils::gen_keytime;
    use crate::utils::gen_signkey;

    #[test]
    fn it_works() {
        let keytime = gen_keytime();
        println!("{:?}", keytime)
    }

    #[test]
    fn test_signkey() {
        let key = gen_signkey("hello".to_string());
        println!("{:?}", key)
    }
}
