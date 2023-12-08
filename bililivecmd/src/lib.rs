use flate2::write::ZlibDecoder;
use futures::{stream::SplitSink, SinkExt, StreamExt};
use handle::{LiveCmdHandle, LiveCmdHandleOP, LiveCmdHandleRAW};
use proto::{
    CGuard, CLike, CSendGift, CSuperChat, CSuperChatDel, LiveOpenPlatformCmd, RawProto, CDM,
    LIVE_OPEN_PLATFORM_GUARD, LIVE_OPEN_PLATFORM_LIKE, LIVE_OPEN_PLATFORM_SEND_GIFT,
    LIVE_OPEN_PLATFORM_SUPER_CHAT, LIVE_OPEN_PLATFORM_SUPER_CHAT_DEL,
};
use serde_json::Value;
use std::io::prelude::*;
use std::sync::Arc;
use tokio::{net::TcpStream, sync::RwLock, time::Duration};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{http::Uri, Message},
    MaybeTlsStream, WebSocketStream,
};

use crate::proto::LIVE_OPEN_PLATFORM_DM;

pub mod handle;
pub mod proto;
pub mod test_handle;

pub struct CmdAgent {
    is_working: bool,
    pub params: CmdAgentParams,
    pub raw_handles: Arc<RwLock<Vec<Arc<dyn LiveCmdHandleRAW>>>>,
    pub op_handles: Arc<RwLock<Vec<Arc<dyn LiveCmdHandleOP>>>>,
    pub cmd_handles: Arc<RwLock<Vec<Arc<dyn LiveCmdHandle>>>>,
}

#[derive(Debug, Clone, Default)]
pub struct CmdAgentParams {
    pub auth_body: String,
    pub server_url: String,
    pub app_id: i64,
    pub user_code: String,
}

type Writer = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

