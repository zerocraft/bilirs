use serde::{Deserialize, Serialize};

/// OP_HEARTBEAT : 2 客户端发送的心跳包(30秒发送一次)
///
/// OP_AUTH : 7 客户端发送的鉴权包(客户端发送的第一个包)
///
/// OP_HEARTBEAT_REPLY : 3 服务器收到心跳包的回复
///
/// OP_SEND_SMS_REPLY : 5 服务器推送的弹幕消息包
///
/// OP_AUTH_REPLY : 8 服务器收到鉴权包后的回复
///
#[derive(Default, Debug, Clone)]
pub struct RawProto {
    pub packet_length: u32,
    pub header_length: u16,
    pub version: u16,
    pub operation: u32,
    pub sequence_id: u32,
    pub body: Vec<u8>,
}

impl RawProto {
    pub fn new(operation: u32, body: Vec<u8>) -> Self {
        let packet_length = 16 + body.len() as u32;
        Self {
            packet_length,
            header_length: 16,
            operation,
            body,
            ..Default::default()
        }
    }
}

impl TryFrom<Vec<u8>> for RawProto {
    fn try_from(raw: Vec<u8>) -> Result<Self, Self::Error> {
        if raw.len() < 16 {
            println!("Error raw data:{:?}", raw);
            return Err("Error raw data!");
        }
        let packet_length = u32::from_be_bytes(raw[0..4].try_into().unwrap());
        let header_length = u16::from_be_bytes(raw[4..6].try_into().unwrap());
        let version = u16::from_be_bytes(raw[6..8].try_into().unwrap());
        let operation = u32::from_be_bytes(raw[8..12].try_into().unwrap());
        let sequence_id = u32::from_be_bytes(raw[12..16].try_into().unwrap());
        let body = raw[16..].to_vec();
        Ok(RawProto {
            packet_length,
            header_length,
            version,
            operation,
            sequence_id,
            body,
        })
    }

    type Error = &'static str;
}

