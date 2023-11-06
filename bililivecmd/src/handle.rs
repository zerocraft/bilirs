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
