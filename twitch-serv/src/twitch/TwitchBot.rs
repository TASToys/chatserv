extern crate tungstenite;
extern crate ttfmt;
extern crate url;

extern crate serde;
extern crate serde_json;


use self::url::Url;

use self::tungstenite::{Message, connect};

use self::ttfmt::ChatFmt;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub fn connect_to_twitch() {

    let (mut socket, response) = connect(Url::parse("wss://irc-ws.chat.twitch.tv").unwrap())
        .expect("Can't connect");

    for &(ref header, _ /*value*/) in response.headers.iter() {
        println!("* {}", header);
    }


    socket.write_message(Message::Text("CAP REQ :twitch.tv/tags twitch.tv/commands twitch.tv/membership".into())).unwrap();
    socket.write_message(Message::Text("NICK justinfan56468464".into())).unwrap();
    socket.write_message(Message::Text("JOIN #dwangoac".into())).unwrap();
    // socket.write_message(Message::Text("JOIN #grandpoobear".into())).unwrap();

    loop {

        let msg = socket.read_message().expect("Error reading messaguse tungstenite::{Message, connect};e");
        fmtMessage(msg)
    }
}

#[allow(non_snake_case)]
fn fmtMessage(msg:Message) {

    let string = msg.to_string();
    let splitMsg:Vec<&str> = string.split(';').collect();
    //Because rust is insane
    let sl2 = splitMsg.clone();
    let sl3 = splitMsg.clone();
//    let find = splitMsg.iter().position(|&p| p == "display-name").unwrap();

    let chat = ChatFmt::ChatFmt::new("twitch".to_string());

    println!("msg: {}", string);
    println!("{}", splitMsg.len());
    if splitMsg.len() != 1 && splitMsg.len() <= 13 {

        println!("{}|{}|{}", getBadges(splitMsg), getColor(sl2), getDispalyName(sl3));
        // chat = ChatFmt::ChatFmt::fmt_chat_msg("#dwangoac", getDispalyName(sl3), "", "")
    }
}

#[derive(Debug)]
struct TwitchMeta {
    badges:String,
    color:String,
    emotes:String,
    twitch_name:String,
    twitch_id:String,
    isSub:bool,
    isMod:bool,

}

#[allow(non_snake_case)]
fn twitchJson(msg:Vec<&str>) -> serde_json::Value {
    let twi = TwitchMeta {
        badges: getBadges(msg),
        color: getColor(msg),
        emotes: getEmote(msg),
        twitch_name: getEmote(msg)
    };
}

#[allow(non_snake_case)]
fn getBadges(msg:Vec<&str>) -> String {
    let newMsg = msg[0];
    return getAfterEquals(newMsg.to_string());
}


#[allow(non_snake_case)]
fn getColor(msg:Vec<&str>) -> String {
    let newMessage = msg[1];
    return getAfterEquals(newMessage.to_string());
}

#[allow(non_snake_case)]
fn getDisplayName(msg:Vec<&str>) -> String {
    let newMessage = msg[2];
    return getAfterEquals(newMessage.to_string());
}

#[allow(non_snake_case)]
fn getEmote(msg:Vec<&str>) -> String {
    let newMessage = msg[3];
    return getAfterEquals(newMessage.to_string());
}

fn getMessageId(msg:Vec<&str>) -> String {
    let newMessage = msg[4];
    return getAfterEquals(newMessage.to_string())
}

fn isMod(msg:Vec<&str>) -> bool {
    let newMessage = msg[5];

    if newMessage.to_string() == 1 {
        return true;
    }

    return false;
}

fn getRoomId(msg:Vec<&str>) -> String {
    let newMessage = msg[6];
    return getAfterEquals(newMessage.to_string());
}

fn isSub(msg:Vec<&str>) -> bool {
    let newMessage = msg[7];

    if newMessage.to_string() == 1 {
        return true;
    }

    return false;
}

fn getTwitchTime(msg:Vec<&str>) -> String {
    let newMessage = msg[8];
    return getAfterEquals(newMessage.to_string());
}

fn isTurbo(msg:Vec<&str>) -> bool {
    let newMessage = msg[9];

    if newMessage.to_string() == 1 {
        return true;
    }

    return false;
}

fn getUserId(msg:Vec<&str>) -> String {
    let newMessage = msg[10];
    return getAfterEquals(newMessage.to_string());
}

fn getTwitchMessage(msg:String) -> String {
    let newMessage: Vec<&str> = msg.split("PRIVMSG").collect();
    return newMessage[1];
}

fn getTwitchMessageSplit(msg:String) -> Vec<String> {
    let newMessage: Vec<&str> = msg.split(' ').collect();
    
}

#[allow(non_snake_case)]
fn getAfterEquals(msg:String) -> String {

    let eSplit:Vec<&str> = msg.split('=').collect();

    return eSplit[1].to_string();
}