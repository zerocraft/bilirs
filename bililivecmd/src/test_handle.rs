use crate::{
    handle::{LiveCmdHandle, LiveCmdHandleOP, LiveCmdHandleRAW},
    proto::*,
    CmdAgentParams,
};
use async_trait::async_trait;

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