impl CmdAgent {
    pub fn new(params: CmdAgentParams) -> Self {
        CmdAgent {
            params,
            is_working: false,
            raw_handles: Arc::new(RwLock::new(Vec::new())),
            op_handles: Arc::new(RwLock::new(Vec::new())),
            cmd_handles: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn is_working(&self) -> bool {
        self.is_working
    }

    pub async fn start(&mut self) {
        //构建websocket客户端
        let server_uri = match Uri::try_from(self.params.server_url.clone()) {
            Ok(uri) => uri,
            Err(e) => {
                eprintln!("Invalid URI {} {:?}", e, self.params);
                return;
            }
        };
        let (ws_stream, _) = match connect_async(server_uri).await {
            Ok(result) => result,
            Err(e) => {
                eprintln!("WebSocket connection failed {} {:?}", e, self.params);
                return;
            }
        };
        let (writer, read) = ws_stream.split();
        // 接收消息
        let raw_handles = Arc::clone(&self.raw_handles);
        let op_handles = Arc::clone(&self.op_handles);
        let cmd_handles = Arc::clone(&self.cmd_handles);
        let agent_params = self.params.clone();
        tokio::spawn(async move {
            let mut reader = read;
            while let Some(message) = reader.next().await {
                match message {
                    Ok(msg) => {
                        if let Message::Binary(bytes) = msg {
                            let raw_handles = Arc::clone(&raw_handles);
                            let op_handles = Arc::clone(&op_handles);
                            let cmd_handles = Arc::clone(&cmd_handles);
                            handle(
                                bytes,
                                &raw_handles,
                                &op_handles,
                                &cmd_handles,
                                agent_params.clone(),
                            )
                            .await;
                        } else if let Message::Ping(_p) = msg {
                        } else {
                            eprintln!("No Binary Data {:?}", msg);
                        }
                    }
                    Err(e) => eprintln!("Failed to receive {e}"),
                }
            }
        });
        // 发送AUTH包
        let writer = self.send_auth(writer).await;
        if writer.is_err() {
            eprintln!("Failed auth {:?}", self.params);
            return;
        }
        // 发送心跳
        let mut writer = writer.unwrap();
        let agent_params = self.params.clone();
        tokio::spawn(async move {
            loop {
                println!("cmd heartbeat");
                let proto = RawProto::new(2, Vec::new());
                let result = writer.send(Message::Binary(proto.clone().into())).await;
                if result.is_err() {
                    eprintln!("Failed to send message {:?} {:?}", proto, agent_params);
                }
                tokio::time::sleep(Duration::from_secs(30)).await;
            }
        });
        //正常运行标识
        self.is_working = true;
    }

    async fn send_auth(
        &self,
        mut write: Writer,
    ) -> Result<Writer, tokio_tungstenite::tungstenite::Error> {
        write
            .send(Message::Binary(
                RawProto::new(7, self.params.auth_body.clone().as_bytes().to_vec()).into(),
            ))
            .await?;
        Ok(write)
    }
}

///消息处理
#[allow(unused_must_use, clippy::let_underscore_future)]
async fn handle(
    bytes: Vec<u8>,
    raw_handles: &Arc<RwLock<Vec<Arc<dyn LiveCmdHandleRAW>>>>,
    op_handles: &Arc<RwLock<Vec<Arc<dyn LiveCmdHandleOP>>>>,
    cmd_handles: &Arc<RwLock<Vec<Arc<dyn LiveCmdHandle>>>>,
    params: CmdAgentParams,
) {
    //处理原始数据
    for raw in raw_handles.read().await.iter() {
        let bytes = bytes.clone();
        let params = params.clone();
        raw.handle(bytes, params).await;
    }
    //处理Proto数据
    if let Ok(proto) = RawProto::try_from(bytes) {
        if proto.version == 2 {
            //处理压缩
            let mut writer = Vec::new();
            let mut z = ZlibDecoder::new(writer);
            let r = z.write_all(&proto.body);
            if r.is_err() {
                return;
            }
            let r = z.finish();
            if r.is_err() {
                return;
            }
            writer = r.unwrap();
            //递归消息处理
            handle(writer, raw_handles, op_handles, cmd_handles, params.clone());
            return;
        }
        for op in op_handles.read().await.iter() {
            let proto: RawProto = proto.clone();
            let params = params.clone();
            op.handle(proto, params).await;
        }
        //弹幕消息包
        if proto.operation == 5 {
            //处理解析后的Cmd
            match String::from_utf8(proto.body) {
                Ok(json) => match serde_json::from_str::<Value>(&json) {
                    Ok(v) => {
                        if let Some((_, v)) = v.as_object().and_then(|m| m.iter().next()) {
                            if let Some(cmd) = v.as_str() {
                                match cmd {
                                    LIVE_OPEN_PLATFORM_DM => {
                                        if let Ok(pcmd) =
                                            serde_json::from_str::<LiveOpenPlatformCmd<CDM>>(&json)
                                        {
                                            for handle in cmd_handles.read().await.iter() {
                                                let params = params.clone();
                                                handle.handle_dm(pcmd.data.clone(), params).await;
                                            }
                                        }
                                    }
                                    LIVE_OPEN_PLATFORM_SEND_GIFT => {
                                        if let Ok(pcmd) =
                                            serde_json::from_str::<LiveOpenPlatformCmd<CSendGift>>(
                                                &json,
                                            )
                                        {
                                            for handle in cmd_handles.read().await.iter() {
                                                let params = params.clone();
                                                handle
                                                    .handle_send_gift(pcmd.data.clone(), params)
                                                    .await;
                                            }
                                        }
                                    }
                                    LIVE_OPEN_PLATFORM_SUPER_CHAT => {
                                        if let Ok(pcmd) =
                                            serde_json::from_str::<LiveOpenPlatformCmd<CSuperChat>>(
                                                &json,
                                            )
                                        {
                                            for handle in cmd_handles.read().await.iter() {
                                                let params = params.clone();
                                                handle
                                                    .handle_super_chat(pcmd.data.clone(), params)
                                                    .await;
                                            }
                                        }
                                    }
                                    LIVE_OPEN_PLATFORM_SUPER_CHAT_DEL => {
                                        if let Ok(pcmd) = serde_json::from_str::<
                                            LiveOpenPlatformCmd<CSuperChatDel>,
                                        >(
                                            &json
                                        ) {
                                            for handle in cmd_handles.read().await.iter() {
                                                let params = params.clone();
                                                handle
                                                    .handle_super_chat_del(
                                                        pcmd.data.clone(),
                                                        params,
                                                    )
                                                    .await;
                                            }
                                        }
                                    }
                                    LIVE_OPEN_PLATFORM_GUARD => {
                                        if let Ok(pcmd) =
                                            serde_json::from_str::<LiveOpenPlatformCmd<CGuard>>(
                                                &json,
                                            )
                                        {
                                            for handle in cmd_handles.read().await.iter() {
                                                let params = params.clone();
                                                handle
                                                    .handle_guard(pcmd.data.clone(), params)
                                                    .await;
                                            }
                                        }
                                    }
                                    LIVE_OPEN_PLATFORM_LIKE => {
                                        if let Ok(pcmd) =
                                            serde_json::from_str::<LiveOpenPlatformCmd<CLike>>(
                                                &json,
                                            )
                                        {
                                            for handle in cmd_handles.read().await.iter() {
                                                let params = params.clone();
                                                handle.handle_like(pcmd.data.clone(), params).await;
                                            }
                                        }
                                    }
                                    _ => eprintln!("Unkonw Cmd {cmd}"),
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("Json Decode Error {e} {json}"),
                },
                Err(e) => eprintln!("Body From utf8 Error {e}"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_handle::TestHandler, CmdAgent, CmdAgentParams};
    use std::sync::Arc;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_agent() {
        let mut agent = CmdAgent::new(CmdAgentParams {
            auth_body: "".to_string(),
            server_url: "".to_string(),
            ..Default::default()
        });
        let handle = Arc::new(TestHandler {});
        let raw = Arc::clone(&handle);
        agent.raw_handles.write().await.push(raw);
        let op = Arc::clone(&handle);
        agent.op_handles.write().await.push(op);
        let cmd = Arc::clone(&handle);
        agent.cmd_handles.write().await.push(cmd);
        agent.start().await;
        loop {
            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    }

    #[tokio::test]
    async fn mult_test() {
        tokio::spawn(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3)).await;
                println!("loop1 turn");
                for i in 0..2 {
                    tokio::spawn(async move {
                        println!("inner:{}", i);
                    });
                }
                eprintln!("loop failed something!");
                println!("loop1 turn end");
            }
        });
        tokio::spawn(async {
            loop {
                tokio::time::sleep(Duration::from_secs(10)).await;
                println!("loop2 turn");
                println!("loop2 end");
            }
        });
        for _ in 0..2 {
            println!("main loop");
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    }
}
