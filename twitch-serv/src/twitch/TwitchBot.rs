extern crate tungstenite;
extern crate ttfmt;
extern crate url;

extern crate serde;
extern crate serde_json;


use self::url::Url;

// mod MessageEvents;

use twitch::MessageEvents;

use self::tungstenite::{Message, connect};

pub fn connect_to_twitch() {

    let (mut socket, response) = connect(Url::parse("wss://irc-ws.chat.twitch.tv").unwrap())
        .expect("Can't connect");

    for &(ref header, _ /*value*/) in response.headers.iter() {
        println!("* {}", header);
    }


    socket.write_message(Message::Text("CAP REQ :twitch.tv/tags twitch.tv/commands twitch.tv/membership".into())).unwrap();
    socket.write_message(Message::Text("NICK justinfan56468464".into())).unwrap();
//    socket.write_message(Message::Text("JOIN #dwangoac".into())).unwrap();
    socket.write_message(Message::Text("JOIN #illyohs".into())).unwrap();
    socket.write_message(Message::Text("JOIN #Kitboga".into())).unwrap();
    socket.write_message(Message::Text("JOIN #grandpoobear".into())).unwrap();

    loop {

        let msg = socket.read_message().expect("Error reading messaguse tungstenite::{Message, connect};e");
        println!("{}", msg);
        fmtMessage(msg);
    }
}


#[allow(non_snake_case)]
fn fmtMessage(msg:Message) {

    let string = msg.to_string();
    let ostr = msg.to_string();
    let splitMsg:Vec<&str> = string.split(';').collect();
    let chansplit:Vec<&str> = ostr.split('#').collect();


    if string.contains("ROOMSTATE") {
        let stateMsg = stateFmt(splitMsg, chansplit[0].to_string());

        println!("{:?}", stateMsg);
    } else if string.contains("PRIVMSG") {
        let ps = privmsgFmt(splitMsg, chansplit.clone()[1].to_string());
        println!("{:?}", ps);
    }
}

fn stateFmt(msg:Vec<&str>, channel:String) -> MessageEvents::RoomState {
    let lang = getAfterEquals(msg[0].to_string());
    let mut isR9k:bool = false;
    let mut isSubOnly:bool = false ;
    let slow = getAfterEquals(msg.clone()[6].to_string()).parse::<i32>().unwrap();

    if getAfterEquals(msg.clone()[3].to_string()) != "0".to_string() {
        isR9k = true;
    }


    if getAfterEquals(msg.clone()[3].to_string()) != "0".to_string() {
        isSubOnly = true;
    }
    let state = MessageEvents::RoomState {
        broadcaster_lang: lang,
        r9k: isR9k,
        slow: slow,
        is_sub_only: isSubOnly,
        channel: "".to_string()
    };

    return state;
}

fn privmsgFmt(msg:Vec<&str>, channel:String) -> MessageEvents::PrivMsg {

    let bs = getAfterEquals(msg[0].to_string());
    let badges:Vec<&str> = bs.split(",").collect();
    let badgeVex :Vec<String> = badges.iter().map(|s| s.to_string()).collect();

    let em = getAfterEquals(msg[3].to_string());
    let emotes:Vec<&str> = em.split(",").collect();
    let emoteVex:Vec<String> = emotes.iter().map(|s| s.to_string()).collect();

    let name = getAfterEquals(msg[2].to_string());

    let user_id = getAfterEquals(msg[10].to_string());

    let msg_id = getAfterEquals(msg[4].to_string());

    let color = getAfterEquals(msg[1].to_string());

    let room_id = getAfterEquals(msg[6].to_string());

    let mut isSub = false;

    let mut isTurbo = false;

    let mut isMod = false;

    if getAfterEquals(msg[7].to_string()) == "1" {
        isSub = true;
    }

    if getAfterEquals(msg[9].to_string()) == "1" {
        isTurbo = true;
    }

    if getAfterEquals(msg[5].to_string()) == "1" {
        isMod = true;
    }

    let prMsg = MessageEvents::PrivMsg {
        display_name: name,
        user_id: user_id,
        user_type: "".to_string(),
        msg_id: msg_id,
        room_id: room_id,
        color: color,
        badges: badgeVex,
        emotes: emoteVex,
        is_mod: isMod,
        is_subscriber: isSub,
        is_turbo: isTurbo,
        message: "".to_string()
    };

    return prMsg;
}

#[allow(non_snake_case)]
fn getAfterEquals(msg:String) -> String {

    let eSplit:Vec<&str> = msg.split('=').collect();

    return eSplit[1].to_string();
}