impl From<RawProto> for Vec<u8> {
    fn from(mut p: RawProto) -> Self {
        p.packet_length = 16 + p.body.len() as u32;
        let mut result = Vec::new();
        result.extend(p.packet_length.to_be_bytes());
        result.extend(p.header_length.to_be_bytes());
        result.extend(p.version.to_be_bytes());
        result.extend(p.operation.to_be_bytes());
        result.extend(p.sequence_id.to_be_bytes());
        result.extend(p.body.iter());
        result
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LiveOpenPlatformCmd<T>
where
    T: Serialize + Default,
{
    pub cmd: String,
    pub data: T,
}

pub const LIVE_OPEN_PLATFORM_DM: &str = "LIVE_OPEN_PLATFORM_DM";
/// LIVE_OPEN_PLATFORM_DM
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CDM {
    pub uname: String,                   // 用户昵称
    pub uid: i64,                        // 用户UID
    pub uface: String,                   // 用户头像
    pub timestamp: i64,                  // 弹幕发送时间秒级时间戳
    pub room_id: i64,                    // 弹幕接收的直播间
    pub msg: String,                     // 弹幕内容
    pub msg_id: String,                  // 消息唯一id
    pub guard_level: i64,                // 对应房间大航海等级
    pub fans_medal_wearing_status: bool, // 该房间粉丝勋章佩戴情况
    pub fans_medal_name: String,         // 粉丝勋章名
    pub fans_medal_level: i64,           // 对应房间勋章信息
    pub emoji_img_url: String,           // 表情包图片地址
    pub dm_type: i64,                    // 弹幕类型 0：普通弹幕 1：表情包弹幕
}

pub const LIVE_OPEN_PLATFORM_SEND_GIFT: &str = "LIVE_OPEN_PLATFORM_SEND_GIFT";
/// LIVE_OPEN_PLATFORM_SEND_GIFT
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CSendGift {
    pub room_id: i64,                    // 房间号
    pub uid: i64,                        // 送礼用户UID
    pub uname: String,                   // 送礼用户昵称
    pub uface: String,                   // 送礼用户头像
    pub gift_id: i64,                    // 道具id(盲盒:爆出道具id)
    pub gift_name: String,               // 道具名(盲盒:爆出道具名)
    pub gift_num: i64,                   // 赠送道具数量
    pub price: i64,                      // 礼物爆出单价，(1000 = 1元 = 10电池),盲盒:爆出道具的价值
    pub paid: bool,                      // 是否是付费道具
    pub fans_medal_level: i64,           // 实际送礼人的勋章信息
    pub fans_medal_name: String,         // 粉丝勋章名
    pub fans_medal_wearing_status: bool, // 该房间粉丝勋章佩戴情况
    pub guard_level: i64,                // 大航海等级
    pub timestamp: i64,                  // 收礼时间秒级时间戳
    pub anchor_info: CAnchorInfo,        // 结构体 主播信息
    pub msg_id: String,                  // 消息唯一id
    pub gift_icon: String,               // 道具icon
    pub combo_gift: bool,                // 是否是combo道具
    pub combo_info: CComboInfo,          // 结构体 连击信息
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CAnchorInfo {
    pub uid: i64,      // 收礼主播uid
    pub uname: String, // 收礼主播昵称
    pub uface: String, // 收礼主播头像
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CComboInfo {
    pub combo_base_num: i64, // 每次连击赠送的道具数量
    pub combo_count: i64,    // 连击次数
    pub combo_id: String,    // 连击id
    pub combo_timeout: i64,  // 连击有效期秒
}

pub const LIVE_OPEN_PLATFORM_SUPER_CHAT: &str = "LIVE_OPEN_PLATFORM_SUPER_CHAT";
/// LIVE_OPEN_PLATFORM_SUPER_CHAT
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CSuperChat {
    pub room_id: i64,                    // 直播间id
    pub uid: i64,                        // 购买用户UID
    pub uname: String,                   // 购买的用户昵称
    pub uface: String,                   // 购买用户头像
    pub message_id: i64,                 // 留言id(风控场景下撤回留言需要)
    pub message: String,                 // 留言内容
    pub rmb: i64,                        // 支付金额(元)
    pub timestamp: i64,                  // 赠送时间秒级
    pub start_time: i64,                 // 生效开始时间
    pub end_time: i64,                   // 生效结束时间
    pub guard_level: i64,                // 对应房间大航海等级
    pub fans_medal_level: i64,           // 对应房间勋章信息
    pub fans_medal_name: String,         // 对应房间勋章名字
    pub fans_medal_wearing_status: bool, // 该房间粉丝勋章佩戴情况
    pub msg_id: String,                  // 消息唯一id
}

pub const LIVE_OPEN_PLATFORM_SUPER_CHAT_DEL: &str = "LIVE_OPEN_PLATFORM_SUPER_CHAT_DEL";
/// LIVE_OPEN_PLATFORM_SUPER_CHAT_DEL
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CSuperChatDel {
    pub room_id: i64,          // 直播间id
    pub message_ids: Vec<i64>, // 留言id
    pub msg_id: String,        // 消息唯一id
}

pub const LIVE_OPEN_PLATFORM_GUARD: &str = "LIVE_OPEN_PLATFORM_GUARD";
/// LIVE_OPEN_PLATFORM_GUARD
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CGuard {
    pub user_info: CUserInfo,            // 用户信息
    pub guard_level: i64,                // 大航海等级
    pub guard_num: i64,                  // 大航海数量
    pub guard_unit: String,              // 大航海单位
    pub fans_medal_level: i64,           // 粉丝勋章等级
    pub fans_medal_name: String,         // 粉丝勋章名
    pub fans_medal_wearing_status: bool, // 该房间粉丝勋章佩戴情况
    pub room_id: i64,                    // 房间号
    pub msg_id: String,                  // 消息唯一id
    pub timestamp: i64,                  // 上舰时间秒级时间戳
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CUserInfo {
    pub uid: i64,      // 用户uid
    pub uname: String, // 用户昵称
    pub uface: String, // 用户头像
}

pub const LIVE_OPEN_PLATFORM_LIKE: &str = "LIVE_OPEN_PLATFORM_LIKE";
/// LIVE_OPEN_PLATFORM_LIKE
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CLike {
    pub uname: String,                   // 用户昵称
    pub uid: i64,                        // 用户UID
    pub uface: String,                   // 用户头像
    pub timestamp: i64,                  // 时间秒级时间戳
    pub room_id: i64,                    // 发生的直播间
    pub like_text: String,               // 点赞文案( “xxx点赞了”)
    pub like_conut: i64,                 // 对单个用户最近2秒的点赞次数聚合
    pub fans_medal_wearing_status: bool, // 该房间粉丝勋章佩戴情况
    pub fans_medal_name: String,         // 粉丝勋章名
    pub fans_medal_level: i64,           // 对应房间勋章信息
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trans() {
        let proto = RawProto::new(2, Vec::new());
        println!("{:?}", &proto);
        let bytes: Vec<u8> = proto.into();
        println!("{:?}", bytes);
        let mut proto: RawProto = bytes.try_into().unwrap();
        println!("{:?}", &proto);
        proto.operation = 7;
        proto.body = "{json:0}".to_string().as_bytes().to_vec();
        println!("{:?}", &proto);
        let bytes: Vec<u8> = proto.into();
        println!("{:?}", bytes);
        let proto: RawProto = bytes.try_into().unwrap();
        println!("{:?}", &proto);
    }

    #[test]
    fn test_data_dm() {
        let data = LiveOpenPlatformCmd {
            cmd: LIVE_OPEN_PLATFORM_DM.to_string(),
            data: CDM::default(),
        };
        println!("{:?}", data);
        let data = serde_json::to_string_pretty(&data).unwrap();
        println!("{}", data);
    }
    #[test]
    fn test_data_send_gift() {
        let data = LiveOpenPlatformCmd {
            cmd: LIVE_OPEN_PLATFORM_SEND_GIFT.to_string(),
            data: CSendGift::default(),
        };
        println!("{:?}", data);
        let data = serde_json::to_string_pretty(&data).unwrap();
        println!("{}", data);
    }

    #[test]
    fn test_data_super_chat() {
        let data = LiveOpenPlatformCmd {
            cmd: LIVE_OPEN_PLATFORM_SUPER_CHAT.to_string(),
            data: CSuperChat::default(),
        };
        println!("{:?}", data);
        let data = serde_json::to_string_pretty(&data).unwrap();
        println!("{}", data);
    }

    #[test]
    fn test_data_super_chat_del() {
        let data = LiveOpenPlatformCmd {
            cmd: LIVE_OPEN_PLATFORM_SUPER_CHAT_DEL.to_string(),
            data: CSuperChatDel::default(),
        };
        println!("{:?}", data);
        let data = serde_json::to_string_pretty(&data).unwrap();
        println!("{}", data);
    }

    #[test]
    fn test_data_guard() {
        let data = LiveOpenPlatformCmd {
            cmd: LIVE_OPEN_PLATFORM_GUARD.to_string(),
            data: CGuard::default(),
        };
        println!("{:?}", data);
        let data = serde_json::to_string_pretty(&data).unwrap();
        println!("{}", data);
    }

    #[test]
    fn test_data_like() {
        let data = LiveOpenPlatformCmd {
            cmd: LIVE_OPEN_PLATFORM_LIKE.to_string(),
            data: CLike::default(),
        };
        println!("{:?}", data);
        let data = serde_json::to_string_pretty(&data).unwrap();
        println!("{}", data);
    }
}
