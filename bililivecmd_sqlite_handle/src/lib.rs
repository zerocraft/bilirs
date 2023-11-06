use async_trait::async_trait;
use bililivecmd::{handle::LiveCmdHandle, proto::*, CmdAgentParams};
use entities::dm;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection};

pub mod entities;
pub mod trans;

fn env_connect_str() -> String {
    dotenvy::var("DATABASE_URL").unwrap()
}

#[derive(Default)]
pub struct SqliteHandler {
    pub console_saved: bool,
    db: DatabaseConnection,
}

impl SqliteHandler {
    pub async fn new(cs: Option<String>) -> Self {
        let cs = match cs {
            Some(s) => s,
            None => env_connect_str(),
        };
        Self {
            console_saved: false,
            db: Database::connect(cs).await.unwrap(),
        }
    }
}

#[async_trait]
impl LiveCmdHandle for SqliteHandler {
    async fn handle_dm(&self, cmd: CDM, _params: CmdAgentParams) {
        let new: dm::ActiveModel = cmd.into();
        let r = new.insert(&self.db).await;
        if self.console_saved && r.is_ok() {
            println!("saved dm:{:?}", r.unwrap());
        }
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

#[cfg(test)]
mod tests {
    use sea_orm::{ActiveModelTrait, Database};

    use crate::{entities::dm, env_connect_str};

    #[tokio::test]
    async fn test_save() {
        let cs = env_connect_str();
        println!("{}", cs);
        let db = Database::connect(cs).await.unwrap();
        for i in 1..10usize {
            let new = dm::ActiveModel {
                ..Default::default()
            };
            let result = new.insert(&db).await.unwrap();
            println!("{i}:{:?}", result);
        }
    }
}
