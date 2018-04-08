extern crate tungstenite;
extern crate ttfmt;

extern crate url;

use self::url::Url;

use self::tungstenite::{Message, connect};

use std::vec;
use std::collections::HashMap;

use self::ttfmt::FidoFmt;

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
//    let find = splitMsg.iter().position(|&p| p == "display-name").unwrap();

    println!("msg: {}", string);
    println!("{}", splitMsg.len());

    if splitMsg.len() != 1 && splitMsg.len() <= 13 {
       println!("{}", getBadges(splitMsg))
    }
}


fn getBadges(msg:Vec<&str>) -> String {
    let mut newMsg = msg[0];

    return newMsg.to_string();
}


fn getColor(msg:Vec<&str>) -> String {
    let mut newMessage = msg[1];
    let mut toTrim:String = newMessage.to_string();
    return toTrim.trim_left_matches('=').to_string();
}

fn getDispalyName(msg:Vec<&str>) -> String {
    let mut newMessage = msg[2];
    let mut toTrim:String = newMessage.to_string();
    return toTrim.trim_left_matches('=').to_string();
}

fn getEmote(msg:Vec<&str>) -> String {
    let mut newMessage = msg[3];
    let mut toTrim:String = newMessage.to_string();
    return toTrim.trim_left_matches('=').to_string();
}



// #[allow(non_snake_case)]
// fn getMsgKey(key:String) -> String {
//     let mkey:String = key.join("=").concat();
//     return "".to_string();
// }