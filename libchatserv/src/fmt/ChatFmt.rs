extern crate serde;
extern crate serde_json;

use std::vec;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
pub struct ChatFmt {
    platform:String,
    channel:String,
    timestamp:String,
    sender:String,
    message:String,
    split_msg:Vec<String>,
}

impl ChatFmt {
    pub fn new(platform:String) -> ChatFmt {
        ChatFmt {
            platform: platform,
            channel: String::new(),
            timestamp: String::new(),
            sender: String::new(),
            message: String::new(),
            split_msg: Vec::new(),
        }
    }

    pub fn format_chat_msg(mut self, channel:String, sender:String, msg:String) -> ChatFmt {
        self.channel = channel;
        let time = SystemTime::now();
        let unixtime = time.duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.timestamp = unixtime.to_string();
        self.sender = sender;
        self.message = msg;
        self.split_msg = self.message.split(" ").map(|s| s.to_string()).collect();

        self


    }

    pub fn build(self) -> ChatFmt {
        ChatFmt {
            platform: self.platform,
            channel: self.channel,
            timestamp: self.timestamp,
            sender: self.sender,
            message: self.message,
            split_msg: self.split_msg,
        }
    }
}

pub fn toJson(msg: ChatFmt) -> String {

    let fmt = ChatFmt {
        platform: msg.platform,
        channel: msg.channel,
        timestamp: msg.timestamp,
        sender: msg.sender,
        message: msg.message,
        split_msg: msg.split_msg,
    };

    let out = serde_json::to_string(&fmt).expect("error");

    out
}