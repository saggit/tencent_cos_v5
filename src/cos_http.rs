use reqwest::Client;


pub struct CosClient {
    base_url: String,
    appid: String,
    key: String,
    http_client: Client,
}

impl CosClient {
  pub fn new(base_url: String, appid: String, key: String) -> Self {
    Self {
        base_url: base_url,
        appid: appid,
        key: key,
        http_client: Client::new(),
    }
  }
}