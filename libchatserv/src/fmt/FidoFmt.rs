extern crate serde;
extern crate serde_json;

use std::vec;

#[derive(Serialize, Deserialize)]
pub struct FidoFmt {
    plugin_id: i32,
    plugin_type: i32,
    source_platform: String,
    source_channel: String,
    timestamp: String,
    source_user: String,
    data: Vec<serde_json::Value>,
}

impl FidoFmt {
    pub fn create_message(plugin_id:i32, plugin_type:i32, source_platform:String, source_channel: String,
                          timestamp: String, source_user: String, data: Vec<serde_json::Value>) -> FidoFmt {

        FidoFmt {
            plugin_id: plugin_id,
            plugin_type: plugin_type,
            source_platform: source_platform,
            source_channel: source_channel,
            timestamp: timestamp,
            source_user: source_user,
            data: data,
        }
    }
}

pub fn toJson(fido: FidoFmt) -> String {
    let fidostr = serde_json::to_string(&fido).expect("message not found");

    return fidostr;
}
