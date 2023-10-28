use crate::{
    agent::{apiurl, ApiAgent},
    error::ServiceError,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiResponse<T>
where
    T: Serialize + Default,
{
    pub code: u32,
    pub data: Option<T>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StartData {
    pub anchor_info: AnchorInfo,
    pub game_info: GameInfo,
    pub websocket_info: WebSocketInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameInfo {
    pub game_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebSocketInfo {
    pub auth_body: String,
    pub wss_link: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnchorInfo {
    pub room_id: u32,
    pub uname: String,
    pub uface: String,
    pub uid: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EndData {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HeartBeatData {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchHeartBeatData {
    pub failed_game_ids: Vec<String>,
}

#[async_trait]
pub trait V2apis {
    const START_URL: &'static str = "/v2/app/start";
    fn start_json(code: String, app_id: i64) -> String {
        json!({"code":code, "app_id":app_id}).to_string()
    }
    async fn start(
        &self,
        code: String,
        app_id: i64,
    ) -> Result<ApiResponse<StartData>, ServiceError>;

    const END_URL: &'static str = "/v2/app/end";
    fn end_json(app_id: i64, game_id: String) -> String {
        json!({"app_id":app_id, "game_id":game_id}).to_string()
    }
    async fn end(&self, app_id: i64, game_id: String)
        -> Result<ApiResponse<EndData>, ServiceError>;

    const HEARTBEAT_URL: &'static str = "/v2/app/heartbeat";
    fn heartbeat_json(game_id: String) -> String {
        json!({"game_id":game_id}).to_string()
    }
    async fn heartbeat(&self, game_id: String) -> Result<ApiResponse<HeartBeatData>, ServiceError>;

    const BATCHHEARTBEAT_URL: &'static str = "/v2/app/batchHeartbeat";
    fn batch_heartbeat_json(game_ids: Vec<String>) -> String {
        json!({"game_ids":game_ids}).to_string()
    }
    async fn batch_heartbeat(
        &self,
        game_ids: Vec<String>,
    ) -> Result<ApiResponse<BatchHeartBeatData>, ServiceError>;
}

#[async_trait]
impl V2apis for ApiAgent {
    async fn start(
        &self,
        code: String,
        app_id: i64,
    ) -> Result<ApiResponse<StartData>, ServiceError> {
        let req_body = Self::start_json(code, app_id);
        let url = apiurl(Self::START_URL);
        let req = self.build_request(url, req_body);
        let res = req.send().await?.text().await?;
        Ok(serde_json::from_str::<ApiResponse<StartData>>(&res)?)
    }

    async fn end(
        &self,
        app_id: i64,
        game_id: String,
    ) -> Result<ApiResponse<EndData>, ServiceError> {
        let req_body = Self::end_json(app_id, game_id);
        let url = apiurl(Self::END_URL);
        let req = self.build_request(url, req_body);
        let res = req.send().await?.text().await?;
        Ok(serde_json::from_str::<ApiResponse<EndData>>(&res)?)
    }

    async fn heartbeat(&self, game_id: String) -> Result<ApiResponse<HeartBeatData>, ServiceError> {
        let req_body = Self::heartbeat_json(game_id);
        let url = apiurl(Self::HEARTBEAT_URL);
        let req = self.build_request(url, req_body);
        let res = req.send().await?.text().await?;
        Ok(serde_json::from_str::<ApiResponse<HeartBeatData>>(&res)?)
    }

    async fn batch_heartbeat(
        &self,
        game_ids: Vec<String>,
    ) -> Result<ApiResponse<BatchHeartBeatData>, ServiceError> {
        let req_body = Self::batch_heartbeat_json(game_ids);
        let url = apiurl(Self::BATCHHEARTBEAT_URL);
        let req = self.build_request(url, req_body);
        let res = req.send().await?.text().await?;
        Ok(serde_json::from_str::<ApiResponse<BatchHeartBeatData>>(
            &res,
        )?)
    }
}

#[cfg(test)]
mod tests {
    use super::{ApiResponse, StartData, V2apis};
    use crate::agent::ApiAgent;
    use serde_json::json;

    #[test]
    fn test_start_request_json() {
        let json = ApiAgent::start_json("code".to_string(), 1234567890123);
        println!("{}", json);
    }

    #[test]
    fn test_start_api_serde_json() {
        let start = ApiResponse::<StartData> {
            message: "ok".to_string(),
            ..Default::default()
        };
        let json_str = serde_json::to_string(&start).unwrap();
        println!("{}", json_str);
        let json_obj = json!({
            "code": 0,
            "message": "ok",
            "data": {
                //  场次信息
                "game_info": {
                    //  场次id,心跳key(心跳保持20s-60s)调用一次,超过60s无心跳自动关闭,长连停止推送消息
                    "game_id": ""
                },
                //  长连信息
                "websocket_info": {
                    //  长连使用的请求json体 第三方无需关注内容,建立长连时使用即可
                    "auth_body": "",
                    //  wss 长连地址
                    "wss_link": [""]
                },
                //  主播信息
                "anchor_info": {
                    // 主播房间号
                    "room_id": 0,
                    // 主播昵称
                    "uname": "",
                    // 主播头像
                    "uface": "",
                    // 主播uid
                    "uid": 0
                }
            }
        });
        let json_str = serde_json::to_string(&json_obj).unwrap();
        println!("{}", json_str);
    }
}
