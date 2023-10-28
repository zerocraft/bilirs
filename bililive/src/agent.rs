pub const BASE_API_URL: &str = "https://live-open.biliapi.com";

use crate::auth::{self, Auth};
use reqwest::{header::HeaderMap, Client, RequestBuilder};

pub struct ApiAgent {
    http_client: Client,
    auth: Auth,
}

impl ApiAgent {
    pub fn new(auth: Auth) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            auth,
        }
    }

    pub fn build_request(&self, url: String, body: String) -> RequestBuilder {
        self.http_client
            .post(url)
            .headers(self.build_headers(body.clone()))
            .body(body)
    }

    fn build_headers(&self, body_str: String) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let content_md5 = auth::md5(body_str.clone());
        headers.append(auth::HK_ACCEPT, auth::HV_ACCEPT.parse().unwrap());
        headers.append(auth::HK_TYPE, auth::HV_TYPE.parse().unwrap());
        self.auth.build_headers(content_md5, &mut headers);
        headers
    }
}

pub fn apiurl(url: &str) -> String {
    format!("{}{}", BASE_API_URL, url)
}

#[cfg(test)]
mod tests {
    use crate::{
        agent::ApiAgent, apiv2::V2apis, auth::Auth, env_access_key, env_access_secret, env_app_id,
        env_live_code,
    };

    #[tokio::test]
    async fn test_api_start() {
        let _code = env_live_code();
        let _agent = ApiAgent::new(Auth::new(env_access_key(), env_access_secret()));
        let _res = _agent.start(_code, env_app_id()).await;
        if _res.is_ok() {
            let r = _res.unwrap();
            println!("ApiResponse:{} {}", r.code, r.message);
            if let Some(data) = r.data {
                println!("game_info.game_id:{}", data.game_info.game_id);
                println!("websocket_info.wss_link:{:?}", data.websocket_info.wss_link);
                println!(
                    "data.websocket_info.auth_body:{:?}",
                    data.websocket_info.auth_body
                );
                println!("data.anchor_info.room_id:{}", data.anchor_info.room_id);
                println!("data.anchor_info.uid:{}", data.anchor_info.uid);
                println!("anchor_info.uname:{}", data.anchor_info.uname);
            }
        }
    }
}
