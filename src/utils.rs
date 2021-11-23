use std::time::{SystemTime, UNIX_EPOCH, Duration};

// https://cloud.tencent.com/document/product/436/7778
// TODO move expire to config object
const SIGN_EXPIRE: u64 = 60 * 60 * 24;

pub fn gen_keytime() -> String {
    let sys_time = SystemTime::now();
    let some_duration = Duration::from_secs(SIGN_EXPIRE);

    match sys_time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs().to_string() + "," + &(n.as_secs() + SIGN_EXPIRE).to_string(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::gen_keytime;

    #[test]
    fn it_works() {
        let keytime = gen_keytime();
        println!("{:?}", keytime)
    }
}
