use async_trait::async_trait;

use crate::{proto::*, CmdAgentParams};

/// 解析后的Cmd处理
#[async_trait]
pub trait LiveCmdHandle: Send + Sync {
    async fn handle_dm(&self, cmd: CDM, params: CmdAgentParams);
    async fn handle_send_gift(&self, cmd: CSendGift, params: CmdAgentParams);
    async fn handle_super_chat(&self, cmd: CSuperChat, params: CmdAgentParams);
    async fn handle_super_chat_del(&self, cmd: CSuperChatDel, params: CmdAgentParams);
    async fn handle_guard(&self, cmd: CGuard, params: CmdAgentParams);
    async fn handle_like(&self, cmd: CLike, params: CmdAgentParams);
}

/// Proto数据处理
#[async_trait]
pub trait LiveCmdHandleOP: Send + Sync {
    async fn handle(&self, proto: RawProto, params: CmdAgentParams);
}

/// 原始数据处理
#[async_trait]
pub trait LiveCmdHandleRAW: Send + Sync {
    async fn handle(&self, bytes: Vec<u8>, params: CmdAgentParams);
}

#[derive(Default)]
pub struct TestHandler;

#[async_trait]
impl LiveCmdHandleRAW for TestHandler {
    async fn handle(&self, bytes: Vec<u8>, _params: CmdAgentParams) {
        println!("LiveCmdHandleRAW {:?}", bytes);
    }
}

#[async_trait]
impl LiveCmdHandleOP for TestHandler {
    async fn handle(&self, proto: RawProto, _params: CmdAgentParams) {
        println!("LiveCmdHandleOP {:?}", proto);
    }
}

#[async_trait]
impl LiveCmdHandle for TestHandler {
    async fn handle_dm(&self, cmd: CDM, _params: CmdAgentParams) {
        println!("handle_dm {:?}", cmd);
    }
    async fn handle_send_gift(&self, cmd: CSendGift, _params: CmdAgentParams) {
        println!("handle_send_gift {:?}", cmd);
    }
    async fn handle_super_chat(&self, cmd: CSuperChat, _params: CmdAgentParams) {
        println!("handle_super_chat {:?}", cmd);
    }
    async fn handle_super_chat_del(&self, cmd: CSuperChatDel, _params: CmdAgentParams) {
        println!("handle_super_chat_del {:?}", cmd);
    }
    async fn handle_guard(&self, cmd: CGuard, _params: CmdAgentParams) {
        println!("handle_guard {:?}", cmd);
    }
    async fn handle_like(&self, cmd: CLike, _params: CmdAgentParams) {
        println!("handle_like {:?}", cmd);
    }
}
