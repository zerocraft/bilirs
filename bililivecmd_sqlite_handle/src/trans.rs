use bililivecmd::proto::*;
use sea_orm::Set;

use crate::entities::dm;

impl From<CDM> for dm::ActiveModel {
    fn from(v: CDM) -> Self {
        dm::ActiveModel {
            u_name: Set(Some(v.uname)),
            u_id: Set(Some(v.uid)),
            u_face: Set(Some(v.uface)),
            timestamp: Set(Some(v.timestamp)),
            room_id: Set(Some(v.room_id)),
            msg: Set(Some(v.msg)),
            msg_id: Set(Some(v.msg_id)),
            guard_level: Set(Some(v.guard_level)),
            fans_medal_wearing_status: Set(Some(v.fans_medal_wearing_status)),
            fans_medal_name: Set(Some(v.fans_medal_name)),
            fans_medal_level: Set(Some(v.fans_medal_level)),
            emoji_img_url: Set(Some(v.emoji_img_url)),
            dm_type: Set(Some(v.dm_type)),
            ..Default::default()
        }
    }
}
