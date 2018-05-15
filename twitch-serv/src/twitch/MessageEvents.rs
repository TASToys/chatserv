use std::option;

#[derive(Debug)]
pub enum Events {
    JOIN(String),
    PART(String),
    // NAMES(Vec<String>),
    PRIVMSG(PrivMsg),
    ROOMSTATE(RoomState),
    USERNOTICE(UserNotice),
    USERSTATE(UserState)
    
}

#[derive(Debug)]
pub struct PrivMsg {
    pub display_name:String,
    pub user_id:String,
    pub user_type:String,
    pub msg_id:String,
    pub room_id:String,
    pub color:String,
    pub badges:Vec<String>,
    pub emotes:Vec<String>,
    pub is_mod:bool,
    pub is_subscriber:bool,
    pub is_turbo:bool,
    pub message:String
}

#[derive(Debug)]
pub struct RoomState {
    pub broadcaster_lang:String,
    pub r9k:bool,
    pub slow:i32,
    pub is_sub_only:bool,
    pub channel:String
}

#[derive(Debug)]
pub struct UserNotice {
    badges:Vec<String>,
    color:String,
    display_name:String,
    emotes:Option<Vec<String>>,
    id:String,
    login:String,
    is_mod:bool,
    msg_id:MsgId,
    room_id:String,
    system_msg:String,
    tmi_sent_ts:String,
    is_turbo:bool,
    user_type:String
}

#[derive(Debug)]
pub enum MsgId {
    SUB(SubId),
    RESUB(ReSubId),
    SUBGIFT(SubGift),
    RAID(RaidId),
    RITUAL(RitualId)
}

#[derive(Debug)]
pub struct SubId {
    msg_param_months:String,
    msg_param_sub_plan:String,
    msg_param_sub_plan_name:String,
}

#[derive(Debug)]
pub struct ReSubId {
    msg_param_months:String,
    msg_param_sub_plan:String,
    msg_param_sub_plan_name:String,
}

#[derive(Debug)]
pub struct SubGift {
    msg_param_recipient_display_name:String,
    msg_param_recipient_id:String,
    msg_param_recipient_user_name:String
}

#[derive(Debug)]
pub struct RaidId {
    msg_param_display_name:String,
    msg_param_login:String,
    msg_param_viewer_count:i32,

}

#[derive(Debug)]
pub struct RitualId {
    msg_param_ritual_name:String
}

#[derive(Debug)]
pub struct UserState {
    badges:Vec<String>,
    color:String,
    display_name:Option<String>,
    emotes:Option<Vec<String>>,
    is_mod:bool,
    is_subscriber:bool,
    is_turbo:bool,
    user_type:String
}

impl Events {
    
}