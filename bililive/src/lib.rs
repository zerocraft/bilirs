use agent::ApiAgent;
use apiv2::V2apis;
use auth::Auth;
use bililivecmd::{CmdAgent, CmdAgentParams};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread::{self};
use tokio::sync::Mutex;
use tokio::time::Duration;

pub mod agent;
pub mod apiv2;
pub mod auth;
pub mod error;

pub struct ApiService {
    api_agnet: Arc<ApiAgent>,
    game_rooms: Arc<Mutex<HashMap<String, GameRoom>>>,
}

#[derive(Debug, Clone)]
pub struct GameRoom {
    app_id: i64,
    game_id: String,
}

impl ApiService {
    pub fn new(auth: Auth) -> Self {
        Self {
            game_rooms: Arc::new(Mutex::new(HashMap::new())),
            api_agnet: Arc::new(ApiAgent::new(auth)),
        }
    }

    /// 开启服务
    pub async fn service_start(&mut self) {
        let game_rooms = Arc::clone(&self.game_rooms);
        let api_agent = Arc::clone(&self.api_agnet);
        let future = async move {
            loop {
                let rooms_clone = game_rooms.lock().await.clone();
                let room_len = rooms_clone.len();
                match room_len {
                    1 => {
                        if let Some(keys) = rooms_clone.keys().next() {
                            let hr = api_agent.heartbeat(keys.clone()).await;
                            if let Ok(res) = hr {
                                println!("heartbeat\n room:{:?}\n rescode:{}", keys, res.code);
                            }
                        }
                    }
                    2.. => {
                        let keys = rooms_clone.keys().cloned().collect::<Vec<String>>();
                        let hr = api_agent.batch_heartbeat(keys).await;
                        if let Ok(res) = hr {
                            println!(
                                "batch_heartbeat\n rooms:{:?}\n rescode:{}",
                                rooms_clone, res.code
                            );
                        }
                    }
                    _ => {
                        println!("no rooms");
                    }
                }
                tokio::time::sleep(Duration::from_secs(20)).await;
            }
        };
        tokio::spawn(future);
    }

    pub async fn new_project(&mut self, code: String, app_id: i64) -> Option<CmdAgent> {
        let res = self.api_agnet.start(code.clone(), app_id).await;
        if let Ok(resp) = res {
            if let Some(data) = resp.data {
                let game_id = data.game_info.game_id;
                if !game_id.is_empty() {
                    self.game_rooms
                        .lock()
                        .await
                        .insert(game_id.clone(), GameRoom { app_id, game_id });
                }
                println!("{:?}", data.websocket_info);
                return Some(CmdAgent::new(CmdAgentParams {
                    auth_body: data.websocket_info.auth_body,
                    server_url: data.websocket_info.wss_link.first().unwrap().clone(),
                    app_id,
                    user_code: code,
                }));
            }
        }
        None
    }

    pub async fn stop_project(&mut self, game_id: String) {
        if let Some(game_room) = self.game_rooms.lock().await.remove(&game_id) {
            let res = self
                .api_agnet
                .end(game_room.app_id, game_room.game_id.clone())
                .await;
            if let Ok(resp) = res {
                println!("{:?} {:?}", game_room, resp);
            }
        }
    }

    pub async fn stop_all_projects(&mut self) {
        stop_all_projects(self);
    }
}

impl Drop for ApiService {
    fn drop(&mut self) {
        println!("droping...");
        stop_all_projects(self);
        println!("droped");
    }
}

fn stop_all_projects(service: &mut ApiService) {
    let agent = Arc::clone(&service.api_agnet);
    let rooms = Arc::clone(&service.game_rooms);
    thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let binding = rooms.lock().await;
            let keys = binding.keys().clone();
            for k in keys {
                if let Some(game_room) = rooms.lock().await.remove(k) {
                    let _ = agent.end(game_room.app_id, game_room.game_id).await;
                }
            }
        });
    });
}

#[allow(unused)]
fn env_access_key() -> String {
    dotenvy::var("ACCESS_KEY").unwrap()
}

#[allow(unused)]
fn env_access_secret() -> String {
    dotenvy::var("ACCESS_SECRET").unwrap()
}

#[allow(unused)]
fn env_live_code() -> String {
    dotenvy::var("LIVE_CODE").unwrap()
}

#[allow(unused)]
fn env_app_id() -> i64 {
    dotenvy::var("APP_ID").unwrap().parse::<i64>().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{
        auth::Auth, env_access_key, env_access_secret, env_app_id, env_live_code, ApiService,
    };
    use bililivecmd::handle::TestHandler;
    use std::sync::Arc;
    use tokio::time::Duration;

    #[tokio::test]
    async fn it_works() {
        let body = reqwest::get("https://space.bilibili.com/14523660/")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        println!("{:?}", body);
    }

    #[tokio::test]
    async fn test_service_start() {
        // 使用AccessKey和Secret建立服务
        let mut service = ApiService::new(Auth::new(env_access_key(), env_access_secret()));
        // 使用直播code和app_id开启项目，在启动服务后会自动按频率发送心跳
        // 认证成功同时还会创建一个长连接代理（可独立工作）
        let mut agent = service
            .new_project(env_live_code(), env_app_id())
            .await
            .unwrap();
        // 为长连接代理添加处理对象 （可选择性 是否需要处理层 或 多个处理层） raw -> proto -> cmd
        let handle = Arc::new(TestHandler::default());
        // // 处理原始字符串
        // let raw = Arc::clone(&handle);
        // //
        // agent.raw_handles.lock().await.push(raw);
        // // 处理proto对象
        // let op = Arc::clone(&handle);
        // agent.op_handles.lock().await.push(op);
        // 处理弹幕消息包（Proto.Operation==5）
        let cmd = Arc::clone(&handle);
        agent.cmd_handles.lock().await.push(cmd);
        // 启动长连接代理
        agent.start().await;
        // 启动服务（用于自动发送项目心跳）,正常退出时会自动调用end api
        service.service_start().await;

        //主进程保持运行
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
